// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! P1b (Variante C) — Import fremder Bloecke ueber den Validator-Port.
//!
//! Zwei Aussagen werden geprueft, nicht eine:
//! 1. Es gibt **keinen** Weg an der Validierung vorbei: ohne Validator und bei
//!    Ablehnung bleibt der Chain-State unveraendert.
//! 2. Der Knoten dekodiert nichts selbst — alle Formfragen beantwortet der Port.

use atc_node::bootstrap::Genesis;
use atc_node::chain::{Block, Chain};
use atc_node::import::{ContainerValidator, ImportError, PayloadRejection};

/// Stub, der alles annimmt (Datenpayloads sind ohne Codebezug gueltig).
struct Annehmend;
impl ContainerValidator for Annehmend {
    fn validate_block_payload(&self, _payload: &str) -> Result<(), PayloadRejection> {
        Ok(())
    }
}

/// Stub, der wie die Systemgrenze `ATCB` als externe Eingabe abweist.
struct NurHuelle;
impl ContainerValidator for NurHuelle {
    fn validate_block_payload(&self, payload: &str) -> Result<(), PayloadRejection> {
        if payload.starts_with("atcb:") || payload.starts_with("ATCB") {
            Err(PayloadRejection::new("block_payload", "P-002"))
        } else {
            Ok(())
        }
    }
}

fn kette() -> Chain {
    Chain::from_genesis(&Genesis::devnet())
}

fn fremder_block(kette: &Chain, payload: &str) -> Block {
    let best = kette.best();
    Block::build_for_test(best.height + 1, best.hash, payload)
}

#[test]
fn import_mit_validator_haengt_an() {
    let mut chain = kette();
    let block = fremder_block(&chain, "state-root-abc");
    let before = chain.height();
    chain.import_block(block, Some(&Annehmend)).expect("Import");
    assert_eq!(chain.height(), before + 1);
    chain.verify().expect("Kette bleibt gueltig");
}

#[test]
fn ohne_validator_kein_fremdblock() {
    let mut chain = kette();
    let height = chain.height();
    let hash = chain.best_hash();
    let block = fremder_block(&chain, "atce:00");
    let err = chain
        .import_block(block, None)
        .expect_err("muss abgelehnt werden");
    assert_eq!(err.code(), "I-001");
    assert_eq!(chain.height(), height, "Hoehe unveraendert");
    assert_eq!(chain.best_hash(), hash, "Best-Hash unveraendert");
}

#[test]
fn abgelehnte_payload_aendert_keinen_zustand() {
    let mut chain = kette();
    let height = chain.height();
    let hash = chain.best_hash();
    let block = fremder_block(&chain, "atcb:41544342");
    let err = chain
        .import_block(block, Some(&NurHuelle))
        .expect_err("ATCB ist keine externe Route");
    match err {
        ImportError::Payload(r) => assert_eq!((r.stage, r.code), ("block_payload", "P-002")),
        other => panic!("erwartet Payload(P-002), erhalten {other:?}"),
    }
    assert_eq!(chain.height(), height);
    assert_eq!(chain.best_hash(), hash);
}

#[test]
fn falscher_blockhash_wird_vor_der_payload_geprueft() {
    let mut chain = kette();
    let mut block = fremder_block(&chain, "state-root");
    block.payload = "manipuliert".to_string(); // Hash passt nicht mehr zum Inhalt
    let err = chain
        .import_block(block, Some(&Annehmend))
        .expect_err("Hash");
    assert_eq!(err.code(), "I-002");
    assert_eq!(chain.height(), 0);
}

#[test]
fn verkettung_wird_geprueft() {
    let mut chain = kette();
    let best = chain.best();
    let block = Block::build_for_test(best.height + 5, best.hash, "x");
    let err = chain
        .import_block(block, Some(&Annehmend))
        .expect_err("Hoehe");
    assert_eq!(err.code(), "I-003");

    let block = Block::build_for_test(1, [7u8; 32], "x");
    let err = chain
        .import_block(block, Some(&Annehmend))
        .expect_err("Vorgaenger");
    assert_eq!(err.code(), "I-003");
}

/// Kein alternativer Importpfad: `blocks.push` darf nur an zwei Stellen stehen —
/// in der lokalen Produktion und hinter der Pruefung im Import.
#[test]
fn kein_zweiter_importpfad_im_quelltext() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut treffer = Vec::new();
    for entry in std::fs::read_dir(&src).expect("src") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("lesbar");
        let n = text.matches("blocks.push").count();
        if n > 0 {
            treffer.push((path.file_name().unwrap().to_string_lossy().to_string(), n));
        }
    }
    treffer.sort();
    assert_eq!(
        treffer,
        vec![("chain.rs".to_string(), 2)],
        "nur produce + append_validated"
    );
}
