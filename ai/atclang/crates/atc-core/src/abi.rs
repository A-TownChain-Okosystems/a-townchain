// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Kanonische Vertrags-ABI (Semantik Stufe 4, Roadmap G7).
//!
//! Erzeugt aus einer geparsten Quelle eine deterministische Beschreibung der
//! Vertragsschnittstelle: Zustandsfelder, Events und Funktionssignaturen.
//! Die Serialisierung ist absichtlich von Hand geschrieben und stabil sortiert,
//! damit derselbe Vertrag immer dieselbe ABI und denselben Hash ergibt —
//! Grundlage fuer Registry-Bindung (ATC-STD-SC-019) und spaeteres Codegen.
//!
//! Was die ABI *nicht* ist: kein Bytecode. Sie beschreibt die Schnittstelle,
//! nicht die Ausfuehrung; Codegen bleibt G11.

use crate::ast::{Program, Stmt, TypeHint};
use sha2::{Digest, Sha256};

/// Ein Zustandsfeld oder ein Funktions-/Event-Parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub ty: String,
    /// Nur bei Zustandsfeldern: gibt es einen Vorgabewert?
    pub default: Option<bool>,
}

/// Ein Event mit seinen Feldern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventAbi {
    pub name: String,
    pub fields: Vec<Field>,
}

/// Eine Funktionssignatur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnAbi {
    pub name: String,
    pub params: Vec<Field>,
    pub ret: Option<String>,
    /// `true` fuer `native fn`, dessen Koerper ausserhalb des ATCLang-Subsets liegt.
    pub native: bool,
    /// Tokenweise Signaturtext nativer Funktionen (nur wenn `native`).
    pub signature: Option<String>,
}

/// Die Schnittstelle eines Vertrags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractAbi {
    pub contract: String,
    pub bases: Vec<String>,
    pub state: Vec<Field>,
    pub events: Vec<EventAbi>,
    pub functions: Vec<FnAbi>,
}

fn type_name(hint: &TypeHint) -> String {
    hint.name.trim().to_string()
}

impl ContractAbi {
    /// Kanonisches JSON: feste Schluesselreihenfolge, sortierte Symbole.
    pub fn to_json(&self) -> String {
        let mut state = self.state.clone();
        state.sort_by(|a, b| a.name.cmp(&b.name));
        let mut events = self.events.clone();
        events.sort_by(|a, b| a.name.cmp(&b.name));
        let mut functions = self.functions.clone();
        functions.sort_by(|a, b| a.name.cmp(&b.name));

        let mut out = String::from("{\"contract\":");
        out.push_str(&json_str(&self.contract));
        out.push_str(",\"bases\":[");
        let mut bases = self.bases.clone();
        bases.sort();
        for (i, b) in bases.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(&json_str(b));
        }
        out.push_str("],\"state\":[");
        for (i, f) in state.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(&state_json(f));
        }
        out.push_str("],\"events\":[");
        for (i, e) in events.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str("{\"name\":");
            out.push_str(&json_str(&e.name));
            out.push_str(",\"fields\":[");
            for (j, f) in e.fields.iter().enumerate() {
                if j > 0 {
                    out.push(',');
                }
                out.push_str(&param_json(f));
            }
            out.push_str("]}");
        }
        out.push_str("],\"functions\":[");
        for (i, f) in functions.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str("{\"name\":");
            out.push_str(&json_str(&f.name));
            out.push_str(",\"params\":[");
            for (j, p) in f.params.iter().enumerate() {
                if j > 0 {
                    out.push(',');
                }
                out.push_str(&param_json(p));
            }
            out.push_str("],\"returns\":");
            match &f.ret {
                Some(r) => out.push_str(&json_str(r)),
                None => out.push_str("null"),
            }
            out.push_str(",\"native\":");
            out.push_str(if f.native { "true" } else { "false" });
            if let Some(sig) = &f.signature {
                out.push_str(",\"signature\":");
                out.push_str(&json_str(sig));
            }
            out.push('}');
        }
        out.push_str("]}");
        out
    }

    /// SHA-256 ueber das kanonische JSON (hex, klein geschrieben).
    pub fn sha256(&self) -> String {
        let digest = Sha256::digest(self.to_json().as_bytes());
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }
}

