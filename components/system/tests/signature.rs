// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Gate C — die Signaturpruefung ist vollstaendig.
//!
//! Zwei Saetze von Aussagen, die hier getrennt bleiben:
//!  * Kryptografie: die Signatur deckt genau die kanonischen Policy-Bytes und
//!    nur diese (Domain-Trennung, Laengenpraefixe, kompakte Kodierung).
//!  * Berechtigung: eine gueltige Signatur beweist die Kontrolle ueber einen
//!    Schluessel, **nicht** die Erlaubnis. Erst die Autorisierung des Signierers
//!    in der Authority entscheidet.

use atc_exec_admission::{
    Authority, Component, ExecutionPolicy, IsolationState, Registry, Request, Revocation, Scope,
    SignatureVerifier,
};
use atc_system::{hex, AuthorizedKeyVerifier, SealKey};
use atc_vm::artifact::sha256_daten;

const NOW_US: i64 = 1_800_000_000_000_000;
const RIGHTS: u16 = 0b11;

fn policy() -> ExecutionPolicy {
    ExecutionPolicy {
        code_hash: sha256_daten(b"vertrag-1"),
        signer_identity: String::new(), // wird gesetzt
        source_provenance: Some("atclang:4ops".into()),
        generation_proof: None,
        component_identity: Component::AtcVm,
        version: 1,
        requested_capabilities: RIGHTS,
        execution_scope: Scope::Contract,
        expiration_us: NOW_US + 60_000_000,
        revocation_status: Revocation::Active,
    }
}

struct Aufbau {
    registry: Registry,
    authority: Authority,
    verifier: AuthorizedKeyVerifier,
    policy: ExecutionPolicy,
    signature: [u8; 64],
    schluessel: SealKey,
}

fn aufbau() -> Aufbau {
    let schluessel = SealKey::from_seed([7u8; 32]).expect("Schluessel");
    let mut policy = policy();
    policy.signer_identity = schluessel.address();
    let signature = schluessel.sign_policy(&policy);
    let verifier = AuthorizedKeyVerifier::new(schluessel.public_key()).expect("Verifier");

    let mut registry = Registry::new();
    registry
        .register(policy.clone())
        .expect("erste Registrierung");

    let mut authority = Authority::new(IsolationState::Available);
    authority.authorize_signer(policy.signer_identity.clone());
    authority.set_ceiling(Component::AtcVm, RIGHTS);

    Aufbau {
        registry,
        authority,
        verifier,
        policy,
        signature,
        schluessel,
    }
}

fn request<'a>(
    policy: &ExecutionPolicy,
    signature: Option<&'a [u8]>,
    component: Component,
    rights: u16,
    observed: [u8; 32],
    now_us: i64,
) -> Request<'a> {
    Request {
        code_hash: policy.code_hash,
        observed_hash: observed,
        policy_bytes: policy.canonical_bytes(),
        component,
        requested_capabilities: rights,
        signature,
        now_us,
    }
}

