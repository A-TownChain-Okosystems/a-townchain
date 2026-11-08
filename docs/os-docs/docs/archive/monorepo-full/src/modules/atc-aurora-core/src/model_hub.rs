// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Model Hub — manages AI model registry and inference
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub context_window: usize,
    pub is_loaded: bool,
}

pub struct ModelHub {
    models: HashMap<String, ModelInfo>,
    default_model: String,
}

impl ModelHub {
    pub fn new() -> Self {
        let mut hub = Self { models: HashMap::new(), default_model: String::new() };
        hub.register("shiva-1.0", "1.0.0", 8192);
        hub.set_default("shiva-1.0");
        hub
    }

    pub fn register(&mut self, name: &str, version: &str, ctx: usize) {
        self.models.insert(name.into(), ModelInfo {
            name: name.into(), version: version.into(),
            context_window: ctx, is_loaded: true,
        });
    }

    pub fn set_default(&mut self, name: &str) { self.default_model = name.into(); }
    pub fn get_default(&self) -> &str { &self.default_model }

    pub fn inference(&self, model: &str, prompt: &str) -> String {
        if !self.models.contains_key(model) {
            return format!("[Error: Model '{}' not found]", model);
        }
        format!("[{}: {} → response ({} chars)]", model, &prompt[..prompt.len().min(40)], prompt.len() * 2)
    }

    pub fn list_models(&self) -> Vec<&ModelInfo> { self.models.values().collect() }
    pub fn model_count(&self) -> usize { self.models.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_hub() {
        let mut hub = ModelHub::new();
        assert!(hub.model_count() >= 1);
        hub.register("shiva-2.0", "2.0.0", 32768);
        assert_eq!(hub.model_count(), 2);
        let r = hub.inference("shiva-1.0", "test prompt");
        assert!(r.contains("shiva-1.0"));
    }
}
