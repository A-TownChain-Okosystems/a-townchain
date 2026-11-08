// Security audit framework
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Severity { Critical, High, Medium, Low, Pass }

#[derive(Debug, Clone)]
pub struct AuditReport {
    pub category: String,
    pub severity: Severity,
    pub description: String,
    pub recommendation: String,
}

pub struct SecurityAuditor {
    checks: Vec<AuditReport>,
}

impl SecurityAuditor {
    pub fn new() -> Self { Self { checks: Vec::new() } }

    pub fn add_check(&mut self, category: &str, severity: Severity, desc: &str, rec: &str) {
        self.checks.push(AuditReport {
            category: category.to_string(),
            severity,
            description: desc.to_string(),
            recommendation: rec.to_string(),
        });
    }

    pub fn audit_chain_integrity(&mut self, block_count: u64, valid: bool) {
        let sev = if valid { Severity::Pass } else { Severity::Critical };
        self.add_check("Chain Integrity", sev,
            &format!("Checked {} blocks", block_count),
            if valid { "All blocks valid" } else { "Invalid blocks detected — immediate action required" });
    }

    pub fn audit_validators(&mut self, validator_count: usize, threshold_met: bool) {
        let sev = if threshold_met { Severity::Pass } else { Severity::High };
        self.add_check("Validator Security", sev,
            &format!("{} validators registered", validator_count),
            if threshold_met { "Threshold met" } else { "Insufficient validators — need 2/3 majority" });
    }

    pub fn report(&self) -> &Vec<AuditReport> { &self.checks }

    pub fn has_critical(&self) -> bool {
        self.checks.iter().any(|c| c.severity == Severity::Critical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_pass() {
        let mut auditor = SecurityAuditor::new();
        auditor.audit_chain_integrity(100, true);
        assert!(!auditor.has_critical());
    }

    #[test]
    fn test_audit_fail() {
        let mut auditor = SecurityAuditor::new();
        auditor.audit_chain_integrity(100, false);
        assert!(auditor.has_critical());
    }

    #[test]
    fn test_validator_check() {
        let mut auditor = SecurityAuditor::new();
        auditor.audit_validators(10, true);
        assert_eq!(auditor.report().len(), 1);
        assert_eq!(auditor.report()[0].severity, Severity::Pass);
    }
}
