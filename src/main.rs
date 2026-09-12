// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! atc-vm-runner — fuehrt eine .ops-Datei des EXEC-CHAIN-Assemblers aus.
//!
//! AD-008: ATVM ist die Ausfuehrungsgrenze — dieser Runner ist native
//! Rust-Infrastruktur (keine Consensus-Semantik). Explizite Fehler-
//! behandlung throughout (Owner-Regel: kein unwrap() im konsens-
//! kritischen Pfad), Exit-Codes als Evidenz:
//!   0 = PASS (Simulation/Ausfuehrung erfolgreich, evtl. --expect erfuellt)
//!   1 = Ausfuehrungsfehler oder Vektor-Abweichung
//!   2 = Benutzungsfehler (Argumente/Datei)

mod ops;
mod vm;

use std::env;
use std::fs;
use std::process::exit;

enum RunOutcome {
    Passed { result: u64, n_ops: usize },
    Failed(String),
}

fn usage() -> ! {
    eprintln!("atc-vm-runner — ATVM-Ausfuehrung des EXEC-GATE-Subsets");
    eprintln!("");
    eprintln!("Nutzung:");
    eprintln!("  atc-vm-runner --ops <datei.ops> [--expect <u64>]");
    eprintln!("");
    eprintln!("  --ops     Pfad zur .ops-Datei (EXEC-CHAIN-Assembler-Format)");
    eprintln!("  --expect  erwarteter Vektorwert (fail-fast gegen Testvektor)");
    exit(2);
}

fn parse_args(args: Vec<String>) -> Result<(String, Option<u64>), String> {
    let mut ops_path: Option<String> = None;
    let mut expect: Option<u64> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ops" => match args.get(i + 1) {
                Some(p) => {
                    ops_path = Some(p.clone());
                    i += 2;
                }
                None => return Err("--ops benoetigt einen Dateipfad".to_string()),
            },
            "--expect" => match args.get(i + 1) {
                Some(v) => {
                    let parsed: u64 = v
                        .parse()
                        .map_err(|_| format!("--expect ist keine u64-Zahl: {v}"))?;
                    expect = Some(parsed);
                    i += 2;
                }
                None => return Err("--expect benoetigt einen u64-Wert".to_string()),
            },
            other => return Err(format!("unbekanntes Argument: {other}")),
        }
    }
    match ops_path {
        Some(p) => Ok((p, expect)),
        None => Err("--ops ist Pflicht".to_string()),
    }
}

fn run_ops_file(path: &str, expect: Option<u64>) -> RunOutcome {
    // Datei lesen — explizit, kein unwrap
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            return RunOutcome::Failed(format!("ERROR: Datei nicht lesbar ({path}): {e}"));
        }
    };

    // Parsen — fail-closed gegen unbekannte Ops
    let program = match ops::parse_ops(&text) {
        Ok(p) => p,
        Err(e) => {
            return RunOutcome::Failed(format!("ERROR: .ops-Parse: {e:?}"));
        }
    };
    let n_ops = program.len();

    // ATVM-Ausfuehrung
    let mut machine = vm::Vm::new(program);
    let stack = match machine.run() {
        Ok(s) => s,
        Err(e) => {
            return RunOutcome::Failed(format!("ERROR: ATVM: {e:?}"));
        }
    };

    // Ergebniswert: Top-of-Stack (leerer Stack = 0, wie die Referenzsimulatoren)
    let result: u64 = match stack.last() {
        Some(v) => *v,
        None => 0,
    };

    // Fail-fast gegen den erwarteten Vektorwert
    if let Some(expected) = expect {
        if result != expected {
            return RunOutcome::Failed(format!(
                "ERROR: Ausfuehrung {result} != erwartet {expected}"
            ));
        }
    }
    RunOutcome::Passed { result, n_ops }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (ops_path, expect) = match parse_args(args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {e}");
            usage();
        }
    };

    match run_ops_file(&ops_path, expect) {
        RunOutcome::Passed { result, n_ops } => {
            println!("OK: {ops_path} — {n_ops} Ops ausgefuehrt, Ergebnis {result} (ATVM PASS)");
            exit(0);
        }
        RunOutcome::Failed(msg) => {
            eprintln!("{msg}");
            exit(1);
        }
    }
}
