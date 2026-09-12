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
    Passed { result: u64, n_ops: usize, state_evidence: Option<String> },
    Failed(String),
}

/// Parst "idx:val"-Slot-Spezifikationen (explizit, kein unwrap).
fn parse_slot_pair(flag: &str, raw: &str) -> Result<(usize, u64), String> {
    let parts: Vec<&str> = raw.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("{flag} erwartet <slot>:<wert>, bekam: {raw}"));
    }
    let idx: usize = parts[0]
        .parse()
        .map_err(|_| format!("{flag}: Slot-Index ist keine Zahl: {raw}"))?;
    let val: u64 = parts[1]
        .parse()
        .map_err(|_| format!("{flag}: Slot-Wert ist keine u64-Zahl: {raw}"))?;
    Ok((idx, val))
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

struct Args {
    ops_path: String,
    expect: Option<u64>,
    caller: u64,
    set_slots: Vec<(usize, u64)>,
    expect_slots: Vec<(usize, u64)>,
}

fn parse_args(args: Vec<String>) -> Result<Args, String> {
    let mut ops_path: Option<String> = None;
    let mut expect: Option<u64> = None;
    let mut caller: u64 = 0;
    let mut set_slots: Vec<(usize, u64)> = Vec::new();
    let mut expect_slots: Vec<(usize, u64)> = Vec::new();
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
            "--caller" => match args.get(i + 1) {
                Some(v) => {
                    caller = v
                        .parse()
                        .map_err(|_| format!("--caller ist keine u64-Zahl: {v}"))?;
                    i += 2;
                }
                None => return Err("--caller benoetigt eine Identitaet (u64)".to_string()),
            },
            "--set-slot" => match args.get(i + 1) {
                Some(v) => {
                    set_slots.push(parse_slot_pair("--set-slot", v)?);
                    i += 2;
                }
                None => return Err("--set-slot benoetigt <slot>:<wert>".to_string()),
            },
            "--expect-slot" => match args.get(i + 1) {
                Some(v) => {
                    expect_slots.push(parse_slot_pair("--expect-slot", v)?);
                    i += 2;
                }
                None => return Err("--expect-slot benoetigt <slot>:<wert>".to_string()),
            },
            other => return Err(format!("unbekanntes Argument: {other}")),
        }
    }
    match ops_path {
        Some(p) => Ok(Args { ops_path: p, expect, caller, set_slots, expect_slots }),
        None => Err("--ops ist Pflicht".to_string()),
    }
}

fn run_ops_file(args: &Args) -> RunOutcome {
    let path = &args.ops_path;
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

    // ATVM-Ausfuehrung im Contract-Kontext (Caller-Identitaet + Storage)
    let mut storage: Vec<u64> = Vec::new();
    for (idx, val) in &args.set_slots {
        if *idx >= storage.len() {
            storage.resize(idx + 1, 0);
        }
        storage[*idx] = *val;
    }
    let mut machine = vm::Vm::with_context(program, args.caller, storage);
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
    if let Some(expected) = args.expect {
        if result != expected {
            return RunOutcome::Failed(format!(
                "ERROR: Ausfuehrung {result} != erwartet {expected}"
            ));
        }
    }
    // Storage-Evidenz: erwartete Slots muessen exakt stimmen
    let final_state = machine.state();
    for (idx, want) in &args.expect_slots {
        let got = final_state.get(*idx).copied().unwrap_or(0);
        if got != *want {
            return RunOutcome::Failed(format!(
                "ERROR: Storage-Slot {idx} = {got}, erwartet {want}"
            ));
        }
    }
    let state_evidence = if args.expect_slots.is_empty() {
        None
    } else {
        Some(format!("{:?}", final_state))
    };
    RunOutcome::Passed { result, n_ops, state_evidence }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed = match parse_args(args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {e}");
            usage();
        }
    };

    match run_ops_file(&parsed) {
        RunOutcome::Passed { result, n_ops, state_evidence } => {
            match state_evidence {
                Some(ev) => println!(
                    "OK: {} — {n_ops} Ops, Ergebnis {result}, Storage {ev} (ATVM PASS)",
                    parsed.ops_path
                ),
                None => println!(
                    "OK: {} — {n_ops} Ops ausgefuehrt, Ergebnis {result} (ATVM PASS)",
                    parsed.ops_path
                ),
            }
            exit(0);
        }
        RunOutcome::Failed(msg) => {
            eprintln!("{msg}");
            exit(1);
        }
    }
}
