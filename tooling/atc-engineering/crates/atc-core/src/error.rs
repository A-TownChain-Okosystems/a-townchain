//! Fehler-Modell der Control Plane (SPEC-001 §34: fail closed).
//!
//! Jede unbekannte oder ungültige Eingabe führt zu einem Fehler — nie zu einem
//! stillschweigenden Fallback. "BLOCK, nicht ALLOW" ist im Typsystem verankert:
//! es existiert kein Konstrukt, das einen Fehler in einen ALLOW-Zustand verwandelt.

use std::fmt;

/// Kanonische Fehler der Engineering-Domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// Ungültige oder nicht konforme ID.
    InvalidId(String),
    /// Unbekannter oder ungültiger Zustand.
    InvalidState(String),
    /// Ungültige Konfiguration.
    ConfigInvalid(String),
    /// Nicht unterstützte Operation, Version oder Standard.
    Unsupported(String),
    /// Verpflichtende Evidence fehlt (No Evidence, No Trust).
    MissingEvidence(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::InvalidId(m) => write!(f, "invalid id: {m}"),
            CoreError::InvalidState(m) => write!(f, "invalid state: {m}"),
            CoreError::ConfigInvalid(m) => write!(f, "invalid config: {m}"),
            CoreError::Unsupported(m) => write!(f, "unsupported: {m}"),
            CoreError::MissingEvidence(m) => write!(f, "missing evidence: {m}"),
        }
    }
}

impl std::error::Error for CoreError {}

/// Result-Alias der Domain.
pub type Result<T> = std::result::Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_messages_are_stable() {
        assert_eq!(
            CoreError::InvalidId("empty".into()).to_string(),
            "invalid id: empty"
        );
        assert_eq!(
            CoreError::MissingEvidence("release".into()).to_string(),
            "missing evidence: release"
        );
    }

    #[test]
    fn implements_std_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}
        assert_error(&CoreError::Unsupported("x".into()));
    }
}
