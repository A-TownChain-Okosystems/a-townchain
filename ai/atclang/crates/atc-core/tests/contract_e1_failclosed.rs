// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! E1 (Audit 2026-11-12, F1): Vertrags-Lowering fail-closed im Monorepo.
//!
//! Abnahmekriterien, die diese Datei bindet:
//! - `Stmt::Contract` wird nicht stillschweigend uebersprungen (Gate vor dem Lowering).
//! - Nicht unterstuetzte Vertragsruempfe erzeugen einen expliziten Compilerfehler,
//!   der Vertrag, Basen, Funktionen, Zustandsfelder, Events und `require`-Guards nennt.
//! - `atc run` liefert keinen falschen Exit 0 und keine Ausgabe fuer Vertraege.
//! - `atc check` meldet keinen Vertrag als "validiert".
//! - `atc run` ohne Einstiegspunkt (leer oder reine Deklaration) ist ein Fehler.
//! - Kein Ersatzprogramm (`Push(0); Halt`) als Erfolgspfad.

use atc_core::lower::{executable_subset_gate, lower_program, require_entry_point};
use atc_core::parser::parse_program;
use std::path::{Path, PathBuf};
use std::process::Command;

fn gate(src: &str) -> String {
    let program = parse_program(src).expect("Quelle parst");
    executable_subset_gate(&program)
        .expect_err("Vertrag muss abgelehnt werden")
        .message
}

fn lower_err(src: &str) -> String {
    let program = parse_program(src).expect("Quelle parst");
    lower_program(&program)
        .expect_err("Vertrag muss beim Lowering abgelehnt werden")
        .message
}

fn atc(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_atc"))
        .args(args)
        .output()
        .expect("atc startbar")
}

fn tmp(name: &str, body: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("atc-e1-failclosed");
    std::fs::create_dir_all(&dir).expect("tempdir");
    let path = dir.join(name);
    std::fs::write(&path, body).expect("Quelle schreibbar");
    path
}

// Abnahmekriterium: ein Vertrag mit Logik ist kein ausfuehrbares Programm.
#[test]
fn vertrag_mit_main_42_wird_abgelehnt() {
    let msg = gate("contract C { fn main() -> i64 { return 42; } }");
    assert!(msg.contains("Vertrag 'C'"), "nennt den Vertrag: {msg}");
    assert!(msg.contains("nicht ausfuehrbar"), "nennt den Grund: {msg}");
    assert!(msg.contains("1 Funktionen"), "zaehlt Funktionen: {msg}");
}

#[test]
fn require_guards_werden_gezaehlt() {
    let msg = gate("contract R { fn f() -> i64 { require(1 == 1, \"x\") return 1; } }");
    assert!(msg.contains("1 require-Guards"), "zaehlt require: {msg}");
}

#[test]
fn basen_werden_genannt() {
    let msg = gate("contract K: BaseVault { fn f() -> i64 { return 1; } }");
    assert!(msg.contains("Basen: BaseVault"), "nennt die Basis: {msg}");
}

#[test]
fn zustand_und_events_werden_gezaehlt() {
    let msg =
        gate("contract S { state a: i64 = 1  event E  event F  fn f() -> i64 { return 1; } }");
    assert!(msg.contains("1 Zustandsfelder"), "zaehlt Zustand: {msg}");
    assert!(msg.contains("2 Events"), "zaehlt Events: {msg}");
}

#[test]
fn lowering_gate_und_public_gate_sind_identisch() {
    let src = "contract C { fn f() -> i64 { return 7; } }";
    assert_eq!(
        gate(src),
        lower_err(src),
        "check- und run-Pfad nutzen dasselbe Gate"
    );
}

#[test]
fn vertrag_zwischen_anderen_deklarationen_wird_gefunden() {
    let msg = gate(
        "const PORT: u16 = 4000\nstruct Route { path: String }\n\
         contract Inner { fn m() -> i64 { return 1; } }\nfn main() -> i64 { return 0; }",
    );
    assert!(
        msg.contains("Vertrag 'Inner'"),
        "Vertrag auch mit main abgelehnt: {msg}"
    );
}

// Abnahmekriterium: kein falscher Exit 0, keine Ausgabe, kein Artefakt.
#[test]
fn atc_run_vertrag_exit_1_ohne_ausgabe_und_ohne_artefakt() {
    let file = tmp(
        "contract.atc",
        "contract C { fn main() -> i64 { return 42; } }\n",
    );
    let out = atc(&["run", file.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1), "Exit-Code 1 fuer Vertrag");
    assert!(
        out.stdout.is_empty(),
        "keine Ausgabe: {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("nicht ausfuehrbar"),
        "Grund im stderr: {stderr}"
    );
    let artifact = Path::new(file.to_str().unwrap()).with_extension("atcb");
    assert!(
        !artifact.exists(),
        "kein Bytecode-Artefakt fuer einen Vertrag"
    );
}

#[test]
fn atc_check_vertrag_exit_1() {
    let file = tmp(
        "check_contract.atc",
        "contract C { fn main() -> i64 { return 42; } }\n",
    );
    let out = atc(&["check", file.to_str().unwrap()]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "check darf den Vertrag nicht validieren"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("nicht ausfuehrbar"),
        "Grund im stderr: {stderr}"
    );
    assert!(
        !stderr.contains("validiert"),
        "kein OK fuer einen Vertrag: {stderr}"
    );
}

// Abnahmekriterium: ohne Einstiegspunkt ist nichts ausfuehrbar.
#[test]
fn atc_run_leere_quelle_exit_1() {
    let file = tmp("empty.atc", "\n");
    let out = atc(&["run", file.to_str().unwrap()]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "leere Quelle ist nicht ausfuehrbar"
    );
    assert!(out.stdout.is_empty(), "kein `0` als Scheinerfolg");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("kein Einstiegspunkt"),
        "Grund im stderr: {stderr}"
    );
}

#[test]
fn atc_run_reine_deklarationsquelle_exit_1() {
    let file = tmp("decl.atc", "fn helper() -> i64 { return 5; }\n");
    let out = atc(&["run", file.to_str().unwrap()]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "Deklaration ohne main ist nicht ausfuehrbar"
    );
    assert!(out.stdout.is_empty(), "kein `0` als Scheinerfolg");
}

#[test]
fn require_entry_point_akzeptiert_main() {
    let program = parse_program("fn main() -> i64 { return 42; }").expect("parst");
    require_entry_point(&program).expect("main ist ein Einstiegspunkt");
}

#[test]
fn atc_run_mit_main_bleibt_erfolgreich() {
    let file = tmp("main.atc", "fn main() -> i64 { return 42; }\n");
    let out = atc(&["run", file.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "42");
}

#[test]
fn atc_check_mit_main_bleibt_erfolgreich() {
    let file = tmp("check_main.atc", "fn main() -> i64 { return 42; }\n");
    let out = atc(&["check", file.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0), "Programme bleiben pruefbar");
}

// Beispiel-Korpus: das Gateway ist ein Vertrag mit 10 Funktionen.
#[test]
fn gateway_beispiel_wird_abgelehnt() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/main.atc");
    let src = std::fs::read_to_string(&path).expect("Beispiel lesbar");
    let msg = gate(&src);
    assert!(
        msg.contains("Vertrag 'Gateway'"),
        "nennt den Vertrag: {msg}"
    );
    assert!(
        msg.contains("10 Funktionen"),
        "Gateway hat 10 Funktionen: {msg}"
    );
}
