// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Config Manager — manages Aurora configuration
use std::collections::HashMap;

pub struct ConfigManager {
    config: HashMap<String, String>,
}

impl ConfigManager {
    pub fn new() -> Self {
        let mut cm = Self { config: HashMap::new() };
        cm.set("version", "1.0.0");
        cm.set("max_agents", "12");
        cm.set("default_model", "shiva-1.0");
        cm.set("context_window", "8192");
        cm
    }

    pub fn set(&mut self, key: &str, value: &str) { self.config.insert(key.into(), value.into()); }
    pub fn get(&self, key: &str) -> Option<&String> { self.config.get(key) }
    pub fn get_or_default(&self, key: &str, default: &str) -> String {
        self.config.get(key).cloned().unwrap_or_else(|| default.into())
    }
    pub fn remove(&mut self, key: &str) { self.config.remove(key); }
    pub fn keys(&self) -> Vec<String> { self.config.keys().cloned().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let cm = ConfigManager::new();
        assert_eq!(cm.get("version"), Some(&"1.0.0".to_string()));
        assert_eq!(cm.get_or_default("missing", "fallback"), "fallback");
    }
}
