//! Standards-Registry-Client (Phase 1): lädt `standards.yaml` der atc-standards-Registry.
//!
//! Lifecycle gemäß registry/lifecycle.yaml: idea → proposed → draft → review →
//! candidate → approved → stable → deprecated → retired. Fail-closed gegen
//! unbekannte Status. `standards.yaml`-Einträge sind Inline-Flow-Maps.

use crate::flow::parse_flow_map;
use atc_core::{is_semver, CoreError, Result, StandardId};
use std::collections::BTreeMap;
use std::path::Path;

/// Lifecycle-Status eines Standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Idea,
    Proposed,
    Draft,
    Review,
    Candidate,
    Approved,
    Stable,
    Deprecated,
    Retired,
}

impl Status {
    pub const ALL: [Status; 9] = [
        Status::Idea,
        Status::Proposed,
        Status::Draft,
        Status::Review,
        Status::Candidate,
        Status::Approved,
        Status::Stable,
        Status::Deprecated,
        Status::Retired,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Idea => "idea",
            Status::Proposed => "proposed",
            Status::Draft => "draft",
            Status::Review => "review",
            Status::Candidate => "candidate",
            Status::Approved => "approved",
            Status::Stable => "stable",
            Status::Deprecated => "deprecated",
            Status::Retired => "retired",
        }
    }

    /// Fail-closed Parsing gegen den kanonischen Lifecycle.
    pub fn parse(s: &str) -> Result<Self> {
        for st in Self::ALL {
            if st.as_str() == s {
                return Ok(st);
            }
        }
        Err(CoreError::InvalidState(format!(
            "unknown standard lifecycle status: '{s}'"
        )))
    }

    /// Ein Standard ist nur freigegeben, wenn approved oder stable.
    pub fn is_released(&self) -> bool {
        matches!(self, Status::Approved | Status::Stable)
    }
}

/// Ein Standards-Registry-Eintrag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardEntry {
    pub id: StandardId,
    pub title: String,
    pub version: String,
    pub status: Status,
    pub category: String,
    pub normative: bool,
    pub file: String,
}

/// Die geladene Standards-Registry (nur lesend; SSOT bleibt atc-standards).
#[derive(Debug, Clone, Default)]
pub struct StandardsRegistry {
    entries: Vec<StandardEntry>,
}

