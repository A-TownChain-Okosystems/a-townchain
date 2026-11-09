// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ══════════════════════════════════════════════════════════════════════════
// ATC Update System — Referenz-Durchsetzung der Update-Entscheidung.
//
// Dieses Crate ist reine Entscheidungslogik: keine Netzwerkverbindung, kein
// Schreiben auf Datentraeger, keine Signaturberechnung. Es macht die
// Pruefordnung ausfuhrbar und damit pruefbar, BEVOR ein Kanal oder ein
// Bootloader sie benutzt.
//
// Bewusste Grenzen:
//  * Kryptografie ist NICHT hier. Die Signaturpruefung kommt ueber den Trait
//    `SignatureVerifier` von der Runtime; dieses Crate behauptet keine Krypto.
//  * Hashen ist NICHT hier. Der Aufrufer liefert den beobachteten Hash des
//    Artefakts; verglichen wird gegen den Hash im signierten Manifest.
//  * Kein Rollback-Vorgang auf Platte. `rollback_target` liefert nur die
//    Entscheidung, welcher Stand zulaessig waere.
//
// Grundsaetze:
//  * Fail-closed: jeder unklare Zustand ist eine Ablehnung, nie eine Freigabe.
//  * Kein Trust-on-first-use: eine unbekannte Komponente oder ein unbekannter
//    Signierer wird abgelehnt, nicht registriert.
//  * Genau EIN Audit-Eintrag je Entscheidung, auch bei Ablehnung.
//  * Die Reihenfolge der Pruefungen ist normativ (siehe `rank`).

use std::collections::{BTreeMap, BTreeSet};

/// Domain-Trennung: verhindert, dass eine Signatur aus einem anderen Kontext
/// (Admission, DID, Transaktion) als Update-Signatur durchgeht.
pub const DOMAIN_PREFIX: &[u8] = b"ATC-UPDATE-v1\0";

/// Formatversion des Manifests. Aeltere oder neuere Fassungen werden abgelehnt,
/// solange sie nicht ausdruecklich unterstuetzt sind.
pub const MANIFEST_VERSION: u32 = 1;

/// Obere Grenze fuer ein Update-Artefakt (Bytes). Schuetzt gegen ein Manifest,
/// das den Datentraeger oder den Zwischenspeicher ueberfordert.
pub const MAX_ARTIFACT_BYTES: u64 = 512 * 1024 * 1024;

pub type ComponentId = String;
pub type KeyId = String;
pub type Sha256 = [u8; 32];

// ──────────────────────────────────────────────────────────────────────────
// Fehlercodes in normativer Reihenfolge
// ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdCode {
    /// 1 — Manifest nicht lesbar oder Formatversion unbekannt.
    ManifestUnreadable,
    /// 2 — Komponente ist der Autoritaet nicht bekannt.
    UnknownComponent,
    /// 3 — Komponente steht unter Quarantaene (frueherer Fehlschlag).
    Quarantined,
    /// 4 — Artefaktgroesse unzulaessig (Null oder ueber der Grenze).
    SizeInvalid,
    /// 5 — beobachteter Hash weicht vom signierten Hash ab.
    IntegrityMismatch,
    /// 6 — Signatur fehlt oder ist ungueltig.
    SignatureInvalid,
    /// 7 — Manifest ist abgelaufen.
    Expired,
    /// 8 — Laufzeit ist zu alt fuer dieses Update (min_boot_abi).
    BootAbiTooOld,
    /// 9 — Version ist nicht neuer und Rollback ist nicht zulaessig.
    VersionRejected,
    /// 10 — Signierer oder Version wurde zurueckgezogen.
    Revoked,
    /// 11 — zulaessig.
    Accepted,
}

impl UpdCode {
    /// Normativer Pruefschritt 1..=11. Die Zahl ist die Ordnung, nicht der Name.
    pub const fn rank(self) -> u8 {
        match self {
            UpdCode::ManifestUnreadable => 1,
            UpdCode::UnknownComponent => 2,
            UpdCode::Quarantined => 3,
            UpdCode::SizeInvalid => 4,
            UpdCode::IntegrityMismatch => 5,
            UpdCode::SignatureInvalid => 6,
            UpdCode::Expired => 7,
            UpdCode::BootAbiTooOld => 8,
            UpdCode::VersionRejected => 9,
            UpdCode::Revoked => 10,
            UpdCode::Accepted => 11,
        }
    }

