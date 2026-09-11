// Copyright (c) 2026 Michael Wroblewski - Apache-2.0
//! EXEC-Receipt-Runner (ATC-CONTRACT-EXEC-001 / F-084, SCR-0104).
//! Laedt Assembler-Bytecode (.ops), fuehrt ihn auf der ATVM aus, prueft
//! das erwartete Ergebnis und schreibt einen deterministischen Receipt
//! (JSON, keine Zeitstempel). Exit 1 bei Mismatch (Fail-Gate).

use atc_vm::vm::{Op, Vm};
use std::collections::BTreeMap;

fn main() {
    let ops_path = std::env::var("ATVM_OPS").unwrap_or_else(|_| "build/e2e_adder.ops".to_string());
    let out_path = std::env::var("ATVM_OUT").unwrap_or_else(|_| "build/receipt.json".to_string());
    let text = std::fs::read_to_string(&ops_path).unwrap_or_else(|e| panic!("ops nicht lesbar {}: {}", ops_path, e));
    let mut meta: BTreeMap<String, String> = BTreeMap::new();
    let mut expected: u64 = 0;
    let mut program: Vec<Op> = Vec::new();
    for line in text.lines() {
        let l = line.trim();
        if l.is_empty() { continue; }
        if let Some(rest) = l.strip_prefix('#') {
            if let Some(v) = rest.trim().strip_prefix("expected:") {
                expected = v.trim().parse().expect("expected ist keine u64");
            } else if let Some(idx) = rest.find(':') {
                meta.insert(rest[..idx].trim().to_string(), rest[idx + 1..].trim().to_string());
            }
            continue;
        }
        let mut it = l.split_whitespace();
        let head = it.next().unwrap_or("");
        let arg = it.next();
        program.push(match (head, arg) {
            ("Push", Some(v)) => Op::Push(v.parse().expect("Push-Operand ist keine u64")),
            ("Add", None) => Op::Add,
            ("Sub", None) => Op::Sub,
            ("Mul", None) => Op::Mul,
            ("Dup", None) => Op::Dup,
            ("Swap", None) => Op::Swap,
            ("Eq", None) => Op::Eq,
            ("Lt", None) => Op::Lt,
            ("Halt", None) => Op::Halt,
            ("Jump", Some(v)) => Op::Jump(v.parse().expect("Jump-Ziel ist kein usize")),
            ("JumpIfNotZero", Some(v)) => Op::JumpIfNotZero(v.parse().expect("Jump-Ziel ist kein usize")),
            _ => panic!("unbekanntes Op in Zeile: {}", l),
        });
    }
    let ops_count = program.len();
    let mut vm = Vm::new(program);
    let stack = vm.run().unwrap_or_else(|e| panic!("ATVM-Ausfuehrungsfehler: {:?}", e));
    let result = stack.last().copied().unwrap_or(0);
    let status = if result == expected { "PASS" } else { "FAIL" };
    let receipt = format!(
        "{{\"chain_id\": 658467, \"contract\": \"{}\", \"fn\": \"{}\", \"source_sha256\": \"{}\", \"ops\": {}, \"result\": {}, \"expected\": {}, \"status\": \"{}\", \"vm\": \"atc-vm-0.1.0\"}}\n",
        meta.get("contract").cloned().unwrap_or_default(),
        meta.get("fn").cloned().unwrap_or_default(),
        meta.get("source_sha256").cloned().unwrap_or_default(),
        ops_count, result, expected, status
    );
    std::fs::write(&out_path, receipt).unwrap_or_else(|e| panic!("Receipt nicht schreibbar {}: {}", out_path, e));
    print!("{}", receipt);
    if result != expected {
        std::process::exit(1);
    }
}