impl StandardsRegistry {
    /// Lädt `standards.yaml` von der Registry-Datei.
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path).map_err(|e| {
            CoreError::ConfigInvalid(format!("cannot read {}: {e}", path.display()))
        })?;
        Self::parse(&raw)
    }

    /// Parst den `standards:`-Abschnitt (Liste von Inline-Flow-Maps).
    pub fn parse(raw: &str) -> Result<Self> {
        let mut entries: Vec<StandardEntry> = Vec::new();
        let mut in_section = false;

        for line in raw.lines() {
            let t = line.trim();
            if t.starts_with("standards:") {
                in_section = true;
                continue;
            }
            if !in_section {
                continue;
            }
            // Ende der Liste: nächste Zeile auf Top-Level-Ebene (kein Einzug).
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
                let pairs = parse_flow_map(&map_src)?;
                entries.push(StandardEntry::from_pairs(pairs)?);
            }
            // Kommentare und Leerzeilen innerhalb der Liste werden übersprungen.
        }

        if entries.is_empty() {
            return Err(CoreError::ConfigInvalid(
                "standards registry: no entries found (empty or malformed)".into(),
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for e in &entries {
            if !seen.insert(e.id.as_str()) {
                return Err(CoreError::InvalidId(format!(
                    "duplicate standard id: {}",
                    e.id.as_str()
                )));
            }
        }
        Ok(Self { entries })
    }

    /// Fail-closed Auflösung: unbekannter Standard ist ein Fehler (BLOCK, SPEC-001 §24).
    pub fn resolve(&self, id: &str) -> Result<&StandardEntry> {
        self.entries
            .iter()
            .find(|e| e.id.as_str() == id)
            .ok_or_else(|| CoreError::Unsupported(format!("standard not found: '{id}'")))
    }

    /// Löst einen Standard auf und erzwingt: normativ UND freigegeben (approved/stable).
    /// Nicht-freigegebene Standards dürfen Releases blocken (AGENTS.md: Standards-
    /// Impact; Rule-8-Analogon der Org-Governance).
    pub fn require_normative_released(&self, id: &str) -> Result<&StandardEntry> {
        let e = self.resolve(id)?;
        if !e.normative {
            return Err(CoreError::Unsupported(format!(
                "standard '{}' is not normative",
                e.id.as_str()
            )));
        }
        if !e.status.is_released() {
            return Err(CoreError::InvalidState(format!(
                "standard '{}' has lifecycle status '{}' — not released",
                e.id.as_str(),
                e.status.as_str()
            )));
        }
        Ok(e)
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Deterministische Zählung nach Lifecycle-Status (BTreeMap: sortiert).
    pub fn counts_by_status(&self) -> BTreeMap<&'static str, usize> {
        let mut m: BTreeMap<&'static str, usize> = BTreeMap::new();
        for e in &self.entries {
            *m.entry(e.status.as_str()).or_insert(0) += 1;
        }
        m
    }
}

impl std::str::FromStr for StandardsRegistry {
    type Err = CoreError;

    fn from_str(raw: &str) -> std::result::Result<Self, Self::Err> {
        StandardsRegistry::parse(raw)
    }
}

impl StandardEntry {
    fn from_pairs(pairs: Vec<(String, String)>) -> Result<Self> {
        let get = |k: &str| -> Result<&str> {
            pairs
                .iter()
                .find(|(pk, _)| pk == k)
                .map(|(_, v)| v.as_str())
                .ok_or_else(|| CoreError::ConfigInvalid(format!("missing key '{k}'")))
        };
        let id = StandardId::new(get("id")?)?;
        let title = get("title")?.to_string();
        let version = get("version")?.to_string();
        if !is_semver(&version) {
            return Err(CoreError::ConfigInvalid(format!(
                "standard {}: version '{version}' is not SemVer (x.y.z)",
                id.as_str()
            )));
        }
        let status = Status::parse(get("status")?)?;
        let category = get("category")?.to_string();
        let normative = match get("normative")? {
            "true" => true,
            "false" => false,
            other => {
                return Err(CoreError::ConfigInvalid(format!(
                    "standard {}: normative must be true/false, got '{other}'",
                    id.as_str()
                )))
            }
        };
        let file = get("file")?.to_string();
        Ok(Self {
            id,
            title,
            version,
            status,
            category,
            normative,
            file,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
# ATC Standards Registry
schema_version: "1.0.0"
standards:
  - {id: ATC-STD-000, title: "Standards Governance, Verfassung", version: "1.3.0", status: approved, category: governance, normative: true, file: governance/ATC-STD-000.md}
  - {id: ATC-STD-201, title: "Repository Structure Standard", version: "1.0.1", status: approved, category: repository, normative: true, file: standards/repository/ATC-STD-201.md, supersedes: "ATC-STD-REPO-001@1.0.x"}
  - {id: ATC-STD-XYZ-001, title: "Experimental", version: "0.1.0", status: draft, category: x, normative: false, file: standards/x/ATC-STD-XYZ-001.md}

other_section: true
"#;

    #[test]
    fn parses_sample_registry() {
        let r = SAMPLE.parse::<StandardsRegistry>().unwrap();
        assert_eq!(r.count(), 3);
        assert_eq!(r.counts_by_status().get("approved"), Some(&2));
        assert_eq!(r.counts_by_status().get("draft"), Some(&1));
    }

    #[test]
    fn resolve_and_require() {
        let r = SAMPLE.parse::<StandardsRegistry>().unwrap();
        assert!(r.resolve("ATC-STD-000").is_ok());
        assert!(r.resolve("ATC-STD-XXX").is_err());
        assert!(r.require_normative_released("ATC-STD-000").is_ok());
        // draft => nicht freigegeben
        assert!(r.require_normative_released("ATC-STD-XYZ-001").is_err());
        // nicht normativ
        assert!(r.require_normative_released("ATC-STD-XYZ-001").is_err());
    }

    #[test]
    fn fails_closed_on_unknown_status() {
        let bad = SAMPLE.replace("status: draft", "status: weird");
        assert!(bad.parse::<StandardsRegistry>().is_err());
    }

    #[test]
    fn fails_closed_on_duplicate_ids() {
        let bad = SAMPLE.replace(
            "ATC-STD-XYZ-001, title: \"Experimental\"",
            "ATC-STD-201, title: \"Dup\"",
        );
        assert!(bad.parse::<StandardsRegistry>().is_err());
    }

    #[test]
    fn fails_closed_on_missing_keys() {
        let bad = SAMPLE.replace(", normative: true, file: governance/ATC-STD-000.md}", "}");
        assert!(bad.parse::<StandardsRegistry>().is_err());
    }

    #[test]
    fn fails_closed_on_bad_version() {
        let bad = SAMPLE.replace("version: \"1.3.0\"", "version: \"1.x\"");
        assert!(bad.parse::<StandardsRegistry>().is_err());
    }

    #[test]
    fn empty_registry_is_error() {
        assert!("standards:\n".parse::<StandardsRegistry>().is_err());
    }
}
