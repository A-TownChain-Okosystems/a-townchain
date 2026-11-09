// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Typ- und Signaturpruefung fuer Vertraege (Semantik Stufe 2).
//!
//! Bewusst konservativ: gemeldet wird nur, wenn *beide* Seiten einen bekannten
//! Typ haben und sich widersprechen. Unbekannte Bezeichner, Aufrufe ueber
//! Dateigrenzen und nicht aufloesbare Typnamen gelten als `Unknown` und
//! erzeugen nie einen Befund. Damit bleibt das Gate ehrlich: es behauptet keine
//! Typkorrektheit, die es nicht beweisen kann.
//!
//! Geprueft werden:
//! - `return <expr>` gegen den deklarierten Rueckgabetyp,
//! - `let x: T = <expr>` gegen die Typannotation,
//! - Zuweisungen an Zustandsfelder (`self.x = ...`) gegen den Feldtyp,
//! - `require(...)`, `if` und `while` brauchen eine Bedingung vom Typ `bool`,
//! - `emit <Event>(...)` gegen die Feldtypen der Event-Deklaration.

use crate::ast::{Expr, FunctionDef, Program, Stmt, TypeHint};

/// Ein grober Typ. `Unknown` ist mit allem vertraeglich.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Int,
    Bool,
    Str,
    List(Box<Ty>),
    Map,
    Tuple,
    Named(String),
    Unknown,
}

impl Ty {
    /// Zwei Typen sind vertraeglich, wenn einer unbekannt ist, sie gleich sind
    /// oder beide Listen mit vertraeglichen Elementen sind.
    pub fn compatible(&self, other: &Ty) -> bool {
        match (self, other) {
            (Ty::Unknown, _) | (_, Ty::Unknown) => true,
            // Nominale Typen werden nicht aufgeloest (kein Import-Lookup).
            (Ty::Named(_), _) | (_, Ty::Named(_)) => true,
            (Ty::List(a), Ty::List(b)) => a.compatible(b),
            (a, b) => a == b,
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Ty::Int => "integer".into(),
            Ty::Bool => "bool".into(),
            Ty::Str => "string".into(),
            Ty::List(inner) => format!("list<{}>", inner.describe()),
            Ty::Map => "map".into(),
            Ty::Tuple => "tuple".into(),
            Ty::Named(n) => n.clone(),
            Ty::Unknown => "unknown".into(),
        }
    }
}

/// Ergebnis der Typ-Pruefung.
#[derive(Debug, Default)]
pub struct Report {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

fn is_int_name(name: &str) -> bool {
    let lower = name.trim().to_ascii_lowercase();
    if lower == "int" || lower == "integer" {
        return true;
    }
    // u8..u256 / i8..i256: Praefix abziehen, der Rest muss eine Bitbreite sein.
    let digits = match lower.strip_prefix('u').or_else(|| lower.strip_prefix('i')) {
        Some(rest) => rest,
        None => return false,
    };
    !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
}

fn ty_from_name(raw: &str) -> Ty {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ty::Unknown;
    }
    if raw.starts_with('(') {
        return Ty::Tuple;
    }
    let (head, inner) = match raw.find('<') {
        Some(i) if raw.ends_with('>') => (&raw[..i], Some(&raw[i + 1..raw.len() - 1])),
        Some(i) => (&raw[..i], None),
        None => (raw, None),
    };
    let low = head.trim().to_ascii_lowercase();
    match low.as_str() {
        "list" | "vec" | "array" => {
            Ty::List(Box::new(inner.map(ty_from_name).unwrap_or(Ty::Unknown)))
        }
        "map" | "hashmap" | "dict" | "mapping" => Ty::Map,
        "bool" => Ty::Bool,
        "string" | "str" => Ty::Str,
        other if is_int_name(other) => Ty::Int,
        _ => Ty::Named(head.trim().to_string()),
    }
}

/// Typ aus einer Deklaration.
pub fn ty_from_hint(hint: &TypeHint) -> Ty {
    ty_from_name(&hint.name)
}

struct FnSig {
    params: Vec<Ty>,
    ret: Ty,
}

