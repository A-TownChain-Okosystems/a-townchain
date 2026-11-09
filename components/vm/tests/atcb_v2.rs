// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! ATCB-v2 (ATC-BC-002 Kandidat): ASSERT, REVERT, REVERTIF und Verifier-Regeln.

use atc_vm::vm::{Op, Vm, VmError};

fn v2(program: Vec<Op>) -> Vm {
    Vm::new(program).with_generation(2)
}

#[test]
fn assert_wahr_laeuft_weiter() {
    let mut vm = v2(vec![Op::Push(1), Op::Assert, Op::Push(7), Op::Halt]);
    assert_eq!(vm.run_with_gas(100).unwrap(), vec![7]);
}

#[test]
fn assert_falsch_bricht_ab() {
    let mut vm = v2(vec![Op::Push(0), Op::Assert, Op::Push(7), Op::Halt]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::AssertFailed));
}

#[test]
fn revert_bricht_unbedingt_ab() {
    let mut vm = v2(vec![Op::Push(9), Op::Store(0), Op::Revert(300)]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::Reverted(300)));
    // Ruecklauf darf keinen Zustand hinterlassen.
    assert_eq!(vm.state(), &[] as &[u64]);
}

#[test]
fn revertif_null_bricht_ab() {
    let mut vm = v2(vec![Op::Push(0), Op::Push(300), Op::RevertIf, Op::Halt]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::Reverted(300)));
}

#[test]
fn revertif_ungleich_null_laeuft_weiter() {
    let mut vm = v2(vec![
        Op::Push(1),
        Op::Push(300),
        Op::RevertIf,
        Op::Push(9),
        Op::Halt,
    ]);
    assert_eq!(vm.run_with_gas(100).unwrap(), vec![9]);
}

#[test]
fn v2_befehl_in_v1_abgelehnt() {
    let mut vm = Vm::new(vec![Op::Push(1), Op::Assert, Op::Halt]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::V2OpInV1));
}

#[test]
fn fehlercode_ausserhalb_des_raums_abgelehnt() {
    let mut vm = v2(vec![Op::Revert(7)]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::InvalidRevertCode(7)));
}

#[test]
fn code_nach_halt_abgelehnt() {
    let mut vm = v2(vec![Op::Halt, Op::Push(1)]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::UnreachableCode(1)));
}

#[test]
fn assert_kostet_zwei_gas() {
    let mut vm = v2(vec![Op::Push(1), Op::Assert, Op::Halt]);
    assert_eq!(
        vm.run_with_gas(2),
        Err(VmError::OutOfGas {
            required: 2,
            remaining: 1
        })
    );
}

#[test]
fn stapelunterlauf_abgelehnt() {
    let mut vm = v2(vec![Op::Assert, Op::Halt]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::StackUnderflow));
}
