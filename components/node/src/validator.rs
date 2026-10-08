// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Validator-Betrieb und Proposal-Verifikation fuer atc-node (SCR-0100).
//! Verfiziert Proposer-Berechtigung und Signaturen.
//! Konsens-Semantik bleibt kanonisch in atc-algorithm (ATC-STD-202 §3a).

use atc_algorithm::hash::atc_hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validator {
    pub id: u64,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorProposal {
    pub height: u64,
    pub proposer_id: u64,
    pub block_hash: u64,
    pub signature: String,
}

impl ValidatorProposal {
    pub fn compute_signature(height: u64, proposer_id: u64, block_hash: u64, secret_key: &str) -> String {
        let payload = format!("PROPOSE:{}:{}:{}:{}", height, proposer_id, block_hash, secret_key);
        let digest = atc_hash(payload.as_bytes());
        hex::encode(digest)
    }

    pub fn create(height: u64, proposer: &Validator, block_hash: u64, secret_key: &str) -> Result<Self, String> {
        if !proposer.is_active {
            return Err("Inaktiver Validator kann keine Proposals erstellen".to_string());
        }
        let signature = Self::compute_signature(height, proposer.id, block_hash, secret_key);
        Ok(ValidatorProposal {
            height,
            proposer_id: proposer.id,
            block_hash,
            signature,
        })
    }

    pub fn verify(&self, val_set: &ValidatorSet, secret_key: &str) -> Result<(), String> {
        let expected_proposer = val_set
            .select_proposer(self.height)
            .ok_or_else(|| "Keine aktiven Validatoren im ValidatorSet".to_string())?;

        if expected_proposer.id != self.proposer_id {
            return Err(format!(
                "Falscher Proposer fuer Hoehe {}: Erwartet ID {}, erhalten ID {}",
                self.height, expected_proposer.id, self.proposer_id
            ));
        }

        let expected_sig = Self::compute_signature(self.height, self.proposer_id, self.block_hash, secret_key);
        if self.signature != expected_sig {
            return Err("Ungueltige Proposal-Signatur".to_string());
        }

        Ok(())
    }
}

mod hex {
    pub fn encode(bytes: [u8; 32]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(64);
        for &b in &bytes {
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0x0f) as usize] as char);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_val_set() -> ValidatorSet {
        let mut set = ValidatorSet::new();
        set.add(Validator { id: 1, pubkey: "pub1".into(), voting_power: 10, is_active: true }).unwrap();
        set.add(Validator { id: 2, pubkey: "pub2".into(), voting_power: 20, is_active: true }).unwrap();
        set
    }

    #[test]
    fn validator_set_add_und_duplikat() {
        let mut set = ValidatorSet::new();
        assert!(set.add(Validator { id: 1, pubkey: "p1".into(), voting_power: 5, is_active: true }).is_ok());
        assert!(set.add(Validator { id: 1, pubkey: "p1".into(), voting_power: 5, is_active: true }).is_err());
    }

    #[test]
    fn proposer_auswahl_deterministisch() {
        let set = setup_val_set();
        let p0 = set.select_proposer(0).unwrap();
        let p1 = set.select_proposer(1).unwrap();
        assert_eq!(p0.id, 1);
        assert_eq!(p1.id, 1);
        let p10 = set.select_proposer(10).unwrap();
        assert_eq!(p10.id, 2);
    }

    #[test]
    fn proposal_erstellung_und_verifikation() {
        let set = setup_val_set();
        let proposer = set.select_proposer(0).unwrap().clone();
        let secret = "sec123";

        let prop = ValidatorProposal::create(0, &proposer, 98765, secret).unwrap();
        assert!(prop.verify(&set, secret).is_ok());
    }

    #[test]
    fn proposal_falscher_proposer_abgelehnt() {
        let set = setup_val_set();
        let wrong_proposer = set.get(2).unwrap().clone();
        let secret = "sec123";

        // Hoehe 0 verlangt ID 1, aber wrong_proposer ist ID 2
        let prop = ValidatorProposal::create(0, &wrong_proposer, 98765, secret).unwrap();
        assert!(prop.verify(&set, secret).is_err());
    }

    #[test]
    fn proposal_inaktiver_validator_abgelehnt() {
        let mut set = ValidatorSet::new();
        let v = Validator { id: 1, pubkey: "p1".into(), voting_power: 10, is_active: false };
        let secret = "sec123";

        assert!(ValidatorProposal::create(0, &v, 1234, secret).is_err());
    }
}
