// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ══════════════════════════════════════════════════════════════════════════
// ATC-SEC-EXEC-001 — Referenz-Durchsetzung der Admission-Entscheidung.
//
// KANDIDAT. Dieses Crate ist reine Entscheidungslogik: keine Kernel-Bindung,
// keine ABI-Änderung, kein Loader, keine VM-Anbindung. Es macht die
// Prüfreihenfolge aus `standards/atc/candidates/ATC-SEC-EXEC-001-CANDIDATE.md`
// (Abschnitt 4) ausführbar und damit prüfbar, BEVOR eine Grenze verdrahtet wird.
//
// Bewusste Grenzen:
//  * Kryptografie ist NICHT hier. Die Signaturprüfung kommt über den Trait
//    `SignatureVerifier` von der Runtime; dieses Crate behauptet keine
//    kryptografische Sicherheit, es ordnet Entscheidungen zu.
//  * Es gibt keinen "Trust on first use"-Pfad und keinen Rückfall auf
//    "unsigniert, aber lokal vorhanden": jeder unbekannte Zustand ist BLOCKED.
// ══════════════════════════════════════════════════════════════════════════

use std::collections::{BTreeMap, BTreeSet};

/// Domain-Trennung der Signaturbasis (normativ, Abschnitt 3.2 des Standards).
pub const POLICY_DOMAIN: &[u8] = b"ATC-EXEC-ADM-v1\0";

/// Rechte als Bitmaske, gleiche Breite wie `Rights` im Kernel (u16).
pub type Rights = u16;

/// Die Komponente, an deren Grenze die Ausführung stattfinden soll.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Component {
    Kernel,
    AtcVm,
    Node,
    GlobusInit,
    ServiceSpace,
    AiTool,
    GenesisGenerated,
    Marketplace,
}

impl Component {
    fn label(self) -> &'static str {
        match self {
            Component::Kernel => "kernel",
            Component::AtcVm => "atc-vm",
            Component::Node => "node",
            Component::GlobusInit => "globus-init",
            Component::ServiceSpace => "service-space",
            Component::AiTool => "ai-tool",
            Component::GenesisGenerated => "genesis-generated",
            Component::Marketplace => "marketplace",
        }
    }
}

/// Ausführungsumfang einer Freigabe.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scope {
    Boot,
    Service,
    Contract,
    Tool,
    OneShot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Revocation {
    Active,
    Revoked,
}

/// Die Freigabe selbst — neun Felder, wie im Standard festgelegt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionPolicy {
    pub code_hash: [u8; 32],
    pub signer_identity: String,
    pub source_provenance: Option<String>,
    /// Nur für erzeugten Code (Genesis Engine): Nachweis, woraus er entstand.
    pub generation_proof: Option<String>,
    pub component_identity: Component,
    pub version: u64,
    pub requested_capabilities: Rights,
    pub execution_scope: Scope,
    /// Unix-Mikrosekunden, ganzzahlig. Kein Float (nicht byte-stabil).
    pub expiration_us: i64,
    pub revocation_status: Revocation,
}

impl ExecutionPolicy {
    /// Kanonische Bytefolge: Domain-Präfix, feste Feldreihenfolge, jedes
    /// variable Feld mit u32-Längenpräfix.
    ///
    /// Ohne Längenpräfix wäre `signer="AB", provenance="C"` byte-gleich zu
    /// `signer="A", provenance="BC"` — genau der Fehler, der im
    /// RemoteCapabilityTicket behoben werden musste.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(POLICY_DOMAIN);
        out.extend_from_slice(&self.code_hash);
        push_str(&mut out, &self.signer_identity);
        push_opt(&mut out, self.source_provenance.as_deref());
        push_opt(&mut out, self.generation_proof.as_deref());
        push_str(&mut out, self.component_identity.label());
        out.extend_from_slice(&self.version.to_le_bytes());
        out.extend_from_slice(&self.requested_capabilities.to_le_bytes());
        out.extend_from_slice(&(self.execution_scope as u8).to_le_bytes());
        out.extend_from_slice(&self.expiration_us.to_le_bytes());
        out.extend_from_slice(&[(self.revocation_status as u8)]);
        out
    }
}

fn push_str(out: &mut Vec<u8>, value: &str) {
    let bytes = value.as_bytes();
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(bytes);
}

