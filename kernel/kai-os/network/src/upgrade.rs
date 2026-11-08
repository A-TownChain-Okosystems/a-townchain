//! Protokoll-Upgrade & Versionierung (G2-D, Issue #112).
//!
//! Grundsatz: **Kein Best-Effort-Parsing.** Ein Peer mit inkompatibler
//! Protokollversion wird abgewiesen — er erhält stattdessen den State-Root
//! der letzten verifizierten Grenze als RESYNC-Hinweis. Rollback bedeutet
//! hier nie semantische Umkehr angewendeter Effekte, sondern Rückkehr auf
//! die letzte verifizierte Snapshot-Grenze (Snapshot-Return).

use serde::{Deserialize, Serialize};

/// Aktuelle Protokollversion des Netzwerks.
pub const PROTOCOL_VERSION: u32 = 2;
/// Älteste Version, mit der dieser Knoten noch sprechen kann.
pub const MIN_COMPATIBLE: u32 = 1;

/// Versionsunterstützung eines Knotens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolSupport {
    pub current: u32,
    pub min: u32,
}

impl ProtocolSupport {
    pub fn ours() -> Self {
        Self {
            current: PROTOCOL_VERSION,
            min: MIN_COMPATIBLE,
        }
    }

    pub fn validate(&self) -> Result<(), UpgradeError> {
        if self.min > self.current {
            return Err(UpgradeError::MalformedSupport {
                current: self.current,
                min: self.min,
            });
        }
        Ok(())
    }
}

/// Hello-Nachricht beim Verbindungsaufbau (Version-Verhandlung).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hello {
    pub support: ProtocolSupport,
    /// Resync-Anhaltspunkt für den Fall der Ablehnung: State-Root der
    /// letzten verifizierten Snapshot-Grenze des Senders (Snapshot-Return).
    pub last_verified_root: String,
}

/// Ergebnis der Verhandlung: die Version, auf der beide Seiten sprechen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Negotiated {
    pub version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpgradeError {
    /// Versions-Tupel unlogisch (min > current).
    MalformedSupport { current: u32, min: u32 },
    /// Keine gemeinsame Version — Verbindung wird abgewiesen.
    /// Trägt den Resync-Hinweis des GEGENÜBERS (Snapshot-Return).
    Incompatible {
        ours: (u32, u32),
        theirs: (u32, u32),
        resync_root_hint: String,
    },
    /// Envelope mit unbekannter/zukünftiger Version — fail-closed, kein Parsen des Payloads.
    UnknownEnvelopeVersion { found: u32, supported_until: u32 },
    /// Envelope-Version unterhalb der eigenen Mindestversion.
    EnvelopeTooOld { found: u32, min: u32 },
}

impl std::fmt::Display for UpgradeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpgradeError::MalformedSupport { current, min } =>
                write!(f, "unlogische Version-Unterstuetzung: min {min} > current {current}"),
            UpgradeError::Incompatible { ours, theirs, .. } =>
                write!(f, "inkompatibel: wir ({}, {}), gegenueber ({}, {})", ours.0, ours.1, theirs.0, theirs.1),
            UpgradeError::UnknownEnvelopeVersion { found, supported_until } =>
                write!(f, "Envelope-Version {found} > unterstuetzt bis {supported_until} — payload wird NICHT geparst"),
            UpgradeError::EnvelopeTooOld { found, min } =>
                write!(f, "Envelope-Version {found} < mindestens {min}"),
        }
    }
}

/// Deterministische Verhandlung: die gemeinsame Version ist
/// min(current_a, current_b) und muss >= max(min_a, min_b) sein.
pub fn negotiate(ours: &Hello, theirs: &Hello) -> Result<Negotiated, UpgradeError> {
    ours.support.validate()?;
    theirs.support.validate()?;
    let shared = ours.support.current.min(theirs.support.current);
    let required = ours.support.min.max(theirs.support.min);
    if shared < required {
        return Err(UpgradeError::Incompatible {
            ours: (ours.support.current, ours.support.min),
            theirs: (theirs.support.current, theirs.support.min),
            resync_root_hint: theirs.last_verified_root.clone(),
        });
    }
    Ok(Negotiated { version: shared })
}

/// Versionierter Wire-Envelope: [version][payload]. Beim Dekodieren wird
/// die Version GEGEBENENFALLS geprüft, BEVOR der Payload interpretiert wird.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedEnvelope {
    pub version: u32,
    pub payload: Vec<u8>,
}

impl VersionedEnvelope {
    pub fn encode(version: u32, payload: &[u8]) -> Self {
        Self {
            version,
            payload: payload.to_vec(),
        }
    }

    /// Dekodieren mit Versions-Check: unbekannte Version -> fail-closed,
    /// der Payload wird bewusst NICHT weiterverarbeitet.
    pub fn decode_checked(&self, support: &ProtocolSupport) -> Result<&[u8], UpgradeError> {
        support.validate()?;
        if self.version > support.current {
            return Err(UpgradeError::UnknownEnvelopeVersion {
                found: self.version,
                supported_until: support.current,
            });
        }
        if self.version < support.min {
            return Err(UpgradeError::EnvelopeTooOld {
                found: self.version,
                min: support.min,
            });
        }
        Ok(&self.payload)
    }

    /// Serialisierung für die Leitung.
    pub fn to_wire(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }

    pub fn from_wire(bytes: &[u8]) -> Result<Self, UpgradeError> {
        serde_json::from_slice(bytes).map_err(|_| UpgradeError::UnknownEnvelopeVersion {
            found: 0,
            supported_until: PROTOCOL_VERSION,
        })
    }
}
