//! Repository-Registry-Client (Phase 1): lädt `repositories.yaml` der atc-standards-Registry.
//!
//! Liefert die Fleet-Metadaten für G0/G1 (Identity/Scope) und Policy-Bewertungen.
//! Nur lesend; unbekannte Zusatz-Keys werden toleriert (Forward-Kompatibilität),
//! Pflicht-Keys fehlgeschlagen = Fehler.

use crate::flow::parse_flow_map;
use atc_core::{CoreError, RepositoryId, Result};
use std::path::Path;

/// Ein Repository-Registry-Eintrag (repositories.yaml).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoEntry {
    pub name: RepositoryId,
    pub id: String,
    pub tier: String,
    pub domain: String,
    pub layer: String,
    pub criticality: String,
    pub security_class: String,
    pub maturity_class: String,
    pub canonical: bool,
    pub exempt: bool,
}

impl RepoEntry {
    fn from_pairs(pairs: Vec<(String, String)>) -> Result<Self> {
        let get = |k: &str| -> Result<&str> {
            pairs
                .iter()
                .find(|(pk, _)| pk == k)
                .map(|(_, v)| v.as_str())
                .ok_or_else(|| CoreError::ConfigInvalid(format!("missing key '{k}'")))
        };
        let bool_of = |raw: &str, field: &str, id: &str| -> Result<bool> {
            match raw {
                "true" => Ok(true),
                "false" => Ok(false),
                other => Err(CoreError::ConfigInvalid(format!(
                    "repo {id}: {field} must be true/false, got '{other}'"
                ))),
            }
        };
        let name = RepositoryId::new(get("name")?)?;
        let id = get("id")?.to_string();
        let canonical = bool_of(get("canonical")?, "canonical", &id)?;
        let exempt = bool_of(get("exempt")?, "exempt", &id)?;
        Ok(Self {
            name,
            id,
            tier: get("tier")?.to_string(),
            domain: get("domain")?.to_string(),
            layer: get("layer")?.to_string(),
            criticality: get("criticality")?.to_string(),
            security_class: get("security_class")?.to_string(),
            maturity_class: get("maturity_class")?.to_string(),
            canonical,
            exempt,
        })
    }
}

/// Die geladene Repository-Registry.
#[derive(Debug, Clone, Default)]
pub struct RepoRegistry {
    entries: Vec<RepoEntry>,
}

impl RepoRegistry {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path).map_err(|e| {
            CoreError::ConfigInvalid(format!("cannot read {}: {e}", path.display()))
        })?;
        Self::parse(&raw)
    }

    /// Parst den `repositories:`-Abschnitt.
    pub fn parse(raw: &str) -> Result<Self> {
        let mut entries = Vec::new();
        let mut in_section = false;
        for line in raw.lines() {
            let t = line.trim();
            if t.starts_with("repositories:") {
                in_section = true;
                continue;
            }
            if !in_section {
                continue;
            }
            if !t.is_empty()
                && !t.starts_with('#')
                && !line.starts_with(' ')
                && !line.starts_with('-')
                && !line.starts_with('\t')
            {
                in_section = false;
                continue;
            }
            if line.trim_start().starts_with("- {") {
                let map_src = extract_flow_map(line)?;
                entries.push(RepoEntry::from_pairs(parse_flow_map(&map_src)?)?);
            }
        }
        if entries.is_empty() {
            return Err(CoreError::ConfigInvalid(
                "repository registry: no entries found (empty or malformed)".into(),
            ));
        }
        let mut names = std::collections::BTreeSet::new();
        let mut ids = std::collections::BTreeSet::new();
        for e in &entries {
            if !names.insert(e.name.as_str()) {
                return Err(CoreError::InvalidId(format!(
                    "duplicate repository name: {}",
                    e.name.as_str()
                )));
            }
            if !ids.insert(e.id.clone()) {
                return Err(CoreError::InvalidId(format!(
                    "duplicate repository id: {}",
                    e.id
                )));
            }
        }
        Ok(Self { entries })
    }

    /// Fail-closed Auflösung nach Name.
    pub fn resolve_by_name(&self, name: &str) -> Result<&RepoEntry> {
        self.entries
            .iter()
            .find(|e| e.name.as_str() == name)
            .ok_or_else(|| CoreError::Unsupported(format!("repository not registered: '{name}'")))
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Governance-relevante Repos (nicht exempt) — deterministisch in Registry-Ordnung.
    pub fn governed(&self) -> impl Iterator<Item = &RepoEntry> {
        self.entries.iter().filter(|e| !e.exempt)
    }
}

/// Extrahiert `{...}` inklusive Klammern; Kommentare dahinter fallen weg.
/// Fail-closed gegen nicht terminierte Flow-Maps.
fn extract_flow_map(line: &str) -> Result<String> {
    let start = line
        .find('{')
        .ok_or_else(|| CoreError::ConfigInvalid("flow map without '{'".into()))?;
    let end = line[start..]
        .find('}')
        .ok_or_else(|| CoreError::ConfigInvalid("unterminated flow map".into()))?
        + start;
    Ok(line[start..=end].to_string())
}

impl std::str::FromStr for RepoRegistry {
    type Err = CoreError;

    fn from_str(raw: &str) -> std::result::Result<Self, Self::Err> {
        RepoRegistry::parse(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
schema_version: "1.0.0"
organization: A-TownChain-Okosystems
count: 2

repositories:
  - {name: atc-engineering, tier: T0, profile: atc-engineering, id: ATC-REPO-ENG-001, domain: engineering_governance, layer: L7, criticality: C2, security_class: S2, maturity_class: C, canonical: true, evidence: .atc/evidence/evidence.yaml, exempt: false}
  - {name: atc-standards, tier: T0, profile: atc-standards, id: ATC-REPO-GOV-001, domain: governance, layer: L7, criticality: C1, security_class: S3, maturity_class: A, canonical: true, evidence: .atc/evidence/evidence.yaml, exempt: false}

layer_taxonomy_draft:
  L7: integration
"#;

    #[test]
    fn parses_sample() {
        let r = SAMPLE.parse::<RepoRegistry>().unwrap();
        assert_eq!(r.count(), 2);
        assert_eq!(r.governed().count(), 2);
        let e = r.resolve_by_name("atc-engineering").unwrap();
        assert_eq!(e.id, "ATC-REPO-ENG-001");
        assert_eq!(e.criticality, "C2");
    }

    #[test]
    fn fails_closed_on_unknown_repo() {
        let r = SAMPLE.parse::<RepoRegistry>().unwrap();
        assert!(r.resolve_by_name("demo-repository").is_err());
    }

    #[test]
    fn fails_closed_on_duplicate_name() {
        let bad = SAMPLE.replace("name: atc-standards", "name: atc-engineering");
        assert!(bad.parse::<RepoRegistry>().is_err());
    }

    #[test]
    fn fails_closed_on_missing_key() {
        let bad = SAMPLE.replace(", tier: T0,", ", ");
        assert!(bad.parse::<RepoRegistry>().is_err());
    }

    #[test]
    fn fails_closed_on_bad_bool() {
        let bad = SAMPLE.replacen("canonical: true", "canonical: maybe", 1);
        assert!(bad.parse::<RepoRegistry>().is_err());
    }

    #[test]
    fn exempt_repos_are_not_governed() {
        let bad = SAMPLE.replacen("exempt: false}", "exempt: true}", 1);
        let r = bad.parse::<RepoRegistry>().unwrap();
        assert_eq!(r.governed().count(), 1);
    }
}
