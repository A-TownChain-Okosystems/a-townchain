// Paket C1 — normative Gastabelle GAS_TABLE_V1.
// Nur Tests, keine Aenderung an Kosten oder Ausfuehrung.
use atc_vm::artifact::Artifact;
use atc_vm::context::ChainContext;
use atc_vm::vm::{Op, Vm, VmError, GAS_TABLE_ID, GAS_TABLE_V1};

fn kosten(mut vm: Vm) -> u64 {
    match vm.run_with_gas(0) {
        Err(VmError::OutOfGas { required, .. }) => required,
        Ok(_) => 0,
        Err(e) => panic!("unerwartet: {e:?}"),
    }
}

fn kosten_v1(op: Op) -> u64 {
    kosten(Vm::new(vec![op]))
}

fn kosten_v2(op: Op) -> u64 {
    let art = Artifact::new(2, vec![op]).expect("v2-Artefakt");
    kosten(Vm::from_artifact(&art))
}

fn tabelle(name: &str) -> u64 {
    GAS_TABLE_V1
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, c)| *c)
        .unwrap_or_else(|| panic!("{name} fehlt in GAS_TABLE_V1"))
}

fn context() -> ChainContext {
    ChainContext {
        chain_id: "atc".into(),
        network_id: "devnet".into(),
        genesis_id: "a".repeat(64),
        protocol_version: "1.0.0".into(),
        vm_version: "1.0.0".into(),
    }
}

// --- 19 einzelne Kostentests -------------------------------------------------

#[test]
fn push_kostet_1() {
    assert_eq!(kosten_v1(Op::Push(0)), 1);
    assert_eq!(tabelle("Push"), 1);
}

#[test]
fn add_kostet_2() {
    assert_eq!(kosten_v1(Op::Add), 2);
    assert_eq!(tabelle("Add"), 2);
}

#[test]
fn sub_kostet_2() {
    assert_eq!(kosten_v1(Op::Sub), 2);
    assert_eq!(tabelle("Sub"), 2);
}

#[test]
fn mul_kostet_2() {
    assert_eq!(kosten_v1(Op::Mul), 2);
    assert_eq!(tabelle("Mul"), 2);
}

#[test]
fn div_kostet_2() {
    assert_eq!(kosten_v1(Op::Div), 2);
    assert_eq!(tabelle("Div"), 2);
}

#[test]
fn dup_kostet_1() {
    assert_eq!(kosten_v1(Op::Dup), 1);
    assert_eq!(tabelle("Dup"), 1);
}

#[test]
fn swap_kostet_1() {
    assert_eq!(kosten_v1(Op::Swap), 1);
    assert_eq!(tabelle("Swap"), 1);
}

#[test]
fn eq_kostet_2() {
    assert_eq!(kosten_v1(Op::Eq), 2);
    assert_eq!(tabelle("Eq"), 2);
}

#[test]
fn lt_kostet_2() {
    assert_eq!(kosten_v1(Op::Lt), 2);
    assert_eq!(tabelle("Lt"), 2);
}

#[test]
fn load_kostet_3() {
    assert_eq!(kosten_v1(Op::Load(0)), 3);
    assert_eq!(tabelle("Load"), 3);
}

#[test]
fn store_kostet_3() {
    assert_eq!(kosten_v1(Op::Store(0)), 3);
    assert_eq!(tabelle("Store"), 3);
}

#[test]
fn caller_kostet_1() {
    assert_eq!(kosten_v1(Op::Caller), 1);
    assert_eq!(tabelle("Caller"), 1);
}

#[test]
fn jump_kostet_2() {
    assert_eq!(kosten_v1(Op::Jump(0)), 2);
    assert_eq!(tabelle("Jump"), 2);
}

#[test]
fn jumpifzero_kostet_2() {
    assert_eq!(kosten_v1(Op::JumpIfZero(0)), 2);
    assert_eq!(tabelle("JumpIfZero"), 2);
}

#[test]
fn jumpifnotzero_kostet_2() {
    assert_eq!(kosten_v1(Op::JumpIfNotZero(0)), 2);
    assert_eq!(tabelle("JumpIfNotZero"), 2);
}

#[test]
fn halt_kostet_0() {
    assert_eq!(kosten_v2(Op::Halt), 0);
    assert_eq!(tabelle("Halt"), 0);
}

#[test]
fn assert_kostet_2() {
    assert_eq!(kosten_v2(Op::Assert), 2);
    assert_eq!(tabelle("Assert"), 2);
}

#[test]
fn revert_kostet_1() {
    assert_eq!(kosten_v2(Op::Revert(0)), 1);
    assert_eq!(tabelle("Revert"), 1);
}

#[test]
fn revertif_kostet_3() {
    assert_eq!(kosten_v2(Op::RevertIf), 3);
    assert_eq!(tabelle("RevertIf"), 3);
}

// --- Belastung vor der Wirkung ----------------------------------------------