fn push_opt(out: &mut Vec<u8>, value: Option<&str>) {
    match value {
        None => out.extend_from_slice(&u32::MAX.to_le_bytes()),
        Some(text) => push_str(out, text),
    }
}

/// Fehlercodes aus Abschnitt 4. Die Reihenfolge der Varianten ist die
/// Prüfreihenfolge; `rank()` macht sie vergleichbar.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AdmCode {
    /// 1 — Authority nicht erreichbar oder nicht initialisiert.
    AuthorityUnavailable,
    /// 2 — Policy strukturell ungültig oder nicht kanonisch codiert.
    NonCanonicalPolicy,
    /// 3 — Code Unit unbekannt.
    UnknownCode,
    /// 4 — Integrität stimmt nicht mit `code_hash` überein.
    IntegrityMismatch,
    /// 5 — Signatur ungültig oder Signer nicht autorisiert.
    SignerNotAuthorized,
    /// 6 — Komponente nicht in der Freigabe enthalten.
    ComponentNotAllowed,
    /// 7 — Rechte über die Freigabe hinaus.
    CapabilityNotGranted,
    /// 8 — abgelaufen oder Version hinter der Sperre.
    ExpiredOrRolledBack,
    /// 9 — widerrufen.
    Revoked,
    /// 10 — Provenance unvollständig.
    ProvenanceMissing,
    /// 11 — Isolation nicht verfügbar.
    IsolationUnavailable,
}

impl AdmCode {
    pub fn code(self) -> &'static str {
        match self {
            AdmCode::AuthorityUnavailable => "ADM-010",
            AdmCode::NonCanonicalPolicy => "ADM-009",
            AdmCode::UnknownCode => "ADM-001",
            AdmCode::IntegrityMismatch => "ADM-002",
            AdmCode::SignerNotAuthorized => "ADM-003",
            AdmCode::ComponentNotAllowed => "ADM-004",
            AdmCode::CapabilityNotGranted => "ADM-005",
            AdmCode::ExpiredOrRolledBack => "ADM-006",
            AdmCode::Revoked => "ADM-007",
            AdmCode::ProvenanceMissing => "ADM-008",
            AdmCode::IsolationUnavailable => "ADM-011",
        }
    }

    /// Prüfschritt 1..11. Zwei Entscheidungen werden nur dann verglichen, wenn
    /// sie über denselben Schritt entstanden sind.
    pub fn rank(self) -> u8 {
        match self {
            AdmCode::AuthorityUnavailable => 1,
            AdmCode::NonCanonicalPolicy => 2,
            AdmCode::UnknownCode => 3,
            AdmCode::IntegrityMismatch => 4,
            AdmCode::SignerNotAuthorized => 5,
            AdmCode::ComponentNotAllowed => 6,
            AdmCode::CapabilityNotGranted => 7,
            AdmCode::ExpiredOrRolledBack => 8,
            AdmCode::Revoked => 9,
            AdmCode::ProvenanceMissing => 10,
            AdmCode::IsolationUnavailable => 11,
        }
    }
}

/// Die Registry der Freigaben. Sie ist die SSOT für `code_hash` → Policy.
#[derive(Default)]
pub struct Registry {
    policies: BTreeMap<[u8; 32], ExecutionPolicy>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, policy: ExecutionPolicy) {
        self.policies.insert(policy.code_hash, policy);
    }

    pub fn get(&self, code_hash: &[u8; 32]) -> Option<&ExecutionPolicy> {
        self.policies.get(code_hash)
    }

    pub fn len(&self) -> usize {
        self.policies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.policies.is_empty()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IsolationState {
    Available,
    Unavailable,
}

/// Die Anfrage einer Grenze an die Authority.
pub struct Request<'a> {
    pub code_hash: [u8; 32],
    /// Hash der tatsächlich vorliegenden Artefaktbytes (nach dem Laden berechnet).
    pub observed_hash: [u8; 32],
    /// Bytes, aus denen die Policy gelesen wurde — für die Kanonizitätsprüfung.
    pub policy_bytes: Vec<u8>,
    pub component: Component,
    /// Rechte, die die Runtime jetzt tatsächlich anfordert.
    pub requested_capabilities: Rights,
    pub signature: Option<&'a [u8]>,
    pub now_us: i64,
}

