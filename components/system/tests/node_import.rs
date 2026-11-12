// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! P1b (Variante C) — der reale Importpfad benutzt die System-Ingress-Kette.
//!
//! Geprueft wird die Grenze, nicht nur ein Rueckgabewert:
//! * `SystemPayloadValidator` delegiert an `validate_payload` (Huelle `ATCE`,
//!   `System::check_container`) — kein zweiter Parser, keine zweite Dekodierung.
//! * Dieselbe Payload ergibt dieselbe Entscheidung und denselben Weg wie
//!   `admit_payload`; `ATCB` bleibt ohne externe Route.
//! * Ohne Validator wird ein Fremdblock nicht angenommen, und ein abgelehnter
//!   Block laesst den Chain-State unveraendert.

use atc_node::bootstrap::Genesis;
use atc_node::chain::{Block, Chain};
use atc_node::import::{ContainerValidator, ImportError};
use atc_system::block_payload::{self, PayloadKind};
use atc_system::node_import::{DevnetNode, SystemPayloadValidator};
use atc_system::System;

const NOW_US: i64 = 1_800_000_000_000_000;
const SOURCE: &str =
    "fn main() -> i64 { let i = 0; let s = 0; while i < 4 { let s = s + i; let i = i + 1; } return s; }";

/// Gueltige Huelle aus einem eigenen System (Compare-and-Insert, S1-2: ein
/// zweites Deployment desselben Programms im selben System waere ein Konflikt).
fn huelle() -> Vec<u8> {
    System::devnet()
        .expect("Devnet")
        .deploy(SOURCE, NOW_US)
        .expect("Deployment")
        .container_bytes
}

#[test]
fn gueltige_huelle_wird_ueber_den_port_angenommen() {
    let payload = block_payload::encode_container(&huelle());
    let system = System::devnet().expect("Devnet");
    let validator = SystemPayloadValidator::new(&system);

    assert_eq!(
        validator.payload_kind(&payload).expect("gueltig"),
        PayloadKind::Container,
        "der Port sieht dieselbe Payload wie die Systemgrenze"
    );
    validator
        .validate_block_payload(&payload)
        .expect("angenommen");
}

#[test]
fn derselbe_weg_wie_admit_payload() {
    let container = huelle();
    let payload = block_payload::encode_container(&container);
    let mut system = System::devnet().expect("Devnet");
    let pruefer = System::devnet().expect("Devnet");
    let validator = SystemPayloadValidator::new(&pruefer);

    // Port: nur Pruefung, keine Freigabe, kein Zustand.
    assert!(validator.validate_block_payload(&payload).is_ok());

    // Freigabe laeuft ueber die bestehende Kette — und liefert dieselben Bytes.
    let deployment = block_payload::admit_payload(&mut system, &payload, NOW_US).expect("Freigabe");
    assert_eq!(deployment.container_bytes, container, "keine Konvertierung");
}

#[test]
fn atcb_bleibt_ohne_externe_route() {
    let system = System::devnet().expect("Devnet");
    let validator = SystemPayloadValidator::new(&system);

    let r = validator
        .validate_block_payload("atcb:4154434201000000")
        .expect_err("ATCB ist keine externe Eingabe");
    assert_eq!((r.stage, r.code), ("block_payload", "P-002"));

    // Rohstrom ohne Tag, hexkodiert: derselbe Befund.
    let r = validator
        .validate_block_payload("4154434201000000")
        .expect_err("roher ATCB-Strom");
    assert_eq!((r.stage, r.code), ("block_payload", "P-002"));
}

#[test]
fn nicht_kanonisches_hex_wird_abgelehnt() {
    let payload = block_payload::encode_container(&huelle());
    let system = System::devnet().expect("Devnet");
    let validator = SystemPayloadValidator::new(&system);

    let gross = format!("atce:{}", payload[5..].to_uppercase());
    let r = validator
        .validate_block_payload(&gross)
        .expect_err("Grossbuchstaben");
    assert_eq!((r.stage, r.code), ("block_payload", "P-003"));

    let ungerade = format!("atce:{}", &payload[5..payload.len() - 1]);
    let r = validator
        .validate_block_payload(&ungerade)
        .expect_err("ungerade Laenge");
    assert_eq!((r.stage, r.code), ("block_payload", "P-003"));
}

#[test]
fn datenpayload_bleibt_gueltig() {
    let system = System::devnet().expect("Devnet");
    let validator = SystemPayloadValidator::new(&system);
    assert_eq!(
        validator
            .payload_kind("state-root-552e5a0b")
            .expect("Daten"),
        PayloadKind::Data
    );
}

