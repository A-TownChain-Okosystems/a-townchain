// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! S1-2 / F-1: Registrierung ist Compare-and-Insert, nicht Last-Write-Wins.
//!
//! Abnahmekriterien S1-2-T-01, T-02, T-04, T-07, T-08
//! ([EXEC-S1-2-ADMISSION-REGISTRY-F1.md](../../../docs/decisions/EXEC-S1-2-ADMISSION-REGISTRY-F1.md)).
//! Status: `IMPLEMENTED`, lokal geprueft. Kein `CI_VERIFIED`.

use atc_exec_admission::{
    Authority, Component, Decision, ExecutionPolicy, IsolationState, Registry, RegistryError,
    Request, Revocation, Scope, SignatureVerifier,
};

const CODE: [u8; 32] = [0x11; 32];
const ANDERER_CODE: [u8; 32] = [0x22; 32];
const NOW: i64 = 1_700_000_000_000_000;

fn policy(code_hash: [u8; 32], herkunft: &str) -> ExecutionPolicy {
    ExecutionPolicy {
        code_hash,
        signer_identity: "owner-1".to_string(),
        source_provenance: Some(herkunft.to_string()),
        generation_proof: None,
        component_identity: Component::AtcVm,
        version: 1,
        requested_capabilities: 0,
        execution_scope: Scope::Contract,
        expiration_us: NOW + 86_400_000_000,
        revocation_status: Revocation::Active,
    }
}

struct ImmerGueltig;

impl SignatureVerifier for ImmerGueltig {
    fn verify(&self, _bytes: &[u8], _signature: &[u8]) -> bool {
        true
    }
}

fn beurteile(registry: &Registry, herkunft: &str) -> Decision {
    let policy = policy(CODE, herkunft);
    let mut authority = Authority::new(IsolationState::Available);
    authority.authorize_signer("owner-1");
    authority.set_ceiling(Component::AtcVm, u16::MAX);
    let (mut request, bytes) = Request::unsigned(&policy, Component::AtcVm, 0, NOW);
    request.signature = Some(Box::leak(bytes.into_boxed_slice()));
    authority.admit(registry, &request, &ImmerGueltig)
}

/// S1-2-T-01 — gleiche Policy zweimal: idempotent, keine Wirkung.
#[test]
fn t01_bytegleiche_policy_ist_idempotent() {
    let mut registry = Registry::new();
    registry.register(policy(CODE, "atclang")).expect("erste");
    let vorher = registry.len();
    let kanonisch = registry.get(&CODE).expect("registriert").canonical_bytes();

    for _ in 0..3 {
        registry
            .register(policy(CODE, "atclang"))
            .expect("Wiederholung ist idempotent");
    }

    assert_eq!(registry.len(), vorher, "kein zweiter Eintrag");
    assert_eq!(
        registry.get(&CODE).expect("registriert").canonical_bytes(),
        kanonisch,
        "die registrierte Policy bleibt bytegleich"
    );
}

/// S1-2-T-02 — gleicher `code_hash`, andere Herkunft: Konflikt, nichts geschrieben.
#[test]
fn t02_abweichende_policy_ergibt_konflikt_und_aendert_nichts() {
    let mut registry = Registry::new();
    registry.register(policy(CODE, "atclang")).expect("erste");
    let vorher = registry.get(&CODE).expect("registriert").clone();

    let fehler = registry
        .register(policy(CODE, "atce"))
        .expect_err("abweichende Policy muss abgelehnt werden");

    match fehler {
        RegistryError::PolicyConflict {
            code_hash,
            existing,
            rejected,
        } => {
            assert_eq!(code_hash, CODE);
            assert_ne!(existing, rejected, "beide Bytesaetze sind belegt");
            assert_eq!(existing, vorher.canonical_bytes());
        }
    }
    assert_eq!(registry.len(), 1, "kein zweiter Eintrag");
    assert_eq!(
        registry.get(&CODE).expect("registriert"),
        &vorher,
        "die zuerst registrierte Policy bleibt unveraendert"
    );
}

/// S1-2-T-04 — Reihenfolge umgekehrt: derselbe Konflikt, symmetrisch.
#[test]
fn t04_konflikt_ist_symmetrisch() {
    let mut registry = Registry::new();
    registry.register(policy(CODE, "atce")).expect("erste");

    let fehler = registry
        .register(policy(CODE, "atclang"))
        .expect_err("auch die andere Reihenfolge muss ablehnen");

    assert!(matches!(fehler, RegistryError::PolicyConflict { .. }));
    assert_eq!(
        registry
            .get(&CODE)
            .expect("registriert")
            .source_provenance
            .as_deref(),
        Some("atce"),
        "die Erstregistrierung gewinnt, unabhaengig von der Herkunft"
    );
}

/// S1-2-T-07 — verschiedene Programme: beide werden registriert.
#[test]
fn t07_verschiedene_programme_kollidieren_nicht() {
    let mut registry = Registry::new();
    registry.register(policy(CODE, "atclang")).expect("A");
    registry
        .register(policy(ANDERER_CODE, "atclang"))
        .expect("B");

    assert_eq!(registry.len(), 2);
    assert!(registry.get(&CODE).is_some());
    assert!(registry.get(&ANDERER_CODE).is_some());
}

/// S1-2-T-08 — der abgewiesene Eintrag ist reproduzierbar belegt.
#[test]
fn t08_konflikt_traegt_deterministische_identitaet() {
    let mut registry = Registry::new();
    registry.register(policy(CODE, "atclang")).expect("erste");

    let a = registry.register(policy(CODE, "atce")).unwrap_err();
    let b = registry.register(policy(CODE, "atce")).unwrap_err();

    assert_eq!(a, b, "derselbe Konflikt ergibt dieselbe Identitaet");
    let text = format!("{a}");
    assert!(
        text.contains("Policy-Konflikt") && text.contains("11"),
        "der Text nennt Art und Hash: {text}"
    );
    assert_eq!(registry.len(), 1, "die Ablehnung schreibt nichts");

    // Die registrierte (erste) Policy bleibt entscheidungsfaehig.
    assert!(beurteile(&registry, "atclang").is_authorized());
    // Eine Anfrage mit der abgelehnten Policy wird abgewiesen.
    assert!(!beurteile(&registry, "atce").is_authorized());
}
