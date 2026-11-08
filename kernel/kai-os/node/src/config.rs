//! Validierte, versionierte Konfiguration — fail-closed: jede ungültige,
//! fehlende oder außerhalb des Bereichs liegende Einstellung verweigert den Boot.

use serde::{Deserialize, Serialize};
use std::path::Path;

pub const KNOWN_SUBSYSTEMS: &[&str] = &[
    "consensus",
    "p2p",
    "state-sync",
    "ai-runtime",
    "storage",
    "security",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_name: String,
    pub data_dir: String,
    #[serde(default)]
    pub allow_root: bool,
    pub max_restarts: u32,
    pub restart_backoff_ms: u64,
    pub tick_interval_ms: u64,
    pub shutdown_timeout_ms: u64,
    pub enabled_subsystems: Vec<String>,
}

#[derive(Debug)]
pub enum ConfigError {
    Read(String),
    Parse(String),
    Validation(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Read(e) => write!(f, "config read failed: {e}"),
            ConfigError::Parse(e) => write!(f, "config parse failed: {e}"),
            ConfigError::Validation(e) => write!(f, "config validation failed: {e}"),
        }
    }
}

impl NodeConfig {
    /// Semantische Validierung (fail-closed, ATC-STD-ENG-001 REQ-ENG-006).
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.node_name.trim().is_empty() {
            return Err(ConfigError::Validation("node_name empty".into()));
        }
        if self.data_dir.trim().is_empty() {
            return Err(ConfigError::Validation("data_dir empty".into()));
        }
        if self.max_restarts < 1 || self.max_restarts > 10 {
            return Err(ConfigError::Validation(
                "max_restarts out of range [1,10]".into(),
            ));
        }
        if self.restart_backoff_ms > 60_000 {
            return Err(ConfigError::Validation("restart_backoff_ms > 60000".into()));
        }
        if self.tick_interval_ms == 0 {
            return Err(ConfigError::Validation(
                "tick_interval_ms must be > 0".into(),
            ));
        }
        if self.shutdown_timeout_ms == 0 {
            return Err(ConfigError::Validation(
                "shutdown_timeout_ms must be > 0".into(),
            ));
        }
        if self.enabled_subsystems.is_empty() {
            return Err(ConfigError::Validation("enabled_subsystems empty".into()));
        }
        for s in &self.enabled_subsystems {
            if !KNOWN_SUBSYSTEMS.contains(&s.as_str()) {
                return Err(ConfigError::Validation(format!("unknown subsystem '{s}'")));
            }
        }
        Ok(())
    }

    pub fn from_json(json: &str) -> Result<Self, ConfigError> {
        let cfg: NodeConfig =
            serde_json::from_str(json).map_err(|e| ConfigError::Parse(e.to_string()))?;
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let raw = std::fs::read_to_string(path).map_err(|e| ConfigError::Read(e.to_string()))?;
        Self::from_json(&raw)
    }

    /// Test-Konfiguration (deterministisch, keine Netz-/Zeitabhängigkeit).
    pub fn default_for_test() -> Self {
        Self {
            node_name: "test-node".into(),
            data_dir: "/tmp/kai-os-test".into(),
            allow_root: false,
            max_restarts: 3,
            restart_backoff_ms: 0,
            tick_interval_ms: 10,
            shutdown_timeout_ms: 1_000,
            enabled_subsystems: KNOWN_SUBSYSTEMS.iter().map(|s| s.to_string()).collect(),
        }
    }
}
