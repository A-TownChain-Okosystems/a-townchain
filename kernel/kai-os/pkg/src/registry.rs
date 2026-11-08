//! PkgRegistry: Publish mit Immutabilität + deterministische Resolution.

use crate::manifest::PackageManifest;
use crate::version::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum PkgError {
    DuplicateNameVersion { name: String, version: Version },
    MissingDependency { name: String, req: VersionReq },
    CycleDetected { name: String },
    RegistryPoisoned { name: String },
}

impl std::fmt::Display for PkgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PkgError::DuplicateNameVersion { name, version } => {
                write!(
                    f,
                    "publish denied: {name}@{version} existiert (Immutabilität)"
                )
            }
            PkgError::MissingDependency { name, req } => {
                write!(f, "dependency fehlt: {name} ({req}) (fail-closed)")
            }
            PkgError::CycleDetected { name } => {
                write!(f, "Abhängigkeitszyklus über '{name}' (fail-closed)")
            }
            PkgError::RegistryPoisoned { name } => write!(f, "Registry inkonsistent bei '{name}'"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PkgEntry {
    pub manifest: PackageManifest,
    /// SHA-256 des zugehörigen Contents (Integrität).
    pub integrity: String,
}

/// Aufgelöstes Paket im Plan (sortierbar für deterministische Lockfiles).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedPkg {
    pub name: String,
    pub version: Version,
    pub integrity: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionPlan {
    /// Deterministisch sortiert: nach Name.
    pub packages: Vec<ResolvedPkg>,
}

#[derive(Default)]
pub struct PkgRegistry {
    /// name -> version -> entry
    packages: BTreeMap<String, BTreeMap<Version, PkgEntry>>,
}

impl PkgRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Publish: gleiche Version desselben Pakets ist UNVERÄNDERLICH —
    /// ein zweiter Publish mit anderem Content wird abgewiesen.
    pub fn publish(&mut self, manifest: PackageManifest, content: &[u8]) -> Result<(), PkgError> {
        let integrity = crate::sha_hex(content);
        let slot = self.packages.entry(manifest.name.clone()).or_default();
        if let Some(existing) = slot.get(&manifest.version) {
            if existing.integrity != integrity {
                return Err(PkgError::DuplicateNameVersion {
                    name: manifest.name.clone(),
                    version: manifest.version,
                });
            }
            // Identischer Content: idempotent erlauben.
            return Ok(());
        }
        slot.insert(
            manifest.version,
            PkgEntry {
                manifest,
                integrity,
            },
        );
        Ok(())
    }

    /// Deterministische Wahl: HÖCHSTE Version, die die Req erfüllt.
    pub fn best_match(&self, name: &str, req: &VersionReq) -> Option<Version> {
        self.packages
            .get(name)?
            .keys()
            .rev() // BTreeMap aufsteigend -> rev = absteigend (höchste zuerst)
            .find(|v| req.matches(**v))
            .copied()
    }

    /// Höchste verfügbare Version eines Pakets (für den Root ohne Req).
    pub fn highest(&self, name: &str) -> Option<Version> {
        self.packages.get(name)?.keys().next_back().copied()
    }

    /// Vollständige Resolution eines Root-Pakets: deterministischer DFS mit
    /// gepinnten Versionen, Zyklenerkennung fail-closed.
    /// Reihenfolge: erst rekursiv auflösen, DANN in `resolved` verankern —
    /// sonst short-circuitet die Wiederverwendung und Zyklen bleiben unerkannt.
    pub fn resolve(&self, root: &str) -> Result<ResolutionPlan, PkgError> {
        let mut resolved: BTreeMap<String, ResolvedPkg> = BTreeMap::new();
        let root_version = self.highest(root).ok_or(PkgError::RegistryPoisoned {
            name: root.to_string(),
        })?;
        self.resolve_inner(root, &root_version, &mut Vec::new(), &mut resolved)?;
        Ok(ResolutionPlan {
            packages: resolved.into_values().collect(),
        })
    }

    fn resolve_inner(
        &self,
        name: &str,
        version: &Version,
        path: &mut Vec<String>,
        resolved: &mut BTreeMap<String, ResolvedPkg>,
    ) -> Result<(), PkgError> {
        // Zyklusprüfung ZUERST (fail-closed): ein Vorfahre im Pfad = Zyklus.
        if path.contains(&name.to_string()) {
            return Err(PkgError::CycleDetected {
                name: name.to_string(),
            });
        }
        if resolved.contains_key(name) {
            return Ok(()); // vollständig aufgelöst (früher), deterministische Wiederverwendung
        }
        let entry = self.packages.get(name).and_then(|m| m.get(version)).ok_or(
            PkgError::RegistryPoisoned {
                name: name.to_string(),
            },
        )?;

        path.push(name.to_string());
        // Dependencies in Manifest-Ordnung (deterministisch); Version per best_match gepinnt.
        for dep in &entry.manifest.deps {
            let dep_version =
                self.best_match(&dep.name, &dep.req)
                    .ok_or(PkgError::MissingDependency {
                        name: dep.name.clone(),
                        req: dep.req.clone(),
                    })?;
            self.resolve_inner(&dep.name, &dep_version, path, resolved)?;
        }
        path.pop();

        // ERST nach vollständiger Rekursion verankern (Korrektheitsregel).
        resolved.insert(
            name.to_string(),
            ResolvedPkg {
                name: name.to_string(),
                version: *version,
                integrity: entry.integrity.clone(),
            },
        );
        Ok(())
    }

    pub fn integrity_of(&self, name: &str, version: Version) -> Option<&str> {
        self.packages
            .get(name)?
            .get(&version)
            .map(|e| e.integrity.as_str())
    }
}
