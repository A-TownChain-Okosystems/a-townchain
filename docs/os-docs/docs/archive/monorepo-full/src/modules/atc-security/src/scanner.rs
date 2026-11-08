// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Vulnerability scanner
use std::collections::HashMap;

pub struct VulnerabilityScanner {
    findings: Vec<ScanResult>,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub id: String,
    pub vuln_type: String,
    pub severity: String,
    pub location: String,
    pub description: String,
}

impl VulnerabilityScanner {
    pub fn new() -> Self { Self { findings: Vec::new() } }

    pub fn scan_reentrancy(&mut self, contracts: &[String]) {
        for contract in contracts {
            if contract.contains("external_call") && contract.contains("state_change_after") {
                self.findings.push(ScanResult {
                    id: format!("REENT-{}", self.findings.len()),
                    vuln_type: "Reentrancy".to_string(),
                    severity: "Critical".to_string(),
                    location: contract.clone(),
                    description: "State change after external call detected".to_string(),
                });
            }
        }
    }

    pub fn scan_overflow(&mut self, values: &[u64]) {
        for (i, &v) in values.iter().enumerate() {
            if v > u64::MAX / 2 {
                self.findings.push(ScanResult {
                    id: format!("OVFL-{}", i),
                    vuln_type: "Integer Overflow".to_string(),
                    severity: "High".to_string(),
                    location: format!("value[{}]", i),
                    description: format!("Value {} approaching u64::MAX", v),
                });
            }
        }
    }

    pub fn findings(&self) -> &Vec<ScanResult> { &self.findings }
    pub fn critical_count(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == "Critical").count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_scan() {
        let mut s = VulnerabilityScanner::new();
        s.scan_reentrancy(&["safe_code".to_string()]);
        assert_eq!(s.findings().len(), 0);
    }

    #[test]
    fn test_overflow_detection() {
        let mut s = VulnerabilityScanner::new();
        s.scan_overflow(&[100, u64::MAX - 1]);
        assert_eq!(s.findings().len(), 1);
    }
}
