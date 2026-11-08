//! Verified Cache (S21-S22 nachgeholt, G2-A aus Issue #112).
//!
//! Regel: **Kein ungeprüftes Laden.** Artefakte werden beim Einstellen
//! UND bei jedem einzelnen Ladevorgang gegen den SHA-256 des Manifests
//! verifiziert. Eine Verfälschung im Lager wird beim nächsten Zugriff
//! erkannt — fail-closed, niemals Rückgabe ungeprüfter Bytes.
//! Eviction ist insertion-ordered (FIFO) und damit deterministisch.

use crate::model_registry::{ModelManifest, ModelVersion};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum CacheError {
    /// Hash passt nicht — beim put (falsches Artefakt) oder beim get (Korruption).
    HashMismatch {
        id: String,
        version: ModelVersion,
        expected: String,
        got: String,
    },
    UnknownEntry {
        id: String,
        version: ModelVersion,
    },
}

/// Verifiziertes Artefakt — nur nach bestandener Prüfung konstruierbar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedModel {
    pub manifest: ModelManifest,
    pub bytes: Vec<u8>,
}

/// Der SHA-256-verifizierte Modell-Cache.
#[derive(Debug)]
pub struct VerifiedCache {
    entries: BTreeMap<(String, ModelVersion), (ModelManifest, Vec<u8>)>,
    /// FIFO-Ordnung für deterministische Eviction.
    order: Vec<(String, ModelVersion)>,
    max_entries: usize,
}

impl VerifiedCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: BTreeMap::new(),
            order: Vec::new(),
            max_entries,
        }
    }

    fn sha256_hex(bytes: &[u8]) -> String {
        crate::sha_hex(bytes)
    }

    /// Artefakt einstellen — Hash wird VOR dem Speichern geprüft (fail-closed).
    /// Ein falsches Artefakt wird abgewiesen und NICHT gelagert.
    pub fn put(&mut self, manifest: ModelManifest, bytes: Vec<u8>) -> Result<(), CacheError> {
        let got = Self::sha256_hex(&bytes);
        if got != manifest.sha256 {
            return Err(CacheError::HashMismatch {
                id: manifest.id,
                version: manifest.version,
                expected: manifest.sha256,
                got,
            });
        }
        let key = (manifest.id.clone(), manifest.version);
        // FIFO-Eviction deterministisch: ältestes zuerst raus.
        if self.entries.len() >= self.max_entries {
            if let Some(oldest) = self.order.first().cloned() {
                self.entries.remove(&oldest);
                self.order.remove(0);
            }
        }
        if !self.entries.contains_key(&key) {
            self.order.push(key.clone());
        }
        self.entries.insert(key, (manifest, bytes));
        Ok(())
    }

    /// Artefakt laden — Hash wird bei JEDEM Zugriff erneut geprüft.
    /// Korruption im Lager → Err, niemals ungeprüfte Bytes.
    pub fn get(&self, id: &str, version: ModelVersion) -> Result<VerifiedModel, CacheError> {
        let (manifest, bytes) =
            self.entries
                .get(&(id.to_string(), version))
                .ok_or(CacheError::UnknownEntry {
                    id: id.to_string(),
                    version,
                })?;
        let got = Self::sha256_hex(bytes);
        if got != manifest.sha256 {
            return Err(CacheError::HashMismatch {
                id: id.to_string(),
                version,
                expected: manifest.sha256.clone(),
                got,
            });
        }
        Ok(VerifiedModel {
            manifest: manifest.clone(),
            bytes: bytes.clone(),
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// NUR FÜR TESTS: simuliert Lager-Korruption (Bit-Flip),
    /// um die Verifikation beim Ladevorgang zu prüfen.
    #[cfg(test)]
    pub fn tamper_for_test(&mut self, id: &str, version: ModelVersion) -> bool {
        if let Some((_, bytes)) = self.entries.get_mut(&(id.to_string(), version)) {
            if bytes.is_empty() {
                return false;
            }
            bytes[0] ^= 0xFF;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(id: &str, v: ModelVersion, bytes: &[u8]) -> ModelManifest {
        ModelManifest {
            id: id.into(),
            version: v,
            owner: "test".into(),
            size_bytes: bytes.len() as u64,
            sha256: crate::sha_hex(bytes),
        }
    }

    #[test]
    fn load_detects_storage_tampering() {
        let mut c = VerifiedCache::new(4);
        let v = ModelVersion::new(1, 0, 0);
        c.put(m("vision", v, b"artefakt"), b"artefakt".to_vec())
            .unwrap();

        // Korruption simulieren (Bit-Flip im Lager, z.B. Disk-Fehler)
        assert!(c.tamper_for_test("vision", v));

        // Jeder Ladevorgang verifiziert erneut -> Korruption wird erkannt,
        // ungepruefte Bytes werden NIEMALS zurueckgegeben (fail-closed)
        assert!(matches!(
            c.get("vision", v),
            Err(CacheError::HashMismatch { .. })
        ));
    }
}
