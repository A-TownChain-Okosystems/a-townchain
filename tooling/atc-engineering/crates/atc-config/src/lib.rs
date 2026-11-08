//! atc-config — Konfigurations-Laden der Control Plane (Phase-0-Subset).
//!
//! Spezifikation: ATC-ENGINEERING-SPEC-001 §5 (atc-config), §25 (Standards
//! Version Pinning). Fail-closed: unbekannte Sektionen/Keys, Duplikate und
//! fehlende Pflichtfelder sind Fehler — es existiert kein stillschweigender
//! Default für Pflichtfelder.
//!
//! Phase-0-Subset von `atc-engineering.toml`:
//!
//! ```toml
//! [repository]
//! name = "atc-node"
//! phase = "DEVELOPMENT"
//!
//! [standards]
//! source = "A-TownChain-Okosystems/atc-standards"
//! version = "1.3.0"
//! minimum = "1.3.0"
//! maximum = "1.x"
//!
//! [evidence]
//! required = true
//! ```

use atc_core::{
    is_semver, semver_key, DerivedState, RepositoryId, Result as CoreResult, StandardId,
};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

/// Konfigurationsfehler (fail-closed, zeilengenau für Diagnosen).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    pub line: Option<usize>,
    pub message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(n) => write!(f, "config error (line {n}): {}", self.message),
            None => write!(f, "config error: {}", self.message),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Standards Version Pinning (SPEC-001 §25).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardsPin {
    pub source: String,
    pub version: String,
    pub minimum: String,
    pub maximum: String,
}

impl StandardsPin {
    /// SemVer-Subset-Prüfung `minimum <= version`.
    /// Wildcards werden in Phase 0 nicht unterstützt (fail-closed).
    pub fn validate(&self) -> Result<(), ConfigError> {
        let bad = |m: &str| ConfigError {
            line: None,
            message: m.to_string(),
        };
        for (label, v) in [
            ("source", &self.source),
            ("version", &self.version),
            ("minimum", &self.minimum),
        ] {
            if v.trim().is_empty() {
                return Err(bad(&format!("standards.{label} is empty")));
            }
        }
        if !is_semver(&self.version) || !is_semver(&self.minimum) {
            return Err(bad("standards.version/minimum must be SemVer (x.y.z)"));
        }
        if self.minimum.starts_with('*') || self.version.starts_with('*') {
            return Err(bad("wildcards are not supported in version/minimum"));
        }
        if semver_key(&self.minimum) > semver_key(&self.version) {
            return Err(bad("standards.minimum > standards.version"));
        }
        Ok(())
    }
}

/// Repository-Konfiguration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryConfig {
    pub name: RepositoryId,
    pub phase: DerivedState,
    pub standards: StandardsPin,
    pub evidence_required: bool,
}

/// Lädt und validiert eine `atc-engineering.toml` (Phase-0-Subset).
/// Deterministisch: keine Environment-Abhängigkeit, keine Defaults für Pflichtfelder.
pub fn load(path: &Path) -> CoreResult<RepositoryConfig> {
    let raw = std::fs::read_to_string(path).map_err(|e| {
        atc_core::CoreError::ConfigInvalid(format!("cannot read {}: {e}", path.display()))
    })?;
    parse(&raw)
}

