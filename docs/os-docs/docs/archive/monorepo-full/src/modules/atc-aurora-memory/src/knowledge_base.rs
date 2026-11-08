// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Knowledge base — stores facts and relationships
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KnowledgeEntry {
    pub id: u64,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f64,
    pub timestamp: u64,
}

pub struct KnowledgeBase {
    entries: Vec<KnowledgeEntry>,
    index: HashMap<String, Vec<usize>>,
}

impl KnowledgeBase {
    pub fn new() -> Self { Self { entries: Vec::new(), index: HashMap::new() } }

    pub fn add(&mut self, subject: &str, predicate: &str, object: &str, confidence: f64) -> u64 {
        let id = self.entries.len() as u64;
        let entry = KnowledgeEntry {
            id, subject: subject.into(), predicate: predicate.into(),
            object: object.into(), confidence, timestamp: 0,
        };
        self.index.entry(subject.into()).or_default().push(self.entries.len());
        self.entries.push(entry);
        id
    }

    pub fn query(&self, subject: &str) -> Vec<&KnowledgeEntry> {
        self.index.get(subject)
            .map(|indices| indices.iter().filter_map(|&i| self.entries.get(i)).collect())
            .unwrap_or_default()
    }

    pub fn count(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kb() {
        let mut kb = KnowledgeBase::new();
        kb.add("atc", "has_chain_id", "658467", 1.0);
        kb.add("atc", "uses_consensus", "PoW+PoS+PoH", 0.95);
        assert_eq!(kb.count(), 2);
        let results = kb.query("atc");
        assert_eq!(results.len(), 2);
    }
}
