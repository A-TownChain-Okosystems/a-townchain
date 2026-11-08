// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATCLang Compiler-CLI (Frontend-Gate, SCR-0083/0085).
//! `compile`: Lex -> Parse -> kanonische AST-JSON auf stdout (differentialfaehig).
//! `check`:   stille Validierung, Ergebnis nur via Exit-Code.
//! Noch NICHT Bestandteil: AST->Bytecode-Lowering (der Bytecode-Verifizierer ist
//! als Bibliothek ueber atc_core::bytecode erreichbar; Lowering folgt via SCR).

use atc_core::artifact::{Artifact, ArtifactMetadata};
use atc_core::lower::{lower_program, CompiledProgram};
use atc_core::parser::parse_program;
use std::env;
use std::fs;
use std::process::ExitCode;

const USAGE: &str = "ATCLang Compiler-CLI

USAGE:
    atc compile <datei.atc>   kompiliert: kanonische AST-JSON auf stdout
    atc run     <datei.atc>   kompiliert, verifiziert und fuehrt aus
    atc check   <datei.atc>   validiert eine Datei (Exit-Code 0 = ok)
    atc check   <verzeichnis> validiert alle .atc eines Verzeichnisses
                              projektweit (Basen, Typnamen, Duplikate)
    atc check --strict <ziel> behandelt Warnungen als Fehler
    atc abi     <ziel>        kanonische Vertrags-ABI (JSONL, eine Zeile je
                              Vertrag, inkl. abi_sha256)
    atc abi --hash <ziel>     nur die ABI-Hashes
    atc abi --digest <ziel>   ein Sammel-Digest je Quelldatei
    atc storage <ziel>        Zustands-Slots je Quelle (Layout-Hash, S1-Faehigkeit)

EXIT-CODES:
    0  ok
    1  Kompilier-/Parse-/Laufzeit-Fehler
    2  Aufruffehler";

fn compile_to_bytecode(src: &str) -> Result<CompiledProgram, String> {
    let program = parse_program(src).map_err(|e| e.message)?;
    lower_program(&program).map_err(|e| e.message)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    // Aufruf: atc <modus> <datei> | atc check --strict <datei>
    let strict = args.iter().skip(1).any(|a| a == "--strict");
    let hashes_only = args.iter().skip(1).any(|a| a == "--hash");
    let digest_only = args.iter().skip(1).any(|a| a == "--digest");
    // Alle Blocker einer Datei sammeln statt beim ersten Fehler abzubrechen.
    let all_blockers = args.iter().skip(1).any(|a| a == "--all");
    let positional: Vec<&String> = args
        .iter()
        .filter(|a| *a != "--strict" && *a != "--hash" && *a != "--digest" && *a != "--all")
        .collect();
    if positional.len() != 2 {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    let mode = positional[0].as_str();
    let path = positional[1].as_str();
    // Verzeichnisse werden projektweit geprueft, bevor eine Datei gelesen wird.
    if mode == "check" && std::path::Path::new(path).is_dir() {
        return check_project_dir(path, strict, all_blockers);
    }
    if mode == "abi" {
        return emit_abi(path, hashes_only, digest_only);
    }
    if mode == "storage" {
        return emit_storage(path);
    }
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Fehler: Datei nicht lesbar ({path}): {e}");
            return ExitCode::from(2);
        }
    };
    if mode == "check" && all_blockers {
        let (_, blockers) = atc_core::parser::parse_program_collecting(&src);
        for blocker in &blockers {
            eprintln!("Kompilierfehler in {path}: {}", blocker.message);
        }
        return if blockers.is_empty() {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        };
    }
    let program = match parse_program(&src) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Kompilierfehler in {path}: {}", e.message);
            return ExitCode::from(1);
        }
    };
    match mode {
        "run" => {
            let compiled = match compile_to_bytecode(&src) {
                Ok(c) => c,
                Err(msg) => {
                    eprintln!("Kompilierfehler in {path}: {msg}");
                    return ExitCode::from(1);
                }
            };
            let metadata = ArtifactMetadata {
                language_version: "1.0".into(),
                compiler_version: env!("CARGO_PKG_VERSION").to_string(),
                target_profile: "l1-deterministic-v1".into(),
                capabilities: Vec::new(),
            };
            let artifact = match Artifact::from_program(&compiled, metadata) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Artifact-Fehler in {path}: {e:?}");
                    return ExitCode::from(1);
                }
            };
            let verified = match artifact.verify() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Artifact-Verifikation fehlgeschlagen in {path}: {e:?}");
                    return ExitCode::from(1);
                }
            };
            match atc_core::vm::execute_verified(&verified) {
                Ok(value) => {
                    println!("{value}");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Laufzeitfehler in {path}: {e:?}");
                    ExitCode::from(1)
                }
            }
        }
        "compile" => {
            println!("{}", program.to_json());
            let mut fns = 0;
            let mut stmts = 0;
            for s in &program.statements {
                stmts += 1;
                if let atc_core::ast::Stmt::Fn(f) = s {
                    fns += 1;
                    stmts += f.body.len();
                }
            }
            eprintln!("OK: {path} ({fns} Funktionen, {stmts} Anweisungen)");
            ExitCode::SUCCESS
        }
        "check" => {
            // Semantische Stufe: erst nach erfolgreichem Parse.
            let report = file_report(&program);
            for w in &report.warnings {
                eprintln!("  warnung: {w}");
            }
            if !report.errors.is_empty() {
                for e in &report.errors {
                    eprintln!("  - {e}");
                }
                eprintln!(
                    "FEHLER: {path} verletzt {} semantische Regel(n)",
                    report.errors.len()
                );
                return ExitCode::FAILURE;
            }
            if strict && !report.warnings.is_empty() {
                eprintln!(
                    "FEHLER (--strict): {path} hat {} offene Warnung(en)",
                    report.warnings.len()
                );
                return ExitCode::FAILURE;
            }
            eprintln!("OK: {path} validiert");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// Alle .atc-Dateien unterhalb eines Verzeichnisses, sortiert.
fn scan_atc(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            scan_atc(&p, out);
        } else if p.extension().and_then(|e| e.to_str()) == Some("atc") {
            out.push(p);
        }
    }
}

