// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Native EXEC-GATE assembler for the ATC-95 migration.
//!
//! This module replaces the former Python `exec_chain/assemble.py` production
//! path. It deliberately implements only the normative EXEC-GATE subset:
//! `let`, `return`, u64 literals/parameters, + - * / and parentheses.
//! The implementation is deterministic and dependency-free.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Parser {
    tokens: Vec<String>,
    pos: usize,
    env: HashMap<String, Vec<String>>,
}

impl Parser {
    fn new(tokens: Vec<String>, env: HashMap<String, Vec<String>>) -> Self {
        Self { tokens, pos: 0, env }
    }

    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.pos).map(String::as_str)
    }

    fn take(&mut self) -> Result<String, String> {
        match self.tokens.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Ok(v.clone())
            }
            None => Err("unexpected end of expression".to_string()),
        }
    }

    fn expr(&mut self) -> Result<Vec<String>, String> {
        let mut ops = self.term()?;
        loop {
            let op = match self.peek() {
                Some("+") => "+",
                Some("-") => "-",
                _ => break,
            };
            self.pos += 1;
            ops.extend(self.term()?);
            ops.push(if op == "+" { "Add" } else { "Sub" }.to_string());
        }
        Ok(ops)
    }

    fn term(&mut self) -> Result<Vec<String>, String> {
        let mut ops = self.factor()?;
        loop {
            let op = match self.peek() {
                Some("*") => "*",
                Some("/") => "/",
                _ => break,
            };
            self.pos += 1;
            ops.extend(self.factor()?);
            ops.push(if op == "*" { "Mul" } else { "Div" }.to_string());
        }
        Ok(ops)
    }

    fn factor(&mut self) -> Result<Vec<String>, String> {
        let tok = self.take()?;
        if tok == "(" {
            let ops = self.expr()?;
            match self.take()?.as_str() {
                ")" => Ok(ops),
                _ => Err("unbalanced parenthesis".to_string()),
            }
        } else if tok.bytes().all(|b| b.is_ascii_digit()) && !tok.is_empty() {
            let value = tok.parse::<u64>().map_err(|_| format!("invalid u64 literal: {tok}"))?;
            Ok(vec![format!("Push {value}")])
        } else if let Some(ops) = self.env.get(&tok) {
            Ok(ops.clone())
        } else {
            Err(format!("unknown identifier in expression: {tok}"))
        }
    }
}

fn tokenize(source: &str) -> Result<Vec<String>, String> {
    let chars: Vec<char> = source.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            out.push(chars[start..i].iter().collect());
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            out.push(chars[start..i].iter().collect());
            continue;
        }
        if matches!(c, '+' | '-' | '*' | '/' | '(' | ')') {
            out.push(c.to_string());
            i += 1;
            continue;
        }
        return Err(format!("invalid character in expression: {c}"));
    }
    Ok(out)
}

fn find_function(source: &str) -> Result<(Vec<String>, String), String> {
    let fn_pos = source.find("fn ").ok_or_else(|| "no function definition found".to_string())?;
    let after_name = &source[fn_pos + 3..];
    let open_paren_rel = after_name.find('(').ok_or_else(|| "function parameter list missing".to_string())?;
    let params_start = fn_pos + 3 + open_paren_rel + 1;
    let params_end = source[params_start..]
        .find(')')
        .map(|p| params_start + p)
        .ok_or_else(|| "function parameter list is unclosed".to_string())?;
    let params_raw = &source[params_start..params_end];
    let params = params_raw
        .split(',')
        .filter_map(|p| p.trim().split(':').next())
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    let body_open = source[params_end..]
        .find('{')
        .map(|p| params_end + p)
        .ok_or_else(|| "function body missing".to_string())?;
    let body_end = source[body_open + 1..]
        .find('}')
        .map(|p| body_open + 1 + p)
        .ok_or_else(|| "function body is unclosed".to_string())?;
    Ok((params, source[body_open + 1..body_end].to_string()))
}

