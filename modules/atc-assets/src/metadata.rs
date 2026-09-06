// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Token metadata definitions
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TokenMetadata {
    pub token_id: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub supply: u64,
    pub attributes: HashMap<String, String>,
}

impl TokenMetadata {
    pub fn new(id: u64, name: &str, symbol: &str, decimals: u8) -> Self {
        Self { token_id: id, name: name.into(), symbol: symbol.into(), decimals, supply: 0, attributes: HashMap::new() }
    }
    pub fn set_attribute(&mut self, key: &str, val: &str) {
        self.attributes.insert(key.into(), val.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_metadata() {
        let mut m = TokenMetadata::new(1, "ATC Coin", "ATC", 8);
        m.set_attribute("type", "fungible");
        assert_eq!(m.attributes.get("type"), Some(&"fungible".to_string()));
    }
}
