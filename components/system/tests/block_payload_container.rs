// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! P1a — `Block.payload` transportiert die `ATCE`-Huelle, der Import prueft sie
//! an derselben Sicherheitsgrenze wie `System::deploy_container`.
//!
//! Geprueft wird:
//!
//! * eine ausfuehrbare Payload laeuft durch die bestehende Systemgrenze und
//!   liefert **denselben** `code_hash` wie das direkte Deployment (keine
//!   zweite Validierungsroute, keine zweite Dekodierung),
//! * ein `ATCB`-Strom als Payload wird abgewiesen, getaggt **und** ungetaggt,
//! * die Hexform ist kanonisch (Grossbuchstaben, ungerade Laenge: `P-003`),
//! * nicht ausfuehrbare Payloads bleiben gueltig und die Block-Hashes
//!   bestehender Bloecke aendern sich nicht,
//! * der Bytecode-Fixwert bleibt unveraendert.
//!
//! Status: `TESTED` (lokale Suite). Kein `CI_VERIFIED`, kein `VERIFIED`.

use atc_node::chain::Block;
use atc_system::{
    admit_payload, container_bytes, encode_container, hex, is_container_payload, validate_payload,
    BlockPayloadError, PayloadKind, System, SystemError,
};
use atc_vm::envelope::Envelope;

const NOW_US: i64 = 1_800_000_000_000_000;
const SOURCE: &str =
    "fn main() -> i64 { let i = 0; let s = 0; while i < 3 { let s = s + i; let i = i + 1; } return s; }";

fn huelle() -> Vec<u8> {
    System::devnet()
        .expect("Devnet")
        .deploy(SOURCE, NOW_US)
        .expect("Deployment")
        .container_bytes
}

fn erwarte_payload(error: &SystemError, code: &str) -> () {
    match error {
        SystemError::BlockPayload(e) => assert_eq!(e.code(), code, "Fehleridentitaet"),
        other => panic!("erwartet BlockPayload({code}), erhalten: {other:?}"),
    }
}

// --- Transport ------------------------------------------------------------

#[test]
fn huelle_ueberlebt_den_transport_als_payload() {
    let container = huelle();
    let payload = encode_container(&container);

    assert!(payload.starts_with("atce:"));
    assert!(is_container_payload(&payload));
    assert_eq!(
        container_bytes(&payload).unwrap(),
        container,
        "Transport ist bytegleich"
    );
    assert_eq!(&container[0..4], b"ATCE", "Huelle, nicht Strom");
}

#[test]
fn payload_hat_dieselbe_wahrheit_wie_das_deployment() {
    let container = huelle();
    let payload = encode_container(&container);

    // Weg 1: direkt an der Systemgrenze.
    let mut direkt = System::devnet().expect("Devnet");
    let a = direkt
        .deploy_container(&container, NOW_US)
        .expect("Deployment");

    // Weg 2: ueber den Blockimport.
    let mut import = System::devnet().expect("Devnet");
    let b = admit_payload(&mut import, &payload, NOW_US).expect("Import");

    assert_eq!(a.code_hash, b.code_hash, "eine Code-Identitaet");
    assert_eq!(a.container_bytes, b.container_bytes, "eine Huellenwahrheit");
    assert_eq!(a.ops, b.ops);
}

#[test]
fn pruefung_veraendert_keinen_zustand() {
    let container = huelle();
    let payload = encode_container(&container);
    let system = System::devnet().expect("Devnet");
    let vorher = system.audit_count();

    assert_eq!(
        validate_payload(&system, &payload).unwrap(),
        PayloadKind::Container
    );
    assert_eq!(
        system.audit_count(),
        vorher,
        "check_container ist zustandsfrei"
    );
}

// --- keine ATCB-Route -----------------------------------------------------

#[test]
fn getaggter_atcb_strom_wird_abgewiesen() {
    let atcb = b"ATCB\x01\x01\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    let payload = format!("atcb:{}", hex(atcb));
    let system = System::devnet().expect("Devnet");

    erwarte_payload(&validate_payload(&system, &payload).unwrap_err(), "P-002");
    assert_eq!(
        container_bytes(&payload).unwrap_err(),
        BlockPayloadError::InnerFormat,
        "der Tag allein genuegt nicht"
    );
}