/// Parst den Konfigurationstext (Sektionen `[name]`, `key = value`).
pub fn parse(raw: &str) -> CoreResult<RepositoryConfig> {
    let mut sections: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let mut seen: Vec<(String, String)> = Vec::new();
    let mut current: Option<String> = None;

    for line in raw.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if t.starts_with('[') {
            if !t.ends_with(']') || t.len() < 3 {
                return Err(cfg_err("malformed section header"));
            }
            let name = t[1..t.len() - 1].trim().to_string();
            if name.is_empty() || !name.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
                return Err(cfg_err(&format!("invalid section name '{name}'")));
            }
            if sections.contains_key(&name) {
                return Err(cfg_err(&format!("duplicate section [{name}]")));
            }
            sections.insert(name.clone(), BTreeMap::new());
            seen.push((name.clone(), String::new()));
            current = Some(name);
            continue;
        }
        let section = match &current {
            Some(s) => s.clone(),
            None => return Err(cfg_err("key outside of any section")),
        };
        let Some((k, v)) = t.split_once('=') else {
            return Err(cfg_err("expected 'key = value'"));
        };
        let key = k.trim().to_string();
        let val = v.trim().trim_matches('"').to_string();
        if key.is_empty() {
            return Err(cfg_err("empty key"));
        }
        if seen.iter().any(|(s, kk)| *s == section && kk == &key) {
            return Err(cfg_err(&format!("duplicate key '{key}' in [{section}]")));
        }
        seen.push((section.clone(), key.clone()));
        sections
            .get_mut(&section)
            .unwrap_or(&mut BTreeMap::new())
            .insert(key, val);
    }

    let repo = section(&sections, "repository")?;
    let name_raw = required(repo, "repository", "name")?;
    let phase_raw = required(repo, "repository", "phase")?;
    let phase: DerivedState = phase_raw
        .parse()
        .map_err(|_| cfg_err("repository.phase: unknown derived state"))?;

    let std_map = section(&sections, "standards")?;
    let pin = StandardsPin {
        source: required(std_map, "standards", "source")?.clone(),
        version: required(std_map, "standards", "version")?.clone(),
        minimum: required(std_map, "standards", "minimum")?.clone(),
        maximum: required(std_map, "standards", "maximum")?.clone(),
    };
    pin.validate()
        .map_err(|e| atc_core::CoreError::ConfigInvalid(e.message))?;

    let ev = section(&sections, "evidence")?;
    let req_raw = required(ev, "evidence", "required")?;
    let evidence_required = match req_raw.as_str() {
        "true" => true,
        "false" => false,
        other => {
            return Err(cfg_err(&format!(
                "evidence.required: expected true/false, got '{other}'"
            )));
        }
    };

    Ok(RepositoryConfig {
        name: RepositoryId::new(name_raw)?,
        phase,
        standards: pin,
        evidence_required,
    })
}

/// Lädt optional gebundene Standards-IDs ([standards.bound]-Liste nicht Teil des
/// Phase-0-Subsets; Validierung einzelner IDs bleibt atc-core überlassen).
pub fn validate_standard_id(raw: &str) -> CoreResult<StandardId> {
    StandardId::new(raw)
}

fn section<'a>(
    sections: &'a BTreeMap<String, BTreeMap<String, String>>,
    name: &str,
) -> CoreResult<&'a BTreeMap<String, String>> {
    sections
        .get(name)
        .ok_or_else(|| cfg_err(&format!("missing required section [{name}]")))
}

fn required<'a>(
    map: &'a BTreeMap<String, String>,
    section: &str,
    key: &str,
) -> CoreResult<&'a String> {
    map.get(key)
        .ok_or_else(|| cfg_err(&format!("missing required key '{key}' in [{section}]")))
}

fn cfg_err(msg: &str) -> atc_core::CoreError {
    atc_core::CoreError::ConfigInvalid(msg.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"
# atc-engineering.toml
[repository]
name = "atc-node"
phase = "DEVELOPMENT"

[standards]
source = "A-TownChain-Okosystems/atc-standards"
version = "1.3.0"
minimum = "1.3.0"
maximum = "1.x"

[evidence]
required = true
"#;

    #[test]
    fn parses_valid_config() {
        let c = parse(VALID).unwrap();
        assert_eq!(c.name.as_str(), "atc-node");
        assert_eq!(c.phase, DerivedState::Development);
        assert_eq!(c.standards.version, "1.3.0");
        assert!(c.evidence_required);
    }

    #[test]
    fn fails_closed_on_unknown_section() {
        assert!(parse("[wat]\nx = \"1\"\n").is_err());
    }

    #[test]
    fn fails_closed_on_missing_required_key() {
        let raw = "[repository]\nname = \"atc-node\"\nphase = \"DEVELOPMENT\"\n";
        let err = parse(raw).unwrap_err();
        assert!(err.to_string().contains("standards"));
    }

    #[test]
    fn fails_closed_on_duplicate_key() {
        let raw = "[repository]\nname = \"a\"\nname = \"b\"\n";
        assert!(parse(raw).is_err());
    }

    #[test]
    fn fails_closed_on_bad_bool() {
        let raw = VALID.replacen("required = true", "required = yes", 1);
        assert!(parse(&raw).is_err());
    }

    #[test]
    fn pin_rejects_minimum_above_version() {
        let raw = VALID.replacen("minimum = \"1.3.0\"", "minimum = \"2.0.0\"", 1);
        assert!(parse(&raw).is_err());
    }

    #[test]
    fn pin_rejects_wildcard_version() {
        let pin = StandardsPin {
            source: "s".into(),
            version: "*".into(),
            minimum: "1.0.0".into(),
            maximum: "1.x".into(),
        };
        assert!(pin.validate().is_err());
    }
}
