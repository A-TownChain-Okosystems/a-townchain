// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! atc-vm-runner — native ATVM execution and EXEC-GATE assembly.
//!
//! AD-008: ATVM is the execution boundary — native Rust infrastructure.
//! The EXEC-GATE assembler replaces the former Python production path.

mod assembler;
mod context;
mod ops;
mod vm;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

enum RunOutcome {
    Passed { result: u64, n_ops: usize, state_evidence: Option<String> },
    Failed(String),
}

fn parse_slot_pair(flag: &str, raw: &str) -> Result<(usize, u64), String> {
    let parts: Vec<&str> = raw.split(':').collect();
    if parts.len() != 2 { return Err(format!("{flag} expects <slot>:<value>, got: {raw}")); }
    let idx = parts[0].parse::<usize>().map_err(|_| format!("{flag}: invalid slot index: {raw}"))?;
    let val = parts[1].parse::<u64>().map_err(|_| format!("{flag}: invalid u64 value: {raw}"))?;
    Ok((idx, val))
}

fn usage() -> ! {
    eprintln!("atc-vm-runner — native EXEC-GATE assembler + ATVM execution");
    eprintln!();
    eprintln!("Execution:");
    eprintln!("  atc-vm-runner --ops <file.ops> [--expect <u64>]");
    eprintln!();
    eprintln!("Native assembly:");
    eprintln!("  atc-vm-runner --contract <file.atc> --vector <file.json> --out <dir>");
    eprintln!();
    eprintln!("  --ops          ATVM .ops input");
    eprintln!("  --expect       expected vector value");
    eprintln!("  --contract     ATCLang EXEC-GATE contract input");
    eprintln!("  --vector       JSON execution vector");
    eprintln!("  --out          directory for generated .ops");
    eprintln!("  --caller       contract caller identity (u64)");
    eprintln!("  --set-slot     initial storage slot <slot>:<value>");
    eprintln!("  --expect-slot  final storage slot <slot>:<value>");
    exit(2);
}

struct Args {
    ops_path: Option<String>, contract_path: Option<String>, vector_path: Option<String>, out_dir: Option<String>,
    expect: Option<u64>, caller: u64, set_slots: Vec<(usize, u64)>, expect_slots: Vec<(usize, u64)>,
}

fn parse_args(args: Vec<String>) -> Result<Args, String> {
    let mut parsed = Args { ops_path: None, contract_path: None, vector_path: None, out_dir: None, expect: None, caller: 0, set_slots: Vec::new(), expect_slots: Vec::new() };
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--ops" => { parsed.ops_path = Some(args.get(i + 1).ok_or("--ops requires a path")?.clone()); i += 2; }
            "--contract" => { parsed.contract_path = Some(args.get(i + 1).ok_or("--contract requires a path")?.clone()); i += 2; }
            "--vector" => { parsed.vector_path = Some(args.get(i + 1).ok_or("--vector requires a path")?.clone()); i += 2; }
            "--out" => { parsed.out_dir = Some(args.get(i + 1).ok_or("--out requires a directory")?.clone()); i += 2; }
            "--expect" => { let raw = args.get(i + 1).ok_or("--expect requires a u64")?; parsed.expect = Some(raw.parse::<u64>().map_err(|_| format!("--expect is not u64: {raw}"))?); i += 2; }
            "--caller" => { let raw = args.get(i + 1).ok_or("--caller requires a u64")?; parsed.caller = raw.parse::<u64>().map_err(|_| format!("--caller is not u64: {raw}"))?; i += 2; }
            "--set-slot" => { let raw = args.get(i + 1).ok_or("--set-slot requires <slot>:<value>")?; parsed.set_slots.push(parse_slot_pair("--set-slot", raw)?); i += 2; }
            "--expect-slot" => { let raw = args.get(i + 1).ok_or("--expect-slot requires <slot>:<value>")?; parsed.expect_slots.push(parse_slot_pair("--expect-slot", raw)?); i += 2; }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(parsed)
}

fn run_ops_file(args: &Args, path: &str) -> RunOutcome {
    let text = match fs::read_to_string(path) { Ok(t) => t, Err(e) => return RunOutcome::Failed(format!("ERROR: cannot read .ops file {path}: {e}")) };
    let program = match ops::parse_ops(&text) { Ok(p) => p, Err(e) => return RunOutcome::Failed(format!("ERROR: .ops parse: {e:?}")) };
    let n_ops = program.len();
    let mut storage = Vec::new();
    for (idx, val) in &args.set_slots { if *idx >= storage.len() { storage.resize(idx + 1, 0); } storage[*idx] = *val; }
    let mut machine = vm::Vm::with_context(program, args.caller, storage);
    let stack = match machine.run() { Ok(s) => s, Err(e) => return RunOutcome::Failed(format!("ERROR: ATVM: {e:?}")) };
    let result = stack.last().copied().unwrap_or(0);
    if let Some(expected) = args.expect { if result != expected { return RunOutcome::Failed(format!("ERROR: execution {result} != expected {expected}")); } }
    let final_state = machine.state();
    for (idx, want) in &args.expect_slots { let got = final_state.get(*idx).copied().unwrap_or(0); if got != *want { return RunOutcome::Failed(format!("ERROR: storage slot {idx} = {got}, expected {want}")); } }
    let state_evidence = if args.expect_slots.is_empty() { None } else { Some(format!("{:?}", final_state)) };
    RunOutcome::Passed { result, n_ops, state_evidence }
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let args = match parse_args(raw_args) { Ok(v) => v, Err(e) => { eprintln!("ERROR: {e}"); usage(); } };
    if let Some(contract) = &args.contract_path {
        let vector = match &args.vector_path { Some(v) => v, None => { eprintln!("ERROR: --vector is required with --contract"); exit(2); } };
        let out = match &args.out_dir { Some(v) => PathBuf::from(v), None => { eprintln!("ERROR: --out is required with --contract"); exit(2); } };
        match assembler::assemble(std::path::Path::new(contract), std::path::Path::new(vector), &out) {
            Ok(path) => { println!("OK: native ATCLang assembly -> {} (simulation PASS)", path.display()); exit(0); }
            Err(e) => { eprintln!("ERROR: native assembler: {e}"); exit(1); }
        }
    }
    let ops = match &args.ops_path { Some(v) => v, None => { eprintln!("ERROR: either --ops or --contract is required"); usage(); } };
    match run_ops_file(&args, ops) {
        RunOutcome::Passed { result, n_ops, state_evidence } => {
            if let Some(ev) = state_evidence { println!("OK: {ops} — {n_ops} Ops, result {result}, storage {ev} (ATVM PASS)"); }
            else { println!("OK: {ops} — {n_ops} Ops executed, result {result} (ATVM PASS)"); }
            exit(0);
        }
        RunOutcome::Failed(msg) => { eprintln!("{msg}"); exit(1); }
    }
}
