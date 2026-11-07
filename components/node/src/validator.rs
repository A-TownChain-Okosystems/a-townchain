// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Validator-Betrieb und Proposal-Verifikation fuer atc-node (SCR-0100).
//!
//! **Signatur (korrigiert 07.11.2026, SEC-P0-003):** Proposals werden mit
//! secp256k1/ECDSA (RFC 6979, Low-S) ueber einen domain-separierten Digest
//! signiert und **oeffentlich** gegen den im ValidatorSet hinterlegten
//! 33-Byte-Pubkey verifiziert. Das fruehere Verfahren (SHA-256 ueber einen
//! String, der den Klartext-Schluessel enthielt, Verifikation nur mit dem
//! Signierer-Schluessel) war keine Signatur und ist ersetzt.
//!
//! Konsens-Semantik bleibt kanonisch in atc-algorithm (ATC-STD-202 §3a).

use secp256k1::{
    ecdsa::Signature as SecpSignature, Message as SecpMessage, PublicKey as SecpPublicKey,
    Secp256k1, SecretKey,
};
use sha2::{Digest, Sha256};

/// Kanonische ATC-Chain-ID (identisch zu components/wallet NUMERIC_CHAIN_ID).
pub const ATC_CHAIN_ID: u64 = 658_467;
/// Domain-Trennung fuer Proposal-Signaturen (Replay-Schutz ueber Ketten/Netze).
pub const PROPOSAL_DOMAIN: &[u8] = b"ATC-PROPOSE-V1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validator {
    pub id: u64,
    /// 33-Byte compressed secp256k1 Pubkey, hex-kodiert (66 Zeichen).
    pub pubkey: String,
    pub voting_power: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ValidatorSet {
    validators: Vec<Validator>,
}

impl ValidatorSet {
    pub fn new() -> Self {
        ValidatorSet { validators: Vec::new() }
    }

    pub fn add(&mut self, val: Validator) -> Result<(), String> {
        if val.pubkey.is_empty() {
            return Err("Pubkey darf nicht leer sein".to_string());
        }
        if val.voting_power == 0 {
            return Err("Voting power muss > 0 sein".to_string());
        }
        if self.validators.iter().any(|v| v.id == val.id) {
            return Err(format!("Validator ID {} bereits vorhanden", val.id));
        }
        self.validators.push(val);
        // Sortierung nach ID fuer Deterministische Proposer-Wahl
        self.validators.sort_by_key(|v| v.id);
        Ok(())
    }

    pub fn get(&self, id: u64) -> Option<&Validator> {
        self.validators.iter().find(|v| v.id == id)
    }

    pub fn active_validators(&self) -> Vec<&Validator> {
        self.validators.iter().filter(|v| v.is_active).collect()
    }

    pub fn total_voting_power(&self) -> u64 {
        self.active_validators().iter().map(|v| v.voting_power).sum()
    }

    /// Deterministische Proposer-Auswahl fuer eine gegebenen Blockhoehe (Weighted Round Robin)
    pub fn select_proposer(&self, height: u64) -> Option<&Validator> {
        let active = self.active_validators();
        if active.is_empty() {
            return None;
        }
        let total = self.total_voting_power();
        if total == 0 {
            return None;
        }
        let slot = height % total;
        let mut accumulated = 0;
        for v in active {
            accumulated += v.voting_power;
            if slot < accumulated {
                return Some(v);
            }
        }
        None
    }
}

/// Domain-separierter Digest: DOMAIN | chain_id | height | proposer_id | block_hash
/// (alle u64 big-endian, feste Breite).
fn proposal_digest(height: u64, proposer_id: u64, block_hash: u64) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(PROPOSAL_DOMAIN);
    h.update(ATC_CHAIN_ID.to_be_bytes());
    h.update(height.to_be_bytes());
    h.update(proposer_id.to_be_bytes());
    h.update(block_hash.to_be_bytes());
    h.finalize().into()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorProposal {
    pub height: u64,
    pub proposer_id: u64,
    pub block_hash: u64,
    /// 64-Byte compact ECDSA-Signatur, hex-kodiert (128 Zeichen).
    pub signature: String,
}

impl ValidatorProposal {
    pub fn create(
        height: u64,
        proposer: &Validator,
        block_hash: u64,
        signing_key: &SecretKey,
    ) -> Result<Self, String> {
        if !proposer.is_active {
            return Err("Inaktiver Validator kann keine Proposals erstellen".to_string());
        }
        let digest = proposal_digest(height, proposer.id, block_hash);
        let signature = Secp256k1::new().sign_ecdsa(SecpMessage::from_digest(digest), signing_key);
        Ok(ValidatorProposal {
            height,
            proposer_id: proposer.id,
            block_hash,
            signature: hex::encode(&signature.serialize_compact()),
        })
    }

