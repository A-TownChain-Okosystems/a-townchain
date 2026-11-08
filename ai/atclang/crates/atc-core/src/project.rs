// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Projektweite Aufloesung ueber Dateigrenzen (Semantik Stufe 3).
//!
//! Einzeldatei-Pruefungen (`semantics`, `types`) koennen nur sehen, was in einer
//! Datei steht. Dieses Modul indexiert mehrere Quellen und beantwortet:
//!
//! - ist ein Vertragsname projektweit eindeutig (oder ein Mirror-Duplikat),
//! - loest jede Basis (`contract A : B`) auf einen bekannten Vertrag auf,
//! - ist jeder verwendete Typname bekannt (Builtin, deklariert oder importiert),
//!
//! Bewusst konservativ: gemeldet wird nur, was gegen den *Projektindex*
//! nachweislich fehlt. Externe Namespaces (etwa eine Registry ausserhalb des
//! Verzeichnisses) werden als Warnung gefuehrt, nicht als Fehler.

use crate::ast::{Program, Stmt};
use crate::types::{ty_from_hint, Ty};
use std::collections::BTreeMap;

/// Ergebnis der projektweiten Pruefung.
#[derive(Debug, Default)]
pub struct Report {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContractEntry {
    pub name: String,
    pub path: String,
    pub bases: Vec<String>,
}

/// Projektindex ueber alle geladenen Quellen.
#[derive(Debug, Default)]
pub struct Project {
    /// Vertragsname -> Vorkommen (mehr als eins = Mirror/Duplikat).
    pub contracts: BTreeMap<String, Vec<ContractEntry>>,
    /// Deklarierte Typnamen (Structs, Enums) mit Quelldatei.
    pub structs: BTreeMap<String, Vec<String>>,
    pub enums: BTreeMap<String, Vec<String>>,
    /// Importierte Namen (letztes Segment eines `use`-Pfads).
    pub imports: BTreeMap<String, Vec<String>>,
    /// Dateien ohne parsebare Quelle.
    pub broken: Vec<(String, String)>,
    /// Top-Level-Items je Datei (fuer die Typnamens-Pruefung).
    items: Vec<(String, Vec<Stmt>)>,
}

/// Baut den Index aus (Dateiname, geparstes Programm)-Paaren.
pub fn build(files: &[(String, Program)]) -> Project {
    let mut project = Project::default();
    for (path, program) in files {
        project
            .items
            .push((path.clone(), program.statements.clone()));
        for stmt in &program.statements {
            match stmt {
                Stmt::Contract { name, bases, .. } => {
                    project
                        .contracts
                        .entry(name.clone())
                        .or_default()
                        .push(ContractEntry {
                            name: name.clone(),
                            path: path.clone(),
                            bases: bases.clone(),
                        });
                }
                Stmt::Struct { name, .. } => {
                    project
                        .structs
                        .entry(name.clone())
                        .or_default()
                        .push(path.clone());
                }
                Stmt::Enum { name, .. } => {
                    project
                        .enums
                        .entry(name.clone())
                        .or_default()
                        .push(path.clone());
                }
                Stmt::Use { path: use_path } => {
                    if let Some(last) = use_path.split("::").last() {
                        let last = last.trim();
                        if !last.is_empty() {
                            project
                                .imports
                                .entry(last.to_string())
                                .or_default()
                                .push(path.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }
    project
}

/// Kanonische Sprach-Builtins der Kette (WAL-Typen, keine Projektdeklaration).
const CANONICAL_TYPES: &[&str] = &[
    "Address", "Hash", "Bytes", "Signature", "PublicKey", "Timestamp", "Amount", "TokenId",
];

impl Project {
    /// Ist der Typname projektweit bekannt?
    pub fn type_known(&self, name: &str) -> bool {
        let ty = ty_from_hint(&crate::ast::TypeHint {
            name: name.to_string(),
        });
        if !matches!(ty, Ty::Named(_)) {
            return true; // Builtin, Liste, Map, Tupel
        }
        CANONICAL_TYPES.contains(&name)
            || self.structs.contains_key(name)
            || self.enums.contains_key(name)
            || self.imports.contains_key(name)
            || self.contracts.contains_key(name)
    }

    /// Projektweite Befunde.
    pub fn check(&self) -> Report {
        let mut report = Report::default();

        // 1. Vertragsnamen muessen projektweit eindeutig sein.
        for (name, entries) in &self.contracts {
            if entries.len() > 1 {
                let mut paths: Vec<&str> =
                    entries.iter().map(|e| e.path.as_str()).collect();
                paths.sort_unstable();
                report.warnings.push(format!(
                    "contract {name} ist {}x im Projekt deklariert ({}) — pro Datei geprueft, projektweit mehrdeutig",
                    entries.len(),
                    paths.join(", ")
                ));
            }
        }

        // 2. Basen muessen aufloesbar sein.
        for (name, entries) in &self.contracts {
            for entry in entries {
                for base in &entry.bases {
                    if !self.contracts.contains_key(base) {
                        report.warnings.push(format!(
                            "contract {name} ({}) erbt von '{base}', das im Projektindex nicht deklariert ist",
                            entry.path
                        ));
                    }
                }
            }
        }

        // 3. Typnamen in Signaturen muessen bekannt sein (je Datei und Name einmal).
        let mut unknown: BTreeMap<(String, String), usize> = BTreeMap::new();
        for (path, items) in &self.items {
            for stmt in items {
                for ty_name in referenced_types(stmt) {
                    if !self.type_known(&ty_name) {
                        *unknown.entry((path.clone(), ty_name)).or_insert(0) += 1;
                    }
                }
            }
        }
        for ((path, ty_name), count) in unknown {
            report.warnings.push(format!(
                "{path}: Typname '{ty_name}' ist weder Builtin noch deklariert noch importiert ({count}x)"
            ));
        }

        report
    }

}

/// Sammelt alle Typnamen, die eine Deklaration referenziert.
pub fn referenced_types(stmt: &Stmt) -> Vec<String> {
    let mut out = Vec::new();
    collect_types(stmt, &mut out);
    out
}

fn push_hint(hint: &crate::ast::TypeHint, out: &mut Vec<String>) {
    for name in hint.name.split(|c| c == '<' || c == '>' || c == ',' || c == '(' || c == ')') {
        let name = name.trim();
        if !name.is_empty() && !name.chars().all(|c| c.is_ascii_digit()) {
            out.push(name.to_string());
        }
    }
}

fn collect_types(stmt: &Stmt, out: &mut Vec<String>) {
    match stmt {
        Stmt::State { ty, .. } => push_hint(ty, out),
        Stmt::StateBlock { fields } => {
            for (_, ty, _) in fields {
                push_hint(ty, out);
            }
        }
        Stmt::Event { fields, .. } => {
            for (_, ty) in fields {
                push_hint(ty, out);
            }
        }
        Stmt::Struct { fields, .. } => {
            for (_, ty) in fields {
                push_hint(ty, out);
            }
        }
        Stmt::Fn(f) => {
            for p in &f.params {
                push_hint(&p.type_hint, out);
            }
            if let Some(r) = &f.return_type {
                push_hint(r, out);
            }
        }
        Stmt::Contract { body, .. } => {
            for item in body {
                collect_types(item, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    fn index(files: &[(&str, &str)]) -> Project {
        let parsed: Vec<(String, Program)> = files
            .iter()
            .map(|(p, src)| {
                (
                    p.to_string(),
                    parse_program(src).expect("Parse-Fehler im Testfall"),
                )
            })
            .collect();
        build(&parsed)
    }

    #[test]
    fn doppelter_vertragsname_wird_gemeldet() {
        let project = index(&[
            ("a.atc", "contract T { fn f() -> int { return 1; } }"),
            ("b.atc", "contract T { fn f() -> int { return 2; } }"),
        ]);
        let r = project.check();
        assert!(
            r.warnings.iter().any(|w| w.contains("mehrdeutig")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn unbekannte_basis_wird_gemeldet() {
        let project = index(&[("a.atc", "contract T : Fehlt { fn f() -> int { return 1; } }")]);
        let r = project.check();
        assert!(
            r.warnings.iter().any(|w| w.contains("erbt von 'Fehlt'")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn basis_aus_anderer_datei_loest_auf() {
        let project = index(&[
            ("base.atc", "contract Base { state owner: Address }"),
            ("a.atc", "contract T : Base { fn f() -> int { return 1; } }"),
        ]);
        let r = project.check();
        assert!(
            !r.warnings.iter().any(|w| w.contains("erbt von")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn typname_aus_anderer_datei_ist_bekannt() {
        let project = index(&[
            ("types.atc", "struct Proposal { id: u64 }"),
            (
                "a.atc",
                "contract T { fn f(p: Proposal) -> int { return 1; } }",
            ),
        ]);
        let r = project.check();
        assert!(
            !r.warnings.iter().any(|w| w.contains("Typname")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn unbekannter_typname_wird_einmal_je_datei_gemeldet() {
        let project = index(&[(
            "a.atc",
            "contract T { state a: bytes32 state b: bytes32 fn f(x: bytes32) -> int { return 1; } }",
        )]);
        let r = project.check();
        let hits: Vec<&String> = r
            .warnings
            .iter()
            .filter(|w| w.contains("Typname 'bytes32'"))
            .collect();
        assert_eq!(hits.len(), 1, "{:?}", r.warnings);
        assert!(hits[0].contains("3x"), "{:?}", hits);
    }

    #[test]
    fn kanonische_und_importierte_typen_sind_bekannt() {
        let project = index(&[(
            "a.atc",
            "use ATC::Types::Verdict\ncontract T { state a: Address state b: Hash fn f(v: Verdict) -> int { return 1; } }",
        )]);
        let r = project.check();
        assert!(
            !r.warnings.iter().any(|w| w.contains("Typname")),
            "{:?}",
            r.warnings
        );
    }
}