impl Request<'_> {
    /// Bequemer Bau für Tests und Aufrufer ohne Signaturpfad.
    pub fn unsigned(
        policy: &ExecutionPolicy,
        component: Component,
        requested_capabilities: Rights,
        now_us: i64,
    ) -> (Request<'static>, Vec<u8>) {
        let bytes = policy.canonical_bytes();
        (
            Request {
                code_hash: policy.code_hash,
                observed_hash: policy.code_hash,
                policy_bytes: bytes.clone(),
                component,
                requested_capabilities,
                signature: None,
                now_us,
            },
            bytes,
        )
    }
}

/// Signaturprüfung kommt von der Runtime. Dieses Crate erfindet keine Krypto.
pub trait SignatureVerifier {
    fn verify(&self, policy_bytes: &[u8], signature: &[u8]) -> bool;
}

/// Verifier, der alles akzeptiert — nur für Tests der Entscheidungslogik.
pub struct AcceptAll;

impl SignatureVerifier for AcceptAll {
    fn verify(&self, _policy_bytes: &[u8], _signature: &[u8]) -> bool {
        true
    }
}

pub struct RejectAll;

impl SignatureVerifier for RejectAll {
    fn verify(&self, _policy_bytes: &[u8], _signature: &[u8]) -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    Authorized,
    Blocked,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Decision {
    pub outcome: Outcome,
    pub code: Option<AdmCode>,
    pub detail: String,
}

impl Decision {
    fn allow() -> Self {
        Decision {
            outcome: Outcome::Authorized,
            code: None,
            detail: "Freigabe gueltig, Ausführung unter den erteilten Rechten".into(),
        }
    }

    fn block(code: AdmCode, detail: impl Into<String>) -> Self {
        Decision {
            outcome: Outcome::Blocked,
            code: Some(code),
            detail: detail.into(),
        }
    }

    pub fn is_authorized(&self) -> bool {
        self.outcome == Outcome::Authorized
    }
}

/// Ein Protokolleintrag. Append-only; jede Entscheidung erzeugt genau einen.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AuditEntry {
    pub now_us: i64,
    pub code_hash: [u8; 32],
    pub component: Component,
    pub authorized: bool,
    pub code: Option<&'static str>,
    pub detail: String,
}

/// Die Authority: autorisierte Signierer, Rechtsobergrenzen je Komponente,
/// Versionssperren, Isolation und Widerrufe.
pub struct Authority {
    initialized: bool,
    authorized_signers: BTreeSet<String>,
    ceilings: BTreeMap<Component, Rights>,
    min_versions: BTreeMap<[u8; 32], u64>,
    revoked: BTreeSet<[u8; 32]>,
    isolation: IsolationState,
    log: Vec<AuditEntry>,
}

impl Authority {
    pub fn new(isolation: IsolationState) -> Self {
        Authority {
            initialized: true,
            authorized_signers: BTreeSet::new(),
            ceilings: BTreeMap::new(),
            min_versions: BTreeMap::new(),
            revoked: BTreeSet::new(),
            isolation,
            log: Vec::new(),
        }
    }

    /// Ausgangszustand vor der Initialisierung: alles wird abgelehnt.
    pub fn uninitialized() -> Self {
        let mut authority = Authority::new(IsolationState::Available);
        authority.initialized = false;
        authority
    }

    pub fn authorize_signer(&mut self, signer: impl Into<String>) {
        self.authorized_signers.insert(signer.into());
    }

    pub fn set_ceiling(&mut self, component: Component, rights: Rights) {
        self.ceilings.insert(component, rights);
    }

    pub fn lock_min_version(&mut self, code_hash: [u8; 32], version: u64) {
        self.min_versions.insert(code_hash, version);
    }

    pub fn revoke(&mut self, code_hash: [u8; 32]) {
        self.revoked.insert(code_hash);
    }

    pub fn set_isolation(&mut self, isolation: IsolationState) {
        self.isolation = isolation;
    }

    pub fn log(&self) -> &[AuditEntry] {
        &self.log
    }

    /// Die Prüfung in der normativen Reihenfolge 1..11, fail-closed.
    pub fn admit(
        &mut self,
        registry: &Registry,
        request: &Request<'_>,
        verifier: &dyn SignatureVerifier,
    ) -> Decision {
        let decision = self.evaluate(registry, request, verifier);
        self.log.push(AuditEntry {
            now_us: request.now_us,
            code_hash: request.code_hash,
            component: request.component,
            authorized: decision.is_authorized(),
            code: decision.code.map(AdmCode::code),
            detail: decision.detail.clone(),
        });
        decision
    }

