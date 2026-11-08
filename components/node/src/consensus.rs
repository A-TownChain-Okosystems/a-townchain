// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Konsens-Runden fuer atc-node (SCR-0117).
//!
//! Ablauf einer Runde:
//! 1. Proposer wird deterministisch aus dem ValidatorSet gewaehlt
//!    (`ValidatorSet::select_proposer`, Weighted Round Robin ueber die Hoehe).
//! 2. Der Proposer bildet die Payload, der Blockhash wird **vor** dem Anhaengen
//!    berechnet (`Chain::preview_hash`) und als Proposal registriert.
//! 3. Jeder Validator signiert einen Vote ueber den vollen 32-Byte-Blockhash
//!    (secp256k1/ECDSA, domain-separiert). Kein 64-Bit-Traversal wie in
//!    `validator::ValidatorProposal` — der Vote deckt den echten Hash ab.
//! 4. Erreicht die signierte Voting-Power 2/3 der aktiven Power, ist der Block
//!    final und wird an die Kette gehaengt und im Ledger committet.
//!
//! Ehrlichkeit: Der Vote-Austausch zwischen Prozessen ist noch nicht im
//! Gossip-Wire (dort laeuft bisher nur Pull-Sync von Bloecken). Runden laufen
//! innerhalb eines Prozesses ueber die dort gehaltenen Validator-Schluessel.
//! Kein Slashing, kein Reorg, keine View-Wechsel bei Timeout.

