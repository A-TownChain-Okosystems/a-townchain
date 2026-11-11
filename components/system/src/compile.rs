// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Stufe 1 des Systems: ATCLang-Quelle -> verifiziertes Kompilat.
//!
//! Der Pfad ist der kanonische: `tokenize` -> `parse_program` -> `lower_program`
//! -> `CompiledProgram::verify`. Es gibt keinen zweiten, schwaecheren Weg:
//! `lower_program` verifiziert bereits jede erzeugte Funktion, und `compile`
//! verifiziert das Gesamtkompilat erneut, bevor es zurueckgegeben wird.

use atc_core::lexer;
use atc_core::lower::{self, CompiledProgram};
use atc_core::parser;

/// Warum eine Quelle nicht kompiliert wurde.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    Lex(String),
    Parse(String),
    Lower(String),
    Verify(String),
}

impl CompileError {
    pub fn message(&self) -> String {
        match self {
            CompileError::Lex(m) => format!("Lexer: {m}"),
            CompileError::Parse(m) => format!("Parser: {m}"),
            CompileError::Lower(m) => format!("Lowering: {m}"),
            CompileError::Verify(m) => format!("Verifizierer: {m}"),
        }
    }
}

/// Kompiliert ATCLang-Quelltext und gibt ausschliesslich verifizierte Kompilate
/// zurueck (verify before trust).
pub fn compile(source: &str) -> Result<CompiledProgram, CompileError> {
    lexer::tokenize(source).map_err(|e| CompileError::Lex(format!("{e:?}")))?;
    let program =
        parser::parse_program(source).map_err(|e| CompileError::Parse(format!("{e:?}")))?;
    let compiled = lower::lower_program(&program).map_err(|e| CompileError::Lower(e.message))?;
    compiled
        .verify()
        .map_err(|e| CompileError::Verify(e.message))?;
    Ok(compiled)
}
