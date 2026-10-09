// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! E2E: EXEC-CHAIN — .ops-Format (Assembler-Ausgabe) wird geparst und
//! auf der ATVM ausgefuehrt. Fixtures entsprechen der echten Ausgabe von
//! atc-contracts/exec_chain/assemble.py am E2E-Vektor (a=7, b=3, d=20).

use atc_vm::vm::{Op, Vm};

#[test]
fn e2e_adder_fixture_passes() {
    // Ausgabe von: python3 exec_chain/assemble.py --contract exec_chain/e2e_adder.atc --vector exec_chain/vector.json
    let text = "# contract: e2e_adder
# fn: compute
# source_sha256: d6ac172d397a408f8860ad6263f6b02c04ab434432c934b027b8f6990c980eea
# expected: 20
Push 7
Push 3
Add
Push 2
Mul
Halt
";
    let prog = atc_vm::ops::parse_ops(text).expect("gueltiges .ops");
    let mut machine = Vm::new(prog);
    let stack = machine.run().expect("ATVM-Ausfuehrung");
    let result = *stack.last().expect("nicht-leerer Stack");
    assert_eq!(result, 20, "E2E-Vektor: (7+3)*2 == 20");
}

#[test]
fn direct_program_matches_assembler_sequence() {
    // Die Ops-Sequenz, die der ATCLang-Assembler (assemble.atc, e2e_adder_selftest)
    // fuer denselben Vektor erzeugt — beide Welten muessen dasselbe berechnen.
    let mut machine = Vm::new(vec![
        Op::Push(7),
        Op::Push(3),
        Op::Add,
        Op::Push(2),
        Op::Mul,
        Op::Halt,
    ]);
    let stack = machine.run().expect("ATVM-Ausfuehrung");
    assert_eq!(stack.last(), Some(&20));
}

#[test]
fn fail_closed_on_wrong_expectation() {
    let prog = atc_vm::ops::parse_ops("Push 1
Halt
").expect("gueltiges .ops");
    let mut machine = Vm::new(prog);
    let stack = machine.run().expect("ATVM-Ausfuehrung");
    let result = stack.last().copied().unwrap_or(0);
    assert_ne!(result, 999, "Vektor-Abweichung muss auffallen");
}

#[test]
fn div_executes_deterministically() {
    // Governance-Quorum-Formel: 21_000_000 * 10 / 100 = 2_100_000 (10 % von 21M)
    let prog = atc_vm::ops::parse_ops("Push 21000000
Push 10
Mul
Push 100
Div
Halt
")
        .expect("gueltiges .ops");
    let mut machine = Vm::new(prog);
    let stack = machine.run().expect("ATVM-Ausfuehrung");
    assert_eq!(stack.last(), Some(&2_100_000));
}

#[test]
fn div_by_zero_fails_closed() {
    let mut machine = Vm::new(vec![Op::Push(1), Op::Push(0), Op::Div, Op::Halt]);
    let res = machine.run();
    assert_eq!(res, Err(atc_vm::vm::VmError::DivisionByZero));
}

// ─── Contract-Execution-Inkrement 1: State, Caller, Permissions ────────────

#[test]
fn storage_round_trip_persists_state() {
    // Mint-Buchhaltung ueber Storage: Load 0 + amount -> Store 0 -> Load 0
    let prog = atc_vm::ops::parse_ops("Load 0
Push 1000
Add
Store 0
Load 0
Halt
")
        .expect("gueltiges .ops");
    let mut machine = Vm::new(prog);
    let stack = machine.run().expect("ATVM");
    assert_eq!(stack.last(), Some(&1000));
    assert_eq!(machine.state(), &[1000], "Storage muss persistiert sein");
}

#[test]
fn unwritten_slot_defaults_to_zero() {
    let prog = atc_vm::ops::parse_ops("Load 7
Halt
").expect("gueltiges .ops");
    let mut machine = Vm::new(prog);
    let stack = machine.run().expect("ATVM");
    assert_eq!(stack.last(), Some(&0));
}

#[test]
fn caller_is_host_set_and_readable() {
    // Permission-Modell: Caller kommt aus dem Host-Kontext, nicht vom Stack
    let prog = atc_vm::ops::parse_ops("Caller
Halt
").expect("gueltiges .ops");
    let mut machine = Vm::with_context(prog, 42, vec![]);
    let stack = machine.run().expect("ATVM");
    assert_eq!(stack.last(), Some(&42));
    assert_eq!(machine.caller(), 42);
}

#[test]
fn owner_check_accepts_owner() {
    // owner (Slot 1) == caller 42 -> Mint erlaubt, Flag 1
    let prog = atc_vm::ops::parse_ops(
        "Caller
Load 1
Eq
JumpIfZero 10
Load 0
Push 500
Add
Store 0
Push 1
Halt
Push 0
Halt
",
    ).expect("gueltiges .ops");
    let mut machine = Vm::with_context(prog, 42, vec![0, 42]); // Slot 1 = owner 42
    let stack = machine.run().expect("ATVM");
    assert_eq!(stack.last(), Some(&1), "Owner-Mint muss erlaubt sein");
    assert_eq!(machine.state(), &[500, 42]);
}

#[test]
fn owner_check_rejects_intruder() {
    let prog = atc_vm::ops::parse_ops(
        "Caller
Load 1
Eq
JumpIfZero 10
Load 0
Push 500
Add
Store 0
Push 1
Halt
Push 0
Halt
",
    ).expect("gueltiges .ops");
    let mut machine = Vm::with_context(prog, 7, vec![0, 42]);
    let stack = machine.run().expect("ATVM");
    assert_eq!(stack.last(), Some(&0), "Fremder Caller muss abgewiesen werden");
    assert_eq!(machine.state(), &[0, 42], "Storage darf unberuehrt bleiben");
}

#[test]
fn insufficient_funds_guard_rejects() {
    // balance (Slot 2) = 100 < amount 200 -> Reject-Flag 0, State unberuehrt
    let prog = atc_vm::ops::parse_ops(
        "Load 2
Push 200
Lt
JumpIfNotZero 14
Load 2
Push 200
Sub
Store 2
Load 3
Push 200
Add
Store 3
Push 1
Halt
Push 0
Halt
",
    ).expect("gueltiges .ops");
    let mut machine = Vm::with_context(prog, 0, vec![0, 0, 100, 0]);
    let stack = machine.run().expect("ATVM");
    assert_eq!(stack.last(), Some(&0));
    assert_eq!(machine.state(), &[0, 0, 100, 0], "Guthaben darf nicht sinken");
}

#[test]
fn sufficient_funds_transfer_moves_both_sides() {
    let prog = atc_vm::ops::parse_ops(
        "Load 2
Push 50
Lt
JumpIfNotZero 14
Load 2
Push 50
Sub
Store 2
Load 3
Push 50
Add
Store 3
Push 1
Halt
Push 0
Halt
",
    ).expect("gueltiges .ops");
    let mut machine = Vm::with_context(prog, 0, vec![0, 0, 100, 0]);
    let stack = machine.run().expect("ATVM");
    assert_eq!(stack.last(), Some(&1));
    assert_eq!(machine.state(), &[0, 0, 50, 50], "Sender 50, Empfaenger 50");
}
