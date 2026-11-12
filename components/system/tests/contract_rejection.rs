// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! E1 (Audit 2026-11-12, F1), Abnahmekriterien 4 und 5: `System::deploy` lehnt
//! einen Vertrag ab, statt ihn mit einem Ersatzprogramm `Push(0); Halt` anzunehmen.
//!
//! Vor dem Fix war das Gegenteil der Fall: `deploy(token.atc)` gelang, `execute`
//! lieferte `Ok(Some(0))`, und in der Registry stand ein `code_hash` ueber ein
//! zwei Instruktionen langes Nichts. Der Test prueft deshalb nicht nur den
//! Fehlercode, sondern auch, dass Registry, Audit und Kette unveraendert bleiben.

use atc_system::{compile, CompileError, System, SystemError};

const NOW_US: i64 = 1_800_000_000_000_000;

fn token_quelle() -> String {
    std::fs::read_to_string("../../apps/atc-contracts/contracts/token.atc")
        .expect("token.atc lesbar")
}

#[test]
fn deploy_lehnt_vertrag_ab_und_schreibt_nichts() {
    let mut system = System::devnet().expect("devnet");
    let registry_vorher = system.registry().len();
    let audit_vorher = system.audit_count();
    let hoehe_vorher = system.chain().height();

    let err = system
        .deploy(&token_quelle(), NOW_US)
        .expect_err("Vertrag darf nicht als Deployment durchgehen");

    match err {
        SystemError::Compile(CompileError::Lower(msg)) => {
            assert!(
                msg.contains("nicht ausfuehrbar") && msg.contains("FranchiseToken"),
                "Lowering-Grund bleibt bis zum Aufrufer erhalten: {msg}"
            );
            assert!(
                msg.contains("11 Funktionen"),
                "verlorene Bestandteile werden benannt: {msg}"
            );
        }
        other => panic!("erwartet wird ein Compile-/Lowering-Fehler, war {other:?}"),
    }

    assert_eq!(
        system.registry().len(),
        registry_vorher,
        "keine Policy fuer einen abgelehnten Vertrag"
    );
    assert_eq!(system.audit_count(), audit_vorher, "kein Audit-Eintrag");
    assert_eq!(
        system.chain().height(),
        hoehe_vorher,
        "kein Block, kein Zustandswechsel"
    );
}

#[test]
fn kein_ersatzprogramm_fuer_vertragsquellen() {
    // Ohne Verteidigung in der Tiefe: waere der Vertrag nur uebersprungen worden,
    // ergaebe sich hier genau das fruehere `Push(0); Halt; Halt`.
    let err = compile(&token_quelle()).expect_err("Compile muss scheitern");
    assert!(
        matches!(err, CompileError::Lower(_)),
        "Fehler entsteht im Lowering, nicht erst im Backend: {err:?}"
    );
}

#[test]
fn programm_ohne_vertrag_deployt_weiterhin() {
    let mut system = System::devnet().expect("devnet");
    let dep = system
        .deploy("fn main() -> i64 { return 7; }", NOW_US)
        .expect("Programm ohne Vertrag bleibt deploybar");
    let mut state = atc_system::ContractState::from_words(Vec::new());
    let receipt = system
        .execute(&dep, &mut state, 200_000, NOW_US, 1)
        .expect("Ausfuehrung");
    assert_eq!(receipt.result(), Some(7));
}
