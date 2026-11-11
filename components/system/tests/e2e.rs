// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Gate D und E — Gas blockiert wirklich, Zustand bleibt atomar.
//!
//! Geprueft wird nicht "es kommt ein Fehler", sondern: keine Ausfuehrung,
//! keine Zustandsaenderung, kein Block, kein Umweg.

use atc_system::{ContractState, System, SystemError, DEFAULT_GAS};

const NOW_US: i64 = 1_800_000_000_000_000;
const SOURCE: &str =
    "fn main() -> i64 { let i = 0; let s = 0; while i < 4 { let s = s + i; let i = i + 1; } return s; }";

fn system_mit_vertrag() -> (System, atc_system::Deployment) {
    let mut system = System::devnet().expect("Devnet-System");
    let deployment = system.deploy(SOURCE, NOW_US).expect("Deployment");
    (system, deployment)
}

#[test]
fn durchlauf_erzeugt_block_und_zustand() {
    let (mut system, deployment) = system_mit_vertrag();
    let mut state = ContractState::new();
    let receipt = system
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .expect("Ausfuehrung");

    assert_eq!(receipt.result(), Some(6), "0+1+2+3 = 6");
    assert!(receipt.changed_state(), "Zustand muss sich aendern");
    assert_eq!(receipt.block_height, 1);
    assert_eq!(system.chain().height(), 1);
    system.chain().verify().expect("Kette konsistent");
    assert_eq!(state.words(), &[4, 6], "i = 4, s = 6");
}

#[test]
fn gas_null_fuehrt_nichts_aus_und_aendert_nichts() {
    let (mut system, deployment) = system_mit_vertrag();
    let mut state = ContractState::from_words(vec![1, 2, 3]);
    let vorher = state.clone();
    let hoehe_vorher = system.chain().height();

    let error = system
        .execute(&deployment, &mut state, 0, NOW_US, 1)
        .expect_err("Gas 0 muss blockieren");

    assert_eq!(
        error.stage(),
        "vm",
        "Abbruch muss in der VM erfolgen: {error}"
    );
    assert!(matches!(error, SystemError::Vm(_)));
    assert_eq!(state, vorher, "kein Zustand darf ueberschrieben werden");
    assert_eq!(system.chain().height(), hoehe_vorher, "kein Block");
}

#[test]
fn ein_gas_weniger_als_noetig_aendert_nichts() {
    let (mut system, deployment) = system_mit_vertrag();

    // Minimales Budget suchen, ab dem die Ausfuehrung gelingt.
    let mut minimal = None;
    for budget in 0..2_000u64 {
        let mut probe = ContractState::new();
        if system
            .execute(&deployment, &mut probe, budget, NOW_US, 1)
            .is_ok()
        {
            minimal = Some(budget);
            break;
        }
    }
    let minimal = minimal.expect("irgendein Budget muss ausreichen");
    assert!(minimal > 0);

    let mut state = ContractState::from_words(vec![9, 9]);
    let vorher = state.clone();
    let hoehe_vorher = system.chain().height();
    let error = system
        .execute(&deployment, &mut state, minimal - 1, NOW_US, 1)
        .expect_err("ein Gas zu wenig muss blockieren");
    assert_eq!(error.stage(), "vm");
    assert_eq!(state, vorher);
    assert_eq!(system.chain().height(), hoehe_vorher);

    // Genau das Budget laeuft durch.
    let mut genau = ContractState::from_words(vec![9, 9]);
    let receipt = system
        .execute(&deployment, &mut genau, minimal, NOW_US, 1)
        .expect("genau das Budget muss reichen");
    assert_eq!(receipt.block_height, hoehe_vorher + 1);
}

#[test]
fn zwei_systeme_mit_gleicher_eingabe_erzeugen_gleiche_wurzel_und_gleichen_block() {
    let (mut a, deployment_a) = system_mit_vertrag();
    let (mut b, deployment_b) = system_mit_vertrag();
    assert_eq!(
        deployment_a.code_hash, deployment_b.code_hash,
        "gleiche Quelle, gleicher Hash"
    );

    let mut state_a = ContractState::new();
    let mut state_b = ContractState::new();
    let receipt_a = a
        .execute(&deployment_a, &mut state_a, DEFAULT_GAS, NOW_US, 1)
        .unwrap();
    let receipt_b = b
        .execute(&deployment_b, &mut state_b, DEFAULT_GAS, NOW_US, 1)
        .unwrap();

    assert_eq!(receipt_a.state_root_after, receipt_b.state_root_after);
    assert_eq!(receipt_a.block_hash, receipt_b.block_hash);
}

