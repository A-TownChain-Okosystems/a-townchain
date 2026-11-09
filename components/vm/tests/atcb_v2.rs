// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! ATCB-v2 (ATC-BC-002 Kandidat): ASSERT, REVERT, REVERTIF und Verifier-Regeln.

use atc_vm::artifact::Artifact;
use atc_vm::vm::{Op, Vm, VmError};

fn v2(program: Vec<Op>) -> Vm {
    let artefakt = Artifact::new(2, program).expect("v2-Artefakt");
    Vm::from_artifact(&artefakt)
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
    assert_eq!(vm.state(), &[] as &[u64]);
}

// Wahrheitstabelle ATC-BC-002 Abschnitt 15: Bedingung entscheidet, Code ist der Grund.
#[test]
fn revertif_falsch_code_null_laeuft_weiter() {
    let mut vm = v2(vec![
        Op::Push(0),
        Op::Push(0),
        Op::RevertIf,
        Op::Push(9),
        Op::Halt,
    ]);
    assert_eq!(vm.run_with_gas(100).unwrap(), vec![9]);
}

#[test]
fn revertif_falsch_code_gesetzt_laeuft_weiter() {
    let mut vm = v2(vec![
        Op::Push(0),
        Op::Push(300),
        Op::RevertIf,
        Op::Push(9),
        Op::Halt,
    ]);
    assert_eq!(vm.run_with_gas(100).unwrap(), vec![9]);
}

#[test]
fn revertif_wahr_code_null_allgemeiner_ruecklauf() {
    let mut vm = v2(vec![Op::Push(1), Op::Push(0), Op::RevertIf, Op::Halt]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::Reverted(0)));
}

#[test]
fn revertif_wahr_code_gesetzt_ruecklauf_mit_grund() {
    let mut vm = v2(vec![Op::Push(1), Op::Push(300), Op::RevertIf, Op::Halt]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::Reverted(300)));
}

#[test]
fn revertif_wahr_nach_store_kein_zustand() {
    let mut vm = v2(vec![
        Op::Push(5),
        Op::Store(0),
        Op::Push(1),
        Op::Push(0),
        Op::RevertIf,
    ]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::Reverted(0)));
    assert_eq!(vm.state(), &[] as &[u64]);
}

// Fehlercode-Grenzen.
#[test]
fn code_0x00ff_abgelehnt() {
    let mut vm = v2(vec![Op::Revert(0x00FF)]);
    assert_eq!(
        vm.run_with_gas(100),
        Err(VmError::InvalidRevertCode(0x00FF))
    );
}

#[test]
fn code_0x0100_angenommen() {
    let mut vm = v2(vec![Op::Revert(0x0100)]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::Reverted(0x0100)));
}

#[test]
fn code_0x01ff_angenommen() {
    let mut vm = v2(vec![Op::Revert(0x01FF)]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::Reverted(0x01FF)));
}

#[test]
fn code_0x0200_abgelehnt() {
    let mut vm = v2(vec![Op::Revert(0x0200)]);
    assert_eq!(
        vm.run_with_gas(100),
        Err(VmError::InvalidRevertCode(0x0200))
    );
}

// v1-Regressionsnachweis: Verhalten und Gas unveraendert.
#[test]
fn v1_arithmetik_unveraendert() {
    let mut vm = Vm::new(vec![Op::Push(2), Op::Push(3), Op::Add, Op::Halt]);
    assert_eq!(vm.run_with_gas(4).unwrap(), vec![5]);
    let mut knapp = Vm::new(vec![Op::Push(2), Op::Push(3), Op::Add, Op::Halt]);
    assert_eq!(
        knapp.run_with_gas(3),
        Err(VmError::OutOfGas {
            required: 2,
            remaining: 1
        })
    );
}

#[test]
fn v1_ignoriert_v2_strenge_nach_halt() {
    // v1 darf Code nach HALT weiterhin ausfuehren (keine neue Strenge).
    let mut vm = Vm::new(vec![Op::Halt, Op::Push(1)]);
    assert_eq!(vm.run_with_gas(100).unwrap(), vec![]);
}

#[test]
fn v2_befehl_in_v1_abgelehnt() {
    let mut vm = Vm::new(vec![Op::Push(1), Op::Assert, Op::Halt]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::V2OpInV1));
}

#[test]
fn code_nach_halt_in_v2_abgelehnt() {
    let mut vm = v2(vec![Op::Halt, Op::Push(1)]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::UnreachableCode(1)));
}

#[test]
fn stapelunterlauf_abgelehnt() {
    let mut vm = v2(vec![Op::Assert, Op::Halt]);
    assert_eq!(vm.run_with_gas(100), Err(VmError::StackUnderflow));
}

// P0-1 (VM-AUD-003-A): Nachweis, dass jeder oeffentliche Pfad die Grenzen
// erzwingt. Vorher: 5000 Stapelwerte, 70001 Speicherplaetze und Teilzustand
// nach Fehler waren reproduzierbar.
#[test]
fn p0_1_stapelgrenze_gilt_auch_ohne_gasbudget() {
    let mut p = vec![Op::Push(1); 5000];
    p.push(Op::Halt);
    let mut vm = Vm::new(p);
    assert_eq!(vm.run(), Err(VmError::StackLimitExceeded));
    let mut p2 = vec![Op::Push(1); 5000];
    p2.push(Op::Halt);
    let mut vm2 = Vm::new(p2);
    assert_eq!(vm2.run_with_gas(100_000), Err(VmError::StackLimitExceeded));
}

