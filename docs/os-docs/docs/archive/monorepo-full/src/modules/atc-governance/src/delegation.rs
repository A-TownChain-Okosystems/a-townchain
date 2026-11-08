// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Vote delegation
use std::collections::HashMap;

pub struct Delegation {
    delegations: HashMap<String, String>,
}

impl Delegation {
    pub fn new() -> Self { Self { delegations: HashMap::new() } }

    pub fn delegate(&mut self, from: &str, to: &str) -> Result<(), String> {
        if from == to { return Err("Cannot delegate to self".into()); }
        if self.would_create_cycle(from, to) { return Err("Cycle detected".into()); }
        self.delegations.insert(from.into(), to.into());
        Ok(())
    }

    pub fn undelegate(&mut self, from: &str) { self.delegations.remove(from); }

    pub fn get_delegate(&self, from: &str) -> Option<&String> { self.delegations.get(from) }

    fn would_create_cycle(&self, from: &str, to: &str) -> bool {
        let mut current = to;
        for _ in 0..100 {
            if current == from { return true; }
            match self.delegations.get(current) {
                Some(next) => current = next,
                None => break,
            }
        }
        false
    }

    pub fn resolve(&self, voter: &str) -> String {
        let mut current = voter;
        let mut visited = std::collections::HashSet::new();
        visited.insert(current.to_string());
        while let Some(delegate) = self.delegations.get(current) {
            if !visited.insert(delegate.clone()) { break; }
            current = delegate;
        }
        current.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegation() {
        let mut d = Delegation::new();
        assert!(d.delegate("alice", "bob").is_ok());
        assert!(d.delegate("bob", "charlie").is_ok());
        assert_eq!(d.resolve("alice"), "charlie");
        assert!(d.delegate("charlie", "alice").is_err());
    }
}
