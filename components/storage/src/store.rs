// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! In-Memory-KV-Store mit deterministischem Root-Hash (STORAGE-Spezifikationen, MVP).

pub trait KvStore {
    fn put(&mut self, key: &str, value: &[u8]);
    fn get(&self, key: &str) -> Option<Vec<u8>>;
    fn delete(&mut self, key: &str) -> bool;
    fn keys(&self) -> Vec<String>;
}

#[derive(Default)]
pub struct InMemoryStore {
    data: std::collections::BTreeMap<String, Vec<u8>>,
}

fn fnv1a(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

impl KvStore for InMemoryStore {
    fn put(&mut self, key: &str, value: &[u8]) {
        self.data.insert(key.to_string(), value.to_vec());
    }
    fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.data.get(key).cloned()
    }
    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
    fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

impl InMemoryStore {
    /// Deterministischer Root-Hash ueber sortierte Eintraege (MVP-Hash FNV-1a).
    pub fn root(&self) -> u64 {
        let mut acc: u64 = 0;
        for (k, v) in &self.data {
            acc ^= fnv1a(k.as_bytes()).wrapping_add(fnv1a(v));
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get_delete() {
        let mut s = InMemoryStore::default();
        s.put("a", &[1, 2]);
        assert_eq!(s.get("a"), Some(vec![1, 2]));
        assert_eq!(s.get("b"), None);
        assert!(s.delete("a"));
        assert!(!s.delete("a"));
        assert!(s.keys().is_empty());
    }

    #[test]
    fn root_aendert_sich_deterministisch() {
        let mut a = InMemoryStore::default();
        let mut b = InMemoryStore::default();
        a.put("x", &[9]);
        b.put("x", &[9]);
        assert_eq!(a.root(), b.root());
        let before = a.root();
        a.put("y", &[1]);
        assert_ne!(a.root(), before);
        a.delete("y");
        assert_eq!(a.root(), before);
    }
}
