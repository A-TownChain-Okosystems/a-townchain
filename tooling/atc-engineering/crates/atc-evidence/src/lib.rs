//! atc-evidence — machine-readable Evidence Plane foundation.
//!
//! No Evidence, No Trust: evidence is explicit, versioned and fail-closed.
//! The crate intentionally has no external serialization dependency in phase 1;
//! it emits deterministic JSON suitable for archival and later schema validation.

use atc_core::{EvidenceId, Result as CoreResult, StandardId};
use std::fs;
use std::path::{Path, PathBuf};

pub const EVIDENCE_SCHEMA: &str = "ATC-EVD-001";
pub const EVIDENCE_SCHEMA_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus {
    Pass,
    Fail,
    Unknown,
    Missing,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Unknown => "UNKNOWN",
            Self::Missing => "MISSING",
        }
    }

    pub fn is_trustworthy(self) -> bool {
        matches!(self, Self::Pass)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    pub id: EvidenceId,
    pub evidence_type: String,
    pub repository: String,
    pub commit: String,
    pub producer: String,
    pub status: EvidenceStatus,
    pub standards: Vec<StandardId>,
    pub artifacts: Vec<PathBuf>,
    pub provenance: String,
    pub integrity_algorithm: String,
    pub integrity_digest: String,
}

impl EvidenceRecord {
    pub fn new(
        id: EvidenceId,
        evidence_type: impl Into<String>,
        repository: impl Into<String>,
        commit: impl Into<String>,
        producer: impl Into<String>,
        status: EvidenceStatus,
    ) -> Self {
        Self {
            id,
            evidence_type: evidence_type.into(),
            repository: repository.into(),
            commit: commit.into(),
            producer: producer.into(),
            status,
            standards: Vec::new(),
            artifacts: Vec::new(),
            provenance: String::new(),
            integrity_algorithm: "SHA-256".into(),
            integrity_digest: String::new(),
        }
    }

    pub fn add_standard(&mut self, standard: StandardId) {
        if !self.standards.contains(&standard) {
            self.standards.push(standard);
        }
    }

    pub fn add_artifact(&mut self, artifact: impl Into<PathBuf>) {
        let artifact = artifact.into();
        if !self.artifacts.contains(&artifact) {
            self.artifacts.push(artifact);
        }
    }

    pub fn validate(&self) -> CoreResult<()> {
        if self.repository.trim().is_empty() {
            return Err(atc_core::CoreError::MissingEvidence("repository".into()));
        }
        if self.commit.trim().is_empty() {
            return Err(atc_core::CoreError::MissingEvidence("commit".into()));
        }
        if self.producer.trim().is_empty() {
            return Err(atc_core::CoreError::MissingEvidence("producer".into()));
        }
        if self.integrity_digest.trim().is_empty() {
            return Err(atc_core::CoreError::MissingEvidence(
                "integrity digest".into(),
            ));
        }
        Ok(())
    }

    pub fn to_json(&self) -> String {
        let standards = self
            .standards
            .iter()
            .map(|s| format!("\"{}\"", escape(s.as_str())))
            .collect::<Vec<_>>()
            .join(",");
        let artifacts = self
            .artifacts
            .iter()
            .map(|p| format!("\"{}\"", escape(&p.display().to_string())))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\n  \"schema\": \"{}\",\n  \"schema_version\": \"{}\",\n  \"id\": \"{}\",\n  \"type\": \"{}\",\n  \"repository\": \"{}\",\n  \"commit\": \"{}\",\n  \"producer\": \"{}\",\n  \"status\": \"{}\",\n  \"standards\": [{}],\n  \"artifacts\": [{}],\n  \"provenance\": \"{}\",\n  \"integrity\": {{\"algorithm\": \"{}\", \"digest\": \"{}\"}},\n  \"immutable\": true\n}}\n",
            EVIDENCE_SCHEMA,
            EVIDENCE_SCHEMA_VERSION,
            escape(self.id.as_str()),
            escape(&self.evidence_type),
            escape(&self.repository),
            escape(&self.commit),
            escape(&self.producer),
            self.status.as_str(),
            standards,
            artifacts,
            escape(&self.provenance),
            escape(&self.integrity_algorithm),
            escape(&self.integrity_digest),
        )
    }

    pub fn write_to(&self, root: &Path) -> CoreResult<PathBuf> {
        self.validate()?;
        let dir = root.join(".atc/evidence");
        fs::create_dir_all(&dir)
            .map_err(|e| atc_core::CoreError::ConfigInvalid(format!("create evidence dir: {e}")))?;
        let path = dir.join(format!("{}.json", self.id));
        if path.exists() {
            return Err(atc_core::CoreError::InvalidState(format!(
                "evidence already exists: {}",
                path.display()
            )));
        }
        fs::write(&path, self.to_json())
            .map_err(|e| atc_core::CoreError::ConfigInvalid(format!("write evidence: {e}")))?;
        Ok(path)
    }
}

fn escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0C}' => escaped.push_str("\\f"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            control if control <= '\u{1F}' => {
                escaped.push_str(&format!("\\u{:04x}", control as u32));
            }
            other => escaped.push(other),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_pass_is_not_trustworthy() {
        assert!(EvidenceStatus::Pass.is_trustworthy());
        assert!(!EvidenceStatus::Fail.is_trustworthy());
        assert!(!EvidenceStatus::Unknown.is_trustworthy());
        assert!(!EvidenceStatus::Missing.is_trustworthy());
    }

    #[test]
    fn validation_is_fail_closed() {
        let id = EvidenceId::new("EVD-2026-000001").unwrap();
        let record = EvidenceRecord::new(
            id,
            "TEST",
            "atc-node",
            "abc",
            "atc-test",
            EvidenceStatus::Pass,
        );
        assert!(record.validate().is_err());
    }

    #[test]
    fn json_escaping_covers_every_json_control_character() {
        let input: String = (0u8..=0x1f).map(char::from).collect();
        let escaped = escape(&input);
        assert!(!escaped.chars().any(|ch| ch <= '\u{1f}'));
        for control in 0u32..=0x1f {
            let ch = char::from_u32(control).unwrap();
            assert!(!escaped.contains(ch));
        }
        assert!(escaped.contains("\\u0000"));
        assert!(escaped.contains("\\b"));
        assert!(escaped.contains("\\f"));
        assert!(escaped.contains("\\t"));
        assert!(escaped.contains("\\u001f"));
    }

    #[test]
    fn json_contains_schema_and_immutable_marker() {
        let id = EvidenceId::new("EVD-2026-000002").unwrap();
        let mut record = EvidenceRecord::new(
            id,
            "TEST",
            "atc-node",
            "abc",
            "atc-test",
            EvidenceStatus::Pass,
        );
        record.integrity_digest = "deadbeef".into();
        let json = record.to_json();
        assert!(json.contains("ATC-EVD-001"));
        assert!(json.contains("\"immutable\": true"));
        assert!(json.contains("\"status\": \"PASS\""));
    }
}
