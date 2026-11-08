//! Derived State (SPEC-001 §4: "No Evidence, No Trust").
//!
//! Declared state ist nicht vertrauenswürdig. Der Derived State wird aus
//! Evidence abgeleitet und kennt keine Sprünge nach oben ohne Validierung.

use crate::error::CoreError;

/// Zustandsleiter des Derived State.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DerivedState {
    NotReady,
    Development,
    Validated,
    TestnetReady,
    ProductionReady,
}

impl DerivedState {
    pub const ALL: [DerivedState; 5] = [
        DerivedState::NotReady,
        DerivedState::Development,
        DerivedState::Validated,
        DerivedState::TestnetReady,
        DerivedState::ProductionReady,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            DerivedState::NotReady => "NOT_READY",
            DerivedState::Development => "DEVELOPMENT",
            DerivedState::Validated => "VALIDATED",
            DerivedState::TestnetReady => "TESTNET_READY",
            DerivedState::ProductionReady => "PRODUCTION_READY",
        }
    }

    /// Ein Derived State ist nur erreichbar, wenn der Vorgänger erreicht war.
    /// Sprünge nach oben werden abgelehnt (Ord-basiert, deterministisch).
    pub fn can_advance_to(&self, target: DerivedState) -> bool {
        self <= &target
    }
}

impl std::str::FromStr for DerivedState {
    type Err = CoreError;

    /// Fail-closed Parsing: unbekannte Zustände werden abgelehnt.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        for st in Self::ALL {
            if st.as_str() == s {
                return Ok(st);
            }
        }
        Err(CoreError::InvalidState(format!(
            "unknown derived state: '{s}'"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_all_states() {
        for st in DerivedState::ALL {
            assert_eq!(st.as_str().parse::<DerivedState>().unwrap(), st);
        }
    }

    #[test]
    fn unknown_state_fails_closed() {
        assert!("PRODUCTION_READY!!!".parse::<DerivedState>().is_err());
        assert!("".parse::<DerivedState>().is_err());
    }

    #[test]
    fn advance_rules() {
        assert!(DerivedState::Development.can_advance_to(DerivedState::Validated));
        assert!(!DerivedState::ProductionReady.can_advance_to(DerivedState::NotReady));
        assert!(DerivedState::NotReady.can_advance_to(DerivedState::NotReady));
    }
}