use crate::chain::{hash_hex, Chain};
use crate::mempool::{TxEnvelope, TxLedger};
use crate::validator::{Validator, ValidatorSet, ATC_CHAIN_ID};
use secp256k1::{
    ecdsa::Signature as SecpSignature, Message as SecpMessage, PublicKey as SecpPublicKey,
    Secp256k1, SecretKey,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Domain-Trennung: Votes gelten nur in diesem Kontext (kein Replay aus
/// Proposal-, Handoff- oder Tx-Signaturen).
pub const VOTE_DOMAIN: &[u8] = b"ATC-CONSENSUS-VOTE-V1";

/// Domain-separierter Digest: DOMAIN | chain_id | height | validator_id | block_hash(32)
fn vote_digest(height: u64, validator_id: u64, block_hash: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(VOTE_DOMAIN);
    h.update(ATC_CHAIN_ID.to_be_bytes());
    h.update(height.to_be_bytes());
    h.update(validator_id.to_be_bytes());
    h.update(block_hash);
    h.finalize().into()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub height: u64,
    pub validator_id: u64,
    pub block_hash: [u8; 32],
    /// 64-Byte compact ECDSA-Signatur, hex-kodiert.
    pub signature: String,
}

impl Vote {
    pub fn create(
        height: u64,
        validator_id: u64,
        block_hash: [u8; 32],
        signing_key: &SecretKey,
    ) -> Self {
        let digest = vote_digest(height, validator_id, &block_hash);
        let signature =
            Secp256k1::new().sign_ecdsa(SecpMessage::from_digest(digest), signing_key);
        Vote {
            height,
            validator_id,
            block_hash,
            signature: hex_encode(&signature.serialize_compact()),
        }
    }

    /// Oeffentliche Verifikation gegen den im Set hinterlegten Pubkey.
    pub fn verify(&self, set: &ValidatorSet) -> Result<(), String> {
        let validator = set
            .get(self.validator_id)
            .ok_or_else(|| format!("Unbekannter Validator {}", self.validator_id))?;
        if !validator.is_active {
            return Err(format!("Validator {} ist nicht aktiv", self.validator_id));
        }
        let pk_bytes = hex_decode(&validator.pubkey)
            .ok_or_else(|| "Validator-Pubkey ist kein gueltiges Hex".to_string())?;
        let pubkey = SecpPublicKey::from_slice(&pk_bytes)
            .map_err(|_| "Validator-Pubkey ist kein gueltiger secp256k1-Punkt".to_string())?;
        let sig_bytes = hex_decode(&self.signature)
            .ok_or_else(|| "Vote-Signatur ist kein gueltiges Hex".to_string())?;
        let sig_arr: [u8; 64] = sig_bytes
            .as_slice()
            .try_into()
            .map_err(|_| "Vote-Signatur muss 64 Byte lang sein".to_string())?;
        let sig = SecpSignature::from_compact(&sig_arr)
            .map_err(|_| "Vote-Signatur ist nicht parsebar".to_string())?;
        let digest = vote_digest(self.height, self.validator_id, &self.block_hash);
        Secp256k1::verification_only()
            .verify_ecdsa(SecpMessage::from_digest(digest), &sig, &pubkey)
            .map_err(|_| "Ungueltige Vote-Signatur".to_string())
    }
}

/// Quorum-Bescheinigung fuer genau einen Block einer Hoehe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuorumCertificate {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub voters: Vec<u64>,
    pub signed_power: u64,
    pub total_power: u64,
}

impl QuorumCertificate {
    /// 2/3 der aktiven Voting-Power (aufgerundet), als kleinste ganze Zahl.
    pub fn required_power(total_power: u64) -> u64 {
        // kleinste P mit 3*P >= 2*total
        (2 * total_power).div_ceil(3)
    }

    pub fn has_quorum(&self) -> bool {
        self.total_power > 0 && self.signed_power * 3 >= self.total_power * 2
    }

    pub fn summary(&self) -> String {
        format!(
            "Hoehe {} | Hash {} | Stimmen {}/{} Power",
            self.height,
            hash_hex(&self.block_hash),
            self.signed_power,
            self.total_power
        )
    }
}

#[derive(Debug)]
pub struct RoundOutcome {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub proposer_id: u64,
    pub voter_ids: Vec<u64>,
    pub finalized: bool,
    pub reason: String,
}

/// Runden-Engine: haelt ValidatorSet, eigene Schluessel und die eingegangenen
/// Proposals/Votes.
pub struct ConsensusEngine {
    set: ValidatorSet,
    keys: BTreeMap<u64, SecretKey>,
    proposals: BTreeMap<u64, ([u8; 32], u64)>,
    votes: BTreeMap<(u64, u64), Vote>,
}

impl ConsensusEngine {
    pub fn new(set: ValidatorSet) -> Self {
        ConsensusEngine {
            set,
            keys: BTreeMap::new(),
            proposals: BTreeMap::new(),
            votes: BTreeMap::new(),
        }
    }

    /// Devnet-Set mit `n` Validatoren (Power 1) und allen Schluesseln im Prozess.
    /// Schluessel sind deterministisch aus einem Seed abgeleitet, damit Neustarts
    /// dieselben Validatoren ergeben.
    pub fn devnet(n: usize) -> Self {
        let mut set = ValidatorSet::new();
        let mut engine = ConsensusEngine::new(ValidatorSet::new());
        let n = n.max(1);
        for i in 1..=n as u64 {
            let key = devnet_key(i);
            let pubkey = SecpPublicKey::from_secret_key(&Secp256k1::new(), &key);
            set.add(Validator {
                id: i,
                pubkey: hex_encode(&pubkey.serialize()),
                voting_power: 1,
                is_active: true,
            })
            .expect("Devnet-Validator muss gueltig sein");
            engine.keys.insert(i, key);
        }
        engine.set = set;
        engine
    }

    pub fn validator_set(&self) -> &ValidatorSet {
        &self.set
    }

    pub fn required_power(&self) -> u64 {
        QuorumCertificate::required_power(self.set.total_voting_power())
    }

    pub fn holds_key(&self, validator_id: u64) -> bool {
        self.keys.contains_key(&validator_id)
    }

    /// Registriert einen Blockvorschlag. Der Proposer muss der deterministisch
    /// gewaehlte Validator dieser Hoehe sein.
    pub fn add_proposal(
        &mut self,
        height: u64,
        block_hash: [u8; 32],
        proposer_id: u64,
    ) -> Result<(), String> {
        let expected = self
            .set
            .select_proposer(height)
            .ok_or_else(|| "Keine aktiven Validatoren".to_string())?;
        if expected.id != proposer_id {
            return Err(format!(
                "Unzulaessiger Proposer fuer Hoehe {}: erwartet {}, erhalten {}",
                height, expected.id, proposer_id
            ));
        }
        if let Some((known_hash, known_proposer)) = self.proposals.get(&height) {
            if known_hash != &block_hash || known_proposer != &proposer_id {
                return Err(format!(
                    "Konflikt: fuer Hoehe {} liegt bereits ein anderer Proposal vor",
                    height
                ));
            }
            return Ok(());
        }
        self.proposals.insert(height, (block_hash, proposer_id));
        Ok(())
    }

    /// Alle lokal gehaltenen Schluessel voten fuer diesen Block.
    pub fn cast_local_votes(&self, height: u64, block_hash: [u8; 32]) -> Vec<Vote> {
        self.keys
            .iter()
            .map(|(id, key)| Vote::create(height, *id, block_hash, key))
            .collect()
    }

    /// Nimmt einen fremden Vote an: Signatur, Proposal-Bezug und Doppelstimme
    /// werden geprueft.
    pub fn add_vote(&mut self, vote: Vote) -> Result<(), String> {
        vote.verify(&self.set)?;
        if let Some(existing) = self.votes.get(&(vote.height, vote.validator_id)) {
            if existing.block_hash == vote.block_hash {
                return Ok(());
            }
            return Err(format!(
                "Equivocation: Validator {} hat fuer Hoehe {} bereits anders gestimmt",
                vote.validator_id, vote.height
            ));
        }
        let (proposed_hash, _) = self
            .proposals
            .get(&vote.height)
            .ok_or_else(|| format!("Vote ohne Proposal fuer Hoehe {}", vote.height))?;
        if proposed_hash != &vote.block_hash {
            return Err(format!(
                "Vote fuer Hoehe {} zeigt auf fremden Block",
                vote.height
            ));
        }
        self.votes.insert((vote.height, vote.validator_id), vote);
        Ok(())
    }

    /// Baut die Quorum-Bescheinigung, sobald genug Power signiert hat.
    pub fn try_finalize(&self, height: u64, block_hash: [u8; 32]) -> Option<QuorumCertificate> {
        let mut voters = Vec::new();
        let mut signed_power = 0u64;
        for ((h, validator_id), vote) in &self.votes {
            if *h != height || vote.block_hash != block_hash {
                continue;
            }
            if let Some(v) = self.set.get(*validator_id) {
                if v.is_active {
                    voters.push(*validator_id);
                    signed_power += v.voting_power;
                }
            }
        }
        let qc = QuorumCertificate {
            height,
            block_hash,
            voters,
            signed_power,
            total_power: self.set.total_voting_power(),
        };
        if qc.has_quorum() {
            Some(qc)
        } else {
            None
        }
    }

    fn clear_height(&mut self, height: u64) {
        self.proposals.remove(&height);
        self.votes.retain(|(h, _), _| *h != height);
    }

    /// Fuehrt eine komplette Runde aus: Proposal, lokale Votes, Quorum, Commit.
    ///
    /// Die Kette bleibt unveraendert, solange kein Quorum erreicht ist.
    pub fn run_round(
        &mut self,
        chain: &mut Chain,
        txs: &[TxEnvelope],
        ledger: &mut TxLedger,
    ) -> Result<RoundOutcome, String> {
        let height = chain.height() + 1;
        let proposer = self
            .set
            .select_proposer(height)
            .ok_or_else(|| "Keine aktiven Validatoren".to_string())?
            .id;

        let payload = chain.tx_payload(txs, ledger)?;
        let block_hash = chain.preview_hash(&payload);

        // Ohne eigenen Proposer-Schluessel kann dieser Prozess die Runde nicht
        // eroeffnen; ein fremder Proposal muss per add_proposal eintreffen.
        if self.holds_key(proposer) {
            self.add_proposal(height, block_hash, proposer)?;
        } else if self.proposals.get(&height).map(|(h, _)| h) != Some(&block_hash) {
            return Ok(RoundOutcome {
                height,
                block_hash,
                proposer_id: proposer,
                voter_ids: Vec::new(),
                finalized: false,
                reason: format!(
                    "Proposer {} ist nicht lokal; warte auf fremden Proposal",
                    proposer
                ),
            });
        }

        for vote in self.cast_local_votes(height, block_hash) {
            self.add_vote(vote)?;
        }

        let voter_ids: Vec<u64> = self
            .votes
            .keys()
            .filter(|(h, _)| *h == height)
            .map(|(_, id)| *id)
            .collect();

        match self.try_finalize(height, block_hash) {
            Some(qc) => {
                let committed = chain.commit_payload(&payload, txs, ledger)?;
                if committed != block_hash {
                    return Err(format!(
                        "Blockhash weicht vom Proposal ab: {} != {}",
                        hash_hex(&committed),
                        hash_hex(&block_hash)
                    ));
                }
                self.clear_height(height);
                Ok(RoundOutcome {
                    height,
                    block_hash,
                    proposer_id: proposer,
                    voter_ids,
                    finalized: true,
                    reason: qc.summary(),
                })
            }
            None => {
                let signed: u64 = voter_ids
                    .iter()
                    .filter_map(|id| self.set.get(*id))
                    .map(|v| v.voting_power)
                    .sum();
                Ok(RoundOutcome {
                    height,
                    block_hash,
                    proposer_id: proposer,
                    voter_ids,
                    finalized: false,
                    reason: format!(
                        "Quorum nicht erreicht: {} von {} benoetigter Power",
                        signed,
                        self.required_power()
                    ),
                })
            }
        }
    }
}

/// Devnet-Schluessel deterministisch aus der Validator-ID.
fn devnet_key(id: u64) -> SecretKey {
    let mut digest: [u8; 32] = Sha256::digest(format!("atc-devnet-validator-{}", id)).into();
    loop {
        if let Ok(key) = SecretKey::from_slice(&digest) {
            return key;
        }
        digest = Sha256::digest(digest).into();
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    let s = s.as_bytes();
    if s.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    for pair in s.chunks(2) {
        let hi = (pair[0] as char).to_digit(16)?;
        let lo = (pair[1] as char).to_digit(16)?;
        out.push(((hi << 4) | lo) as u8);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::Genesis;

    fn engine(n: usize) -> ConsensusEngine {
        ConsensusEngine::devnet(n)
    }

    fn kette() -> Chain {
        Chain::from_genesis(&Genesis::devnet())
    }

    fn ledger() -> TxLedger {
        TxLedger::new()
    }

    #[test]
    fn quorum_power_ist_aufgerundete_zwei_drittel() {
        assert_eq!(QuorumCertificate::required_power(3), 2);
        assert_eq!(QuorumCertificate::required_power(4), 3);
        assert_eq!(QuorumCertificate::required_power(7), 5);
        assert_eq!(QuorumCertificate::required_power(1), 1);
    }

    #[test]
    fn quatre_quorum_acceptiert_zwei_von_drei_lehnt_einen_ab() {
        let one = QuorumCertificate {
            height: 1,
            block_hash: [7u8; 32],
            voters: vec![1],
            signed_power: 1,
            total_power: 3,
        };
        let two = QuorumCertificate {
            signed_power: 2,
            voters: vec![1, 2],
            ..one.clone()
        };
        assert!(!one.has_quorum());
        assert!(two.has_quorum());
    }

    #[test]
    fn vote_signatur_deckt_den_vollen_blockhash_ab() {
        let eng = engine(3);
        let hash = [3u8; 32];
        let vote = Vote::create(1, 1, hash, &devnet_key(1));
        assert!(vote.verify(eng.validator_set()).is_ok());

        let mut manipuliert = vote.clone();
        manipuliert.block_hash = [4u8; 32];
        assert!(manipuliert.verify(eng.validator_set()).is_err());

        let mut fremder_ausweis = vote.clone();
        fremder_ausweis.validator_id = 2;
        assert!(fremder_ausweis.verify(eng.validator_set()).is_err());
    }

    #[test]
    fn unzulaessiger_proposer_wird_abgelehnt() {
        let mut eng = engine(3);
        let erlaubt = eng.validator_set().select_proposer(1).unwrap().id;
        let falsch = if erlaubt == 1 { 2 } else { 1 };
        assert!(eng.add_proposal(1, [1u8; 32], falsch).is_err());
        assert!(eng.add_proposal(1, [1u8; 32], erlaubt).is_ok());
    }

    #[test]
    fn equivocation_wird_erkannt() {
        let mut eng = engine(3);
        let proposer = eng.validator_set().select_proposer(1).unwrap().id;
        let hash = [9u8; 32];
        eng.add_proposal(1, hash, proposer).unwrap();
        let vote = Vote::create(1, 2, hash, &devnet_key(2));
        eng.add_vote(vote.clone()).unwrap();
        eng.add_vote(vote).expect("gleicher Vote ist idempotent");

        let anderer_hash = [8u8; 32];
        let mut wechsel = Vote::create(1, 2, anderer_hash, &devnet_key(2));
        wechsel.height = 1;
        let fehler = eng.add_vote(wechsel).unwrap_err();
        assert!(fehler.contains("Equivocation"), "unerwartet: {fehler}");
    }

    #[test]
    fn runde_finalisiert_und_haengt_genau_einen_block_an() {
        let mut eng = engine(3);
        let mut chain = kette();
        let mut led = ledger();
        let hoehe_vorher = chain.height();

        let out = eng.run_round(&mut chain, &[], &mut led).unwrap();
        assert!(out.finalized, "Runde nicht finalisiert: {}", out.reason);
        assert_eq!(out.height, hoehe_vorher + 1);
        assert_eq!(chain.height(), hoehe_vorher + 1);
        assert_eq!(chain.best_hash(), out.block_hash);
        assert_eq!(out.voter_ids.len(), 3);
    }

    #[test]
    fn mehrere_runden_rotieren_den_proposer() {
        let mut eng = engine(3);
        let mut chain = kette();
        let mut led = ledger();
        let mut proposer = Vec::new();
        for _ in 0..4 {
            let out = eng.run_round(&mut chain, &[], &mut led).unwrap();
            assert!(out.finalized);
            proposer.push(out.proposer_id);
        }
        assert_eq!(chain.height(), 4);
        assert!(
            proposer.iter().any(|p| *p != proposer[0]),
            "Proposer blieb konstant: {:?}",
            proposer
        );
    }

    #[test]
    fn ohne_quorum_bleibt_die_kette_unveraendert() {
        // Set mit 3 Validatoren, aber nur der Proposer haelt einen Schluessel:
        // 1 von 3 Power, also kein Quorum.
        let voll = engine(3);
        let proposer = voll.validator_set().select_proposer(1).unwrap().id;
        let mut eng = ConsensusEngine::new(voll.validator_set().clone());
        eng.keys.insert(proposer, devnet_key(proposer));
        let mut chain = kette();
        let mut led = ledger();
        let vorher = chain.best_hash();

        let out = eng.run_round(&mut chain, &[], &mut led).unwrap();
        assert!(!out.finalized, "Runde ohne Quorum darf nicht finalisieren");
        assert_eq!(chain.height(), 0);
        assert_eq!(chain.best_hash(), vorher);
    }

    #[test]
    fn fremder_proposal_mit_falschem_hash_blockiert_die_runde() {
        let mut eng = engine(3);
        let mut chain = kette();
        let mut led = ledger();
        let height = chain.height() + 1;
        let proposer = eng.validator_set().select_proposer(height).unwrap().id;
        // Proposal fuer einen anderen Block registrieren als den baubaren.
        eng.add_proposal(height, [42u8; 32], proposer).unwrap();
        let fehler = eng.run_round(&mut chain, &[], &mut led).unwrap_err();
        assert!(fehler.contains("Konflikt"), "unerwartet: {fehler}");
        assert_eq!(chain.height(), 0);
    }
}
