// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Phase 1 und 2 zu D-0: die Huelle `ATCE`/`0x02` ist der externe Eingang.
//!
//! Geprueft wird die Sicherheitsgrenze, nicht nur "es kommt ein Fehler":
//!
//! * `System::deploy_container` nimmt ausschliesslich `ATCE` an und prueft die
//!   Huelle **vor** Uebersetzung, Freigabe und Ausfuehrung.
//! * `System::execute` liest den Strom ueber die Huelle; ein Deployment ohne
//!   Huelle oder mit abweichenden inneren Bytes wird nicht ausgefuehrt.
//! * Es gibt **keinen** Rueckfall vom aeusseren auf das innere Format.
//! * Die Sollwerte der Fehlermatrix sind mit dem unabhaengigen
//!   Referenzdekoder `tools/atc_reference_decoder.py` gegengeprueft.
//!
//! Status: `TESTED` (lokale Suite). Kein `CI_VERIFIED`, kein `VERIFIED`.

use atc_system::{ContractState, Deployment, System, SystemError, DEFAULT_GAS};
use atc_vm::artifact::Artifact;
use atc_vm::envelope::Envelope;

const NOW_US: i64 = 1_800_000_000_000_000;
const SOURCE_A: &str =
    "fn main() -> i64 { let i = 0; let s = 0; while i < 4 { let s = s + i; let i = i + 1; } return s; }";
const SOURCE_B: &str = "fn main() -> i64 { let x = 7; return x; }";

fn system_mit_vertrag() -> (System, Deployment) {
    let mut system = System::devnet().expect("Devnet-System");
    let deployment = system.deploy(SOURCE_A, NOW_US).expect("Deployment");
    (system, deployment)
}

/// Erwartetes Paar `(Code, Offset)` der Huelle, gegengeprueft mit
/// `python3 tools/atc_reference_decoder.py <hex>`.
fn erwarte(error: &SystemError, code: &str, offset: usize) {
    match error {
        SystemError::Container(e) => {
            assert_eq!(
                (e.code(), e.offset()),
                (code, offset),
                "Fehleridentitaet der Huelle: {error}"
            );
        }
        other => panic!("erwartet Container({code} bei {offset}), erhalten: {other:?}"),
    }
}

/// Huelle fuer `SOURCE_A`. Bewusst aus einem eigenen System: die Registrierung
/// ist Compare-and-Insert (S1-2), ein zweiter Zulassungsweg fuer dasselbe
/// Programm waere im selben System ein Policy-Konflikt.
fn gueltige_huelle(_system: &mut System) -> Vec<u8> {
    System::devnet()
        .expect("Devnet")
        .deploy(SOURCE_A, NOW_US)
        .expect("Deployment")
        .container_bytes
}

// --- Phase 1: Eingang ------------------------------------------------------

#[test]
fn huelle_ist_der_externe_eingang() {
    let mut system = System::devnet().expect("Devnet");
    let container = gueltige_huelle(&mut system);

    // Die Huelle beginnt mit 0x41 ('A' von ATCE), nicht mit einem Opcode.
    assert_eq!(container[0], 0x41);
    assert_eq!(&container[0..4], b"ATCE");
    assert_eq!(container[4], 0x02);

    let aus_huelle = system
        .deploy_container(&container, NOW_US)
        .expect("Huelle angenommen");
    let aus_quelle = System::devnet()
        .expect("Devnet")
        .deploy(SOURCE_A, NOW_US)
        .expect("Quelle");

    assert_eq!(
        aus_huelle.code_hash, aus_quelle.code_hash,
        "beide Wege muessen denselben Bytecode-Hash ergeben"
    );
    assert_eq!(aus_huelle.ops, aus_quelle.ops);
    assert_eq!(
        aus_huelle.artifact_bytes, aus_quelle.artifact_bytes,
        "die innere Ableitung ist bytegleich"
    );
    assert_eq!(aus_huelle.source_instructions, 0, "Quelle ist unbekannt");
    assert_eq!(aus_huelle.divergences.len(), 0);
}

