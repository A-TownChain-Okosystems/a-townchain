//! Kanonische IDs der Engineering-Domain (SPEC-001 §19, §21).
//!
//! IDs sind newtypes mit fail-closed Validierung: ungültige Eingaben
//! werden abgelehnt, nie normalisiert.

use crate::error::{CoreError, Result};
use std::fmt;

/// Repository-ID: kebab-case plus Punkt, `[a-z0-9.-]`, 1..=100 Zeichen
/// (z. B. `atc-node`, Spezial-Repo `.github`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RepositoryId(String);

/// Standard-ID: Großbuchstaben/Ziffern/Bindestrich (z. B. `ATC-STD-000`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StandardId(String);

/// Evidence-ID: `EVD-YYYY-NNNNNN` (z. B. `EVD-2026-000001`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EvidenceId(String);

impl RepositoryId {
    pub fn new(raw: &str) -> Result<Self> {
        let s = raw.trim();
        if s.is_empty() {
            return Err(CoreError::InvalidId("repository id: empty".into()));
        }
        if s.len() > 100 {
            return Err(CoreError::InvalidId("repository id: >100 chars".into()));
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.')
        {
            return Err(CoreError::InvalidId(format!(
                "repository id '{s}': only [a-z0-9.-] allowed"
            )));
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl StandardId {
    pub fn new(raw: &str) -> Result<Self> {
        let s = raw.trim();
        if s.is_empty() {
            return Err(CoreError::InvalidId("standard id: empty".into()));
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(CoreError::InvalidId(format!(
                "standard id '{s}': only [A-Z0-9-] allowed"
            )));
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl EvidenceId {
    /// `EVD-2026-000001` — Jahreszahl 4-stellig, Sequenz 1..=6 Stellen.
    pub fn new(raw: &str) -> Result<Self> {
        let s = raw.trim();
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 || parts[0] != "EVD" {
            return Err(CoreError::InvalidId(format!(
                "evidence id '{s}': expected EVD-YYYY-NNNNNN"
            )));
        }
        let year_ok = parts[1].len() == 4
            && parts[1].chars().all(|c| c.is_ascii_digit())
            && parts[1]
                .parse::<u16>()
                .map(|y| (2000..=9999).contains(&y))
                .unwrap_or(false);
        let seq_ok = !parts[2].is_empty()
            && parts[2].len() <= 6
            && parts[2].chars().all(|c| c.is_ascii_digit());
        if !(year_ok && seq_ok) {
            return Err(CoreError::InvalidId(format!(
                "evidence id '{s}': expected EVD-YYYY-NNNNNN"
            )));
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

macro_rules! impl_display {
    ($t:ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}
impl_display!(RepositoryId);
impl_display!(StandardId);
impl_display!(EvidenceId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_id_accepts_kebab_case() {
        assert_eq!(RepositoryId::new("atc-node").unwrap().as_str(), "atc-node");
    }

    #[test]
    fn repository_id_rejects_invalid() {
        assert!(RepositoryId::new("").is_err());
        assert!(RepositoryId::new("Atc_Node").is_err());
        assert!(RepositoryId::new("atc node").is_err());
        assert!(RepositoryId::new("atc_node").is_err());
    }

    #[test]
    fn repository_id_accepts_dot_repos() {
        assert_eq!(RepositoryId::new(".github").unwrap().as_str(), ".github");
    }

    #[test]
    fn standard_id_accepts_atc_format() {
        assert_eq!(
            StandardId::new("ATC-STD-000").unwrap().as_str(),
            "ATC-STD-000"
        );
        assert!(StandardId::new("atc-std-000").is_err());
    }

    #[test]
    fn evidence_id_format() {
        assert!(EvidenceId::new("EVD-2026-000001").is_ok());
        assert!(EvidenceId::new("EVD-26-1").is_err());
        assert!(EvidenceId::new("EVDX-2026-000001").is_err());
        assert!(EvidenceId::new("EVD-2026-").is_err());
    }
}