    pub const fn code(self) -> &'static str {
        match self {
            UpdCode::ManifestUnreadable => "UPD-001",
            UpdCode::UnknownComponent => "UPD-002",
            UpdCode::Quarantined => "UPD-003",
            UpdCode::SizeInvalid => "UPD-004",
            UpdCode::IntegrityMismatch => "UPD-005",
            UpdCode::SignatureInvalid => "UPD-006",
            UpdCode::Expired => "UPD-007",
            UpdCode::BootAbiTooOld => "UPD-008",
            UpdCode::VersionRejected => "UPD-009",
            UpdCode::Revoked => "UPD-010",
            UpdCode::Accepted => "UPD-000",
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Manifest
// ──────────────────────────────────────────────────────────────────────────

/// Ein signiertes Update-Manifest. Es beschreibt genau ein Artefakt einer
/// Komponente. Alles, was zum Entscheiden noetig ist, steht im Manifest —
/// nicht in der Umgebung des Aufrufers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateManifest {
    pub manifest_version: u32,
    pub component: ComponentId,
    /// Streng monoton je Komponente. Gleiche Version = Replay.
    pub version: u64,
    pub artifact_sha256: Sha256,
    pub artifact_size: u64,
    /// Aelteste Boot-ABI, die dieses Update tragen kann.
    pub min_boot_abi: u16,
    /// Ablauf in Mikrosekunden seit Epoche. Ganzzahl, kein Gleitkomma:
    /// Gleitkomma waere plattformabhaengig und damit nicht deterministisch.
    pub expires_at_us: i64,
    /// Erlaubt dieses Manifest einen Rueckschritt auf eine freigegebene Version.
    pub rollback_permitted: bool,
    /// Kennung des Signierers. Die Pruefung der Kennung gegen den Schluessel
    /// ist Sache des `SignatureVerifier`.
    pub signer: KeyId,
}

impl UpdateManifest {
    /// Kanonische Bytes fuer die Signaturpruefung.
    ///
    /// Regeln: Domain-Praefix zuerst, danach jedes Feld mit Laengenpraefix
    /// (u32) fuer variable Teile, damit keine zwei verschiedenen Manifeste
    /// dieselben Bytes ergeben (Laengenpraefix-Kollision).
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(128);
        out.extend_from_slice(DOMAIN_PREFIX);
        out.extend_from_slice(&self.manifest_version.to_be_bytes());
        push_len_prefixed(&mut out, self.component.as_bytes());
        out.extend_from_slice(&self.version.to_be_bytes());
        out.extend_from_slice(&self.artifact_sha256);
        out.extend_from_slice(&self.artifact_size.to_be_bytes());
        out.extend_from_slice(&self.min_boot_abi.to_be_bytes());
        out.extend_from_slice(&self.expires_at_us.to_be_bytes());
        out.push(u8::from(self.rollback_permitted));
        push_len_prefixed(&mut out, self.signer.as_bytes());
        out
    }

    /// Einfache Plausibilitaet: Formatversion bekannt und Komponentenname nicht leer.
    pub fn is_readable(&self) -> bool {
        self.manifest_version == MANIFEST_VERSION && !self.component.trim().is_empty()
    }
}

fn push_len_prefixed(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    out.extend_from_slice(bytes);
}

// ──────────────────────────────────────────────────────────────────────────
// Anfrage und Entscheidung
// ──────────────────────────────────────────────────────────────────────────

/// Was die Runtime beobachtet hat. Der Aufrufer behauptet nichts, er liefert
/// Messwerte; die Autoritaet entscheidet.
#[derive(Debug, Clone)]
pub struct Request {
    pub manifest: UpdateManifest,
    /// Beobachteter Hash des heruntergeladenen Artefakts.
    pub observed_sha256: Sha256,
    /// Beobachtete Groesse des Artefakts.
    pub observed_size: u64,
    pub signature: Vec<u8>,
    /// Aktuell installierter Stand der Komponente, falls vorhanden.
    pub installed_version: Option<u64>,
    /// Boot-ABI der laufenden Laufzeit.
    pub boot_abi: u16,
    pub now_us: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Update darf eingespielt werden. `version` ist der Zielstand.
    Accepted {
        component: ComponentId,
        version: u64,
    },
    /// Update darf nicht eingespielt werden.
    Rejected { code: UpdCode },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Accepted { version: u64 },
    Rejected { code: UpdCode, rank: u8 },
}

