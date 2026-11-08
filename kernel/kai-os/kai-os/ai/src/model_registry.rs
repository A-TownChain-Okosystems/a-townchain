//! Model Registry (S21-S22 nachgeholt, G2-A aus Issue #112).
//!
//! Deterministische Modell-Registrierung nach dem Muster des Package Managers:
//! - Publish-Immutabilität: gleiche Version mit anderem SHA-256 wird abgewiesen,
//!   identischer Publish ist idempotent
//! - Versionswahl: höchste PASSENDE Version (Caret respektiert Major-Grenze),
//!   unabhängig von der Publish-Reihenfolge
//! - Fail-closed: unbekannte Modelle/Versionsanforderungen sind Fehler, keine Fallbacks

use std::collections::BTreeMap;

/// SemVer-Dreifach (deterministisch geordnet).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModelVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ModelVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl std::fmt::Display for ModelVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Versionsanforderung: Exact oder Caret (Major-Grenze).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionReq {
    Exact(ModelVersion),
    Caret(ModelVersion),
}

impl VersionReq {
    pub fn matches(&self, v: ModelVersion) -> bool {
        match *self {
            VersionReq::Exact(e) => v == e,
            VersionReq::Caret(c) => v >= c && v.major == c.major,
        }
    }
}

/// Modell-Manifest: Metadaten + Integrität.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelManifest {
    pub id: String,
    pub version: ModelVersion,
    pub owner: String,
    pub size_bytes: u64,
    /// SHA-256 über die Modell-Artefakt-Bytes (Verifikationsgrundlage).
    pub sha256: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegistryError {
    /// Gleiche Version mit anderem SHA-256 — Immutabilitätsverstoß.
    DuplicateIdVersion {
        id: String,
        version: ModelVersion,
    },
    UnknownModel {
        id: String,
    },
    /// Modell bekannt, aber keine Version erfüllt die Anforderung.
    NoMatchingVersion {
        id: String,
        req: VersionReq,
    },
}

/// Die deterministische Model Registry.
#[derive(Debug, Default)]
pub struct ModelRegistry {
    models: BTreeMap<String, BTreeMap<ModelVersion, ModelManifest>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Publish mit Immutabilität: identischer Publish idempotent,
    /// abweichender SHA-256 bei gleicher Version → fail-closed.
    pub fn publish(&mut self, m: ModelManifest) -> Result<(), RegistryError> {
        let entry = self.models.entry(m.id.clone()).or_default();
        match entry.get(&m.version) {
            Some(existing) if existing.sha256 == m.sha256 => Ok(()), // idempotent
            Some(_) => Err(RegistryError::DuplicateIdVersion {
                id: m.id,
                version: m.version,
            }),
            None => {
                entry.insert(m.version, m);
                Ok(())
            }
        }
    }

    /// Höchste PASSende Version — deterministisch unabhängig von Publish-Reihenfolge.
    pub fn best_match(&self, id: &str, req: &VersionReq) -> Result<&ModelManifest, RegistryError> {
        let versions = self
            .models
            .get(id)
            .ok_or(RegistryError::UnknownModel { id: id.to_string() })?;
        versions
            .iter()
            .rev()
            .map(|(_, m)| m)
            .find(|m| req.matches(m.version))
            .ok_or(RegistryError::NoMatchingVersion {
                id: id.to_string(),
                req: *req,
            })
    }

    /// Exakte Abfrage.
    pub fn get(&self, id: &str, version: ModelVersion) -> Result<&ModelManifest, RegistryError> {
        self.models
            .get(id)
            .and_then(|v| v.get(&version))
            .ok_or(RegistryError::UnknownModel { id: id.to_string() })
    }

    /// Anzahl registrierter Modelle (eindeutige IDs).
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Versionen eines Modells, sortiert (deterministische Auflistung).
    pub fn versions_of(&self, id: &str) -> Vec<ModelVersion> {
        self.models
            .get(id)
            .map(|v| v.keys().copied().collect())
            .unwrap_or_default()
    }
}