#[test]
fn fremdblock_ohne_validator_wird_nicht_angenommen() {
    let mut chain = Chain::from_genesis(&Genesis::devnet());
    let best = chain.best();
    let block = Block::build_for_test(best.height + 1, best.hash, "state-root");
    let err = chain.import_block(block, None).expect_err("fail-closed");
    assert_eq!(err.code(), "I-001");
    assert_eq!(chain.height(), 0, "kein Zustand");
}

#[test]
fn abgelehnter_block_laesst_chain_und_registry_unveraendert() {
    let container = huelle();
    let payload = block_payload::encode_container(&container);
    let system = System::devnet().expect("Devnet");
    let validator = SystemPayloadValidator::new(&system);

    let mut chain = Chain::from_genesis(&Genesis::devnet());
    let (height, hash) = (chain.height(), chain.best_hash());

    // 1. Gueltige Payload ohne Validator.
    let block = Block::build_for_test(height + 1, hash, &payload);
    assert_eq!(
        chain
            .import_block(block, None)
            .expect_err("ohne Validator")
            .code(),
        "I-001"
    );

    // 2. Abgelehnte Payload (ATCB) mit Validator.
    let block = Block::build_for_test(height + 1, hash, "atcb:41544342");
    match chain
        .import_block(block, Some(&validator))
        .expect_err("ATCB")
    {
        ImportError::Payload(r) => assert_eq!(r.code, "P-002"),
        other => panic!("erwartet Payload(P-002), erhalten {other:?}"),
    }

    // 3. Manipulierte Huelle: Hash stimmt, Huelle nicht.
    let mut kaputt = container.clone();
    kaputt[4] = 0x03; // Generation veraendert -> E-009 aus der Huelle
    let block = Block::build_for_test(height + 1, hash, &block_payload::encode_container(&kaputt));
    match chain
        .import_block(block, Some(&validator))
        .expect_err("Huelle")
    {
        ImportError::Payload(r) => {
            assert_eq!(r.stage, "container");
            assert!(r.code.starts_with("E-"));
        }
        other => panic!("erwartet Payload(E-xxx), erhalten {other:?}"),
    }

    assert_eq!(chain.height(), height, "Hoehe unveraendert");
    assert_eq!(chain.best_hash(), hash, "Best-Hash unveraendert");
    chain.verify().expect("Kette bleibt gueltig");
}

// --- Devnet-Verdrahtung: der Knoten entscheidet, nicht der Aufrufer --------

#[test]
fn devnet_knoten_nimmt_nur_geprueft_an() {
    let container = huelle();
    let payload = block_payload::encode_container(&container);
    let mut node = DevnetNode::start().expect("Knoten startet");
    let best = node.chain().best().clone();

    // Gueltige Huelle: angenommen (der Validator ist Teil des Knotens).
    node.import_block(Block::build_for_test(best.height + 1, best.hash, &payload))
        .expect("gueltige Huelle");
    assert_eq!(node.chain().height(), 1);

    // ATCB: abgelehnt, kein Zustand.
    let best = node.chain().best().clone();
    let err = node
        .import_block(Block::build_for_test(best.height + 1, best.hash, "atcb:41544342"))
        .expect_err("ATCB");
    assert_eq!(err.code(), "I-005");
    assert_eq!(node.chain().height(), 1);

    // Falscher Hash: abgelehnt, bevor die Payload ueberhaupt gefragt wird.
    let mut kaputt = Block::build_for_test(2, best.hash, "state-root");
    kaputt.payload = "manipuliert".into();
    assert_eq!(node.import_block(kaputt).expect_err("Hash").code(), "I-002");
    assert_eq!(node.chain().height(), 1, "keine Annahme, kein Zustand");
}

#[test]
fn devnet_knoten_adoption_prueft_payloads() {
    let mut node = DevnetNode::start().expect("Knoten startet");
    let genesis = node.chain().best().clone();
    let b1 = Block::build_for_test(1, genesis.hash, "state-root-1");
    let b2 = Block::build_for_test(2, b1.hash, "atcb:41544342");
    let err = node
        .adopt_chain(vec![genesis, b1, b2])
        .expect_err("ATCB im Kandidaten");
    assert_eq!(err.code(), "I-005");
    assert_eq!(node.chain().height(), 0, "nichts uebernommen");
}