fn state_json(f: &Field) -> String {
    let mut out = String::from("{\"name\":");
    out.push_str(&json_str(&f.name));
    out.push_str(",\"type\":");
    out.push_str(&json_str(&f.ty));
    if let Some(default) = f.default {
        out.push_str(",\"default\":");
        out.push_str(if default { "true" } else { "false" });
    }
    out.push('}');
    out
}

fn param_json(f: &Field) -> String {
    let mut out = String::from("{\"name\":");
    out.push_str(&json_str(&f.name));
    out.push_str(",\"type\":");
    out.push_str(&json_str(&f.ty));
    out.push('}');
    out
}

fn json_str(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Baut die ABI eines einzelnen Vertrags aus seinem Koerper.
pub fn from_body(contract: &str, bases: &[String], body: &[Stmt]) -> ContractAbi {
    let mut abi = ContractAbi {
        contract: contract.to_string(),
        bases: bases.to_vec(),
        state: Vec::new(),
        events: Vec::new(),
        functions: Vec::new(),
    };
    for item in body {
        match item {
            Stmt::State { name, ty, default } => abi.state.push(Field {
                name: name.clone(),
                ty: type_name(ty),
                default: Some(default.is_some()),
            }),
            Stmt::StateBlock { fields } => {
                for (name, ty, default) in fields {
                    abi.state.push(Field {
                        name: name.clone(),
                        ty: type_name(ty),
                        default: Some(default.is_some()),
                    });
                }
            }
            Stmt::Event { name, fields } => abi.events.push(EventAbi {
                name: name.clone(),
                fields: fields
                    .iter()
                    .map(|(n, t)| Field {
                        name: n.clone(),
                        ty: type_name(t),
                        default: None,
                    })
                    .collect(),
            }),
            Stmt::Fn(f) => abi.functions.push(FnAbi {
                name: f.name.clone(),
                params: f
                    .params
                    .iter()
                    .map(|p| Field {
                        name: p.name.clone(),
                        ty: type_name(&p.type_hint),
                        default: None,
                    })
                    .collect(),
                ret: f.return_type.as_ref().map(type_name),
                native: false,
                signature: None,
            }),
            Stmt::NativeFunction {
                name,
                signature_tokens,
                ..
            } => abi.functions.push(FnAbi {
                name: name.clone(),
                params: Vec::new(),
                ret: None,
                native: true,
                signature: Some(signature_tokens.join(" ")),
            }),
            _ => {}
        }
    }
    abi
}

/// Sammel-Digest einer Quelldatei: SHA-256 ueber die kanonischen ABI-JSONs
/// aller Vertraege, sortiert und mit Zeilenumbruch getrennt. Damit bindet eine
/// Registrierung die Schnittstelle der ganzen Quelle, ohne Vertragsnamen zu
/// duplizieren.
pub fn set_digest(abis: &[ContractAbi]) -> String {
    let mut parts: Vec<String> = abis.iter().map(|a| a.to_json()).collect();
    parts.sort();
    let joined = parts.join("\n");
    let digest = Sha256::digest(joined.as_bytes());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Alle Vertrags-ABIs eines Programms, sortiert nach Vertragsname.
pub fn from_program(program: &Program) -> Vec<ContractAbi> {
    let mut out = Vec::new();
    for stmt in &program.statements {
        if let Stmt::Contract { name, bases, body } = stmt {
            out.push(from_body(name, bases, body));
        }
    }
    out.sort_by(|a, b| a.contract.cmp(&b.contract));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    fn abis(src: &str) -> Vec<ContractAbi> {
        from_program(&parse_program(src).expect("Parse-Fehler"))
    }

    #[test]
    fn abi_enthaelt_zustand_events_und_funktionen() {
        let a = abis(
            r#"contract Token : Base {
                state owner: Address
                state supply: u256 = 0
                event Transfer(from: Address, to: Address, value: u256)
                fn transfer(to: Address, value: u256) -> bool { return true; }
                fn total() -> u256 { return self.supply; }
            }"#,
        );
        assert_eq!(a.len(), 1);
        let abi = &a[0];
        assert_eq!(abi.contract, "Token");
        assert_eq!(abi.bases, vec!["Base".to_string()]);
        assert_eq!(abi.state.len(), 2);
        assert_eq!(abi.events[0].fields.len(), 3);
        let transfer = abi
            .functions
            .iter()
            .find(|f| f.name == "transfer")
            .expect("transfer fehlt");
        assert_eq!(transfer.params.len(), 2);
        assert_eq!(transfer.ret.as_deref(), Some("bool"));
        assert!(!transfer.native);
    }

    #[test]
    fn abi_json_und_hash_sind_deterministisch() {
        let src = "contract T { state a: int fn f() -> int { return 1; } }";
        let first = abis(src);
        let second = abis(src);
        assert_eq!(first[0].to_json(), second[0].to_json());
        assert_eq!(first[0].sha256(), second[0].sha256());
        assert_eq!(first[0].sha256().len(), 64);
    }

    #[test]
    fn hash_aendert_sich_mit_der_signatur() {
        let a = abis("contract T { fn f() -> int { return 1; } }");
        let b = abis("contract T { fn f() -> bool { return true; } }");
        assert_ne!(a[0].sha256(), b[0].sha256());
    }

    #[test]
    fn reihenfolge_der_deklaration_aendert_den_hash_nicht() {
        let a = abis("contract T { state x: int state y: int fn f() -> int { return 1; } fn g() -> int { return 2; } }");
        let b = abis("contract T { fn g() -> int { return 2; } state y: int fn f() -> int { return 1; } state x: int }");
        assert_eq!(a[0].to_json(), b[0].to_json());
        assert_eq!(a[0].sha256(), b[0].sha256());
    }

    #[test]
    fn sammel_digest_ist_reihenfolge_unabhaengig() {
        let a = abis("contract A { fn f() -> int { return 1; } } contract B { state x: int }");
        let b = abis("contract B { state x: int } contract A { fn f() -> int { return 1; } }");
        assert_eq!(set_digest(&a), set_digest(&b));
        assert_eq!(set_digest(&a).len(), 64);
    }

    #[test]
    fn sammel_digest_aendert_sich_mit_jedem_vertrag() {
        let one = abis("contract A { fn f() -> int { return 1; } }");
        let two = abis("contract A { fn f() -> int { return 1; } } contract B { state x: int }");
        assert_ne!(set_digest(&one), set_digest(&two));
    }

    #[test]
    fn native_funktionen_werden_als_solche_gefuehrt() {
        // Der NativeFunction-Knoten entsteht im Parser nur fuer Koerper ausserhalb
        // des Subsets; hier direkt als AST-Knoten geprueft.
        let body = vec![Stmt::NativeFunction {
            name: "abs".to_string(),
            signature_tokens: vec!["(".to_string(), "x".to_string(), ":".to_string()],
            body_tokens: vec![],
        }];
        let abi = from_body("T", &[], &body);
        let f = abi
            .functions
            .iter()
            .find(|f| f.name == "abs")
            .expect("abs fehlt");
        assert!(f.native);
        assert_eq!(f.signature.as_deref(), Some("( x :"));
        assert!(abi.to_json().contains("\"native\":true"), "{}", abi.to_json());
    }

    #[test]
    fn json_wird_korrekt_maskiert() {
        let mut abi = abis("contract T { }").remove(0);
        abi.contract = "T\"quote\\back".to_string();
        let json = abi.to_json();
        assert!(json.contains("\\\"quote\\\\back"), "{json}");
    }
}