    fn evaluate(
        &self,
        registry: &Registry,
        request: &Request<'_>,
        verifier: &dyn SignatureVerifier,
    ) -> Decision {
        // 1 — Authority erreichbar und initialisiert?
        if !self.initialized {
            return Decision::block(
                AdmCode::AuthorityUnavailable,
                "Authority nicht initialisiert",
            );
        }

        // 3 — Code Unit bekannt? (Vor Schritt 2 geprüft wird die Policy selbst.)
        let Some(policy) = registry.get(&request.code_hash) else {
            return Decision::block(
                AdmCode::UnknownCode,
                format!("Hash {:02x?} nicht registriert", &request.code_hash[..4]),
            );
        };

        // 2 — strukturell gültig und kanonisch codiert?
        if policy.canonical_bytes() != request.policy_bytes {
            return Decision::block(
                AdmCode::NonCanonicalPolicy,
                "Policy-Bytes weichen von der kanonischen Codierung ab",
            );
        }

        // 4 — Integrität.
        if request.observed_hash != policy.code_hash {
            return Decision::block(
                AdmCode::IntegrityMismatch,
                "Artefakt-Hash stimmt nicht mit der Freigabe ueberein",
            );
        }

        // 5 — Signatur gültig und Signer autorisiert?
        let Some(signature) = request.signature else {
            return Decision::block(AdmCode::SignerNotAuthorized, "keine Signatur vorgelegt");
        };
        if !verifier.verify(&request.policy_bytes, signature) {
            return Decision::block(AdmCode::SignerNotAuthorized, "Signatur ungueltig");
        }
        if !self.authorized_signers.contains(&policy.signer_identity) {
            return Decision::block(
                AdmCode::SignerNotAuthorized,
                format!("Signer '{}' nicht autorisiert", policy.signer_identity),
            );
        }

        // 6 — Komponente in der Freigabe enthalten?
        if request.component != policy.component_identity {
            return Decision::block(
                AdmCode::ComponentNotAllowed,
                format!(
                    "Freigabe gilt fuer '{}', angefragt '{}'",
                    policy.component_identity.label(),
                    request.component.label()
                ),
            );
        }

        // 7 — Rechte im Rahmen: Anfrage ⊆ Freigabe ⊆ Obergrenze der Komponente.
        let ceiling = self.ceilings.get(&request.component).copied().unwrap_or(0);
        if request.requested_capabilities & !policy.requested_capabilities != 0 {
            return Decision::block(
                AdmCode::CapabilityNotGranted,
                "angeforderte Rechte uebersteigen die Freigabe",
            );
        }
        if policy.requested_capabilities & !ceiling != 0 {
            return Decision::block(
                AdmCode::CapabilityNotGranted,
                "Freigabe uebersteigt die Rechtsobergrenze der Komponente",
            );
        }

        // 8 — Ablauf und Rollback-Schutz.
        if request.now_us >= policy.expiration_us {
            return Decision::block(AdmCode::ExpiredOrRolledBack, "Freigabe abgelaufen");
        }
        if let Some(min) = self.min_versions.get(&policy.code_hash) {
            if policy.version < *min {
                return Decision::block(
                    AdmCode::ExpiredOrRolledBack,
                    format!(
                        "Version {} hinter gesperrter Version {}",
                        policy.version, min
                    ),
                );
            }
        }

        // 9 — Widerruf.
        if policy.revocation_status == Revocation::Revoked
            || self.revoked.contains(&policy.code_hash)
        {
            return Decision::block(AdmCode::Revoked, "Freigabe widerrufen");
        }

        // 10 — Provenance, bei erzeugtem Code mit Generierungsnachweis.
        if policy.source_provenance.is_none() {
            return Decision::block(AdmCode::ProvenanceMissing, "source_provenance fehlt");
        }
        if policy.component_identity == Component::GenesisGenerated
            && policy.generation_proof.is_none()
        {
            return Decision::block(
                AdmCode::ProvenanceMissing,
                "erzeugter Code ohne Generierungsnachweis",
            );
        }

        // 11 — Isolation verfügbar?
        if self.isolation != IsolationState::Available {
            return Decision::block(
                AdmCode::IsolationUnavailable,
                "Isolation nicht verfuegbar, Ausführung untersagt",
            );
        }

        Decision::allow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i64 = 1_700_000_000_000_000;

    fn policy() -> ExecutionPolicy {
        ExecutionPolicy {
            code_hash: [7u8; 32],
            signer_identity: "did:atc:shivacore".into(),
            source_provenance: Some("repo:a-townchain/components/vm@abc123".into()),
            generation_proof: None,
            component_identity: Component::AtcVm,
            version: 3,
            requested_capabilities: 0b0000_0011,
            execution_scope: Scope::Contract,
            expiration_us: NOW + 86_400_000_000,
            revocation_status: Revocation::Active,
        }
    }

    fn registry_with(policy: ExecutionPolicy) -> Registry {
        let mut registry = Registry::new();
        registry.insert(policy);
        registry
    }

    fn authority() -> Authority {
        let mut authority = Authority::new(IsolationState::Available);
        authority.authorize_signer("did:atc:shivacore");
        authority.set_ceiling(Component::AtcVm, 0x1FF);
        authority
    }

    // 1 — gültige Freigabe
    #[test]
    fn gueltige_freigabe_wird_ausgefuehrt() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0b1, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert!(decision.is_authorized(), "{decision:?}");
        assert_eq!(decision.code, None);
    }