impl Outcome {
    pub fn is_accepted(&self) -> bool {
        matches!(self, Outcome::Accepted { .. })
    }
}

/// Signaturpruefung kommt von aussen. Dieses Crate behauptet keine Krypto.
pub trait SignatureVerifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> bool;
}

/// Nur fuer Tests: akzeptiert jede Signatur. Niemals im Betrieb verwenden.
pub struct AcceptAll;
impl SignatureVerifier for AcceptAll {
    fn verify(&self, _payload: &[u8], _signature: &[u8]) -> bool {
        true
    }
}

/// Nur fuer Tests: lehnt jede Signatur ab.
pub struct RejectAll;
impl SignatureVerifier for RejectAll {
    fn verify(&self, _payload: &[u8], _signature: &[u8]) -> bool {
        false
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Audit
// ──────────────────────────────────────────────────────────────────────────

/// Genau ein Eintrag je Entscheidung. Append-only: Eintraege werden nie
/// geaendert oder entfernt, damit die Kette nachvollziehbar bleibt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub component: ComponentId,
    pub version: u64,
    pub code: UpdCode,
    pub rank: u8,
    pub signer: KeyId,
    pub at_us: i64,
}

// ──────────────────────────────────────────────────────────────────────────
// Autoritaet
// ──────────────────────────────────────────────────────────────────────────

/// Die Update-Autoritaet. Sie kennt die zulaessigen Komponenten, die
/// zurueckgezogenen Schluessel und Versionen, die hoechsten installierten
/// Staende und die Quarantaene.
#[derive(Debug, Default)]
pub struct Authority {
    known_components: BTreeSet<ComponentId>,
    revoked_keys: BTreeSet<KeyId>,
    revoked_versions: BTreeMap<ComponentId, BTreeSet<u64>>,
    highest_installed: BTreeMap<ComponentId, u64>,
    quarantined: BTreeSet<ComponentId>,
    /// Zulaessige Rueckschritt-Ziele je Komponente (nur diese sind erlaubt).
    rollback_targets: BTreeMap<ComponentId, BTreeSet<u64>>,
    max_artifact_bytes: u64,
    audit: Vec<AuditEntry>,
}

impl Authority {
    pub fn new() -> Self {
        Self {
            max_artifact_bytes: MAX_ARTIFACT_BYTES,
            ..Self::default()
        }
    }

    /// Sets the maximum artifact size. Never above the hard limit.
    pub fn with_max_artifact_bytes(mut self, bytes: u64) -> Self {
        self.max_artifact_bytes = bytes.min(MAX_ARTIFACT_BYTES);
        self
    }

    pub fn allow_component(&mut self, component: &str) {
        self.known_components.insert(component.to_string());
    }

    pub fn revoke_key(&mut self, key: &str) {
        self.revoked_keys.insert(key.to_string());
    }

    pub fn revoke_version(&mut self, component: &str, version: u64) {
        self.revoked_versions
            .entry(component.to_string())
            .or_default()
            .insert(version);
    }

    pub fn allow_rollback_target(&mut self, component: &str, version: u64) {
        self.rollback_targets
            .entry(component.to_string())
            .or_default()
            .insert(version);
    }

    pub fn quarantine(&mut self, component: &str) {
        self.quarantined.insert(component.to_string());
    }

    /// Aufhebung der Quarantaene ist ein ausdruecklicher Verwaltungsakt,
    /// kein Nebeneffekt eines spaeteren Updates.
    pub fn release_quarantine(&mut self, component: &str) -> bool {
        self.quarantined.remove(component)
    }

    pub fn is_quarantined(&self, component: &str) -> bool {
        self.quarantined.contains(component)
    }

    pub fn highest_installed(&self, component: &str) -> Option<u64> {
        self.highest_installed.get(component).copied()
    }

    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit
    }

