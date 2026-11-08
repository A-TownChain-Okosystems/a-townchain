//! Lockfile: byte-deterministisch, sortiert, SHA-256-verifizierbar.

use crate::registry::{PkgRegistry, ResolutionPlan};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LockEntry {
    pub name: String,
    pub version: crate::version::Version,
    pub integrity: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lockfile {
    pub root: String,
    pub entries: Vec<LockEntry>,
}

impl Lockfile {
    /// Erzeugung aus einem Plan — Einträge sortiert nach Name (byte-deterministisch).
    pub fn from_plan(root: &str, plan: &ResolutionPlan) -> Self {
        let mut entries: Vec<LockEntry> = plan
            .packages
            .iter()
            .map(|p| LockEntry {
                name: p.name.clone(),
                version: p.version,
                integrity: p.integrity.clone(),
            })
            .collect();
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Self {
            root: root.to_string(),
            entries,
        }
    }

    /// Deterministische Serialisierung (stabile Feldordnung).
    pub fn to_text(&self) -> String {
        let mut out = format!("# kai-os lockfile (root: {})\n", self.root);
        for e in &self.entries {
            out.push_str(&format!("{} {} {}\n", e.name, e.version, e.integrity));
        }
        out
    }

    /// Integritätsprüfung gegen die Registry — manipulierter Content scheitert.
    pub fn verify(&self, registry: &PkgRegistry) -> bool {
        self.entries.iter().all(|e| {
            registry
                .integrity_of(&e.name, e.version)
                .map(|i| *i == e.integrity)
                .unwrap_or(false)
        })
    }
}