#[derive(Default)]
struct Scope {
    /// Zustandsfelder des Vertrags (inklusive Basistypen, soweit in der Datei).
    state: Vec<(String, Ty)>,
    /// Funktionssignaturen.
    fns: Vec<(String, FnSig)>,
    /// Event-Feldtypen.
    events: Vec<(String, Vec<Ty>)>,
    /// Struct-Feldtypen (fuer Struct-Literale nicht ausgewertet, aber vorhanden).
    structs: Vec<(String, Vec<(String, Ty)>)>,
}

impl Scope {
    fn state_ty(&self, name: &str) -> Ty {
        self.state
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, t)| t.clone())
            .unwrap_or(Ty::Unknown)
    }

    fn fn_ret(&self, name: &str) -> Ty {
        self.fns
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, s)| s.ret.clone())
            .unwrap_or(Ty::Unknown)
    }

    fn fn_arity(&self, name: &str) -> Option<usize> {
        self.fns
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, s)| s.params.len())
    }
}

fn collect_scope(body: &[Stmt]) -> Scope {
    let mut scope = Scope::default();
    for item in body {
        match item {
            Stmt::State { name, ty, .. } => scope.state.push((name.clone(), ty_from_hint(ty))),
            Stmt::StateBlock { fields } => {
                for (n, t, _) in fields {
                    scope.state.push((n.clone(), ty_from_hint(t)));
                }
            }
            Stmt::Event { name, fields } => scope.events.push((
                name.clone(),
                fields.iter().map(|(_, t)| ty_from_hint(t)).collect(),
            )),
            Stmt::Struct { name, fields } => scope.structs.push((
                name.clone(),
                fields
                    .iter()
                    .map(|(n, t)| (n.clone(), ty_from_hint(t)))
                    .collect(),
            )),
            Stmt::Fn(f) => scope.fns.push((f.name.clone(), signature(f))),
            _ => {}
        }
    }
    scope
}

fn signature(f: &FunctionDef) -> FnSig {
    FnSig {
        params: f
            .params
            .iter()
            .map(|p| ty_from_hint(&p.type_hint))
            .collect(),
        ret: f
            .return_type
            .as_ref()
            .map(ty_from_hint)
            .unwrap_or(Ty::Unknown),
    }
}

/// Prueft ein Programm (alle Vertraege) und liefert Befunde.
pub fn check_program(program: &Program) -> Report {
    let mut contracts: Vec<(String, Vec<String>, Vec<Stmt>)> = Vec::new();
    for stmt in &program.statements {
        if let Stmt::Contract { name, bases, body } = stmt {
            contracts.push((name.clone(), bases.clone(), body.clone()));
        }
    }

    let mut report = Report::default();
    for (name, bases, body) in &contracts {
        let mut scope = collect_scope(body);
        for base in bases {
            if let Some((_, _, base_body)) = contracts.iter().find(|(n, _, _)| n == base) {
                let base_scope = collect_scope(base_body);
                scope.state.extend(base_scope.state);
                scope.fns.extend(base_scope.fns);
                scope.events.extend(base_scope.events);
                scope.structs.extend(base_scope.structs);
            }
        }
        for item in body {
            if let Stmt::Fn(f) = item {
                check_fn(name, f, &scope, &mut report);
            }
        }
    }
    report
}

fn check_fn(contract: &str, f: &FunctionDef, scope: &Scope, report: &mut Report) {
    let ret = f
        .return_type
        .as_ref()
        .map(ty_from_hint)
        .unwrap_or(Ty::Unknown);
    let mut locals: Vec<(String, Ty)> = f
        .params
        .iter()
        .map(|p| (p.name.clone(), ty_from_hint(&p.type_hint)))
        .collect();
    check_block(contract, &f.name, &ret, &f.body, scope, &mut locals, report);
}

