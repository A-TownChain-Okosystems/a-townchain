// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// DNS record registry
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: RecordType,
    pub value: String,
    pub ttl: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecordType { A, Cname, Txt, Mx }

pub struct DnsRegistry {
    records: HashMap<String, DnsRecord>,
}

impl DnsRegistry {
    pub fn new() -> Self { Self { records: HashMap::new() } }

    pub fn add(&mut self, name: &str, rtype: RecordType, value: &str, ttl: u64) {
        self.records.insert(name.into(), DnsRecord {
            name: name.into(), record_type: rtype, value: value.into(), ttl,
        });
    }

    pub fn lookup(&self, name: &str) -> Option<&DnsRecord> { self.records.get(name) }
    pub fn remove(&mut self, name: &str) -> bool { self.records.remove(name).is_some() }
    pub fn count(&self) -> usize { self.records.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry() {
        let mut r = DnsRegistry::new();
        r.add("node1.atc", RecordType::A, "192.168.1.1", 3600);
        assert!(r.lookup("node1.atc").is_some());
        assert!(r.remove("node1.atc"));
        assert!(r.lookup("node1.atc").is_none());
    }
}
