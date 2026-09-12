// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Parser fuer das .ops-Textformat des EXEC-CHAIN-Assemblers
//! (atc-contracts/exec_chain, ATC-CONTRACT-EXEC-001 / F-084).
//!
//! Format: eine Op pro Zeile; `#`-Zeilen sind Header-Kommentare
//! (contract, fn, source_sha256, expected). Fail-closed: unbekannte
//! Zeilen sind Fehler, kein stillschweigendes Ueberspringen.

use crate::vm::Op;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpsError {
    UnknownOp { line: usize, text: String },
    BadPush { line: usize, value: String },
    Empty,
}

/// Parst den Ops-Text in ein ausfuehrbares Programm.
/// Deterministisch: reine Funktion des Inputs, keine Umgebung.
pub fn parse_ops(text: &str) -> Result<Vec<Op>, OpsError> {
    let mut program: Vec<Op> = Vec::new();
    let mut saw_op = false;
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let op = parse_line(idx + 1, line)?;
        saw_op = true;
        program.push(op);
    }
    if !saw_op {
        return Err(OpsError::Empty);
    }
    Ok(program)
}

fn parse_line(line_no: usize, line: &str) -> Result<Op, OpsError> {
    if let Some(rest) = line.strip_prefix("Push ") {
        let value = rest.trim();
        let v: u64 = value.parse().map_err(|_| OpsError::BadPush {
            line: line_no,
            value: value.to_string(),
        })?;
        return Ok(Op::Push(v));
    }
    match line {
        "Add" => Ok(Op::Add),
        "Sub" => Ok(Op::Sub),
        "Mul" => Ok(Op::Mul),
        "Div" => Ok(Op::Div),
        "Dup" => Ok(Op::Dup),
        "Swap" => Ok(Op::Swap),
        "Eq" => Ok(Op::Eq),
        "Lt" => Ok(Op::Lt),
        "Halt" => Ok(Op::Halt),
        _ => Err(OpsError::UnknownOp {
            line: line_no,
            text: line.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_e2e_adder_program() {
        let text = "# contract: e2e_adder\n# fn: compute\n# expected: 20\nPush 7\nPush 3\nAdd\nPush 2\nMul\nHalt\n";
        let prog = parse_ops(text).expect("gueltiges Programm");
        assert_eq!(
            prog,
            vec![
                Op::Push(7),
                Op::Push(3),
                Op::Add,
                Op::Push(2),
                Op::Mul,
                Op::Halt,
            ]
        );
    }

    #[test]
    fn rejects_unknown_op_fail_closed() {
        let res = parse_ops("Push 1\nFrobnicate\n");
        assert_eq!(
            res,
            Err(OpsError::UnknownOp {
                line: 2,
                text: "Frobnicate".to_string()
            })
        );
    }

    #[test]
    fn rejects_bad_push_value() {
        let res = parse_ops("Push abc\n");
        assert!(matches!(res, Err(OpsError::BadPush { line: 1, .. })));
    }

    #[test]
    fn rejects_empty_program() {
        let res = parse_ops("# nur kommentare\n\n");
        assert_eq!(res, Err(OpsError::Empty));
    }

    #[test]
    fn jump_ops_parse_as_unknown_in_ops_format() {
        // Jump/JumpIfNotZero sind Bytecode-intern; das .ops-Textformat des
        // EXEC-GATE-Subsets kennt sie nicht — fail-closed, kein Guessing.
        let res = parse_ops("Jump 2\n");
        assert!(matches!(res, Err(OpsError::UnknownOp { .. })));
    }
}
