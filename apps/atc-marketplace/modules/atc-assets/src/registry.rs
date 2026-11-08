// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Asset registry and lookup
use std::collections::HashMap;
use crate::metadata::TokenMetadata;

pub struct AssetRegistry {
    assets: HashMap<u64, TokenMetadata>,
}

impl AssetRegistry {
    pub fn new() -> Self { Self { assets: HashMap::new() } }
    pub fn register(&mut self, metadata: TokenMetadata) -> Result<(), String> {
        if self.assets.contains_key(&metadata.token_id) { return Err("Token already registered".into()); }
        self.assets.insert(metadata.token_id, metadata);
        Ok(())
    }
    pub fn lookup(&self, id: u64) -> Option<&TokenMetadata> { self.assets.get(&id) }
    pub fn count(&self) -> usize { self.assets.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_registry() {
        let mut r = AssetRegistry::new();
        let m = TokenMetadata::new(1, "Test", "TST", 8);
        assert!(r.register(m).is_ok());
        assert!(r.lookup(1).is_some());
        assert!(r.register(TokenMetadata::new(1, "Dupe", "DUP", 8)).is_err());
    }
}
