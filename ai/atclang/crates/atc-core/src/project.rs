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
    /// Top-Level-Funktionen (nicht in einem Vertrag) mit Quelldatei.
    pub functions: BTreeMap<String, Vec<String>>,
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
                Stmt::Fn(f) => {
                    project
                        .functions
                        .entry(f.name.clone())
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
                Stmt::UseGroup { names, .. } => {
                    for name in names {
                        project
                            .imports
                            .entry(name.clone())
                            .or_default()
                            .push(path.clone());
                    }
                }
                Stmt::Import { source, alias } => {
                    // Ein Dateiimport bringt seinen Alias in den Namensraum;
                    // ohne Alias gilt der Dateistamm ("std/crypto.atc" -> "crypto").
                    let name = alias.clone().unwrap_or_else(|| {
                        source
                            .rsplit('/')
                            .next()
                            .unwrap_or(source)
                            .trim_end_matches(".atc")
                            .to_string()
                    });
                    if !name.is_empty() {
                        project.imports.entry(name).or_default().push(path.clone());
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

/// Kanonische Host-Funktionen (Krypto, Umgebung). Sie werden nicht in ATCLang
/// deklariert, sondern von der Runtime bereitgestellt; ein Import ist zulaessig.
const CANONICAL_FUNCS: &[&str] = &[
    "sha256", "keccak256", "blake2b", "msg_sender", "now", "self_address", "balance_of",
    "ed25519_verify", "secp256k1_verify", "verify_signature", "hash_concat",
];

impl Project {
    /// Ist der Typname projektweit deklariert (ohne Importe)?
    fn declared_type(&self, name: &str) -> bool {
        CANONICAL_TYPES.contains(&name)
            || self.structs.contains_key(name)
            || self.enums.contains_key(name)
            || self.contracts.contains_key(name)
    }

    /// Ist der Typname projektweit bekannt (deklariert oder importiert)?
    pub fn type_known(&self, name: &str) -> bool {
        let ty = ty_from_hint(&crate::ast::TypeHint {
            name: name.to_string(),
        });
        if !matches!(ty, Ty::Named(_)) {
            return true; // Builtin, Liste, Map, Tupel
        }
        self.declared_type(name) || self.imports.contains_key(name)
    }

    /// Ist ein importierbares Symbol bekannt (Typ, Top-Level-Funktion, Builtin)?
    pub fn symbol_known(&self, name: &str) -> bool {
        // Importe zaehlen hier bewusst nicht: sonst wuerde ein Tippfehler im
        // `use`-Pfad sich selbst als bekannt bestaetigen.
        self.functions.contains_key(name)
            || CANONICAL_FUNCS.contains(&name)
            || self.declared_type(name)
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

        // 4. Importe muessen auf ein bekanntes Symbol zeigen.
        for (path, items) in &self.items {
            for stmt in items {
                // Einzel- und Gruppenimport pruefen dieselbe Regel: der Pfad
                // beginnt bei der Wurzel ATC, und jedes eingefuehrte Symbol
                // muss deklariert oder kanonisches Builtin sein.
                let (namespace, symbols, written): (String, Vec<String>, String) = match stmt {
                    Stmt::Use { path: use_path } => {
                        let symbol = use_path.split("::").last().unwrap_or("").trim();
                        if symbol.is_empty() {
                            let warning = format!("{path}: leerer use-Pfad '{use_path}'");
                            report.warnings.push(warning);
                            continue;
                        }
                        (
                            use_path
                                .rsplit_once("::")
                                .map(|(ns, _)| ns)
                                .unwrap_or("")
                                .to_string(),
                            vec![symbol.to_string()],
                            use_path.clone(),
                        )
                    }
                    Stmt::UseGroup {
                        path: group_path,
                        names,
                    } => (
                        group_path.trim().to_string(),
                        names.clone(),
                        format!("{group_path}::{{{}}}", names.join(", ")),
                    ),
                    _ => continue,
                };
                // ATCLang hat genau eine Wurzel-Namespace: ATC.
                if !namespace.is_empty() && namespace.split("::").next() != Some("ATC") {
                    report.warnings.push(format!(
                        "{path}: use {written}: Wurzel-Namespace muss 'ATC' sein"
                    ));
                }
                for symbol in &symbols {
                    if !self.symbol_known(symbol) {
                        report.warnings.push(format!(
                            "{path}: use {written}: Symbol '{symbol}' ist weder deklariert noch kanonisches Builtin (Namespace '{namespace}' wird nicht aufgeloest)"
                        ));
                        continue;
                    }
                    // Import, der eine lokale Deklaration derselben Datei verdeckt.
                    let local = self
                        .contracts
                        .get(symbol)
                        .map(|e| e.iter().any(|x| &x.path == path))
                        .unwrap_or(false)
                        || self
                            .structs
                            .get(symbol)
                            .map(|v| v.iter().any(|x| x == path))
                            .unwrap_or(false)
                        || self
                            .enums
                            .get(symbol)
                            .map(|v| v.iter().any(|x| x == path))
                            .unwrap_or(false)
                        || self
                            .functions
                            .get(symbol)
                            .map(|v| v.iter().any(|x| x == path))
                            .unwrap_or(false);
                    if local {
                        report.warnings.push(format!(
                            "{path}: use {written} importiert '{symbol}', das dieselbe Datei bereits deklariert"
                        ));
                    }
                }
            }
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
    for name in hint.name.split(['<', '>', ',', '(', ')']) {
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
    fn unbekanntes_importsymbol_wird_gemeldet() {
        let project = index(&[(
            "a.atc",
            "use ATC::Types::Adress\ncontract T { state a: Adress }",
        )]);
        let r = project.check();
        assert!(
            r.warnings.iter().any(|w| w.contains("Symbol 'Adress'")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn fremde_wurzel_namespace_wird_gemeldet() {
        let project = index(&[(
            "a.atc",
            "use EVM::Types::Address\ncontract T { state a: Address }",
        )]);
        let r = project.check();
        assert!(
            r.warnings.iter().any(|w| w.contains("Wurzel-Namespace")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn kanonische_importe_loesen_auf() {
        let project = index(&[(
            "a.atc",
            "use ATC::Types::Address\nuse ATC::Crypto::sha256\ncontract T { state a: Address }",
        )]);
        let r = project.check();
        assert!(
            !r.warnings.iter().any(|w| w.contains("Symbol '")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn import_aus_anderer_datei_loest_auf() {
        let project = index(&[
            ("types.atc", "struct Verdict { ok: bool }"),
            ("a.atc", "use ATC::Types::Verdict\ncontract T { fn f(v: Verdict) -> bool { return v.ok; } }"),
        ]);
        let r = project.check();
        assert!(
            !r.warnings.iter().any(|w| w.contains("Symbol '")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn import_der_eigene_deklaration_verdeckt_wird_gemeldet() {
        let project = index(&[(
            "a.atc",
            "use ATC::Types::Address\nstruct Address { raw: u64 }",
        )]);
        let r = project.check();
        assert!(
            r.warnings.iter().any(|w| w.contains("bereits deklariert")),
            "{:?}",
            r.warnings
        );
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

    #[test]
    fn gruppenimport_bringt_jeden_namen_in_den_namensraum() {
        let project = index(&[
            ("a.atc", "struct Socket { a: u64 }"),
            ("b.atc", "use ATC::Net::{Socket}"),
        ]);
        assert!(project.imports.contains_key("Socket"));
        let r = project.check();
        assert!(
            !r.warnings.iter().any(|w| w.contains("Socket")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn gruppenimport_meldet_fremden_wurzel_namespace() {
        let project = index(&[("a.atc", "use FOO::Bar::{Socket}")]);
        let r = project.check();
        assert!(
            r.warnings
                .iter()
                .any(|w| w.contains("Wurzel-Namespace muss 'ATC' sein")),
            "{:?}",
            r.warnings
        );
    }

    #[test]
    fn dateiimport_alias_wird_importname() {
        let project = index(&[("a.atc", "import \"std/crypto.atc\" as Crypto")]);
        assert!(project.imports.contains_key("Crypto"));
    }

    #[test]
    fn dateiimport_ohne_alias_nutzt_dateistamm() {
        let project = index(&[("a.atc", "import \"std/crypto.atc\"")]);
        assert!(project.imports.contains_key("crypto"));
    }
}