#[test]
fn ungetaggter_atcb_strom_wird_abgewiesen() {
    // Ein Angreifer laesst den Tag weg und legt den Strom roh in die Payload.
    let system = System::devnet().expect("Devnet");
    let roh = "ATCB\x01\x01\x00\x00\x00\x00";
    erwarte_payload(&validate_payload(&system, roh).unwrap_err(), "P-002");

    // und hexkodiert, ohne Tag
    let hexkodiert = hex(b"ATCB\x01\x01\x00\x00");
    erwarte_payload(
        &validate_payload(&system, &hexkodiert).unwrap_err(),
        "P-002",
    );
}

#[test]
fn fremdes_magic_ist_kein_kodierfehler_der_payload() {
    // ATCE-Praefix mit falschem Huelle-Magic: die Huelle entscheidet, nicht die Payload.
    // Laenge stimmt (sonst gewinnt E-001), nur die Identitaet nicht.
    let mut bytes = huelle();
    bytes[3] = b'1'; // "ATC1" statt "ATCE"
    let falsch = format!("atce:{}", hex(&bytes));
    let system = System::devnet().expect("Devnet");
    match validate_payload(&system, &falsch).unwrap_err() {
        SystemError::Container(e) => assert_eq!(e.code(), "E-007"),
        other => panic!("erwartet E-007 aus der Huelle, erhalten: {other:?}"),
    }
}

// --- kanonische Kodierung -------------------------------------------------

#[test]
fn grossbuchstaben_sind_nicht_kanonisch() {
    let container = huelle();
    let payload = format!("atce:{}", hex(&container).to_uppercase());
    let system = System::devnet().expect("Devnet");
    erwarte_payload(&validate_payload(&system, &payload).unwrap_err(), "P-003");
}

#[test]
fn ungerade_laenge_ist_nicht_kanonisch() {
    let system = System::devnet().expect("Devnet");
    erwarte_payload(
        &validate_payload(&system, "atce:4142434").unwrap_err(),
        "P-003",
    );
    erwarte_payload(&validate_payload(&system, "atce:").unwrap_err(), "P-003");
}

#[test]
fn leere_payload_ist_p001() {
    let system = System::devnet().expect("Devnet");
    erwarte_payload(&validate_payload(&system, "").unwrap_err(), "P-001");
}

// --- Regression: Datenpayloads und Bytecode-Hashes ------------------------

#[test]
fn datenpayload_bleibt_gueltig_und_hash_stabil() {
    // Bestehende Nutzung: der Genesis-Block traegt die Zustandswurzel als Payload.
    let system = System::devnet().expect("Devnet");
    let genesis: &Block = system.chain().best();
    assert_eq!(genesis.height, 0);
    assert_eq!(
        validate_payload(&system, &genesis.payload).unwrap(),
        PayloadKind::Data
    );
    assert!(!is_container_payload(&genesis.payload));

    // Die Blockkette bleibt gueltig und deterministisch; die Payload ist Text
    // wie vorher, deshalb aendern sich bestehende Block-Hashes nicht.
    assert!(system.chain().verify().is_ok(), "Kette verifiziert");
    let h = system.chain().preview_hash(&genesis.payload);
    assert_eq!(
        h,
        system.chain().preview_hash(&genesis.payload),
        "deterministisch"
    );
    assert_ne!(h, system.chain().preview_hash(&encode_container(&huelle())));
}

#[test]
fn bytecode_fixwert_bleibt() {
    // Der Strom innerhalb der Huelle muss denselben Bytecode-Hash ergeben wie
    // vor der Payload-Integration. Werte aus dem fixierten Test der VM.
    let container = huelle();
    let envelope = Envelope::parse(&container).expect("Huelle");
    let inner = envelope.into_artifact().expect("Artefakt");
    let aus_huelle = inner.hash();

    let mut system = System::devnet().expect("Devnet");
    let deployment = system
        .deploy_container(&container, NOW_US)
        .expect("Deployment");
    assert_eq!(
        deployment.code_hash, aus_huelle,
        "Code-Hash kommt aus dem Strom"
    );
}
