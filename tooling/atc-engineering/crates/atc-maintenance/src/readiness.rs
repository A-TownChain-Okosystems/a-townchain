//! MAINT-000 §7.2 — Maintenance Readiness Gate (freigegeben SCR-0124).
//!
//! Normative Kernregel: "No Readiness Record / No honest PASS = No Release".
//! Implementiert SCR-0123 Welle 1 (Readiness-Gate) nativ in der Control Plane:
//!   1. SCHEMA-Prüfung: 10 Pflichtfelder, state ∈ {complete, incomplete}, ref belegt.
//!   2. Pflicht-Abdeckung: Live-Release-Komponenten MÜSSEN einen Record haben.
//!   3. Fake-PASS-Erkennung: result=PASS nur zulässig wenn alle Felder complete.
//!      Ehrliches result=FAIL ist ein legitimer BLOCK-Zustand, kein Fehler.

use atc_core::{CoreError, Result};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

/// Die 10 Pflichtfelder eines Readiness-Records (MAINT-000 §7.2, Schema
/// registry/maintenance/readiness/*.yaml).
pub const REQUIRED_FIELDS: [&str; 10] = [
    "maintenance_owner",
    "maintenance_documentation",
    "dependency_inventory",
    "security_process",
    "test_suite",
    "rollback_strategy",
    "compatibility_strategy",
    "monitoring",
    "evidence_collection",
    "lifecycle_status",
];

/// Ein einzelnes Readiness-Feld.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub state: FieldState,
    pub ref_: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldState {
    Complete,
    Incomplete,
}

impl FieldState {
    pub fn parse(s: &str) -> Result<Self> {
        match s.trim() {
            "complete" => Ok(FieldState::Complete),
            "incomplete" => Ok(FieldState::Incomplete),
            other => Err(CoreError::InvalidState(format!(
                "readiness field state must be complete/incomplete, got '{other}'"
            ))),
        }
    }
}

/// Ein Readiness-Record (eine Komponente).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessRecord {
    /// Komponenten-Name (Datei-Stamm oder Registry-Name).
    pub component: String,
    pub fields: BTreeMap<String, Field>,
    /// result aus der Datei (PASS/FAIL); fehlt => None.
    pub declared_result: Option<bool>,
}

impl ReadinessRecord {
    /// Lädt und parst einen Readiness-Record (fail-closed, Block-YAML-Subset).
    pub fn load(component: &str, path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path).map_err(|e| {
            CoreError::ConfigInvalid(format!("cannot read {}: {e}", path.display()))
        })?;
        Self::parse(component, &raw)
    }

    /// Parst das Block-YAML-Subset:
    /// `maintenance_readiness:` → 2-Space `field:` → 4-Space `state:`/`ref:`;
    /// danach Top-Level `result: PASS|FAIL`.
    pub fn parse(component: &str, raw: &str) -> Result<Self> {
        let mut fields: BTreeMap<String, Field> = BTreeMap::new();
        let mut declared_result: Option<bool> = None;
        let mut current_field: Option<String> = None;
        let mut in_readiness = false;

        for line in raw.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            let indent = line.len() - line.trim_start().len();
            if indent == 0 {
                in_readiness = t.starts_with("maintenance_readiness:");
                if let Some(v) = t.strip_prefix("result:") {
                    declared_result = Some(match v.trim() {
                        "PASS" => true,
                        "FAIL" => false,
                        other => {
                            return Err(CoreError::ConfigInvalid(format!(
                                "result must be PASS/FAIL, got '{other}'"
                            )))
                        }
                    });
                }
                continue;
            }
            if in_readiness && indent == 2 && t.ends_with(':') {
                current_field = Some(t[..t.len() - 1].to_string());
                continue;
            }
            if let (Some(field), true) = (&current_field, indent >= 4) {
                if let Some(v) = t.strip_prefix("state:") {
                    let state = FieldState::parse(v)?;
                    let e = fields.entry(field.clone()).or_insert(Field {
                        state,
                        ref_: String::new(),
                    });
                    e.state = state;
                } else if let Some(v) = t.strip_prefix("ref:") {
                    let v = v.trim().trim_matches('"').to_string();
                    let e = fields.entry(field.clone()).or_insert(Field {
                        state: FieldState::Complete,
                        ref_: String::new(),
                    });
                    e.ref_ = v;
                }
            }
        }
        Ok(Self {
            component: component.to_string(),
            fields,
            declared_result,
        })
    }

    /// Prüft gegen die 10 Pflichtfelder; liefert die Fehlerliste (leer = schema-ok).
    pub fn schema_violations(&self) -> Vec<String> {
        let mut out = Vec::new();
        for f in REQUIRED_FIELDS {
            match self.fields.get(f) {
                None => out.push(format!("missing field '{f}'")),
                Some(field) => {
                    if field.ref_.trim().is_empty() {
                        out.push(format!("field '{f}': ref is empty"));
                    }
                }
            }
        }
        out
    }

    pub fn incomplete_fields(&self) -> Vec<String> {
        self.fields
            .iter()
            .filter(|(_, f)| f.state == FieldState::Incomplete)
            .map(|(k, _)| k.clone())
            .collect()
    }
}

/// Gate-Entscheidung: PASS oder BLOCK mit Begründungen (Evidence-First).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateOutcome {
    Pass(String),
    Block(Vec<String>),
}

impl GateOutcome {
    pub fn is_pass(&self) -> bool {
        matches!(self, GateOutcome::Pass(_))
    }
}

