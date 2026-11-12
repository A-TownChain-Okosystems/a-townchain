// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! E1 (Audit 2026-11-12, F1): Vertragsruempfe werden nicht zu Bytecode gesenkt.
//!
//! Vor dem Fix uebersprang `lower_program` `Stmt::Contract` und erzeugte ein
//! Ersatzprogramm `ConstI64(0)/Return`. Ergebnis war ein Erfolg, der nichts
//! bedeutet: `atc run token.atc` gab 0 aus, `System::deploy` nahm den Vertrag an,
//! die Kette fuehrte `Push(0); Halt; Halt` aus. Diese Datei haelt fest, dass
//! daraus ein expliziter Fehler geworden ist: kein stilles Verwerfen, kein
//! Ersatzprogramm, keine Ausfuehrung eines Vertrags ohne Logik.

use atc_core::ast::{Program, Stmt};
use atc_core::lower::lower_program;
use atc_core::parser::parse_program;
use std::path::{Path, PathBuf};

fn vertraege<'a>(stmts: &'a [Stmt], out: &mut Vec<&'a Stmt>) {
    for s in stmts {
        if matches!(s, Stmt::Contract { .. }) {
            out.push(s);
        }
    }
}

fn vertragsnamen(prog: &Program) -> Vec<String> {
    let mut found = Vec::new();
    vertraege(&prog.statements, &mut found);
    found
        .iter()
        .map(|s| match s {
            Stmt::Contract { name, .. } => name.clone(),
            _ => unreachable!(),
        })
        .collect()
}

// Erfuellt Abnahmekriterium 1 und 2: nicht mehr stillschweigend uebersprungen,
// expliziter Compilerfehler.
#[test]
fn vertrag_mit_logik_wird_abgelehnt_nicht_uebersprungen() {
    let src = "contract C { fn main() -> i64 { return 42; } }";
    let prog = parse_program(src).expect("Vertrag muss weiterhin parsbar sein");
    assert_eq!(vertragsnamen(&prog), vec!["C".to_string()]);

    let err = lower_program(&prog).expect_err("Vertrag darf nicht zu Bytecode werden");
    assert!(
        err.message.contains("'C'") && err.message.contains("nicht ausfuehrbar"),
        "Fehler nennt den Vertrag und den Grund: {}",
        err.message
    );
}

// Erfuellt Abnahmekriterium 5: kein Ersatzprogramm. Ein Programm entsteht gar
// nicht erst, deshalb kann auch kein `Push(0); Halt` uebrig bleiben.
#[test]
fn kein_ersatzprogramm_statt_vertragslogik() {
    for src in [
        "contract C { fn f() -> bool { return true; } }",
        "let a: u64 = 6;\ncontract C { fn f() -> bool { return true; } }",
    ] {
        let prog = parse_program(src).expect("Quelle muss parsen");
        let err = lower_program(&prog).expect_err("kein Kompilat fuer einen Vertrag");
        assert!(
            !err.message.contains("Konstante") && !err.message.is_empty(),
            "Ablehnung ist ein eigener Grund, kein Platzhalter: {}",
            err.message
        );
    }
}

// Ein Vertrag ohne Rumpf ist ebenfalls nicht ausfuehrbar. Der Audit verlangt
// ausdrueckliche Spezifikation, bevor ein leerer Vertrag zulaessig waere.
#[test]
fn leerer_vertrag_wird_ebenfalls_abgelehnt() {
    let prog = parse_program("contract Leer { }").expect("leerer Vertrag parst");
    let err = lower_program(&prog).expect_err("auch ein leerer Vertrag ist kein Programm");
    assert!(err.message.contains("'Leer'"), "{}", err.message);
}

