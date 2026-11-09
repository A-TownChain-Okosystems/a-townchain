// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Semantische Pruefung fuer Vertraege (Stufe 1).
//!
//! Belastbar ohne Typinferenz:
//! - doppelte Deklarationen (state/fn/event) je Vertrag,
//! - `self.<feld>` muss ein deklariertes Zustandsfeld sein; `self.<fn>()` ist erlaubt,
//! - `emit <Event>` muss ein deklariertes Event sein (auch geerbt),
//! - Argumentanzahl von `emit` muss zur Deklaration passen.
//!
//! Fehlende Event-Deklarationen sind Warnungen, keine Ablehnung: die
//! Bestandsvertraege emittieren teils Events, die (noch) nicht deklariert sind.

use crate::ast::{Expr, Program, Stmt};

/// Ergebnis der semantischen Pruefung.
#[derive(Debug, Default)]
pub struct Report {
    /// Verstoesse, die den Vertrag ablehnen.
    pub errors: Vec<String>,
    /// Befunde, die gemeldet, aber nicht abgelehnt werden.
    pub warnings: Vec<String>,
}

impl Report {
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty()
    }
}

#[derive(Default)]
struct Decls {
    states: Vec<String>,
    events: Vec<(String, usize)>,
    fns: Vec<String>,
}

/// Prueft ein Programm und liefert Fehler und Warnungen.
/// SPEC 3.4: `RangeExpr` ist ein Knoten ohne definierten Werttyp.
///
/// Deshalb gilt fuer **jeden** Funktionskoerper, auch ausserhalb von
/// Vertraegen: ein Bereich ist nur als `for`-Iterator und als Index
/// (dort SliceExpr) zulaessig. Als Wert ist er ein Fehler.
fn walk_range_expr(fn_name: &str, expr: &Expr, range_ok: bool, report: &mut Report) {
    match expr {
        Expr::Range { start, end, .. } => {
            if !range_ok {
                report.errors.push(format!(
                    "fn {fn_name}: Bereich ist nur in for oder als Index erlaubt"
                ));
            }
            for part in [start.as_deref(), end.as_deref()].into_iter().flatten() {
                walk_range_expr(fn_name, part, false, report);
            }
        }
        Expr::Slice {
            base, start, end, ..
        } => {
            walk_range_expr(fn_name, base, false, report);
            for part in [start.as_deref(), end.as_deref()].into_iter().flatten() {
                walk_range_expr(fn_name, part, false, report);
            }
        }
        Expr::Unary { operand, .. } => walk_range_expr(fn_name, operand, false, report),
        Expr::Binary { left, right, .. } => {
            walk_range_expr(fn_name, left, false, report);
            walk_range_expr(fn_name, right, false, report);
        }
        Expr::Call { target, args } => {
            walk_range_expr(fn_name, target, false, report);
            for a in args {
                walk_range_expr(fn_name, a, false, report);
            }
        }
        Expr::Field { base, .. } => walk_range_expr(fn_name, base, false, report),
        Expr::Index { base, index } => {
            walk_range_expr(fn_name, base, false, report);
            walk_range_expr(fn_name, index, false, report);
        }
        Expr::Array(items) | Expr::Tuple(items) => {
            for i in items {
                walk_range_expr(fn_name, i, false, report);
            }
        }
        Expr::MapLiteral(entries) => {
            for (k, v) in entries {
                walk_range_expr(fn_name, k, false, report);
                walk_range_expr(fn_name, v, false, report);
            }
        }
        Expr::StructLiteral { fields, .. } => {
            for (_, v) in fields {
                walk_range_expr(fn_name, v, false, report);
            }
        }
        _ => {}
    }
}

