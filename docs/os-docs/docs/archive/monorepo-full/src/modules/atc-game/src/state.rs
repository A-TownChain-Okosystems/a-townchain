// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Game state management
use std::collections::HashMap;
pub struct GameState {
    pub tick: u64,
    pub data: HashMap<String, i64>,
}

impl GameState {
    pub fn new() -> Self { Self { tick: 0, data: HashMap::new() } }
    pub fn set(&mut self, key: &str, val: i64) { self.data.insert(key.into(), val); }
    pub fn get(&self, key: &str) -> Option<&i64> { self.data.get(key) }
    pub fn advance(&mut self) { self.tick += 1; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state() {
        let mut s = GameState::new();
        s.set("score", 42);
        assert_eq!(s.get("score"), Some(&42));
        s.advance();
        assert_eq!(s.tick, 1);
    }
}