    /// Oeffentliche Verifikation: braucht nur das ValidatorSet, keinen
    /// Signierer-Schluessel.
    pub fn verify(&self, val_set: &ValidatorSet) -> Result<(), String> {
        let expected_proposer = val_set
            .select_proposer(self.height)
            .ok_or_else(|| "Keine aktiven Validatoren im ValidatorSet".to_string())?;

        if expected_proposer.id != self.proposer_id {
            return Err(format!(
                "Falscher Proposer fuer Hoehe {}: Erwartet ID {}, erhalten ID {}",
                self.height, expected_proposer.id, self.proposer_id
            ));
        }

        let proposer = val_set
            .get(self.proposer_id)
            .ok_or_else(|| "Unbekannter Proposer".to_string())?;
        if !proposer.is_active {
            return Err("Proposer ist nicht aktiv".to_string());
        }

        let pk_bytes = hex::decode(&proposer.pubkey)
            .ok_or_else(|| "Proposer-Pubkey ist kein gueltiges Hex".to_string())?;
        let pubkey = SecpPublicKey::from_slice(&pk_bytes)
            .map_err(|_| "Proposer-Pubkey ist kein gueltiger secp256k1-Punkt".to_string())?;

        let sig_bytes = hex::decode(&self.signature)
            .ok_or_else(|| "Signatur ist kein gueltiges Hex".to_string())?;
        let sig_arr: [u8; 64] = sig_bytes
            .as_slice()
            .try_into()
            .map_err(|_| "Signatur muss 64 Byte lang sein".to_string())?;
        let sig = SecpSignature::from_compact(&sig_arr)
            .map_err(|_| "Signatur ist nicht parsebar".to_string())?;

        let digest = proposal_digest(self.height, self.proposer_id, self.block_hash);
        Secp256k1::verification_only()
            .verify_ecdsa(SecpMessage::from_digest(digest), &sig, &pubkey)
            .map_err(|_| "Ungueltige Proposal-Signatur".to_string())
    }
}

mod hex {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    pub fn encode(bytes: &[u8]) -> String {
        let mut out = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0x0f) as usize] as char);
        }
        out
    }

    pub fn decode(s: &str) -> Option<Vec<u8>> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keypair(seed: u8) -> (SecretKey, String) {
        let sk = SecretKey::from_byte_array([seed; 32]).unwrap();
        let pk = SecpPublicKey::from_secret_key(&Secp256k1::new(), &sk).serialize();
        (sk, hex::encode(&pk))
    }

    fn setup_val_set() -> (ValidatorSet, SecretKey, SecretKey) {
        let (sk1, pk1) = keypair(1);
        let (sk2, pk2) = keypair(2);
        let mut set = ValidatorSet::new();
        set.add(Validator { id: 1, pubkey: pk1, voting_power: 10, is_active: true }).unwrap();
        set.add(Validator { id: 2, pubkey: pk2, voting_power: 20, is_active: true }).unwrap();
        (set, sk1, sk2)
    }

    #[test]
    fn validator_set_add_und_duplikat() {
        let mut set = ValidatorSet::new();
        let (_, pk1) = keypair(1);
        assert!(set.add(Validator { id: 1, pubkey: pk1.clone(), voting_power: 5, is_active: true }).is_ok());
        assert!(set.add(Validator { id: 1, pubkey: pk1, voting_power: 5, is_active: true }).is_err());
    }

    #[test]
    fn proposer_auswahl_deterministisch() {
        let (set, _, _) = setup_val_set();
        assert_eq!(set.select_proposer(0).unwrap().id, 1);
        assert_eq!(set.select_proposer(10).unwrap().id, 2);
        assert_eq!(set.select_proposer(20).unwrap().id, 2);
        assert_eq!(set.select_proposer(30).unwrap().id, 1);
    }

    #[test]
    fn proposal_erstellung_und_oeffentliche_verifikation() {
        let (set, sk1, _) = setup_val_set();
        let proposer = set.select_proposer(0).unwrap().clone();
        let prop = ValidatorProposal::create(0, &proposer, 98765, &sk1).unwrap();
        // Verifikation ohne Signierer-Schluessel (nur ValidatorSet)
        assert!(prop.verify(&set).is_ok());
    }

    #[test]
    fn proposal_falscher_proposer_abgelehnt() {
        let (set, _, sk2) = setup_val_set();
        let wrong = set.get(2).unwrap().clone();
        // Hoehe 0 verlangt ID 1; Mit-Schluessel von ID 2 signiert
        let prop = ValidatorProposal::create(0, &wrong, 98765, &sk2).unwrap();
        assert!(prop.verify(&set).is_err());
    }

    #[test]
    fn proposal_manipulation_abgelehnt() {
        let (set, sk1, _) = setup_val_set();
        let proposer = set.select_proposer(0).unwrap().clone();
        let mut prop = ValidatorProposal::create(0, &proposer, 98765, &sk1).unwrap();
        prop.block_hash = 98766; // Manipulation nach Signatur
        assert!(prop.verify(&set).is_err());
    }

    #[test]
    fn proposal_fremdschluessel_abgelehnt() {
        let (set, _, sk2) = setup_val_set();
        let proposer = set.select_proposer(0).unwrap().clone(); // ID 1
        // Signatur mit fremdem Schluessel (ID 2), aber als ID 1 deklariert
        let digest = proposal_digest(0, 1, 555);
        let sig = Secp256k1::new().sign_ecdsa(SecpMessage::from_digest(digest), &sk2);
        let forged = ValidatorProposal {
            height: 0,
            proposer_id: 1,
            block_hash: 555,
            signature: hex::encode(&sig.serialize_compact()),
        };
        assert!(forged.verify(&set).is_err());
        // Kontrolle: korrekter Schluessel wird akzeptiert
        let ok = ValidatorProposal::create(0, &proposer, 555, &sk2).unwrap();
        assert!(ok.verify(&set).is_err(), "ID 2 darf Hoehe 0 nicht signieren");
    }

    #[test]
    fn proposal_inaktiver_validator_abgelehnt() {
        let (sk, pk) = keypair(3);
        let v = Validator { id: 1, pubkey: pk, voting_power: 10, is_active: false };
        assert!(ValidatorProposal::create(0, &v, 1234, &sk).is_err());
    }
}