#[test]
fn p0_1_speichergrenze_gilt_auf_beiden_pfaden() {
    let prog = vec![Op::Push(1), Op::Store(70_000), Op::Halt];
    let mut ohne_gas = Vm::new(prog.clone());
    assert_eq!(
        ohne_gas.run(),
        Err(VmError::StorageLimitExceeded {
            requested: 70_001,
            limit: 65_536
        })
    );
    let mut mit_gas = Vm::new(prog);
    assert_eq!(
        mit_gas.run_with_gas(100),
        Err(VmError::StorageLimitExceeded {
            requested: 70_001,
            limit: 65_536
        })
    );
    assert!(ohne_gas.state().is_empty());
}

#[test]
fn p0_1_kein_teilzustand_nach_fehler_ohne_gasbudget() {
    let mut vm = Vm::new(vec![Op::Push(7), Op::Store(0), Op::Add]);
    assert_eq!(vm.run(), Err(VmError::StackUnderflow));
    assert!(
        vm.state().is_empty(),
        "Speicher nach Fehler: {:?}",
        vm.state()
    );
}

#[test]
fn p0_1_ungueltiges_sprungziel_wird_vor_ausfuehrung_abgelehnt() {
    let mut vm = Vm::new(vec![Op::Push(1), Op::Jump(99)]);
    assert_eq!(
        vm.run(),
        Err(VmError::InvalidProgram("jump target out of range"))
    );
    assert!(vm.state().is_empty());
    assert!(vm.run_with_gas(100).is_err());
}

// Kontrollpunkt 1: Jeder oeffentliche Einstieg verifiziert. Wird ein neuer
// Einstieg ergaenzt, der das nicht tut, faellt dieser Test.
#[test]
fn jeder_oeffentliche_einstieg_verifiziert() {
    let ungueltig = vec![Op::Push(1), Op::Jump(99)];
    let mut a = Vm::new(ungueltig.clone());
    assert_eq!(
        a.run(),
        Err(VmError::InvalidProgram("jump target out of range"))
    );
    let mut b = Vm::new(ungueltig.clone());
    assert_eq!(
        b.run_with_gas(100),
        Err(VmError::InvalidProgram("jump target out of range"))
    );
    let mut c = Vm::new(ungueltig);
    let ctx = atc_vm::context::ChainContext {
        chain_id: "atc".into(),
        network_id: "devnet".into(),
        genesis_id: "a".repeat(64),
        protocol_version: "1.0.0".into(),
        vm_version: "1.0.0".into(),
    };
    assert_eq!(
        c.execute_state_transition_with_gas(&ctx, &"a".repeat(64), "1.0.0", "1.0.0", 100),
        Err(VmError::InvalidProgram("jump target out of range"))
    );
}

// Kontrollpunkt 2: Die Gas-Option beeinflusst ausschliesslich die Abrechnung.
// Verifikation, Stapelgrenze, Speichergrenze, Snapshot und Generation gelten
// ohne Budget genauso.
#[test]
fn gasoption_deaktiviert_keine_sicherheitsgrenze() {
    let faelle: Vec<(Vec<Op>, VmError)> = vec![
        (
            vec![Op::Push(1), Op::Jump(99)],
            VmError::InvalidProgram("jump target out of range"),
        ),
        (
            vec![Op::Push(1), Op::Store(70_000), Op::Halt],
            VmError::StorageLimitExceeded {
                requested: 70_001,
                limit: 65_536,
            },
        ),
        (
            {
                let mut p = vec![Op::Push(1); 5000];
                p.push(Op::Halt);
                p
            },
            VmError::StackLimitExceeded,
        ),
        (vec![Op::Revert(0x00FF)], VmError::InvalidRevertCode(0x00FF)),
    ];
    for (prog, erwartet) in faelle {
        let mut ohne_gas = Vm::from_artifact(&Artifact::new(2, prog.clone()).expect("v2"));
        assert_eq!(
            ohne_gas.run(),
            Err(erwartet.clone()),
            "ohne Gas: {:?}",
            prog
        );
        assert!(ohne_gas.state().is_empty());
        let mut mit_gas = Vm::from_artifact(&Artifact::new(2, prog).expect("v2"));
        assert_eq!(mit_gas.run_with_gas(100_000), Err(erwartet), "mit Gas");
        assert!(mit_gas.state().is_empty());
    }
}

// Snapshot gilt auch ohne Gasbudget: kein Teilzustand nach Abbruch.
#[test]
fn snapshot_gilt_ohne_gasbudget() {
    let mut vm =
        Vm::from_artifact(&Artifact::new(2, vec![Op::Push(9), Op::Store(1), Op::Add]).expect("v2"));
    assert_eq!(vm.run(), Err(VmError::StackUnderflow));
    assert!(vm.state().is_empty());
}