fn walk_range_stmt(fn_name: &str, stmt: &Stmt, report: &mut Report) {
    match stmt {
        Stmt::For { iter, body, .. } => {
            if let Expr::Range { start, end, .. } = iter {
                if start.is_none() || end.is_none() {
                    report.errors.push(format!(
                        "fn {fn_name}: for braucht einen Bereich mit Start und Ende"
                    ));
                }
            }
            walk_range_expr(fn_name, iter, true, report);
            for s in body {
                walk_range_stmt(fn_name, s, report);
            }
        }
        Stmt::Let(l) => {
            if let Some(v) = &l.value {
                walk_range_expr(fn_name, v, false, report);
            }
        }
        Stmt::Return { value: Some(v) } => walk_range_expr(fn_name, v, false, report),
        Stmt::Expr(e) => walk_range_expr(fn_name, e, false, report),
        Stmt::Assign { target, value, .. } => {
            walk_range_expr(fn_name, target, false, report);
            walk_range_expr(fn_name, value, false, report);
        }
        Stmt::Emit { args, .. } => {
            for a in args {
                walk_range_expr(fn_name, a, false, report);
            }
        }
        Stmt::Require { cond, message } => {
            walk_range_expr(fn_name, cond, false, report);
            if let Some(m) = message {
                walk_range_expr(fn_name, m, false, report);
            }
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            walk_range_expr(fn_name, cond, false, report);
            for s in then_body {
                walk_range_stmt(fn_name, s, report);
            }
            if let Some(eb) = else_body {
                for s in eb {
                    walk_range_stmt(fn_name, s, report);
                }
            }
        }
        Stmt::While { cond, body } => {
            walk_range_expr(fn_name, cond, false, report);
            for s in body {
                walk_range_stmt(fn_name, s, report);
            }
        }
        _ => {}
    }
}

/// Laeuft ueber alle Funktionskoerper — Vertragsfunktionen und freie.
fn walk_ranges_in_program(program: &Program, report: &mut Report) {
    fn walk_items(stmts: &[Stmt], report: &mut Report) {
        for stmt in stmts {
            match stmt {
                Stmt::Fn(f) => {
                    for s in &f.body {
                        walk_range_stmt(&f.name, s, report);
                    }
                }
                Stmt::Contract { body, .. } => walk_items(body, report),
                _ => {}
            }
        }
    }
    walk_items(&program.statements, report);
}

pub fn check_program(program: &Program) -> Report {
    let mut contracts: Vec<(String, Vec<String>, Vec<Stmt>)> = Vec::new();
    for stmt in &program.statements {
        if let Stmt::Contract { name, bases, body } = stmt {
            contracts.push((name.clone(), bases.clone(), body.clone()));
        }
    }

    let mut report = Report::default();
    for (name, bases, body) in &contracts {
        let own = collect_decls(body);
        let mut effective = Decls {
            states: own.states.clone(),
            events: own.events.clone(),
            fns: own.fns.clone(),
        };
        for base in bases {
            match contracts.iter().find(|(n, _, _)| n == base) {
                Some((_, _, base_body)) => {
                    let base_decls = collect_decls(base_body);
                    effective.states.extend(base_decls.states);
                    effective.events.extend(base_decls.events);
                    effective.fns.extend(base_decls.fns);
                }
                None => report.warnings.push(format!(
                    "contract {name}: Basis '{base}' liegt nicht in dieser Datei, Vererbung ungeprueft"
                )),
            }
        }
        check_contract(name, body, &own, &effective, &mut report);
    }
    walk_ranges_in_program(program, &mut report);
    report
}

fn collect_decls(body: &[Stmt]) -> Decls {
    let mut decls = Decls::default();
    for item in body {
        match item {
            Stmt::State { name, .. } => decls.states.push(name.clone()),
            Stmt::StateBlock { fields } => decls
                .states
                .extend(fields.iter().map(|(n, _, _)| n.clone())),
            Stmt::Event { name, fields } => decls.events.push((name.clone(), fields.len())),
            Stmt::Fn(f) => decls.fns.push(f.name.clone()),
            _ => {}
        }
    }
    decls
}

fn check_contract(name: &str, body: &[Stmt], own: &Decls, effective: &Decls, report: &mut Report) {
    for (label, names) in [
        ("state", &own.states),
        ("fn", &own.fns),
        (
            "event",
            &own.events.iter().map(|(n, _)| n.clone()).collect(),
        ),
    ] {
        let mut seen: Vec<&String> = Vec::new();
        for n in names.iter() {
            if seen.contains(&n) {
                report.errors.push(format!(
                    "contract {name}: {label} '{n}' ist doppelt deklariert"
                ));
            } else {
                seen.push(n);
            }
        }
    }

    for item in body {
        if let Stmt::Fn(f) = item {
            for stmt in &f.body {
                check_stmt(&f.name, stmt, effective, report);
            }
        }
    }
}