#[test]
fn zustand_wird_ueber_die_kette_weitergegeben() {
    let (mut system, deployment) = system_mit_vertrag();
    let mut state = ContractState::new();

    let first = system
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .unwrap();
    let second = system
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .unwrap();

    assert_eq!(
        first.state_root_after, second.state_root_before,
        "Zustand wird durchgereicht"
    );
    assert_eq!(
        second.result(),
        Some(6),
        "Schleife laeuft mit i=4 nicht mehr"
    );
    assert_eq!(second.block_height, 2);
    assert_eq!(
        system.chain().blocks().len(),
        3,
        "Genesis plus zwei Bloecke"
    );
}

#[test]
fn manipulierte_artefaktbytes_werden_abgelehnt() {
    let (mut system, mut deployment) = system_mit_vertrag();
    let laenge = deployment.artifact_bytes.len();
    deployment.artifact_bytes[laenge - 40] ^= 0x01; // im Operationsstrom

    let mut state = ContractState::new();
    let error = system
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .expect_err("veraenderte Bytes muessen abgelehnt werden");

    assert_eq!(
        error.stage(),
        "artifact",
        "der Containerhash faengt es vor der Freigabe: {error}"
    );
    assert_eq!(state, ContractState::new());
    assert_eq!(system.chain().height(), 0);
}

#[test]
fn fremdes_system_kennt_die_freigabe_nicht() {
    let (_, deployment) = system_mit_vertrag();
    let mut fremd = System::devnet().unwrap();
    let mut state = ContractState::new();

    let error = fremd
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .expect_err("unbekannter Code muss blockieren");
    assert_eq!(error.stage(), "admission");
    assert!(
        error.to_string().contains("ADM-001"),
        "erwartet ADM-001: {error}"
    );
    assert_eq!(fremd.chain().height(), 0);
}

#[test]
fn widerrufene_freigabe_wird_abgelehnt() {
    let (mut system, deployment) = system_mit_vertrag();
    system.authority_mut().revoke(deployment.code_hash);
    let mut state = ContractState::new();

    let error = system
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .expect_err("Widerruf muss greifen");
    assert!(
        error.to_string().contains("ADM-007"),
        "erwartet ADM-007: {error}"
    );
    assert_eq!(system.chain().height(), 0);
    assert_eq!(state, ContractState::new());
}

#[test]
fn abgelaufene_freigabe_wird_abgelehnt() {
    let (mut system, deployment) = system_mit_vertrag();
    let mut state = ContractState::new();
    let spaeter = deployment.policy.expiration_us + 1;

    let error = system
        .execute(&deployment, &mut state, DEFAULT_GAS, spaeter, 1)
        .expect_err("Ablauf muss greifen");
    assert!(
        error.to_string().contains("ADM-006"),
        "erwartet ADM-006: {error}"
    );
    assert_eq!(system.chain().height(), 0);
}

#[test]
fn jede_entscheidung_erzeugt_genau_einen_auditeintrag() {
    let (mut system, deployment) = system_mit_vertrag();
    let mut state = ContractState::new();
    assert_eq!(system.audit_count(), 0, "Deployment entscheidet nichts");

    system
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .unwrap();
    assert_eq!(system.audit_count(), 1);

    let _ = system.execute(&deployment, &mut state, 0, NOW_US, 1);
    assert_eq!(
        system.audit_count(),
        2,
        "auch die Ablehnung wird protokolliert"
    );
}

#[test]
fn zwei_vertraege_leben_nebeneinander() {
    let mut system = System::devnet().unwrap();
    let a = system
        .deploy("fn main() -> i64 { return 3 + 4; }", NOW_US)
        .unwrap();
    let b = system
        .deploy("fn main() -> i64 { return 5 * 6; }", NOW_US)
        .unwrap();
    assert_ne!(a.code_hash, b.code_hash);

    let mut state_a = ContractState::new();
    let mut state_b = ContractState::new();
    let ra = system
        .execute(&a, &mut state_a, DEFAULT_GAS, NOW_US, 1)
        .unwrap();
    let rb = system
        .execute(&b, &mut state_b, DEFAULT_GAS, NOW_US, 1)
        .unwrap();

    assert_eq!(ra.result(), Some(7));
    assert_eq!(rb.result(), Some(30));
    assert_ne!(
        ra.state_root_after, rb.state_root_after,
        "die Wurzel bindet den Code"
    );
    system.chain().verify().unwrap();
}