    /// Prueft eine Update-Anfrage. Fail-closed, genau ein Audit-Eintrag.
    pub fn admit(&mut self, request: &Request, verifier: &dyn SignatureVerifier) -> Outcome {
        let code = self.evaluate(request, verifier);
        let outcome = match code {
            UpdCode::Accepted => Outcome::Accepted {
                version: request.manifest.version,
            },
            other => Outcome::Rejected {
                code: other,
                rank: other.rank(),
            },
        };

        if outcome.is_accepted() {
            let component = request.manifest.component.clone();
            let entry = self.highest_installed.entry(component).or_insert(0);
            if request.manifest.version > *entry {
                *entry = request.manifest.version;
            }
        }

        self.audit.push(AuditEntry {
            component: request.manifest.component.clone(),
            version: request.manifest.version,
            code: if outcome.is_accepted() {
                UpdCode::Accepted
            } else {
                match &outcome {
                    Outcome::Rejected { code, .. } => *code,
                    Outcome::Accepted { .. } => UpdCode::Accepted,
                }
            },
            rank: if outcome.is_accepted() {
                UpdCode::Accepted.rank()
            } else {
                match &outcome {
                    Outcome::Rejected { rank, .. } => *rank,
                    Outcome::Accepted { .. } => UpdCode::Accepted.rank(),
                }
            },
            signer: request.manifest.signer.clone(),
            at_us: request.now_us,
        });

        outcome
    }

    /// Die Pruefordnung. Eigene Funktion, damit sie einzeln testbar ist.
    fn evaluate(&self, request: &Request, verifier: &dyn SignatureVerifier) -> UpdCode {
        let m = &request.manifest;

        // 1 — Manifest lesbar
        if !m.is_readable() {
            return UpdCode::ManifestUnreadable;
        }

        // 2 — Komponente bekannt
        if !self.known_components.contains(&m.component) {
            return UpdCode::UnknownComponent;
        }

        // 3 — keine Quarantaene
        if self.quarantined.contains(&m.component) {
            return UpdCode::Quarantined;
        }

        // 4 — Groesse plausibel
        if m.artifact_size == 0
            || m.artifact_size > self.max_artifact_bytes
            || request.observed_size != m.artifact_size
        {
            return UpdCode::SizeInvalid;
        }

        // 5 — Integritaet: der beobachtete Hash muss der signierte sein
        if request.observed_sha256 != m.artifact_sha256 {
            return UpdCode::IntegrityMismatch;
        }

        // 6 — Signatur ueber die kanonischen Bytes
        let payload = m.canonical_bytes();
        if request.signature.is_empty() || !verifier.verify(&payload, &request.signature) {
            return UpdCode::SignatureInvalid;
        }

        // 7 — Ablauf
        if request.now_us > m.expires_at_us {
            return UpdCode::Expired;
        }

        // 8 — Laufzeit alt genug fuer dieses Update
        if request.boot_abi < m.min_boot_abi {
            return UpdCode::BootAbiTooOld;
        }

        // 9 — Version: neuer, oder Rueckschritt ausdruecklich erlaubt und freigegeben
        let current = request
            .installed_version
            .or_else(|| self.highest_installed.get(&m.component).copied());
        if let Some(current) = current {
            if m.version == current {
                return UpdCode::VersionRejected; // Replay
            }
            if m.version < current {
                let permitted = m.rollback_permitted
                    && self
                        .rollback_targets
                        .get(&m.component)
                        .map(|set| set.contains(&m.version))
                        .unwrap_or(false);
                if !permitted {
                    return UpdCode::VersionRejected;
                }
            }
        }

        // 10 — Widerruf von Schluessel oder Version
        if self.revoked_keys.contains(&m.signer) {
            return UpdCode::Revoked;
        }
        if self
            .revoked_versions
            .get(&m.component)
            .map(|set| set.contains(&m.version))
            .unwrap_or(false)
        {
            return UpdCode::Revoked;
        }

        // 11 — zulaessig
        UpdCode::Accepted
    }