fn check_block(
    contract: &str,
    func: &str,
    ret: &Ty,
    body: &[Stmt],
    scope: &Scope,
    locals: &mut Vec<(String, Ty)>,
    report: &mut Report,
) {
    for stmt in body {
        match stmt {
            Stmt::Let(l) => {
                if let (Some(hint), Some(value)) = (&l.type_hint, &l.value) {
                    let expected = ty_from_hint(hint);
                    let actual = expr_ty(value, scope, locals);
                    if !expected.compatible(&actual) {
                        report.errors.push(format!(
                            "contract {contract} / fn {func}: '{name}: {hint}' erhaelt {got}",
                            name = l.name,
                            hint = hint.name,
                            got = actual.describe()
                        ));
                    }
                    locals.push((l.name.clone(), expected));
                } else if let Some(value) = &l.value {
                    locals.push((l.name.clone(), expr_ty(value, scope, locals)));
                } else {
                    locals.push((l.name.clone(), Ty::Unknown));
                }
            }
            Stmt::Assign { target, op, value } => {
                let actual = expr_ty(value, scope, locals);
                let expected = match target {
                    Expr::Ident(n) => lookup(n, scope, locals),
                    Expr::Field { base, name } if is_self(base) => scope.state_ty(name),
                    _ => Ty::Unknown,
                };
                if op.is_some() && !expected.compatible(&Ty::Int) {
                    report.errors.push(format!(
                        "contract {contract} / fn {func}: '{op}' auf {ty} ist nicht definiert",
                        op = op.clone().unwrap_or_default(),
                        ty = expected.describe()
                    ));
                } else if !expected.compatible(&actual) {
                    report.errors.push(format!(
                        "contract {contract} / fn {func}: Zuweisung erwartet {exp}, erhaelt {got}",
                        exp = expected.describe(),
                        got = actual.describe()
                    ));
                }
            }
            Stmt::Require { cond, .. } => {
                let ty = expr_ty(cond, scope, locals);
                if !ty.compatible(&Ty::Bool) {
                    report.errors.push(format!(
                        "contract {contract} / fn {func}: require braucht bool, hat {}",
                        ty.describe()
                    ));
                }
            }
            Stmt::If {
                cond,
                then_body,
                else_body,
            } => {
                let ty = expr_ty(cond, scope, locals);
                if !ty.compatible(&Ty::Bool) {
                    report.errors.push(format!(
                        "contract {contract} / fn {func}: if braucht bool, hat {}",
                        ty.describe()
                    ));
                }
                check_block(contract, func, ret, then_body, scope, locals, report);
                if let Some(eb) = else_body {
                    check_block(contract, func, ret, eb, scope, locals, report);
                }
            }
            Stmt::For { var, body, .. } => {
                // Schleifenvariable ist im Rumpf ein Int (SPEC 3.2: for_stmt).
                locals.push((var.clone(), Ty::Int));
                check_block(contract, func, ret, body, scope, locals, report);
                locals.pop();
            }
            Stmt::While { cond, body } => {
                let ty = expr_ty(cond, scope, locals);
                if !ty.compatible(&Ty::Bool) {
                    report.errors.push(format!(
                        "contract {contract} / fn {func}: while braucht bool, hat {}",
                        ty.describe()
                    ));
                }
                check_block(contract, func, ret, body, scope, locals, report);
            }
            Stmt::Return { value } => match (value, ret) {
                (Some(v), expected) if *expected != Ty::Unknown => {
                    let actual = expr_ty(v, scope, locals);
                    if !expected.compatible(&actual) {
                        report.errors.push(format!(
                            "contract {contract} / fn {func}: return erwartet {exp}, erhaelt {got}",
                            exp = expected.describe(),
                            got = actual.describe()
                        ));
                    }
                }
                (None, expected) if *expected != Ty::Unknown && *expected != Ty::Tuple => {
                    report.errors.push(format!(
                        "contract {contract} / fn {func}: return ohne Wert, erwartet {exp}",
                        exp = expected.describe()
                    ));
                }
                _ => {}
            },
            Stmt::Emit { event, args } => {
                if let Some((_, fields)) = scope.events.iter().find(|(n, _)| n == event) {
                    if fields.len() == args.len() {
                        for (idx, (arg, want)) in args.iter().zip(fields.iter()).enumerate() {
                            let got = expr_ty(arg, scope, locals);
                            if !want.compatible(&got) {
                                report.warnings.push(format!(
                                    "contract {contract} / fn {func}: emit {event} Argument {} erwartet {}, hat {}",
                                    idx + 1,
                                    want.describe(),
                                    got.describe()
                                ));
                            }
                        }
                    }
                }
            }
            Stmt::Expr(e) => {
                expr_ty(e, scope, locals);
            }
            _ => {}
        }
    }
}