// Erfuellt Abnahmekriterium 7: die 11 verlorenen Funktionen und die uebrigen
// Bestandteile des echten Token-Vertrags. Der Test zaehlt sie in der Quelle und
// verlangt, dass genau diese Zahlen in der Ablehnung stehen. Aendert jemand den
// Vertrag, faellt es hier auf, statt still im Nichts zu landen.
#[test]
fn token_vertrag_verliert_keine_bestandteile_still() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../apps/atc-contracts/contracts/token.atc");
    let src = std::fs::read_to_string(&path).expect("token.atc lesbar");
    let prog = parse_program(&src).expect("token.atc parst");
    assert_eq!(
        vertragsnamen(&prog),
        vec!["FranchiseToken".to_string()],
        "genau ein Vertrag"
    );

    let mut found = Vec::new();
    vertraege(&prog.statements, &mut found);
    let Stmt::Contract { body, .. } = found[0] else {
        unreachable!()
    };
    let fns = body.iter().filter(|s| matches!(s, Stmt::Fn(_))).count();
    let states = body
        .iter()
        .filter(|s| matches!(s, Stmt::State { .. } | Stmt::StateBlock { .. }))
        .count();
    let events = body
        .iter()
        .filter(|s| matches!(s, Stmt::Event { .. }))
        .count();
    assert_eq!(fns, 11, "11 Funktionen laut Audit");
    assert_eq!(states, 8, "8 Zustandsfelder laut Audit");
    assert_eq!(events, 5, "5 Events laut Audit");

    let err = lower_program(&prog).expect_err("FranchiseToken ist nicht ausfuehrbar");
    assert!(err.message.contains("'FranchiseToken'"), "{}", err.message);
    assert!(err.message.contains("11 Funktionen"), "{}", err.message);
    assert!(err.message.contains("8 Zustandsfelder"), "{}", err.message);
    assert!(err.message.contains("5 Events"), "{}", err.message);
}

// Erfuellt Abnahmekriterium 6: der fruehere Erfolgspfad bleibt als Programm
// erhalten, aber nur ohne Vertrag. `fn main` liefert weiterhin 42.
#[test]
fn programm_ohne_vertrag_bleibt_unveraendert() {
    let prog = parse_program("fn main() -> i64 { return 42; }").expect("parst");
    let compiled = lower_program(&prog).expect("Programm ohne Vertrag bleibt kompilierbar");
    // Identischer Weg wie `atc run`: Kompilat -> Artefakt -> Verifikation -> VM.
    let metadata = atc_core::artifact::ArtifactMetadata {
        language_version: "1.0".into(),
        compiler_version: env!("CARGO_PKG_VERSION").to_string(),
        target_profile: "l1-deterministic-v1".into(),
        capabilities: Vec::new(),
    };
    let artifact = atc_core::artifact::Artifact::from_program(&compiled, metadata)
        .expect("Artefakt aus Kompilat");
    let verified = artifact
        .verify()
        .expect("Artefakt muss die Verifikation bestehen");
    let wert = atc_core::vm::execute_verified(&verified).expect("laeuft");
    assert_eq!(wert, 42);
}

// Der eigentliche Regressionsschutz: keine einzige Vertragsquelle des Monorepos
// darf ein Kompilat ergeben. Damit ist auch ein kuenftig hinzugefuegter Vertrag
// abgedeckt, ohne dass jemand daran denken muss.
#[test]
fn keine_vertragsquelle_des_monorepos_ergibt_bytecode() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../apps/atc-contracts");
    let mut files: Vec<PathBuf> = Vec::new();
    sammle(&root, &mut files);
    files.sort();
    assert!(files.len() >= 16, "zu wenige Quellen: {}", files.len());

    let mut geprueft = 0usize;
    for file in &files {
        let src = std::fs::read_to_string(file).expect("Quelle lesbar");
        let Ok(prog) = parse_program(&src) else {
            continue;
        };
        if vertragsnamen(&prog).is_empty() {
            continue;
        }
        geprueft += 1;
        assert!(
            lower_program(&prog).is_err(),
            "{} enthaelt einen Vertrag, ergab aber ein Kompilat",
            file.display()
        );
    }
    assert!(geprueft >= 16, "zu wenige Vertraege geprueft: {geprueft}");
}

fn sammle(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            sammle(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("atc") {
            out.push(path);
        }
    }
}