    /// Nach einem fehlgeschlagenen Einspielen: Komponente unter Quarantaene
    /// stellen. Getrennt von `admit`, weil das Einspielen nicht hier passiert.
    pub fn record_failed_apply(&mut self, component: &str) {
        self.quarantined.insert(component.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> UpdateManifest {
        UpdateManifest {
            manifest_version: MANIFEST_VERSION,
            component: "shivacore".to_string(),
            version: 7,
            artifact_sha256: [9u8; 32],
            artifact_size: 1024,
            min_boot_abi: 2,
            expires_at_us: 1_000_000,
            rollback_permitted: false,
            signer: "signer-a".to_string(),
        }
    }

    fn request(m: UpdateManifest) -> Request {
        Request {
            observed_sha256: m.artifact_sha256,
            observed_size: m.artifact_size,
            manifest: m,
            signature: vec![1, 2, 3],
            installed_version: Some(6),
            boot_abi: 2,
            now_us: 500,
        }
    }

    fn authority() -> Authority {
        let mut a = Authority::new();
        a.allow_component("shivacore");
        a
    }

    #[test]
    fn gueltiges_update_wird_angenommen() {
        let mut a = authority();
        let out = a.admit(&request(manifest()), &AcceptAll);
        assert_eq!(out, Outcome::Accepted { version: 7 });
        assert_eq!(a.highest_installed("shivacore"), Some(7));
    }

    #[test]
    fn unbekannte_komponente_wird_abgelehnt() {
        let mut a = Authority::new();
        let out = a.admit(&request(manifest()), &AcceptAll);
        assert_eq!(
            out,
            Outcome::Rejected {
                code: UpdCode::UnknownComponent,
                rank: 2
            }
        );
    }

    #[test]
    fn quarantaene_sperrt_auch_gueltige_manifeste() {
        let mut a = authority();
        a.quarantine("shivacore");
        let out = a.admit(&request(manifest()), &AcceptAll);
        assert_eq!(
            out,
            Outcome::Rejected {
                code: UpdCode::Quarantined,
                rank: 3
            }
        );
        assert!(a.release_quarantine("shivacore"));
        assert!(a.admit(&request(manifest()), &AcceptAll).is_accepted());
    }

    #[test]
    fn groesse_null_oder_abweichend_wird_abgelehnt() {
        let mut a = authority();
        let mut r = request(manifest());
        r.observed_size = 1023;
        assert_eq!(
            a.admit(&r, &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::SizeInvalid,
                rank: 4
            }
        );

        let mut r2 = request(manifest());
        r2.manifest.artifact_size = 0;
        r2.observed_size = 0;
        assert!(matches!(
            a.admit(&r2, &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::SizeInvalid,
                ..
            }
        ));
    }

    #[test]
    fn hash_abweichung_ist_integritaetsbruch() {
        let mut a = authority();
        let mut r = request(manifest());
        r.observed_sha256 = [1u8; 32];
        assert_eq!(
            a.admit(&r, &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::IntegrityMismatch,
                rank: 5
            }
        );
    }

    #[test]
    fn leere_oder_falsche_signatur_wird_abgelehnt() {
        let mut a = authority();
        let mut r = request(manifest());
        r.signature = Vec::new();
        assert_eq!(
            a.admit(&r, &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::SignatureInvalid,
                rank: 6
            }
        );

        assert_eq!(
            a.admit(&request(manifest()), &RejectAll),
            Outcome::Rejected {
                code: UpdCode::SignatureInvalid,
                rank: 6
            }
        );
    }

    #[test]
    fn abgelaufenes_manifest_wird_abgelehnt() {
        let mut a = authority();
        let mut r = request(manifest());
        r.now_us = 1_000_001;
        assert_eq!(
            a.admit(&r, &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::Expired,
                rank: 7
            }
        );
    }

    #[test]
    fn zu_alte_laufzeit_wird_abgelehnt() {
        let mut a = authority();
        let mut r = request(manifest());
        r.boot_abi = 1;
        assert_eq!(
            a.admit(&r, &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::BootAbiTooOld,
                rank: 8
            }
        );
    }

    #[test]
    fn replay_der_installierten_version_wird_abgelehnt() {
        let mut a = authority();
        let mut r = request(manifest());
        r.installed_version = Some(7);
        assert_eq!(
            a.admit(&r, &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::VersionRejected,
                rank: 9
            }
        );
    }

    #[test]
    fn rueckschritt_nur_mit_flag_und_freigabe() {
        let mut a = authority();
        let mut m = manifest();
        m.version = 5;
        m.rollback_permitted = false;
        assert_eq!(
            a.admit(&request(m.clone()), &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::VersionRejected,
                rank: 9
            }
        );

        m.rollback_permitted = true;
        assert_eq!(
            a.admit(&request(m.clone()), &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::VersionRejected,
                rank: 9
            }
        );

        a.allow_rollback_target("shivacore", 5);
        assert!(a.admit(&request(m), &AcceptAll).is_accepted());
    }

    #[test]
    fn widerrufener_schluessel_und_widerrufene_version() {
        let mut a = authority();
        a.revoke_key("signer-a");
        assert_eq!(
            a.admit(&request(manifest()), &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::Revoked,
                rank: 10
            }
        );

        let mut b = authority();
        b.revoke_version("shivacore", 7);
        assert_eq!(
            b.admit(&request(manifest()), &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::Revoked,
                rank: 10
            }
        );
    }

    #[test]
    fn fehlgeschlagenes_einspielen_setzt_quarantaene() {
        let mut a = authority();
        a.record_failed_apply("shivacore");
        assert!(a.is_quarantined("shivacore"));
        assert!(matches!(
            a.admit(&request(manifest()), &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::Quarantined,
                ..
            }
        ));
    }

    #[test]
    fn genau_ein_audit_eintrag_je_entscheidung() {
        let mut a = authority();
        let _ = a.admit(&request(manifest()), &AcceptAll); // angenommen
        let mut bad = request(manifest());
        bad.observed_sha256 = [0u8; 32];
        let _ = a.admit(&bad, &AcceptAll); // abgelehnt
        assert_eq!(a.audit_log().len(), 2);
        assert_eq!(a.audit_log()[0].code, UpdCode::Accepted);
        assert_eq!(a.audit_log()[1].code, UpdCode::IntegrityMismatch);
        assert_eq!(a.audit_log()[1].rank, 5);
    }

    #[test]
    fn reihenfolge_der_fehlercodes_ist_normativ() {
        let codes = [
            UpdCode::ManifestUnreadable,
            UpdCode::UnknownComponent,
            UpdCode::Quarantined,
            UpdCode::SizeInvalid,
            UpdCode::IntegrityMismatch,
            UpdCode::SignatureInvalid,
            UpdCode::Expired,
            UpdCode::BootAbiTooOld,
            UpdCode::VersionRejected,
            UpdCode::Revoked,
            UpdCode::Accepted,
        ];
        for (index, code) in codes.iter().enumerate() {
            assert_eq!(code.rank() as usize, index + 1, "{code:?}");
        }
        assert_eq!(UpdCode::IntegrityMismatch.code(), "UPD-005");
    }

    #[test]
    fn kanonische_bytes_sind_deterministisch_und_domain_getrennt() {
        let a = manifest();
        let b = manifest();
        assert_eq!(a.canonical_bytes(), b.canonical_bytes());
        assert!(a.canonical_bytes().starts_with(DOMAIN_PREFIX));

        // Laengenpraefix verhindert Kollision: "ab"+"c" != "a"+"bc"
        let mut x = manifest();
        x.component = "ab".to_string();
        x.signer = "c".to_string();
        let mut y = manifest();
        y.component = "a".to_string();
        y.signer = "bc".to_string();
        assert_ne!(x.canonical_bytes(), y.canonical_bytes());
    }

    #[test]
    fn unbekannte_manifestversion_wird_abgelehnt() {
        let mut a = authority();
        let mut m = manifest();
        m.manifest_version = 2;
        assert_eq!(
            a.admit(&request(m), &AcceptAll),
            Outcome::Rejected {
                code: UpdCode::ManifestUnreadable,
                rank: 1
            }
        );
    }

    #[test]
    fn harte_obergrenze_laesst_sich_nicht_erhoehen() {
        let a = Authority::new().with_max_artifact_bytes(u64::MAX);
        assert_eq!(a.max_artifact_bytes, MAX_ARTIFACT_BYTES);
    }
}
