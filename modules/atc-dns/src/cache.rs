// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Resolution cache
use std::collections::HashMap;

pub struct DnsCache {
    entries: HashMap<String, String>,
    max_size: usize,
}

impl DnsCache {
    pub fn new(max_size: usize) -> Self { Self { entries: HashMap::new(), max_size } }

    pub fn get(&self, name: &str) -> Option<String> { self.entries.get(name).cloned() }
    pub fn put(&mut self, name: &str, address: &str) {
        if self.entries.len() >= self.max_size {
            if let Some(k) = self.entries.keys().next().cloned() { self.entries.remove(&k); }
        }
        self.entries.insert(name.into(), address.into());
    }
    pub fn remove(&mut self, name: &str) { self.entries.remove(name); }
    pub fn clear(&mut self) { self.entries.clear(); }
    pub fn len(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let mut c = DnsCache::new(2);
        c.put("a", "1"); c.put("b", "2");
        assert_eq!(c.get("a"), Some("1".into()));
        c.put("c", "3");
        assert!(c.len() <= 2);
    }
}
