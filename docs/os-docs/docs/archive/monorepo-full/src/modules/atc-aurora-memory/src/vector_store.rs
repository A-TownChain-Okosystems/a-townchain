// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Vector store — embedding storage and similarity search
use std::collections::HashMap;

pub struct VectorStore {
    vectors: HashMap<String, Vec<f32>>,
}

impl VectorStore {
    pub fn new() -> Self { Self { vectors: HashMap::new() } }

    pub fn store(&mut self, key: &str, vector: Vec<f32>) {
        self.vectors.insert(key.into(), vector);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<f32>> { self.vectors.get(key) }

    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() { return 0.0; }
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 { return 0.0; }
        dot / (norm_a * norm_b)
    }

    pub fn search(&self, query: &[f32], limit: usize) -> Vec<(String, f32)> {
        let mut results: Vec<(String, f32)> = self.vectors.iter()
            .map(|(k, v)| (k.clone(), Self::cosine_similarity(query, v)))
            .filter(|(_, score)| *score > 0.0)
            .collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        results
    }

    pub fn count(&self) -> usize { self.vectors.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vector_store() {
        let mut vs = VectorStore::new();
        vs.store("doc1", vec![1.0, 0.0, 0.0]);
        vs.store("doc2", vec![0.0, 1.0, 0.0]);
        vs.store("doc3", vec![1.0, 1.0, 0.0]);
        let results = vs.search(&[1.0, 0.0, 0.0], 2);
        assert!(results.len() <= 2);
        assert_eq!(results[0].0, "doc1");
        assert!(results[0].1 > 0.99);
    }
}
