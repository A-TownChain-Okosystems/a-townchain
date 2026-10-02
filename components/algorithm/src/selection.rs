// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Hybrid-Selektion (ATC-CONSENSUS-304, MVP): Stake-gewichtete,
//! deterministische Proposer-Wahl je Slot.

use crate::hash::atc_hash;

pub type Stake = u128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validator {
    pub id: u64,
    pub stake: Stake,
}

/// Deterministische, Stake-gewichtete Wahl. None bei leerer/Null-Stake-Menge.
/// Returns None if the total stake cannot be represented or accumulation overflows.
pub fn select_proposer(validators: &[Validator], slot: u64) -> Option<Validator> {
    if validators.is_empty() {
        return None;
    }

    let total = validators
        .iter()
        .try_fold(0u128, |total, validator| total.checked_add(validator.stake))?;

    if total == 0 {
        return None;
    }

    let digest = atc_hash(&slot.to_le_bytes());
    let ticket = u128::from_le_bytes(digest[..16].try_into().ok()?) % total;
    let mut acc = 0u128;
    for v in validators {
        acc = acc.checked_add(v.stake)?;
        if ticket < acc {
            return Some(v.clone());
        }
    }
    validators.last().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leer_und_null_stake() {
        assert!(select_proposer(&[], 1).is_none());
        assert!(select_proposer(&[Validator { id: 1, stake: 0 }], 1).is_none());
    }

    #[test]
    fn deterministisch_pro_slot() {
        let vs = vec![Validator { id: 1, stake: 30 }, Validator { id: 2, stake: 70 }];
        let a = select_proposer(&vs, 5).unwrap();
        let b = select_proposer(&vs, 5).unwrap();
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn jede_wahl_ist_valide() {
        let vs = vec![Validator { id: 1, stake: 10 }, Validator { id: 2, stake: 10 }];
        for slot in 0..100 {
            let sel = select_proposer(&vs, slot).unwrap();
            assert!(sel.id == 1 || sel.id == 2);
        }
    }

    #[test]
    fn u128_stake_boundary_is_supported() {
        let vs = vec![
            Validator { id: 1, stake: u128::MAX - 1 },
            Validator { id: 2, stake: 1 },
        ];
        assert!(select_proposer(&vs, 0).is_some());
    }

    #[test]
    fn total_stake_overflow_fails_closed() {
        let vs = vec![
            Validator { id: 1, stake: u128::MAX },
            Validator { id: 2, stake: 1 },
        ];
        assert!(select_proposer(&vs, 0).is_none());
    }
}