#[test]
fn belastung_erfolgt_vor_der_wirkung_von_store() {
    // Budget 3: Push kostet 1, danach bleiben 2. Store braucht 3 und wirkt nicht.
    let mut vm = Vm::new(vec![Op::Push(1), Op::Store(0)]);
    assert_eq!(
        vm.run_with_gas(3),
        Err(VmError::OutOfGas {
            required: 3,
            remaining: 2
        })
    );
    assert!(
        vm.state().is_empty(),
        "Store darf vor Belastung nicht wirken"
    );
}

#[test]
fn belastung_erfolgt_vor_der_wirkung_von_push() {
    let mut vm = Vm::new(vec![Op::Push(1)]);
    assert_eq!(
        vm.run_with_gas(0),
        Err(VmError::OutOfGas {
            required: 1,
            remaining: 0
        })
    );
}

#[test]
fn belastung_erfolgt_vor_der_wirkung_von_add() {
    // Ohne Stapelwerte wuerde Add unterlaufen; bei zu wenig Gas kommt OutOfGas.
    let mut vm = Vm::new(vec![Op::Add]);
    assert_eq!(
        vm.run_with_gas(1),
        Err(VmError::OutOfGas {
            required: 2,
            remaining: 1
        })
    );
}

#[test]
fn outofgas_rollt_den_zustand_zurueck() {
    // 1 + 3 + 1 = 5 verbraucht, dann fehlt 1 fuer den letzten Push.
    let mut vm = Vm::new(vec![Op::Push(1), Op::Store(0), Op::Push(1), Op::Push(1)]);
    assert_eq!(
        vm.run_with_gas(5),
        Err(VmError::OutOfGas {
            required: 1,
            remaining: 0
        })
    );
    assert!(vm.state().is_empty(), "kein Teilzustand nach E03");
}

#[test]
fn exaktes_budget_laeuft_durch() {
    let prog = vec![Op::Push(1), Op::Store(0), Op::Push(2), Op::Halt]; // 1+3+1+0 = 5
    let mut knapp = Vm::new(prog.clone());
    assert!(matches!(
        knapp.run_with_gas(4),
        Err(VmError::OutOfGas { .. })
    ));
    assert!(knapp.state().is_empty());
    let mut exakt = Vm::new(prog);
    assert_eq!(exakt.run_with_gas(5), Ok(vec![2]));
    assert_eq!(exakt.state(), &[1]);
}

#[test]
fn halt_kostet_nichts() {
    let mut nur_halt = Vm::new(vec![Op::Halt]);
    assert_eq!(nur_halt.run_with_gas(0), Ok(vec![]));
}

#[test]
fn kein_pfad_umgeht_die_tabelle() {
    // [Push(7), Store(0), Halt] = 1 + 3 + 0 = 4
    let prog = vec![Op::Push(7), Op::Store(0), Op::Halt];
    let mut knapp = Vm::new(prog.clone());
    let e =
        knapp.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 3);
    assert!(
        matches!(e, Err(VmError::OutOfGas { .. })),
        "Kettenpfad muss belasten: {e:?}"
    );
    assert!(knapp.state().is_empty());
    let mut exakt = Vm::new(prog);
    assert!(exakt
        .execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 4)
        .is_ok());
    assert_eq!(exakt.state(), &[7]);
}

// --- Tabelle selbst ---------------------------------------------------------

#[test]
fn tabelle_ist_versioniert_vollstaendig_und_unveraenderlich() {
    assert_eq!(GAS_TABLE_ID, "GAS_TABLE_V1");
    assert_eq!(GAS_TABLE_V1.len(), 19, "alle 19 Befehlscodes");
    let mut namen: Vec<&str> = GAS_TABLE_V1.iter().map(|(n, _)| *n).collect();
    namen.sort_unstable();
    namen.dedup();
    assert_eq!(namen.len(), 19, "keine doppelten Eintraege");

    // Jeder Eintrag muss dem entsprechen, was die Ausfuehrung tatsaechlich belastet.
    let ops: Vec<(Op, bool)> = vec![
        (Op::Push(0), true),
        (Op::Add, true),
        (Op::Sub, true),
        (Op::Mul, true),
        (Op::Div, true),
        (Op::Dup, true),
        (Op::Swap, true),
        (Op::Eq, true),
        (Op::Lt, true),
        (Op::Load(0), true),
        (Op::Store(0), true),
        (Op::Caller, true),
        (Op::Jump(0), true),
        (Op::JumpIfZero(0), true),
        (Op::JumpIfNotZero(0), true),
        (Op::Halt, false),
        (Op::Assert, false),
        (Op::Revert(0), false),
        (Op::RevertIf, false),
    ];
    assert_eq!(ops.len(), 19);
    for (op, ist_v1) in ops {
        let name = format!("{op:?}");
        let name = name.split('(').next().unwrap().to_string();
        let gemessen = if ist_v1 { kosten_v1(op) } else { kosten_v2(op) };
        assert_eq!(gemessen, tabelle(&name), "Kosten fuer {name} weichen ab");
    }
}
