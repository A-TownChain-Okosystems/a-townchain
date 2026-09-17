// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Hybrid-Selektion (ATC-CONSENSUS-304, MVP): Stake-gewichtete,
//! deterministische Proposer-Wahl je Slot.

use crate::poh::fnv1a;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validator {
    pub id: u64,
    pub stake: u64,
}

/// Deterministische, Stake-gewichtete Wahl. None bei leerer/Null-Stake-Menge.
pub fn select_proposer(validators: &[Validator], slot: u64) -> Option<Validator> {
    let total: u64 = validators.iter().map(|v| v.stake).sum();
    if validators.is_empty() || total == 0 {
        return None;
    }
    let ticket = fnv1a(&slot.to_le_bytes()) % total;
    let mut acc: u64 = 0;
    for v in validators {
        acc += v.stake;
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
}
