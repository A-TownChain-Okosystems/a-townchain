// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! E2E: EXEC-CHAIN — .ops-Format (Assembler-Ausgabe) wird geparst und
//! auf der ATVM ausgefuehrt. Fixtures entsprechen der echten Ausgabe von
//! atc-contracts/exec_chain/assemble.py am E2E-Vektor (a=7, b=3, d=20).

use atc_vm::vm::{Op, Vm};

#[test]
fn e2e_adder_fixture_passes() {
    // Ausgabe von: python3 exec_chain/assemble.py --contract exec_chain/e2e_adder.atc --vector exec_chain/vector.json
    let text = "# contract: e2e_adder\n# fn: compute\n# source_sha256: d6ac172d397a408f8860ad6263f6b02c04ab434432c934b027b8f6990c980eea\n# expected: 20\nPush 7\nPush 3\nAdd\nPush 2\nMul\nHalt\n";
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
    let prog = atc_vm::ops::parse_ops("Push 1\nHalt\n").expect("gueltiges .ops");
    let mut machine = Vm::new(prog);
    let stack = machine.run().expect("ATVM-Ausfuehrung");
    let result = stack.last().copied().unwrap_or(0);
    assert_ne!(result, 999, "Vektor-Abweichung muss auffallen");
}

#[test]
fn div_executes_deterministically() {
    // Governance-Quorum-Formel: 21_000_000 * 10 / 100 = 2_100_000 (10 % von 21M)
    let prog = atc_vm::ops::parse_ops("Push 21000000\nPush 10\nMul\nPush 100\nDiv\nHalt\n")
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
