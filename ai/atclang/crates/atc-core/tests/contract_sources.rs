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
