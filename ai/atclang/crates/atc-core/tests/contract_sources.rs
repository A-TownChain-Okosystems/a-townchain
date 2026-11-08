// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Regression: alle ATCLang-Vertragsquellen des Monorepos muessen
//! syntaktisch parsen und die semantischen Regeln (Stufe 1) erfuellen.

use std::path::{Path, PathBuf};

fn collect_atc(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_atc(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("atc") {
            out.push(path);
        }
    }
}

#[test]
fn alle_vertragsquellen_parsen_und_validieren() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../apps/atc-contracts");
    let mut files = Vec::new();
    collect_atc(&root, &mut files);
    files.sort();

    assert!(
        files.len() >= 16,
        "zu wenige Vertragsquellen gefunden unter {}: {}",
        root.display(),
        files.len()
    );

    let mut failures = Vec::new();
    for file in &files {
        let src = std::fs::read_to_string(file).expect("Quelle lesbar");
        match atc_core::parser::parse_program(&src) {
            Err(e) => failures.push(format!("{}: Parse: {}", file.display(), e.message)),
            Ok(program) => {
                let report = atc_core::semantics::check_program(&program);
                for e in report.errors {
                    failures.push(format!("{}: {e}", file.display()));
                }
                let typed = atc_core::types::check_program(&program);
                for e in typed.errors {
                    failures.push(format!("{}: {e}", file.display()));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} Quelle(n) mit Befund:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// Projektweite Pruefung (Stufe 3): ueber alle Quellen hinweg muessen Basen und
/// Typnamen aufloesbar sein. Warnungen (etwa die offenen emit-Deklarationen oder
/// Mirror-Duplikate) sind erlaubt, Fehler nicht.
#[test]
fn projektweite_aufloesung_ohne_fehler() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../apps/atc-contracts");
    let mut files = Vec::new();
    collect_atc(&root, &mut files);
    files.sort();

    let mut parsed: Vec<(String, atc_core::ast::Program)> = Vec::new();
    for file in &files {
        let src = std::fs::read_to_string(file).expect("Quelle lesbar");
        let program = atc_core::parser::parse_program(&src)
            .unwrap_or_else(|e| panic!("{}: {}", file.display(), e.message));
        parsed.push((file.display().to_string(), program));
    }

    let project = atc_core::project::build(&parsed);
    let report = project.check();

    assert!(
        report.errors.is_empty(),
        "Projektfehler: {:?}",
        report.errors
    );
    assert!(
        !project.contracts.is_empty(),
        "Projektindex ohne Vertraege"
    );
    // Jeder Vertrag muss projektweit mindestens einmal indiziert sein.
    for (name, entries) in &project.contracts {
        assert!(!name.is_empty() && !entries.is_empty(), "leerer Indexeintrag");
    }
}
