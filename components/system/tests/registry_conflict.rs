// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! S1-2 / F-1 auf Systemebene: ein zweiter Zulassungsweg ueberschreibt die
//! erste Freigabe nicht, sondern wird deterministisch abgewiesen.
//!
//! Abnahmekriterien S1-2-T-03, T-05, T-06
//! ([EXEC-S1-2-ADMISSION-REGISTRY-F1.md](../../../docs/decisions/EXEC-S1-2-ADMISSION-REGISTRY-F1.md)).
//! Der Konflikt traegt bis zur normativen Registrierung von `ADM-012` **keine**
//! Fehleridentitaet (`code: None`); eine provisorische Kennung gibt es nicht.

use atc_system::{ContractState, System, SystemError, DEFAULT_GAS};

const NOW_US: i64 = 1_800_000_000_000_000;
const SOURCE: &str =
    "fn main() -> i64 { let i = 0; let s = 0; while i < 4 { let s = s + i; let i = i + 1; } return s; }";

#[test]
fn t03_zweiter_weg_stoert_die_erste_freigabe_nicht() {
    let mut system = System::devnet().expect("Devnet");
    let aus_quelle = system.deploy(SOURCE, NOW_US).expect("Quelle");
    let container = aus_quelle.container_bytes.clone();

    // Zweiter Zulassungsweg fuer dasselbe Programm: Ablehnung, kein Ersatz.
    let error = system
        .deploy_container(&container, NOW_US)
        .expect_err("zweiter Weg muss abgelehnt werden");
    assert_eq!(error.stage(), "admission", "{error}");
    assert!(
        matches!(error, SystemError::Admission { code: None, .. }),
        "kein provisorischer Code, solange ADM-012 nicht registriert ist: {error}"
    );
    assert_eq!(system.registry().len(), 1, "keine zweite Policy");

    // Die erste Freigabe bleibt vollstaendig wirksam: kein ADM-009, ein Block.
    let mut state = ContractState::new();
    let receipt = system
        .execute(&aus_quelle, &mut state, DEFAULT_GAS, NOW_US, 1)
        .expect("erste Freigabe muss weiter autorisieren");
    assert_eq!(receipt.result(), Some(6));
    assert_eq!(system.chain().height(), 1);
}

#[test]
fn t05_konflikt_ueber_deploy_erzeugt_kein_deployment() {
    // Konflikt ueber den Deploy-Pfad: die vorhandene Freigabe bleibt, der
    // abgewiesene Eintrag hinterlaesst nichts, und eine Wiederholung derselben
    // Quelle ist idempotent statt ein zweiter Eintrag.
    let mut system = System::devnet().expect("Devnet");
    let aus_quelle = system.deploy(SOURCE, NOW_US).expect("Quelle");
    let container = aus_quelle.container_bytes.clone();
    let vorher = system.registry().len();

    let error = system
        .deploy_container(&container, NOW_US)
        .expect_err("Konflikt");
    assert_eq!(error.stage(), "admission", "{error}");
    assert_eq!(
        system.registry().len(),
        vorher,
        "der abgewiesene Eintrag hinterlaesst nichts"
    );
    assert_eq!(system.chain().height(), 0, "kein Block");

    // Dieselbe Quelle ein zweites Mal: bytegleiche Policy, idempotent.
    system
        .deploy(SOURCE, NOW_US)
        .expect("Wiederholung ist idempotent");
    assert_eq!(system.registry().len(), vorher, "kein zweiter Eintrag");
}

#[test]
fn t06_konflikt_ueber_huelle_erzeugt_keinen_zustand() {
    let mut system = System::devnet().expect("Devnet");
    let aus_quelle = system.deploy(SOURCE, NOW_US).expect("Quelle");
    let mut state = ContractState::from_words(vec![1, 2, 3]);
    let vorher = state.clone();

    let error = system
        .deploy_container(&aus_quelle.container_bytes.clone(), NOW_US)
        .expect_err("Konflikt");
    assert!(matches!(error, SystemError::Admission { code: None, .. }));
    assert_eq!(state, vorher);
    assert_eq!(system.chain().height(), 0);
}