fn json_u64_for_key(json: &str, key: &str) -> Result<u64, String> {
    let needle = format!("\"{key}\"");
    let start = json.find(&needle).ok_or_else(|| format!("vector missing key: {key}"))? + needle.len();
    let tail = &json[start..];
    let colon = tail.find(':').ok_or_else(|| format!("vector key has no value: {key}"))? + 1;
    let value = tail[colon..].trim_start();
    let end = value.find(|c: char| !c.is_ascii_digit()).unwrap_or(value.len());
    if end == 0 {
        return Err(format!("vector value for {key} is not an unsigned integer"));
    }
    value[..end]
        .parse::<u64>()
        .map_err(|_| format!("vector value for {key} is not u64"))
}

fn simulate(ops: &[String], expected: u64) -> Result<(), String> {
    let mut stack: Vec<u64> = Vec::new();
    for op in ops {
        if let Some(raw) = op.strip_prefix("Push ") {
            let value = raw.parse::<u64>().map_err(|_| format!("invalid Push operand: {raw}"))?;
            stack.push(value);
        } else {
            let b = stack.pop().ok_or_else(|| format!("stack underflow at {op}"))?;
            let a = stack.pop().ok_or_else(|| format!("stack underflow at {op}"))?;
            let value = match op.as_str() {
                "Add" => a.wrapping_add(b),
                "Sub" => a.wrapping_sub(b),
                "Mul" => a.wrapping_mul(b),
                "Div" => {
                    if b == 0 { return Err("DivisionByZero in native assembler simulation".to_string()); }
                    a / b
                }
                "Halt" => break,
                _ => return Err(format!("unsupported op in native assembler: {op}")),
            };
            stack.push(value);
        }
    }
    let result = stack.last().copied().unwrap_or(0);
    if result != expected {
        return Err(format!("native assembly simulation {result} != expected {expected}"));
    }
    Ok(())
}

pub fn assemble(contract: &Path, vector: &Path, out_dir: &Path) -> Result<PathBuf, String> {
    let source = fs::read_to_string(contract).map_err(|e| format!("cannot read contract: {e}"))?;
    let vector_json = fs::read_to_string(vector).map_err(|e| format!("cannot read vector: {e}"))?;
    let (params, body) = find_function(&source)?;
    let expected = json_u64_for_key(&vector_json, "expected")?;

    let mut env = HashMap::new();
    for param in params {
        let value = json_u64_for_key(&vector_json, &param)?;
        env.insert(param, vec![format!("Push {value}")]);
    }

    let mut final_ops: Option<Vec<String>> = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Some(rest) = line.strip_prefix("let ") {
            let eq = rest.find('=').ok_or_else(|| format!("invalid let statement: {line}"))?;
            let name = rest[..eq].trim();
            let expr = rest[eq + 1..].trim();
            let mut parser = Parser::new(tokenize(expr)?, env.clone());
            let ops = parser.expr()?;
            if parser.peek().is_some() {
                return Err(format!("unconsumed expression tokens in: {line}"));
            }
            env.insert(name.to_string(), ops);
        } else if let Some(expr) = line.strip_prefix("return ") {
            let mut parser = Parser::new(tokenize(expr.trim())?, env.clone());
            let ops = parser.expr()?;
            if parser.peek().is_some() {
                return Err(format!("unconsumed return expression tokens in: {line}"));
            }
            final_ops = Some(ops);
        }
    }

    let mut ops = final_ops.ok_or_else(|| "no return statement in EXEC-GATE subset".to_string())?;
    ops.push("Halt".to_string());
    simulate(&ops, expected)?;

    fs::create_dir_all(out_dir).map_err(|e| format!("cannot create output directory: {e}"))?;
    let stem = contract.file_stem().and_then(|s| s.to_str()).ok_or_else(|| "invalid contract filename".to_string())?;
    let out = out_dir.join(format!("{stem}.ops"));
    let mut text = format!("# contract: {stem}\n# expected: {expected}\n");
    for op in &ops {
        text.push_str(op);
        text.push('\n');
    }
    fs::write(&out, text).map_err(|e| format!("cannot write ops output: {e}"))?;
    Ok(out)
}