impl fmt::Display for GateOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GateOutcome::Pass(c) => write!(f, "PASS: {c}"),
            GateOutcome::Block(reasons) => {
                write!(f, "BLOCK ({}): {}", reasons.len(), reasons.join("; "))
            }
        }
    }
}

/// Wertet EINEN Record gegen §7.2 aus.
pub fn evaluate_record(rec: &ReadinessRecord) -> GateOutcome {
    let schema = rec.schema_violations();
    if !schema.is_empty() {
        return GateOutcome::Block(schema);
    }
    let incomplete = rec.incomplete_fields();
    match rec.declared_result {
        None => GateOutcome::Block(vec!["missing 'result' (PASS/FAIL)".into()]),
        Some(true) => {
            if incomplete.is_empty() {
                GateOutcome::Pass(rec.component.clone())
            } else {
                // Fake-PASS-Erkennung: PASS trotz incomplete-Feldern ist unehrlich.
                let mut reasons = vec![format!(
                    "FAKE-PASS: declared PASS but fields incomplete: {}",
                    incomplete.join(", ")
                )];
                reasons.push("result must be FAIL while gaps exist (honesty principle)".into());
                GateOutcome::Block(reasons)
            }
        }
        Some(false) => {
            // Ehrliches FAIL = legitimer BLOCK-Zustand.
            GateOutcome::Block(vec![format!(
                "declared FAIL (honest): next release BLOCKED until {} closed",
                incomplete.join(", ")
            )])
        }
    }
}

/// Wertet die Pflicht-Abdeckung: jede Live-Release-Komponente muss einen
/// (gültigen) Record haben. Fehlt ein Record => BLOCK.
pub fn evaluate_coverage(records: &[ReadinessRecord], live_components: &[&str]) -> GateOutcome {
    let mut reasons = Vec::new();
    for lc in live_components {
        if !records.iter().any(|r| &r.component == lc) {
            reasons.push(format!("live component '{lc}' has NO readiness record"));
        }
    }
    if reasons.is_empty() {
        GateOutcome::Pass(format!(
            "coverage ok: {}/{} live components recorded",
            live_components.len(),
            live_components.len()
        ))
    } else {
        GateOutcome::Block(reasons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PASS: &str = r#"
maintenance_readiness:
  maintenance_owner:
    state: complete
    ref: "team A"
  maintenance_documentation:
    state: complete
    ref: "docs"
  dependency_inventory:
    state: complete
    ref: "pins"
  security_process:
    state: complete
    ref: "codeql"
  test_suite:
    state: complete
    ref: "unit"
  rollback_strategy:
    state: complete
    ref: "runbook"
  compatibility_strategy:
    state: complete
    ref: "semver"
  monitoring:
    state: complete
    ref: "ci"
  evidence_collection:
    state: complete
    ref: "lock"
  lifecycle_status:
    state: complete
    ref: "operational"
result: PASS
"#;

    fn record_with(incomplete: &[&str], result: &str) -> String {
        let mut s = String::from("maintenance_readiness:\n");
        for f in REQUIRED_FIELDS {
            let state = if incomplete.contains(&f) {
                "incomplete"
            } else {
                "complete"
            };
            s.push_str(&format!(
                "  {f}:\n    state: {state}\n    ref: \"evidence-{f}\"\n"
            ));
        }
        s.push_str(&format!("result: {result}\n"));
        s
    }

    #[test]
    fn honest_pass_passes() {
        let rec = ReadinessRecord::parse("c", VALID_PASS).unwrap();
        assert!(evaluate_record(&rec).is_pass());
        assert_eq!(rec.schema_violations().len(), 0);
    }

    #[test]
    fn honest_fail_blocks_with_reasons() {
        let raw = record_with(&["security_process", "rollback_strategy"], "FAIL");
        let rec = ReadinessRecord::parse("c", &raw).unwrap();
        let out = evaluate_record(&rec);
        assert!(!out.is_pass());
        let s = out.to_string();
        assert!(s.contains("honest"));
        assert!(s.contains("security_process"));
    }

    #[test]
    fn fake_pass_is_detected() {
        let raw = record_with(&["rollback_strategy"], "PASS");
        let rec = ReadinessRecord::parse("c", &raw).unwrap();
        let out = evaluate_record(&rec);
        assert!(!out.is_pass());
        assert!(out.to_string().contains("FAKE-PASS"));
    }

    #[test]
    fn missing_result_blocks() {
        let raw = record_with(&[], "").replace("result: \n", "");
        let rec = ReadinessRecord::parse("c", &raw).unwrap();
        assert!(!evaluate_record(&rec).is_pass());
    }

    #[test]
    fn schema_violations_for_missing_fields() {
        let raw = "maintenance_readiness:\n  maintenance_owner:\n    state: complete\n    ref: x\n";
        let rec = ReadinessRecord::parse("c", raw).unwrap();
        let v = rec.schema_violations();
        assert_eq!(v.len(), 9); // 10 Pflichtfelder, 1 vorhanden
        assert!(v.iter().any(|x| x.contains("security_process")));
    }

    #[test]
    fn fails_closed_on_bad_state() {
        let raw = record_with(&[], "PASS").replace("state: complete", "state: maybe");
        assert!(ReadinessRecord::parse("c", &raw).is_err());
    }

    #[test]
    fn coverage_requires_records_for_live_components() {
        let rec = ReadinessRecord::parse("a", VALID_PASS).unwrap();
        let out = evaluate_coverage(&[rec], &["a", "b"]);
        assert!(!out.is_pass());
        assert!(out.to_string().contains("'b' has NO readiness record"));
    }
}