fn is_self(expr: &Expr) -> bool {
    matches!(expr, Expr::Ident(n) if n == "self")
}

fn lookup(name: &str, scope: &Scope, locals: &[(String, Ty)]) -> Ty {
    if let Some((_, t)) = locals.iter().rev().find(|(n, _)| n == name) {
        return t.clone();
    }
    scope.state_ty(name)
}

fn expr_ty(expr: &Expr, scope: &Scope, locals: &[(String, Ty)]) -> Ty {
    match expr {
        Expr::Int(_) | Expr::BigInt(_) => Ty::Int,
        Expr::String(_) => Ty::Str,
        Expr::Ident(name) => lookup(name, scope, locals),
        // Ein Bereich hat im Kern keinen eigenen Werttyp (siehe Semantik).
        Expr::Range { .. } | Expr::Slice { .. } => Ty::Unknown,
        Expr::Unary { op, operand } => match op.as_str() {
            "!" => Ty::Bool,
            "-" | "+" => {
                let inner = expr_ty(operand, scope, locals);
                if inner.compatible(&Ty::Int) {
                    Ty::Int
                } else {
                    Ty::Unknown
                }
            }
            _ => Ty::Unknown,
        },
        Expr::Binary { op, left, right } => match op.as_str() {
            "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" => Ty::Bool,
            "+" | "-" | "*" | "/" | "%" => {
                let l = expr_ty(left, scope, locals);
                let r = expr_ty(right, scope, locals);
                if l.compatible(&Ty::Int) && r.compatible(&Ty::Int) {
                    Ty::Int
                } else if op == "+" && (l == Ty::Str || r == Ty::Str) {
                    Ty::Str
                } else {
                    Ty::Unknown
                }
            }
            _ => Ty::Unknown,
        },
        Expr::Call { target, .. } => match target.as_ref() {
            Expr::Ident(name) => scope.fn_ret(name),
            Expr::Field { base, name } if is_self(base) => scope.fn_ret(name),
            _ => Ty::Unknown,
        },
        Expr::Field { base, name } => {
            if is_self(base) {
                scope.state_ty(name)
            } else {
                Ty::Unknown
            }
        }
        Expr::Index { base, .. } => match expr_ty(base, scope, locals) {
            Ty::List(inner) => *inner,
            _ => Ty::Unknown,
        },
        Expr::Array(items) => {
            let inner = items
                .first()
                .map(|e| expr_ty(e, scope, locals))
                .unwrap_or(Ty::Unknown);
            Ty::List(Box::new(inner))
        }
        Expr::Tuple(_) => Ty::Tuple,
        Expr::MapLiteral(_) => Ty::Map,
        Expr::StructLiteral { name, .. } => Ty::Named(name.clone()),
    }
}

/// Arity-Check fuer Aufrufe bekannter Funktionen (Warnung, kein Fehler).
pub fn check_call_arities(program: &Program) -> Vec<String> {
    let mut findings = Vec::new();
    for stmt in &program.statements {
        if let Stmt::Contract { name, body, .. } = stmt {
            let scope = collect_scope(body);
            for item in body {
                if let Stmt::Fn(f) = item {
                    for stmt in &f.body {
                        walk_calls(stmt, &scope, name, &f.name, &mut findings);
                    }
                }
            }
        }
    }
    findings
}