fn check_stmt(fn_name: &str, stmt: &Stmt, decls: &Decls, report: &mut Report) {
    match stmt {
        Stmt::Let(l) => {
            if let Some(v) = &l.value {
                check_expr(fn_name, v, decls, report);
            }
        }
        Stmt::Return { value: Some(v) } => check_expr(fn_name, v, decls, report),
        // `return` ohne Wert hat nichts zu pruefen.
        Stmt::Return { value: None } => {}
        Stmt::Expr(e) => check_expr(fn_name, e, decls, report),
        Stmt::Assign { target, value, .. } => {
            check_expr(fn_name, target, decls, report);
            check_expr(fn_name, value, decls, report);
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            check_expr(fn_name, cond, decls, report);
            for s in then_body {
                check_stmt(fn_name, s, decls, report);
            }
            if let Some(else_body) = else_body {
                for s in else_body {
                    check_stmt(fn_name, s, decls, report);
                }
            }
        }
        Stmt::For { iter, body, .. } => {
            match iter {
                Expr::Range { start, end, .. } => {
                    for part in [start.as_deref(), end.as_deref()].into_iter().flatten() {
                        check_expr(fn_name, part, decls, report);
                    }
                }
                other => check_expr(fn_name, other, decls, report),
            }
            for s in body {
                check_stmt(fn_name, s, decls, report);
            }
        }
        Stmt::While { cond, body } => {
            check_expr(fn_name, cond, decls, report);
            for s in body {
                check_stmt(fn_name, s, decls, report);
            }
        }
        Stmt::Require { cond, message } => {
            check_expr(fn_name, cond, decls, report);
            if let Some(m) = message {
                check_expr(fn_name, m, decls, report);
            }
        }
        Stmt::Emit { event, args } => {
            let short = event.rsplit("::").next().unwrap_or(event);
            match decls.events.iter().find(|(n, _)| n == event || n == short) {
                None => report.warnings.push(format!(
                    "fn {fn_name}: emit '{event}' hat keine Event-Deklaration"
                )),
                Some((_, arity)) => {
                    if *arity != args.len() {
                        report.warnings.push(format!(
                            "fn {fn_name}: emit '{event}' hat {} Argument(e), deklariert sind {arity}",
                            args.len()
                        ));
                    }
                }
            }
            for a in args {
                check_expr(fn_name, a, decls, report);
            }
        }
        _ => {}
    }
}

