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
    atc check   <datei.atc>   validiert (Exit-Code 0 = ok)
    atc check --strict <datei.atc>  behandelt Warnungen als Fehler

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
    let positional: Vec<&String> = args.iter().filter(|a| *a != "--strict").collect();
    if positional.len() != 2 {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    let mode = positional[0].as_str();
    let path = positional[1].as_str();
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Fehler: Datei nicht lesbar ({path}): {e}");
            return ExitCode::from(2);
        }
    };
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
            let mut report = atc_core::semantics::check_program(&program);
            // Stufe 2: Typ- und Signaturpruefung (konservativ, siehe types.rs).
            let types = atc_core::types::check_program(&program);
            report.errors.extend(types.errors);
            report.warnings.extend(types.warnings);
            report
                .warnings
                .extend(atc_core::types::check_call_arities(&program));
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