/// Stufe 1 + 2 fuer eine einzelne Datei.
fn file_report(program: &atc_core::ast::Program) -> atc_core::semantics::Report {
    let mut report = atc_core::semantics::check_program(program);
    let types = atc_core::types::check_program(program);
    report.errors.extend(types.errors);
    report.warnings.extend(types.warnings);
    report
        .warnings
        .extend(atc_core::types::check_call_arities(program));
    report
}

/// Projektweite Pruefung ueber alle .atc eines Verzeichnisses.
fn check_project_dir(dir: &str, strict: bool, all_blockers: bool) -> ExitCode {
    let mut paths = Vec::new();
    scan_atc(std::path::Path::new(dir), &mut paths);
    paths.sort();
    if paths.is_empty() {
        eprintln!("Fehler: keine .atc-Quellen unter {dir}");
        return ExitCode::from(2);
    }

    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut parsed: Vec<(String, atc_core::ast::Program)> = Vec::new();

    for p in &paths {
        let label = p.to_string_lossy().to_string();
        match fs::read_to_string(p) {
            Ok(src) if all_blockers => {
                let (program, blockers) = atc_core::parser::parse_program_collecting(&src);
                errors.extend(
                    blockers
                        .iter()
                        .map(|e| format!("{label}: Parse-Fehler: {}", e.message)),
                );
                // Ein Teilstand traegt keine Aussage; nur die Blocker zaehlen.
                if let Some(program) = program {
                    if blockers.is_empty() {
                        let report = file_report(&program);
                        errors.extend(report.errors.iter().map(|e| format!("{label}: {e}")));
                        warnings.extend(report.warnings.iter().map(|w| format!("{label}: {w}")));
                        parsed.push((label.clone(), program));
                    }
                }
            }
            Ok(src) => match parse_program(&src) {
                Ok(program) => {
                    let report = file_report(&program);
                    errors.extend(report.errors.iter().map(|e| format!("{label}: {e}")));
                    warnings.extend(report.warnings.iter().map(|w| format!("{label}: {w}")));
                    parsed.push((label, program));
                }
                Err(e) => errors.push(format!("{label}: Parse-Fehler: {}", e.message)),
            },
            Err(e) => errors.push(format!("{label}: nicht lesbar ({e})")),
        }
    }

    let project = atc_core::project::build(&parsed);
    let project_report = project.check();
    errors.extend(project_report.errors);
    warnings.extend(project_report.warnings);

    for w in &warnings {
        eprintln!("  warnung: {w}");
    }
    for e in &errors {
        eprintln!("  - {e}");
    }
    eprintln!(
        "Projekt {dir}: {} Dateien, {} Vertraege, {} Fehler, {} Warnungen",
        parsed.len(),
        project.contracts.len(),
        errors.len(),
        warnings.len()
    );
    if !errors.is_empty() {
        return ExitCode::FAILURE;
    }
    if strict && !warnings.is_empty() {
        eprintln!("FEHLER (--strict): {} offene Warnung(en)", warnings.len());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// Kanonische Vertrags-ABI ausgeben: JSONL, eine Zeile je Vertrag.
/// `atc storage <ziel>`: Zustandsslots je Quelle. Eine Zeile je Datei:
/// `layout_hash  pfad  slots  storage_bytes  s1=yes|no`.
fn emit_storage(target: &str) -> ExitCode {
    let mut paths = Vec::new();
    if std::path::Path::new(target).is_dir() {
        scan_atc(std::path::Path::new(target), &mut paths);
        paths.sort();
    } else {
        paths.push(std::path::PathBuf::from(target));
    }
    if paths.is_empty() {
        eprintln!("Fehler: keine .atc-Quellen unter {target}");
        return ExitCode::from(2);
    }
    let mut exit = ExitCode::SUCCESS;
    for path in &paths {
        let label = path.display().to_string();
        let src = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Fehler: Datei nicht lesbar ({label}): {e}");
                exit = ExitCode::from(2);
                continue;
            }
        };
        let program = match parse_program(&src) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Kompilierfehler in {label}: {}", e.message);
                exit = ExitCode::from(1);
                continue;
            }
        };
        match atc_core::storage::StorageLayout::from_program(&program) {
            Ok(layout) => {
                if layout.slot_count() == 0 {
                    continue; // Quelle ohne Zustand
                }
                println!(
                    "{}  {}  {}  {}  s1={}",
                    layout.layout_hash(),
                    json_quote(&label),
                    layout.slot_count(),
                    layout.storage_bytes(),
                    if layout.is_s1_executable() { "yes" } else { "no" }
                );
            }
            Err(e) => {
                eprintln!("Storage-Layout in {label} nicht ableitbar: {e}");
                exit = ExitCode::from(1);
            }
        }
    }
    exit
}