fn check_expr(fn_name: &str, expr: &Expr, decls: &Decls, report: &mut Report) {
    match expr {
        Expr::Field { base, name } => {
            if matches!(base.as_ref(), Expr::Ident(root) if root == "self")
                && !decls.states.iter().any(|f| f == name)
                && !decls.fns.iter().any(|f| f == name)
            {
                report.errors.push(format!(
                    "fn {fn_name}: self.{name} ist weder Zustandsfeld noch Funktion"
                ));
            }
            check_expr(fn_name, base, decls, report);
        }
        Expr::Index { base, index } => {
            check_expr(fn_name, base, decls, report);
            check_expr(fn_name, index, decls, report);
        }
        // Die Zulaessigkeit eines Bereichs prueft `walk_ranges` fuer jeden
        // Funktionskoerper; hier nur die Teile durchlaufen.
        Expr::Range { start, end, .. } => {
            for part in [start.as_deref(), end.as_deref()].into_iter().flatten() {
                check_expr(fn_name, part, decls, report);
            }
        }
        Expr::Slice {
            base, start, end, ..
        } => {
            check_expr(fn_name, base, decls, report);
            for part in [start.as_deref(), end.as_deref()].into_iter().flatten() {
                check_expr(fn_name, part, decls, report);
            }
        }
        Expr::Unary { operand, .. } => check_expr(fn_name, operand, decls, report),
        Expr::Binary { left, right, .. } => {
            check_expr(fn_name, left, decls, report);
            check_expr(fn_name, right, decls, report);
        }
        Expr::Call { target, args } => {
            check_expr(fn_name, target, decls, report);
            for a in args {
                check_expr(fn_name, a, decls, report);
            }
        }
        Expr::Array(items) | Expr::Tuple(items) => {
            for i in items {
                check_expr(fn_name, i, decls, report);
            }
        }
        Expr::MapLiteral(entries) => {
            for (k, v) in entries {
                check_expr(fn_name, k, decls, report);
                check_expr(fn_name, v, decls, report);
            }
        }
        Expr::StructLiteral { fields, .. } => {
            for (_, v) in fields {
                check_expr(fn_name, v, decls, report);
            }
        }
        Expr::Int(_) | Expr::BigInt(_) | Expr::String(_) | Expr::Ident(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    fn check(src: &str) -> Report {
        check_program(&parse_program(src).expect("Quelle muss parsen"))
    }

    #[test]
    fn akzeptiert_sauberen_vertrag() {
        let r = check(
            r#"
contract Token {
    state owner: Address
    state balances: Map<Address, u128>
    event Transfer(from_: Address, to: Address, amount: u128)
    fn transfer(to: Address, amount: u128) -> bool {
        emit Transfer(msg_sender(), to, amount)
        self.balances[to] = amount
        return true
    }
}
"#,
        );
        assert!(r.errors.is_empty(), "unerwartete Fehler: {:?}", r.errors);
        assert!(
            r.warnings.is_empty(),
            "unerwartete Warnungen: {:?}",
            r.warnings
        );
    }

    #[test]
    fn lehnt_doppeltes_zustandsfeld_ab() {
        let r = check(
            r#"
contract Token {
    state owner: Address
    state owner: Address
}
"#,
        );
        assert_eq!(r.errors.len(), 1);
        assert!(r.errors[0].contains("state 'owner'"));
    }

    #[test]
    fn lehnt_unbekanntes_self_feld_ab() {
        let r = check(
            r#"
contract Token {
    state owner: Address
    fn f() -> Address { return self.owner_ }
}
"#,
        );
        assert_eq!(r.errors.len(), 1);
        assert!(r.errors[0].contains("self.owner_"));
    }

    #[test]
    fn erlaubt_self_methodenaufruf() {
        let r = check(
            r#"
contract Token {
    state owner: Address
    fn power(a: Address) -> u128 { return 1 }
    fn f() -> u128 { return self.power(self.owner) }
}
"#,
        );
        assert!(r.errors.is_empty(), "unerwartete Fehler: {:?}", r.errors);
    }

    #[test]
    fn meldet_fehlende_event_deklaration_als_warnung() {
        let r = check(
            r#"
contract Token {
    fn f() { emit Transfer(msg_sender()) }
}
"#,
        );
        assert!(r.errors.is_empty());
        assert_eq!(r.warnings.len(), 1);
        assert!(r.warnings[0].contains("Transfer"));
    }

    #[test]
    fn prueft_event_argumentanzahl() {
        let r = check(
            r#"
contract Token {
    event Transfer(to: Address, amount: u128)
    fn f() { emit Transfer(msg_sender()) }
}
"#,
        );
        assert!(r.errors.is_empty());
        assert_eq!(r.warnings.len(), 1);
        assert!(r.warnings[0].contains("Argument"));
    }

    #[test]
    fn erbt_events_aus_basisvertrag() {
        let r = check(
            r#"
contract Base {
    state owner: Address
    event Paused(by: Address)
}
contract Token : Base {
    state supply: u128
    fn pause() { emit Paused(msg_sender()) }
}
"#,
        );
        assert!(r.errors.is_empty(), "unerwartete Fehler: {:?}", r.errors);
        assert!(
            r.warnings.is_empty(),
            "unerwartete Warnungen: {:?}",
            r.warnings
        );
    }

    #[test]
    fn bereich_als_wert_wird_abgelehnt() {
        // SPEC 3.4: RangeExpr ist ein Knoten ohne Werttyp — als Wert Fehler.
        let r = check("fn f() -> int {\n    let r = 0..3\n    return 1\n}");
        assert!(
            r.errors
                .iter()
                .any(|e| e.contains("nur in for oder als Index")),
            "{:?}",
            r.errors
        );
    }

    #[test]
    fn for_bereich_wird_akzeptiert() {
        let r =
            check("fn f() -> int {\n    for i in 0..3 {\n        return i\n    }\n    return 0\n}");
        assert!(r.errors.is_empty(), "{:?}", r.errors);
    }

    #[test]
    fn for_mit_offenem_ende_wird_abgelehnt() {
        let r =
            check("fn f() -> int {\n    for i in 0.. {\n        return i\n    }\n    return 0\n}");
        assert!(
            r.errors.iter().any(|e| e.contains("Start und Ende")),
            "{:?}",
            r.errors
        );
    }

    #[test]
    fn slice_index_wird_akzeptiert() {
        let r = check("fn f(xs: List<u8>) -> int {\n    let a = xs[0..2]\n    return 1\n}");
        assert!(r.errors.is_empty(), "{:?}", r.errors);
    }
}
