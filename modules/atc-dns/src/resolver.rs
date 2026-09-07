// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Name resolution logic
use std::collections::HashMap;
use crate::cache::DnsCache;

pub struct DnsResolver {
    registry: HashMap<String, String>,
    cache: DnsCache,
}

impl DnsResolver {
    pub fn new() -> Self { Self { registry: HashMap::new(), cache: DnsCache::new(100) } }

    pub fn register(&mut self, name: &str, address: &str) {
        self.registry.insert(name.into(), address.into());
    }

    pub fn resolve(&mut self, name: &str) -> Option<String> {
        if let Some(cached) = self.cache.get(name) { return Some(cached); }
        let result = self.registry.get(name).cloned();
        if let Some(ref addr) = result { self.cache.put(name, addr); }
        result
    }

    pub fn unregister(&mut self, name: &str) { self.registry.remove(name); self.cache.remove(name); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let mut r = DnsResolver::new();
        r.register("alice.atc", "ATC1ABC123");
        assert_eq!(r.resolve("alice.atc"), Some("ATC1ABC123".into()));
        assert_eq!(r.resolve("unknown.atc"), None);
    }
}
