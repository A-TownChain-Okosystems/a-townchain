// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// LLM Router — routes requests to optimal model
pub struct LlmRouter {
    rules: Vec<RouterRule>,
}

pub struct RouterRule {
    pub pattern: String,
    pub model: String,
    pub priority: u8,
}

impl LlmRouter {
    pub fn new() -> Self {
        Self {
            rules: vec![
                RouterRule { pattern: "code".into(), model: "shiva-1.0".into(), priority: 1 },
                RouterRule { pattern: "translate".into(), model: "shiva-1.0".into(), priority: 2 },
                RouterRule { pattern: "summarize".into(), model: "shiva-1.0".into(), priority: 3 },
            ],
        }
    }

    pub fn add_rule(&mut self, pattern: &str, model: &str, priority: u8) {
        self.rules.push(RouterRule { pattern: pattern.into(), model: model.into(), priority });
        self.rules.sort_by_key(|r| r.priority);
    }

    pub fn route(&self, prompt: &str) -> String {
        let lower = prompt.to_lowercase();
        for rule in &self.rules {
            if lower.contains(&rule.pattern) {
                return rule.model.clone();
            }
        }
        "shiva-1.0".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router() {
        let router = LlmRouter::new();
        assert_eq!(router.route("write code for me"), "shiva-1.0");
        assert_eq!(router.route("hello world"), "shiva-1.0");
    }
}