fn walk_calls(stmt: &Stmt, scope: &Scope, contract: &str, func: &str, out: &mut Vec<String>) {
    let visit_expr = |e: &Expr, out: &mut Vec<String>| {
        if let Expr::Call { target, args } = e {
            let name = match target.as_ref() {
                Expr::Ident(n) => Some(n.clone()),
                Expr::Field { base, name } if is_self(base) => Some(name.clone()),
                _ => None,
            };
            if let Some(n) = name {
                if let Some(arity) = scope.fn_arity(&n) {
                    if arity != args.len() {
                        out.push(format!(
                            "contract {contract} / fn {func}: Aufruf {n}() erwartet {arity} Argument(e), hat {}",
                            args.len()
                        ));
                    }
                }
            }
        }
    };
    match stmt {
        Stmt::Expr(e) => visit_expr(e, out),
        Stmt::Return { value: Some(e) } => visit_expr(e, out),
        Stmt::Assign { value, .. } => visit_expr(value, out),
        Stmt::Require { cond, .. } => visit_expr(cond, out),
        Stmt::Emit { args, .. } => {
            for a in args {
                visit_expr(a, out);
            }
        }
        Stmt::Let(l) => {
            if let Some(v) = &l.value {
                visit_expr(v, out);
            }
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            visit_expr(cond, out);
            for s in then_body {
                walk_calls(s, scope, contract, func, out);
            }
            if let Some(eb) = else_body {
                for s in eb {
                    walk_calls(s, scope, contract, func, out);
                }
            }
        }
        Stmt::While { cond, body } => {
            visit_expr(cond, out);
            for s in body {
                walk_calls(s, scope, contract, func, out);
            }
        }
        Stmt::For { iter, body, .. } => {
            visit_expr(iter, out);
            for s in body {
                walk_calls(s, scope, contract, func, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(src: &str) -> Report {
        let program = crate::parser::parse_program(src).expect("Parse-Fehler");
        check_program(&program)
    }

    #[test]
    fn falscher_rueckgabetyp_wird_abgelehnt() {
        let r = report(r#"contract T { fn f() -> int { return "text"; } }"#);
        assert_eq!(r.errors.len(), 1, "{:?}", r.errors);
        assert!(
            r.errors[0].contains("return erwartet integer"),
            "{:?}",
            r.errors
        );
    }

    #[test]
    fn passender_rueckgabetyp_ist_sauber() {
        let r = report("contract T { fn f() -> int { return 1 + 2; } }");
        assert!(r.errors.is_empty(), "{:?}", r.errors);
    }

    #[test]
    fn typannotation_am_let_wird_geprueft() {
        let r = report(r#"contract T { fn f() -> int { let n: int = "x"; return 1; } }"#);
        assert_eq!(r.errors.len(), 1, "{:?}", r.errors);
        assert!(r.errors[0].contains("erhaelt string"), "{:?}", r.errors);
    }

    #[test]
    fn require_braucht_bool() {
        let r = report("contract T { fn f() -> int { require(1); return 1; } }");
        assert_eq!(r.errors.len(), 1, "{:?}", r.errors);
        assert!(
            r.errors[0].contains("require braucht bool"),
            "{:?}",
            r.errors
        );
    }

    #[test]
    fn if_und_while_brauchen_bool() {
        let r = report("contract T { fn f() -> int { if 5 { return 1; } return 0; } }");
        assert!(
            r.errors.iter().any(|e| e.contains("if braucht bool")),
            "{:?}",
            r.errors
        );
        let r = report("contract T { fn f() -> int { while 1 { } return 0; } }");
        assert!(
            r.errors.iter().any(|e| e.contains("while braucht bool")),
            "{:?}",
            r.errors
        );
    }

    #[test]
    fn zuweisung_an_zustandsfeld_prueft_den_typ() {
        let r = report(
            r#"contract T { state owner: Address fn f() -> int { self.owner = 42; return 1; } }"#,
        );
        // Address ist nominal/unbekannt geprueft -> kein Befund (konservativ).
        assert!(r.errors.is_empty(), "{:?}", r.errors);

        let r = report(
            r#"contract T { state count: int fn f() -> int { self.count = "viele"; return 1; } }"#,
        );
        assert_eq!(r.errors.len(), 1, "{:?}", r.errors);
        assert!(r.errors[0].contains("erwartet integer"), "{:?}", r.errors);
    }

    #[test]
    fn emit_argumente_werden_gegen_die_deklaration_geprueft() {
        let r = report(
            r#"contract T {
                state owner: Address
                event Moved(to: Address, amount: int)
                fn f() -> int { emit Moved(self.owner, "viel"); return 1; }
            }"#,
        );
        assert!(
            r.warnings
                .iter()
                .any(|w| w.contains("emit Moved Argument 2")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn unbekannte_typen_und_aufrufe_erzeugen_keine_befunde() {
        let r = report(
            r#"contract T {
                state owner: Address
                fn helper(a: Hash) -> bool { return a == a; }
                fn f() -> Result { return self.helper(self.owner); }
            }"#,
        );
        assert!(r.errors.is_empty(), "{:?}", r.errors);
    }
}