#[test]
fn huelle_und_quelle_fuehren_zum_selben_ergebnis() {
    // Zwei getrennte Systeme: die Registry fuehrt je code_hash genau eine
    // Freigabe, und die beiden Wege setzen verschiedene Herkunft
    // (source_provenance) in die Policy. Innerhalb eines Systems ueberschreibt
    // der zweite Weg daher die Freigabe des ersten (Befund F-1 im Nachweis).
    let mut aus_quelle_system = System::devnet().expect("Devnet");
    let mut aus_huelle_system = System::devnet().expect("Devnet");
    let container = gueltige_huelle(&mut aus_huelle_system);

    let a = aus_quelle_system.deploy(SOURCE_A, NOW_US).expect("Quelle");
    let b = aus_huelle_system
        .deploy_container(&container, NOW_US)
        .expect("Huelle");

    assert_eq!(a.code_hash, b.code_hash, "dasselbe Programm");

    let mut state_a = ContractState::new();
    let mut state_b = ContractState::new();
    let ra = aus_quelle_system
        .execute(&a, &mut state_a, DEFAULT_GAS, NOW_US, 1)
        .expect("Ausfuehrung A");
    let rb = aus_huelle_system
        .execute(&b, &mut state_b, DEFAULT_GAS, NOW_US, 1)
        .expect("Ausfuehrung B");

    assert_eq!(ra.result(), Some(6));
    assert_eq!(ra.result(), rb.result());
    assert_eq!(ra.state_root_after, rb.state_root_after);
    assert_eq!(state_a.words(), state_b.words());
}

#[test]
fn bytegleiche_rueckfuehrung_des_bytecodes() {
    let mut system = System::devnet().expect("Devnet");
    let container = gueltige_huelle(&mut system);

    let gelesen = Envelope::parse(&container).expect("Huelle");
    let inner = gelesen.into_artifact().expect("ATCB");
    let deployment = system.deploy_container(&container, NOW_US).expect("Huelle");
    system
        .deploy(SOURCE_A, NOW_US)
        .expect_err("zweiter Zulassungsweg fuer dasselbe Programm ist ein Konflikt");

    assert_eq!(
        inner.to_bytes(),
        deployment.artifact_bytes,
        "der Umlauf ueber die Huelle muss die ATCB-Bytes bytegleich zurueckgeben"
    );
    assert_eq!(inner.hash(), deployment.code_hash);
    // Der Bytecode-Hash stammt aus dem inneren Artefakt, nicht aus der Huelle.
    assert_ne!(
        deployment.container_hash_hex(),
        Artifact::parse(&deployment.artifact_bytes)
            .map(|a| atc_system::hex(&a.hash()))
            .unwrap_or_default(),
        "Huelle und Bytecode bleiben kryptografisch getrennt"
    );
}

// --- Phase 2: Negativmatrix -----------------------------------------------