#[test]
fn gueltige_freigabe_wird_erteilt() {
    let mut a = aufbau();
    let request = request(
        &a.policy,
        Some(&a.signature),
        Component::AtcVm,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    let decision = a.authority.admit(&a.registry, &request, &a.verifier);
    assert!(decision.is_authorized(), "{decision:?}");
    assert_eq!(a.authority.log().len(), 1);
}

#[test]
fn ohne_signatur_wird_blockiert() {
    let mut a = aufbau();
    let request = request(
        &a.policy,
        None,
        Component::AtcVm,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    let decision = a.authority.admit(&a.registry, &request, &a.verifier);
    assert!(!decision.is_authorized());
    assert_eq!(decision.code.map(|c| c.code()), Some("ADM-003"));
}

#[test]
fn nullsignatur_wird_blockiert() {
    let mut a = aufbau();
    let nullen = [0u8; 64];
    let request = request(
        &a.policy,
        Some(&nullen),
        Component::AtcVm,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    let decision = a.authority.admit(&a.registry, &request, &a.verifier);
    assert!(!decision.is_authorized());
    assert_eq!(decision.code.map(|c| c.code()), Some("ADM-003"));
}

#[test]
fn zu_kurze_und_der_kodierte_signatur_werden_abgelehnt_ohne_panik() {
    let mut a = aufbau();
    let kurz = &a.signature[..63];
    let request_kurz = request(
        &a.policy,
        Some(kurz),
        Component::AtcVm,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    assert!(!a
        .authority
        .admit(&a.registry, &request_kurz, &a.verifier)
        .is_authorized());

    // DER-Kodierung statt kompakter Form: gleiche Signatur, falsches Format.
    let gepolstert = vec![0u8; 70];
    let request_der = request(
        &a.policy,
        Some(&gepolstert),
        Component::AtcVm,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    assert!(!a
        .authority
        .admit(&a.registry, &request_der, &a.verifier)
        .is_authorized());
}

#[test]
fn fremder_schluessel_wird_blockiert() {
    let mut a = aufbau();
    let fremd = SealKey::from_seed([9u8; 32]).expect("Schluessel");
    let fremder_verifier = AuthorizedKeyVerifier::new(fremd.public_key()).unwrap();
    let request = request(
        &a.policy,
        Some(&a.signature),
        Component::AtcVm,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    let decision = a.authority.admit(&a.registry, &request, &fremder_verifier);
    assert!(!decision.is_authorized());
    assert_eq!(decision.code.map(|c| c.code()), Some("ADM-003"));
}

#[test]
fn signatur_ist_an_genau_diese_policy_bytes_gebunden() {
    let a = aufbau();
    // Andere Policy, gleiche Signatur: die Bytes weichen ab.
    let mut andere = a.policy.clone();
    andere.version = 2;
    let digest = sha256_daten(&andere.canonical_bytes());
    let falsche = a.schluessel.sign_digest(digest);
    assert_ne!(
        hex(&falsche),
        hex(&a.signature),
        "andere Bytes, andere Signatur"
    );

    assert!(!a.verifier.verify(&andere.canonical_bytes(), &a.signature));
    assert!(a.verifier.verify(&a.policy.canonical_bytes(), &a.signature));
}

#[test]
fn domain_trennung_wird_erzwungen() {
    let a = aufbau();
    // Ohne Domain-Praefix signiert: die Bytes sind andere, die Signatur passt nicht.
    let mut bytes = a.policy.canonical_bytes();
    let ohne_domain = bytes.split_off(atc_exec_admission::POLICY_DOMAIN.len());
    let signatur_ohne_domain = a.schluessel.sign_digest(sha256_daten(&ohne_domain));
    assert!(!a
        .verifier
        .verify(&a.policy.canonical_bytes(), &signatur_ohne_domain));
}

#[test]
fn policy_bytes_ausserhalb_der_kanonischen_form_werden_abgelehnt() {
    let mut a = aufbau();
    let mut request = request(
        &a.policy,
        Some(&a.signature),
        Component::AtcVm,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    request.policy_bytes.push(0x00); // angehaengtes Byte
    let decision = a.authority.admit(&a.registry, &request, &a.verifier);
    assert!(!decision.is_authorized());
    assert_eq!(decision.code.map(|c| c.code()), Some("ADM-009"));
}

#[test]
fn abweichender_beobachteter_hash_wird_abgelehnt() {
    let mut a = aufbau();
    let request = request(
        &a.policy,
        Some(&a.signature),
        Component::AtcVm,
        RIGHTS,
        sha256_daten(b"etwas-anderes"),
        NOW_US,
    );
    let decision = a.authority.admit(&a.registry, &request, &a.verifier);
    assert!(!decision.is_authorized());
    assert_eq!(decision.code.map(|c| c.code()), Some("ADM-002"));
}

#[test]
fn gueltige_signatur_ohne_autorisierung_ist_nicht_genug() {
    let a = aufbau();
    let mut authority = Authority::new(IsolationState::Available);
    // Signierer absichtlich NICHT autorisiert.
    authority.set_ceiling(Component::AtcVm, RIGHTS);
    let request = request(
        &a.policy,
        Some(&a.signature),
        Component::AtcVm,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    let decision = authority.admit(&a.registry, &request, &a.verifier);
    assert!(
        !decision.is_authorized(),
        "Kontrolle ueber den Schluessel ist keine Erlaubnis"
    );
    assert_eq!(decision.code.map(|c| c.code()), Some("ADM-003"));
}

#[test]
fn fremde_komponente_und_zu_viele_rechte_werden_abgelehnt() {
    let mut a = aufbau();
    let falsche_komponente = request(
        &a.policy,
        Some(&a.signature),
        Component::Node,
        RIGHTS,
        a.policy.code_hash,
        NOW_US,
    );
    assert_eq!(
        a.authority
            .admit(&a.registry, &falsche_komponente, &a.verifier)
            .code
            .map(|c| c.code()),
        Some("ADM-004")
    );

    let zu_viele_rechte = request(
        &a.policy,
        Some(&a.signature),
        Component::AtcVm,
        0b1111,
        a.policy.code_hash,
        NOW_US,
    );
    assert_eq!(
        a.authority
            .admit(&a.registry, &zu_viele_rechte, &a.verifier)
            .code
            .map(|c| c.code()),
        Some("ADM-005")
    );
}