fn emit_abi(target: &str, hashes_only: bool, digest_only: bool) -> ExitCode {
    let mut paths = Vec::new();
    if std::path::Path::new(target).is_dir() {
        scan_atc(std::path::Path::new(target), &mut paths);
        paths.sort();
    } else {
        paths.push(std::path::PathBuf::from(target));
    }
    if paths.is_empty() {
        eprintln!("Fehler: keine .atc-Quellen unter {target}");
        return ExitCode::from(2);
    }

    let mut emitted = 0usize;
    for p in &paths {
        let label = p.to_string_lossy().to_string();
        let src = match fs::read_to_string(p) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Fehler: nicht lesbar ({label}): {e}");
                return ExitCode::from(2);
            }
        };
        let program = match parse_program(&src) {
            Ok(prog) => prog,
            Err(e) => {
                eprintln!("Kompilierfehler in {label}: {}", e.message);
                return ExitCode::FAILURE;
            }
        };
        let abis = atc_core::abi::from_program(&program);
        if digest_only {
            if abis.is_empty() {
                eprintln!("Hinweis: keine Vertraege in {label}");
                continue;
            }
            println!(
                "{}  {}  {}",
                atc_core::abi::set_digest(&abis),
                json_quote(&label),
                abis.len()
            );
            emitted += abis.len();
            continue;
        }
        for abi in abis {
            emitted += 1;
            let hash = abi.sha256();
            if hashes_only {
                println!("{hash}  {label}  {}", abi.contract);
            } else {
                println!(
                    "{{\"path\":{},\"abi_sha256\":\"{}\",\"abi\":{}}}",
                    json_quote(&label),
                    hash,
                    abi.to_json()
                );
            }
        }
    }
    if emitted == 0 {
        // Quellen ohne Vertrag sind legitim (Bibliothek, archivierter Stub).
        eprintln!("Hinweis: keine Vertraege in {target} gefunden");
    }
    ExitCode::SUCCESS
}

/// Minimales JSON-String-Escaping fuer Pfadangaben der CLI-Ausgabe.
fn json_quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
