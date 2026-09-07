// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Zone management
use std::collections::HashMap;

pub struct ZoneManager {
    zones: HashMap<String, Vec<String>>,
}

impl ZoneManager {
    pub fn new() -> Self { Self { zones: HashMap::new() } }

    pub fn create_zone(&mut self, zone: &str) { self.zones.insert(zone.into(), Vec::new()); }
    pub fn add_to_zone(&mut self, zone: &str, name: &str) -> Result<(), String> {
        self.zones.get_mut(zone).ok_or("Zone not found")?.push(name.into());
        Ok(())
    }
    pub fn list_zone(&self, zone: &str) -> Option<&Vec<String>> { self.zones.get(zone) }
    pub fn list_zones(&self) -> Vec<String> { self.zones.keys().cloned().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zones() {
        let mut z = ZoneManager::new();
        z.create_zone("mainnet");
        assert!(z.add_to_zone("mainnet", "node1").is_ok());
        assert_eq!(z.list_zone("mainnet").unwrap().len(), 1);
    }
}