#[test]
fn matrix_der_abgelehnten_hullen() {
    let mut system = System::devnet().expect("Devnet");
    let gueltig = gueltige_huelle(&mut system);

    // (Name, mutierte Bytes, erwarteter Code, erwarteter Offset)
    let mut faelle: Vec<(&str, Vec<u8>, &str, usize)> = Vec::new();

    let mut f = gueltig.clone();
    let letzte = f.len() - 1;
    f[letzte] ^= 0xFF;
    faelle.push(("digest manipuliert", f, "E-006", gueltig.len() - 32));

    let mut f = gueltig.clone();
    f[5] = 0x07;
    faelle.push(("falsche Generation", f, "E-009", 5));

    faelle.push(("ungueltige Laenge", gueltig[..41].to_vec(), "E-001", 41));

    let mut f = gueltig.clone();
    f[6..10].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    faelle.push(("Ueberlauf der Anzahl", f, "E-005", gueltig.len() - 32));

    faelle.push((
        "abgeschnittener Tail",
        gueltig[..gueltig.len() - 1].to_vec(),
        "E-005",
        gueltig.len() - 33,
    ));

    let mut f = gueltig.clone();
    f[0..4].copy_from_slice(b"ATC1");
    faelle.push(("falsches Magic", f, "E-007", 0));

    let mut f = gueltig.clone();
    f[4] = 0x03;
    faelle.push(("falsche Fassung", f, "E-003", 4));

    faelle.push(("leere Eingabe", Vec::new(), "E-001", 0));

    for (name, bytes, code, offset) in &faelle {
        // a) Eingang: die Huelle wird abgelehnt.
        let error = system
            .deploy_container(bytes, NOW_US)
            .expect_err(&format!("{name}: deploy_container muss ablehnen"));
        erwarte(&error, code, *offset);

        // b) Vorpruefung ohne Deployment liefert dieselbe Identitaet.
        let error = system
            .check_container(bytes)
            .expect_err(&format!("{name}: check_container muss ablehnen"));
        erwarte(&error, code, *offset);

        // c) Ausfuehrung: derselbe Fehler, keine Zustandsaenderung, kein Block.
        let mut deployment = system.deploy(SOURCE_A, NOW_US).expect("Deployment");
        deployment.container_bytes = bytes.clone();
        let mut state = ContractState::new();
        let hoehe = system.chain().height();
        let error = system
            .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
            .expect_err(&format!("{name}: execute muss ablehnen"));
        erwarte(&error, code, *offset);
        assert_eq!(state, ContractState::new(), "{name}: kein Zustand");
        assert_eq!(system.chain().height(), hoehe, "{name}: kein Block");
    }
}

#[test]
fn kein_rueckfall_auf_das_innere_format() {
    let mut system = System::devnet().expect("Devnet");
    let deployment = system.deploy(SOURCE_A, NOW_US).expect("Deployment");

    // Ein gueltiger ATCB-Container ist an der Huelle ein fremdes Format.
    let atcb = deployment.artifact_bytes.clone();
    assert_eq!(&atcb[0..4], b"ATCB");

    let error = system
        .deploy_container(&atcb, NOW_US)
        .expect_err("ATCB darf kein externer Eingang sein");
    erwarte(&error, "E-007", 0);

    // Auch bei der Ausfuehrung: keine Umdeutung, kein Rueckfall.
    let mut deployment = deployment;
    deployment.container_bytes = atcb;
    let mut state = ContractState::new();
    let error = system
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .expect_err("ATCB an der Huelle darf nicht ausgefuehrt werden");
    erwarte(&error, "E-007", 0);
    assert_eq!(system.chain().height(), 0);
}

#[test]
fn huelle_und_freigabe_muessen_dasselbe_programm_nennen() {
    // Eine strukturell gueltige Huelle mit anderem Programm ist kein Freibrief:
    // der Bytecode-Hash der Freigabe muss dazu passen.
    let mut system = System::devnet().expect("Devnet");
    let a = system.deploy(SOURCE_A, NOW_US).expect("Deployment A");
    let b = system.deploy(SOURCE_B, NOW_US).expect("Deployment B");
    assert_ne!(a.code_hash, b.code_hash);

    let mut deployment = a;
    deployment.container_bytes = b.container_bytes.clone();
    let mut state = ContractState::new();
    let error = system
        .execute(&deployment, &mut state, DEFAULT_GAS, NOW_US, 1)
        .expect_err("fremdes Programm muss abgelehnt werden");
    assert_eq!(error.stage(), "admission", "{error}");
    assert_eq!(state, ContractState::new());
}

#[test]
fn anzahl_ueberlauf_wird_sofort_erkannt() {
    // Kein Durchlauf ueber 4 Milliarden Instruktionen: die Stromgrenze folgt aus
    // der Laenge, die Anzahl ist nur Obergrenze (REQ-SCE-085).
    let mut system = System::devnet().expect("Devnet");
    let mut f = gueltige_huelle(&mut system);
    f[6..10].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());

    let start = std::time::Instant::now();
    let error = system.check_container(&f).expect_err("Ueberlauf");
    erwarte(&error, "E-005", f.len() - 32);
    assert!(
        start.elapsed() < std::time::Duration::from_secs(1),
        "die Pruefung darf nicht linear in der deklarierten Anzahl laufen"
    );
}