    // 2 — unbekannter Code
    #[test]
    fn unbekannter_code_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (request, _) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let mut request = request;
        request.code_hash = [9u8; 32];
        let decision = authority.admit(&registry, &request, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::UnknownCode));
    }

    // 3 — nach dem Signieren verändert (abweichende Codierung)
    #[test]
    fn abweichende_codierung_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (mut request, mut bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        // ein Byte in der Provenance kippen: gleiche Semantik-Behauptung, andere Bytes
        let last = bytes.len() - 1;
        bytes[last] ^= 0x01;
        request.policy_bytes = bytes;
        request.signature = Some(b"sig");
        let decision = authority.admit(&registry, &request, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::NonCanonicalPolicy));
    }

    // 4 — Byte nach dem Signieren im Artefakt verändert
    #[test]
    fn manipulierte_integritaet_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (mut request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        request.observed_hash = [0xAB; 32];
        request.policy_bytes = bytes;
        request.signature = Some(b"sig");
        let decision = authority.admit(&registry, &request, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::IntegrityMismatch));
    }

    // 5a — keine Signatur
    #[test]
    fn fehlende_signatur_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (request, _) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let decision = authority.admit(&registry, &request, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::SignerNotAuthorized));
    }

    // 5b — Signatur ungültig
    #[test]
    fn ungueltige_signatur_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &RejectAll);
        assert_eq!(decision.code, Some(AdmCode::SignerNotAuthorized));
    }

    // 5c — gültige Signatur, aber fremder Signer
    #[test]
    fn fremder_signer_wird_blockiert() {
        let mut policy = policy();
        policy.signer_identity = "did:atc:fremder".into();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::SignerNotAuthorized));
    }

    // 6 — Freigabe gilt für eine andere Komponente
    #[test]
    fn fremde_komponente_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        authority.set_ceiling(Component::Kernel, 0x1FF);
        let (request, bytes) = Request::unsigned(&policy, Component::Kernel, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::ComponentNotAllowed));
    }

    // 7a — angeforderte Rechte über der Freigabe
    #[test]
    fn rechte_ueber_der_freigabe_werden_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0b1000, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::CapabilityNotGranted));
    }

    // 7b — Freigabe über der Obergrenze der Komponente
    #[test]
    fn freigabe_ueber_komponenten_obergrenze_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = Authority::new(IsolationState::Available);
        authority.authorize_signer("did:atc:shivacore");
        authority.set_ceiling(Component::AtcVm, 0b1); // enger als die Freigabe
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::CapabilityNotGranted));
    }

    // 8a — abgelaufen
    #[test]
    fn abgelaufene_freigabe_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let late = policy.expiration_us + 1;
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, late);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::ExpiredOrRolledBack));
    }

    // 8b — Version hinter der Sperre (Rollback)
    #[test]
    fn rollback_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        authority.lock_min_version(policy.code_hash, policy.version + 1);
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::ExpiredOrRolledBack));
    }

    // 9 — widerrufen
    #[test]
    fn widerrufene_freigabe_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        authority.revoke(policy.code_hash);
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::Revoked));
    }

    // 10a — keine Herkunft
    #[test]
    fn fehlende_provenance_wird_blockiert() {
        let mut policy = policy();
        policy.source_provenance = None;
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::ProvenanceMissing));
    }

    // 10b — generierter Code ohne Generierungsnachweis
    #[test]
    fn generierter_code_ohne_nachweis_wird_blockiert() {
        let mut policy = policy();
        policy.component_identity = Component::GenesisGenerated;
        let registry = registry_with(policy.clone());
        let mut authority = Authority::new(IsolationState::Available);
        authority.authorize_signer("did:atc:shivacore");
        authority.set_ceiling(Component::GenesisGenerated, 0x1FF);
        let (request, bytes) = Request::unsigned(&policy, Component::GenesisGenerated, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::ProvenanceMissing));
    }

    // 11 — Isolation fehlt
    #[test]
    fn fehlende_isolation_wird_blockiert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        authority.set_isolation(IsolationState::Unavailable);
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::IsolationUnavailable));
    }

    // 12 — Authority nicht initialisiert: alles zu
    #[test]
    fn nicht_initialisierte_authority_blockiert_alles() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = Authority::uninitialized();
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::AuthorityUnavailable));
    }

    // 13 — Protokollpflicht: jede Entscheidung genau ein Eintrag
    #[test]
    fn jede_entscheidung_wird_protokolliert() {
        let policy = policy();
        let registry = registry_with(policy.clone());
        let mut authority = authority();
        let (ok_request, ok_bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        authority.admit(
            &registry,
            &Request {
                policy_bytes: ok_bytes,
                signature: Some(b"sig"),
                ..ok_request
            },
            &AcceptAll,
        );
        let (bad_request, _) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        authority.admit(&registry, &bad_request, &AcceptAll);

        assert_eq!(authority.log().len(), 2, "{:?}", authority.log());
        assert!(authority.log()[0].authorized);
        assert_eq!(authority.log()[0].code, None);
        assert!(!authority.log()[1].authorized);
        assert_eq!(authority.log()[1].code, Some("ADM-003"));
    }

    // 14 — Fehlercodes liegen auf der normativen Prüfreihenfolge
    #[test]
    fn fehlercodes_folgen_der_pruefreihenfolge() {
        let ordered = [
            AdmCode::AuthorityUnavailable,
            AdmCode::NonCanonicalPolicy,
            AdmCode::UnknownCode,
            AdmCode::IntegrityMismatch,
            AdmCode::SignerNotAuthorized,
            AdmCode::ComponentNotAllowed,
            AdmCode::CapabilityNotGranted,
            AdmCode::ExpiredOrRolledBack,
            AdmCode::Revoked,
            AdmCode::ProvenanceMissing,
            AdmCode::IsolationUnavailable,
        ];
        for (index, code) in ordered.iter().enumerate() {
            assert_eq!(code.rank() as usize, index + 1, "{code:?}");
        }
        assert_eq!(AdmCode::UnknownCode.code(), "ADM-001");
        assert_eq!(AdmCode::IsolationUnavailable.code(), "ADM-011");
    }

    // 15 — Kanonische Codierung: keine Kollision ohne Längenpräfix
    #[test]
    fn laengenpraefix_verhindert_kollision() {
        let mut a = policy();
        a.signer_identity = "AB".into();
        a.source_provenance = Some("C".into());
        let mut b = policy();
        b.signer_identity = "A".into();
        b.source_provenance = Some("BC".into());
        assert_ne!(a.canonical_bytes(), b.canonical_bytes());

        // Gleiche Semantik muss byte-gleich sein.
        let c = policy();
        let d = policy();
        assert_eq!(c.canonical_bytes(), d.canonical_bytes());

        // Ablaufdatum ist ganzzahlig und damit byte-stabil.
        let mut e = policy();
        e.expiration_us = 0;
        let mut f = policy();
        f.expiration_us = 0;
        assert_eq!(e.canonical_bytes(), f.canonical_bytes());
    }

    // 16 — ohne Registry ist nichts bekannt (kein Rückfall)
    #[test]
    fn leere_registry_blockiert() {
        let policy = policy();
        let registry = Registry::new();
        let mut authority = authority();
        let (request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
        let signed = Request {
            policy_bytes: bytes,
            signature: Some(b"sig"),
            ..request
        };
        let decision = authority.admit(&registry, &signed, &AcceptAll);
        assert_eq!(decision.code, Some(AdmCode::UnknownCode));
        assert!(registry.is_empty());
    }
}
