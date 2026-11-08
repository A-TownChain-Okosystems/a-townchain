//! Closed-loop engineering orchestrator.
//!
//! The orchestrator makes the engineering lifecycle executable:
//! discover -> document -> audit -> classify -> remediate -> update impacted
//! artifacts -> re-audit -> verify. It is fail-closed and never claims that a
//! finding is fixed until a later audit pass proves it absent.

use crate::audit::{audit_repository, AuditReport, Finding, FindingKind};
use crate::classification::MaintenanceClass;
use crate::repair::{repair_until_stable, RepairEvent};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineeringPhase {
    Discover,
    Document,
    Audit,
    Classify,
    Remediate,
    ImpactUpdate,
    ReAudit,
    Verify,
    Complete,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FindingRecord {
    pub iteration: usize,
    pub status: &'static str,
    pub kind: FindingKind,
    pub code: &'static str,
    pub maintenance_class: MaintenanceClass,
    pub path: Option<PathBuf>,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineeringPolicy {
    pub max_iterations: usize,
    pub write_evidence: bool,
}

impl Default for EngineeringPolicy {
    fn default() -> Self {
        Self {
            max_iterations: 8,
            write_evidence: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineeringResult {
    pub phase: EngineeringPhase,
    pub iterations: usize,
    pub history: Vec<FindingRecord>,
    pub repair_events: Vec<RepairEvent>,
    pub final_report: AuditReport,
    pub ready: bool,
}

/// Execute the closed-loop engineering lifecycle until the repository is
/// clean, progress stops, or the configured hard limit is reached.
pub fn engineer_until_clean(
    root: &Path,
    policy: &EngineeringPolicy,
) -> Result<EngineeringResult, io::Error> {
    let limit = policy.max_iterations.max(1);
    let mut history = Vec::new();
    let mut all_events = Vec::new();
    let mut last_signature = String::new();

    document_lifecycle(root)?;

    for iteration in 1..=limit {
        let report = audit_repository(root)?;
        record_findings(
            root,
            iteration,
            &report,
            &mut history,
            policy.write_evidence,
        )?;

        if report.is_clean() {
            verify_evidence(root, iteration, policy.write_evidence)?;
            return Ok(EngineeringResult {
                phase: EngineeringPhase::Complete,
                iterations: iteration,
                history,
                repair_events: all_events,
                final_report: report,
                ready: true,
            });
        }

        let signature = finding_signature(&report);
        if signature == last_signature {
            return blocked_result(iteration, history, all_events, report);
        }
        last_signature = signature.clone();

        let run = repair_until_stable(root, 1)?;
        all_events.extend(run.events.clone());

        // Impact-update is explicit evidence, even when the conservative
        // repair layer has no safe automatic change for a finding.
        record_impact_update(root, iteration, &report, policy.write_evidence)?;

        let after = audit_repository(root)?;
        record_resolution(root, iteration, &report, &after, policy.write_evidence)?;

        if after.is_clean() {
            verify_evidence(root, iteration, policy.write_evidence)?;
            return Ok(EngineeringResult {
                phase: EngineeringPhase::Complete,
                iterations: iteration,
                history,
                repair_events: all_events,
                final_report: after,
                ready: true,
            });
        }

        if finding_signature(&after) == signature {
            return blocked_result(iteration, history, all_events, after);
        }
    }

    let final_report = audit_repository(root)?;
    blocked_result(limit, history, all_events, final_report)
}

fn blocked_result(
    iterations: usize,
    history: Vec<FindingRecord>,
    repair_events: Vec<RepairEvent>,
    final_report: AuditReport,
) -> Result<EngineeringResult, io::Error> {
    Ok(EngineeringResult {
        phase: EngineeringPhase::Blocked,
        iterations,
        history,
        repair_events,
        final_report,
        ready: false,
    })
}

fn document_lifecycle(root: &Path) -> Result<(), io::Error> {
    let dir = root.join("docs/engineering");
    fs::create_dir_all(&dir)?;
    let lifecycle = dir.join("ENGINEERING-LIFECYCLE.md");
    if !lifecycle.exists() {
        fs::write(
            lifecycle,
            "# Engineering Lifecycle\n\nDISCOVER -> DOCUMENT -> AUDIT -> CLASSIFY -> REMEDIATE -> "
                .to_owned()
                + "IMPACT-UPDATE -> RE-AUDIT -> VERIFY -> COMPLETE/BLOCKED\n\n"
                + "A finding is closed only after a subsequent executable audit no longer reports it.\n",
        )?;
    }
    Ok(())
}

/// Deterministic MAINT-001 classification for the audit finding kinds.
/// Security findings are never downgraded below M2; unresolved findings remain
/// fail-closed until the later audit removes them.
fn classify(kind: FindingKind) -> MaintenanceClass {
    match kind {
        FindingKind::Security => MaintenanceClass::M2,
        FindingKind::Connectivity => MaintenanceClass::M1,
        FindingKind::Error => MaintenanceClass::M1,
        FindingKind::Consistency => MaintenanceClass::M0,
    }
}

fn record_findings(
    root: &Path,
    iteration: usize,
    report: &AuditReport,
    history: &mut Vec<FindingRecord>,
    enabled: bool,
) -> Result<(), io::Error> {
    for finding in &report.findings {
        history.push(FindingRecord {
            iteration,
            status: "DETECTED",
            kind: finding.kind,
            code: finding.code,
            maintenance_class: classify(finding.kind),
            path: finding.path.clone(),
            message: finding.message.clone(),
        });
    }
    if !enabled {
        return Ok(());
    }
    let path = root.join("docs/engineering/FINDINGS.md");
    let mut text = if path.exists() {
        fs::read_to_string(&path)?
    } else {
        "# Engineering Findings\n\n| Iteration | Status | Class | Kind | Code | Path | Finding |\n|---:|---|---|---|---|---|---|\n".into()
    };
    for finding in &report.findings {
        let file = finding
            .path
            .as_deref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "<repository>".into());
        text.push_str(&format!(
            "| {iteration} | DETECTED | {} | {:?} | `{}` | `{}` | {} |\n",
            classify(finding.kind),
            finding.kind,
            finding.code,
            file,
            finding.message.replace('|', "\\|")
        ));
    }
    fs::write(path, text)
}

fn record_impact_update(
    root: &Path,
    iteration: usize,
    report: &AuditReport,
    enabled: bool,
) -> Result<(), io::Error> {
    if !enabled {
        return Ok(());
    }
    let path = root.join("docs/engineering/IMPACT-MAP.md");
    let mut text = if path.exists() {
        fs::read_to_string(&path)?
    } else {
        "# Engineering Impact Map\n\n| Iteration | Finding | Primary Artifact | Impact Class | Required Re-Audit |\n|---:|---|---|---|---|\n".into()
    };
    for finding in &report.findings {
        let file = finding
            .path
            .as_deref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| Path::new("<repository>").display().to_string());
        let impact = match finding.kind {
            FindingKind::Security => "security + governance + CI",
            FindingKind::Connectivity => "dependency graph + build",
            FindingKind::Consistency => "source + documentation",
            FindingKind::Error => "source + tests",
        };
        text.push_str(&format!(
            "| {iteration} | `{}` | `{file}` | {impact} | yes |\n",
            finding.code
        ));
    }
    fs::write(path, text)
}

fn record_resolution(
    root: &Path,
    iteration: usize,
    before: &AuditReport,
    after: &AuditReport,
    enabled: bool,
) -> Result<(), io::Error> {
    if !enabled {
        return Ok(());
    }
    let path = root.join("docs/engineering/FINDINGS.md");
    let mut text = fs::read_to_string(&path).unwrap_or_default();
    for finding in &before.findings {
        if !after
            .findings
            .iter()
            .any(|current| same_finding(current, finding))
        {
            let file = finding
                .path
                .as_deref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "<repository>".into());
            text.push_str(&format!(
                "| {iteration} | VERIFIED | {} | {:?} | `{}` | `{file}` | closed by subsequent audit |\n",
                classify(finding.kind),
                finding.kind,
                finding.code
            ));
        }
    }
    fs::write(path, text)
}

fn verify_evidence(root: &Path, iteration: usize, enabled: bool) -> Result<(), io::Error> {
    if !enabled {
        return Ok(());
    }
    let path = root.join("docs/engineering/ENGINEERING-RESULT.md");
    fs::write(
        path,
        format!(
            "# Engineering Result\n\nStatus: `READY`\n\nFinal executable audit: clean\nFinal verification iteration: `{iteration}`\n\nReadiness is fail-closed and is valid only for this verified repository state.\n"
        ),
    )
}

fn same_finding(a: &Finding, b: &Finding) -> bool {
    a.kind == b.kind && a.code == b.code && a.path == b.path && a.message == b.message
}

fn finding_signature(report: &AuditReport) -> String {
    let mut items: Vec<String> = report
        .findings
        .iter()
        .map(|finding| {
            format!(
                "{:?}|{}|{}|{}",
                finding.kind,
                finding.code,
                finding.path.as_deref().unwrap_or(Path::new("")).display(),
                finding.message
            )
        })
        .collect();
    items.sort();
    items.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn classification_is_fail_closed_by_kind() {
        assert_eq!(classify(FindingKind::Security), MaintenanceClass::M2);
        assert_eq!(classify(FindingKind::Connectivity), MaintenanceClass::M1);
        assert_eq!(classify(FindingKind::Error), MaintenanceClass::M1);
        assert_eq!(classify(FindingKind::Consistency), MaintenanceClass::M0);
    }

    #[test]
    fn clean_repository_becomes_ready() {
        let root = std::env::temp_dir().join(format!(
            "atc-engineering-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("README.md"), "# demo repository\n").unwrap();
        fs::write(root.join("docs/ENGINEERING_AUDIT.md"), "# Audit\n").unwrap();
        let result = engineer_until_clean(&root, &EngineeringPolicy::default()).unwrap();
        assert!(result.ready);
        assert_eq!(result.phase, EngineeringPhase::Complete);
        let _ = fs::remove_dir_all(root);
    }
}
