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

/// Kein alternativer Importpfad: **alle** Mutationen des Blockbestands liegen in
/// `chain.rs`, und zwar genau drei — zwei Anhängepunkte (lokale Produktion,
/// validierter Import) und eine Ersetzung (validierte Adoption).
///
/// Der Test sucht nicht nur `blocks.push`, sondern jede Form von Schreibzugriff
/// (`blocks =`, `blocks.push`, `blocks.clear/retain/truncate/insert/remove/
/// extend`) und verlangt, dass sie ausschliesslich in `chain.rs` vorkommt. Damit
/// ist auch ein indirekter Pfad in einem anderen Modul ein Testfehler.
#[test]
fn kein_zweiter_importpfad_im_quelltext() {
    const SCHREIBER: [&str; 8] = [
        "blocks.push",
        "blocks =",
        "blocks.clear",
        "blocks.retain",
        "blocks.truncate",
        "blocks.insert",
        "blocks.remove",
        "blocks.extend",
    ];
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut treffer: Vec<(String, usize)> = Vec::new();
    for entry in std::fs::read_dir(&src).expect("src") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("lesbar");
        // Kommentare zaehlen nicht: sie beschreiben die Stellen nur.
        let code: usize = text
            .lines()
            .filter(|l| !l.trim_start().starts_with("//") && !l.trim_start().starts_with("///"))
            .map(|l| SCHREIBER.iter().filter(|m| l.contains(*m)).count())
            .sum();
        if code > 0 {
            treffer.push((path.file_name().unwrap().to_string_lossy().to_string(), code));
        }
    }
    treffer.sort();
    assert_eq!(
        treffer,
        vec![("chain.rs".to_string(), 3)],
        "Schreibzugriffe auf den Blockbestand nur in chain.rs (produce, import_block, adopt_verified)"
    );
}

/// Auch die Feldprivatheit ist Teil der Grenze: ausserhalb von `chain.rs` darf
/// kein Modul das Feld direkt lesen oder schreiben.
#[test]
fn blockbestand_ist_ausserhalb_nicht_zugaenglich() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for entry in std::fs::read_dir(&src).expect("src") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        if path.file_name().and_then(|n| n.to_str()) == Some("chain.rs") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("lesbar");
        // Erlaubt sind nur die Lesezugriffe `.blocks()` und `.blocks_from(…)`.
        for (i, _) in text.match_indices(".blocks") {
            let next = text[i + ".blocks".len()..].chars().next().unwrap_or(' ');
            assert!(
                next == '(' || next == '_',
                "{} greift direkt auf das Feld `blocks` zu (Zeichen danach: {next:?}) — nur chain.rs darf das",
                path.display()
            );
        }
    }
}

// --- Adoption (Gossip-Sync): derselbe Port, kein Ersetzen ohne Pruefung -----

fn kandidat(payloads: &[&str]) -> Vec<Block> {
    let kette = kette();
    let mut out = vec![kette.best().clone()];
    for (i, payload) in payloads.iter().enumerate() {
        let prev = out.last().unwrap().hash;
        out.push(Block::build_for_test(i as u64 + 1, prev, payload));
    }
    out
}

#[test]
fn adoption_ohne_validator_wird_verweigert() {
    let mut kette = kette();
    let bloecke = kandidat(&["a", "b"]);
    let err = kette.adopt_verified(bloecke, None).expect_err("fail-closed");
    assert_eq!(err.code(), "I-001");
    assert_eq!(kette.height(), 0, "nichts uebernommen");
}

#[test]
fn adoption_prueft_jede_payload() {
    let mut kette = kette();
    let bloecke = kandidat(&["state-root-1", "atcb:41544342"]);
    let err = kette.adopt_verified(bloecke, Some(&NurHuelle)).expect_err("ATCB im Kandidaten");
    match err {
        ImportError::Payload(r) => assert_eq!(r.code, "P-002"),
        other => panic!("erwartet Payload(P-002), erhalten {other:?}"),
    }
    assert_eq!(kette.height(), 0, "keine Teiluebernahme");
}

#[test]
fn adoption_mit_validator_uebernimmt_nach_pruefung() {
    let mut kette = kette();
    let bloecke = kandidat(&["state-root-1", "state-root-2"]);
    kette.adopt_verified(bloecke, Some(&NurHuelle)).expect("Adoption");
    assert_eq!(kette.height(), 2);
    kette.verify().expect("Kette gueltig");
}

/// Repo-weite Grenze (P1d-A): ausserhalb der kanonischen Chain darf **keine**
/// Stelle eine Kette aus fremden Bloecken bauen oder ersetzen. Der Test liest
/// alle `.rs`-Dateien des Repos (ohne Kommentarzeilen) und schlaegt bei jeder
/// Wiederkehr von `Chain::from_blocks(`, `kandidat`-Zuweisung oder
/// `pub fn sync_pull` in einer anderen Kette fehl.
#[test]
fn keine_zweite_adoptionsstelle_im_repo() {
    let wurzel = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut treffer: Vec<String> = Vec::new();
    stapel(&wurzel, &mut treffer);
    assert!(treffer.is_empty(), "zweite Adoptionsstelle: {treffer:?}");
}

fn stapel(dir: &std::path::Path, treffer: &mut Vec<String>) {
    let Ok(eintraege) = std::fs::read_dir(dir) else { return };
    for eintrag in eintraege.flatten() {
        let pfad = eintrag.path();
        let name = pfad.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if pfad.is_dir() {
            // Build-Artefakte und VCS nicht durchsuchen.
            if !matches!(name, "target" | ".git" | "node_modules") {
                stapel(&pfad, treffer);
            }
            continue;
        }
        if pfad.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let text = std::fs::read_to_string(&pfad).unwrap_or_default();
        let code: String = text
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                !t.starts_with("//") && !t.starts_with("///") && !t.starts_with("//!")
            })
            .collect::<Vec<_>>()
            .join("\n");
        // Ausnahmen sind nur die beiden Waechter selbst: sie nennen die
        // verbotenen Muster als Sollwert "darf nicht vorkommen". Jede andere
        // Datei, insbesondere Produktionscode, muss frei davon sein.
        let text_pfad = pfad.to_string_lossy().replace('\\', "/");
        if text_pfad.ends_with("components/node/tests/block_import.rs")
            || text_pfad.ends_with("apps/atc-node/tests/two_node_devnet.rs")
        {
            continue;
        }
        let kanonisch = text_pfad.ends_with("components/node/src/chain.rs");
        if code.contains("Chain::from_blocks(") && !kanonisch {
            treffer.push(format!("{}: from_blocks", pfad.display()));
        }
        if code.contains("kette = kandidat") || code.contains("kette = kandidat;") {
            treffer.push(format!("{}: kette = kandidat", pfad.display()));
        }
        if code.contains("pub fn sync_pull") && !kanonisch {
            // Nur die kanonische Implementierung darf Peer-Adoption anbieten.
            if !text_pfad.ends_with("components/node/src/gossip.rs") {
                treffer.push(format!("{}: zweiter sync_pull", pfad.display()));
            }
        }
    }
}
