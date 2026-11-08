// PROVENANZ (Org-Adoption 13.09.2026): Quelle: privates Repo ShivaCoreDev/A-TownChain--kosystems- (05.07.2026). Archiv-Referenz (nicht buildbar gepflegt).

#![no_std]
#![no_main]

use shiva_agent::{agent_task, ShivaAgent, AgentContext};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

// ===================================================
// 1. Task-Definitionen
// ===================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityTask {
    ScanPorts { target: String, ports: Vec<u16> },
    DetectAnomalies { events: Vec<SystemEvent> },
    CheckVulnerabilities { services: Vec<String> },
    MonitorNetwork { interface: String, duration: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub timestamp: u64,
    pub event_type: String,
    pub source: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityResult {
    PortScan { open_ports: Vec<u16>, closed_ports: Vec<u16> },
    Anomalies { detected: Vec<Anomaly>, count: usize },
    Vulnerabilities { found: Vec<Vulnerability>, count: usize },
    NetworkStatus { packets: u64, threats: u64, status: String },
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub event_type: String,
    pub severity: u8,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub service: String,
    pub cve: Option<String>,
    pub severity: u8,
    pub description: String,
}

// ===================================================
// 2. Agent-Implementierung
// ===================================================
#[derive(Default)]
pub struct SecurityAgent {
    threat_patterns: BTreeMap<String, Vec<String>>,
}

#[shiva_agent]
impl SecurityAgent {
    pub fn new() -> Self {
        let mut patterns = BTreeMap::new();
        patterns.insert("bruteforce".to_string(), vec![
            "failed_login".to_string(),
            "repeated_attempt".to_string(),
        ]);
        patterns.insert("malware".to_string(), vec![
            "suspicious_process".to_string(),
            "unknown_service".to_string(),
        ]);
        SecurityAgent { threat_patterns: patterns }
    }

    #[agent_task("scan_ports")]
    pub fn scan_ports(&self, target: String, ports: Vec<u16>) -> SecurityResult {
        // Simulierte Port-Scan (in der Realität: echte Netzwerk-Anfragen)
        let mut open_ports = Vec::new();
        let mut closed_ports = Vec::new();
        for port in ports {
            if port % 2 == 0 { // Demo: gerade Ports sind "offen"
                open_ports.push(port);
            } else {
                closed_ports.push(port);
            }
        }
        SecurityResult::PortScan { open_ports, closed_ports }
    }

    #[agent_task("detect_anomalies")]
    pub fn detect_anomalies(&self, events: Vec<SystemEvent>) -> SecurityResult {
        let mut detected = Vec::new();
        for event in events {
            // Prüfe auf bekannte Threat-Patterns
            for (threat_type, patterns) in &self.threat_patterns {
                if patterns.iter().any(|p| event.event_type.contains(p)) {
                    detected.push(Anomaly {
                        event_type: event.event_type.clone(),
                        severity: 7,
                        description: format!("Potenzielle {} Attacke erkannt", threat_type),
                    });
                }
            }
            // Zusätzlich: Ungewöhnliche Events
            if event.event_type == "unexpected_packet" {
                detected.push(Anomaly {
                    event_type: "unexpected_packet".to_string(),
                    severity: 5,
                    description: "Ungewöhnliches Netzwerkpaket".to_string(),
                });
            }
        }
        SecurityResult::Anomalies { detected, count: detected.len() }
    }

    #[agent_task("check_vulnerabilities")]
    pub fn check_vulnerabilities(&self, services: Vec<String>) -> SecurityResult {
        let mut found = Vec::new();
        for service in services {
            if service.contains("nginx") {
                found.push(Vulnerability {
                    service: service.clone(),
                    cve: Some("CVE-2024-1234".to_string()),
                    severity: 6,
                    description: "Ältere Nginx-Version mit bekannten Schwachstellen".to_string(),
                });
            } else if service.contains("mysql") {
                found.push(Vulnerability {
                    service: service.clone(),
                    cve: Some("CVE-2024-5678".to_string()),
                    severity: 8,
                    description: "MySQL ohne aktuelle Sicherheitspatches".to_string(),
                });
            }
        }
        SecurityResult::Vulnerabilities { found, count: found.len() }
    }

    pub fn init(&mut self, ctx: &AgentContext) -> Result<(), String> {
        log::info!("🔒 Security-Agent initialisiert mit ID: {}", ctx.agent_id);
        Ok(())
    }

    pub fn shutdown(&self) {
        log::info!("🔒 Security-Agent wird beendet.");
    }
}

// ===================================================
// 4. WASM-Exporte
// ===================================================
#[wasm_bindgen]
pub fn process_task(task_json: &str) -> String {
    let agent = SecurityAgent::new();
    let task: SecurityTask = serde_json::from_str(task_json).unwrap();
    let result = match task {
        SecurityTask::ScanPorts { target, ports } => agent.scan_ports(target, ports),
        SecurityTask::DetectAnomalies { events } => agent.detect_anomalies(events),
        SecurityTask::CheckVulnerabilities { services } => agent.check_vulnerabilities(services),
        SecurityTask::MonitorNetwork { interface, duration } => {
            SecurityResult::NetworkStatus {
                packets: duration * 1000,
                threats: duration / 100,
                status: "Normal".to_string(),
            }
        }
    };
    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap();
    }
}
