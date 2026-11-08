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

/// ABI-Regression: fuer jede registrierte Contract-Quelle muss die kanonische
/// ABI deterministisch sein; nur archivierte Eintraege duerfen ohne Vertrag sein.
#[test]
fn registrierte_vertraege_haben_deterministische_abi() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let registry = root.join("standards/contracts/registry/contracts.yaml");
    let yaml = std::fs::read_to_string(&registry)
        .unwrap_or_else(|e| panic!("Registry nicht lesbar ({}): {e}", registry.display()));

    let mut entries: Vec<(String, String, String)> = Vec::new();
    for block in yaml.split("- contract_id:").skip(1) {
        let mut id = String::new();
        let mut status = String::new();
        let mut path = String::new();
        for line in block.lines() {
            let line = line.trim();
            if id.is_empty() {
                id = line.to_string();
            } else if let Some(rest) = line.strip_prefix("status:") {
                status = rest.trim().to_string();
            } else if let Some(rest) = line.strip_prefix("contract_path:") {
                path = rest.trim().to_string();
            }
        }
        assert!(!id.is_empty() && !path.is_empty(), "unvollstaendiger Eintrag: {id}");
        entries.push((id, status, path));
    }
    assert!(entries.len() >= 16, "zu wenige Registry-Eintraege: {}", entries.len());

    for (id, status, rel) in entries {
        let file = root.join(&rel);
        let src = std::fs::read_to_string(&file)
            .unwrap_or_else(|e| panic!("{id}: Quelle fehlt ({}): {e}", file.display()));
        let program = atc_core::parser::parse_program(&src)
            .unwrap_or_else(|e| panic!("{id}: Parse-Fehler: {}", e.message));

        let first = atc_core::abi::from_program(&program);
        let second = atc_core::abi::from_program(&program);

        if status == "archived" {
            assert!(
                first.is_empty(),
                "{id}: archivierter Eintrag enthaelt noch Vertraege"
            );
            continue;
        }

        assert!(!first.is_empty(), "{id}: keine Vertragsdeklaration in {rel}");
        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.to_json(), b.to_json(), "{id}: ABI nicht deterministisch");
            let hash = a.sha256();
            assert_eq!(hash.len(), 64, "{id}: ABI-Hash nicht 256 Bit");
            assert_eq!(hash, b.sha256(), "{id}: ABI-Hash nicht stabil");
            assert!(
                !a.functions.is_empty(),
                "{id}: Vertrag {} ohne Funktionen in der ABI",
                a.contract
            );
        }
    }
}
