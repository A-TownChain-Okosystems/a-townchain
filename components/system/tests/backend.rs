// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Gate B — das Uebersetzer-Backend ist strikt.
//!
//! Jeder Fall hier ist ein Fall, in dem **abgelehnt** statt genaehert wird.
//! Die Programme werden dort, wo die Quelle die Form nicht hergibt, direkt als
//! Kompilat gebaut: so wird genau die Grenze geprueft, nicht der Parser davor.

use atc_core::bytecode::Bytecode;
use atc_core::lower::{CompiledFunction, CompiledProgram};
use atc_system::backend::BackendError;
use atc_system::{compile, translate};
use atc_vm::vm::Op;

fn program(instructions: Vec<atc_core::bytecode::Instruction>) -> CompiledProgram {
    CompiledProgram {
        functions: vec![CompiledFunction {
            name: "__main__".to_string(),
            param_count: 0,
            local_count: 8,
            bytecode: Bytecode { instructions },
        }],
        entry: 0,
    }
}

#[test]
fn gerade_formen_werden_eins_zu_eins_abgebildet() {
    let compiled = compile("fn main() -> i64 { return 1 + 2; }").unwrap();
    let translation = translate(&compiled).unwrap();
    assert_eq!(
        translation.ops,
        vec![Op::Push(1), Op::Push(2), Op::Add, Op::Halt, Op::Halt],
        "Return wird zu Halt, danach folgt der Abschluss-Halt"
    );
    assert_eq!(translation.source_len, 4);
}

#[test]
fn lokale_werden_zu_speicherplaetzen() {
    let compiled = compile("fn main() -> i64 { let a = 4; let b = a + 1; return b; }").unwrap();
    let translation = translate(&compiled).unwrap();
    assert!(translation.ops.contains(&Op::Store(0)));
    assert!(translation.ops.contains(&Op::Load(0)));
}

#[test]
fn erweiterte_vergleiche_verschieben_die_folgenden_pcs() {
    // a != b wird zu Eq, Push(1), Swap, Sub (vier Operationen).
    let compiled = compile("fn main() -> i64 { let a = 1; let b = 2; return a != b; }").unwrap();
    let translation = translate(&compiled).unwrap();
    let ops = &translation.ops;
    let position = ops
        .windows(4)
        .position(|w| w == [Op::Eq, Op::Push(1), Op::Swap, Op::Sub])
        .expect("Erweiterung von != muss als Vierergruppe im Programm stehen");
    // Die Zuordnung zeigt auf den ANFANG der Erweiterung, nie in die Mitte.
    assert!(translation.pc_map.contains(&position));
}

#[test]
fn sprung_ueber_eine_erweiterung_landet_am_anfang_der_zielinstruktion() {
    use atc_core::bytecode::Instruction::*;
    // pc0: 1, pc1: 2, pc2: Ne (4 Ops), pc3: Jump(-2) -> Ziel pc2, also Anfang der Erweiterung.
    let p = program(vec![ConstI64(1), ConstI64(2), Ne, Jump(-2)]);
    let translation = translate(&p).unwrap();
    let jump_index = translation.ops.len() - 2; // vor dem Abschluss-Halt
    let Op::Jump(target) = translation.ops[jump_index] else {
        panic!("erwartet Jump, war {:?}", translation.ops[jump_index]);
    };
    assert_eq!(
        target, translation.pc_map[2],
        "Ziel muss der Anfang der Erweiterung sein"
    );
}

#[test]
fn sprung_ans_programmende_wird_zu_einem_echten_halt() {
    use atc_core::bytecode::Instruction::*;
    // pc0: 1, pc1: Jump(0) -> Ziel = 2 = Programmende.
    let p = program(vec![ConstI64(1), Jump(0)]);
    let translation = translate(&p).unwrap();
    let Op::Jump(target) = translation.ops[1] else {
        panic!("erwartet Jump");
    };
    assert_eq!(
        target,
        translation.ops.len() - 1,
        "Ziel ist der Abschluss-Halt"
    );
    assert_eq!(translation.ops[target], Op::Halt);
}

#[test]
fn sprung_hinter_das_programm_wird_abgelehnt() {
    use atc_core::bytecode::Instruction::*;
    let p = program(vec![ConstI64(1), Jump(50)]);
    assert_eq!(
        translate(&p),
        Err(BackendError::JumpTargetOutOfRange { pc: 1, target: 52 })
    );
}

#[test]
fn rueckspruch_vor_den_programmanfang_wird_abgelehnt() {
    use atc_core::bytecode::Instruction::*;
    let p = program(vec![Jump(-5)]);
    assert_eq!(
        translate(&p),
        Err(BackendError::JumpTargetOutOfRange { pc: 0, target: -4 })
    );
}

#[test]
fn aufruf_wird_abgelehnt() {
    // Die VM hat keine Aufrufkonvention; ein call darf nicht genaehert werden.
    let compiled = compile("fn f() -> i64 { return 1; } fn main() -> i64 { return f(); }").unwrap();
    match translate(&compiled) {
        Err(BackendError::UnsupportedCall { argc, .. }) => assert_eq!(argc, 0),
        other => panic!("erwartet UnsupportedCall, war {other:?}"),
    }
}

#[test]
fn negative_konstante_wird_abgelehnt() {
    use atc_core::bytecode::Instruction::*;
    let p = program(vec![ConstI64(-1), Return]);
    assert_eq!(
        translate(&p),
        Err(BackendError::NegativeConstant { value: -1, pc: 0 })
    );
}

#[test]
fn negation_und_pop_werden_abgelehnt() {
    use atc_core::bytecode::Instruction::*;
    assert_eq!(
        translate(&program(vec![ConstI64(1), Neg, Return])),
        Err(BackendError::UnsupportedInstruction { name: "Neg", pc: 1 })
    );
    assert_eq!(
        translate(&program(vec![ConstI64(1), Pop, Return])),
        Err(BackendError::UnsupportedInstruction { name: "Pop", pc: 1 })
    );
}

#[test]
fn leeres_programm_wird_abgelehnt() {
    assert_eq!(translate(&program(vec![])), Err(BackendError::EmptyProgram));
}

#[test]
fn programm_ueber_der_vm_grenze_wird_abgelehnt() {
    use atc_core::bytecode::Instruction::*;
    // 20 000 Ne-Instruktionen ergeben 80 000 ATCB-Operationen, Grenze ist 65 536.
    let instructions = vec![Ne; 20_000];
    match translate(&program(instructions)) {
        Err(BackendError::TooLarge { size, limit }) => {
            assert_eq!(size, 80_001);
            assert_eq!(limit, 65_536);
        }
        other => panic!("erwartet TooLarge, war {other:?}"),
    }
}

#[test]
fn jede_ablehnung_hat_eine_meldung() {
    use atc_core::bytecode::Instruction::*;
    let cases = vec![
        translate(&program(vec![ConstI64(-1), Return])).unwrap_err(),
        translate(&program(vec![ConstI64(1), Neg, Return])).unwrap_err(),
        translate(&program(vec![ConstI64(1), Jump(99)])).unwrap_err(),
        translate(&program(vec![])).unwrap_err(),
    ];
    for case in cases {
        assert!(
            !case.message().is_empty(),
            "Ablehnung ohne Meldung: {case:?}"
        );
    }
}
