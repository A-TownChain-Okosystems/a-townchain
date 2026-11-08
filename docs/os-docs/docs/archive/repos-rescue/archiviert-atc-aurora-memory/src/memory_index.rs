// Memory index — indexes memories by tags and time
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Memory {
    pub id: u64,
    pub content: String,
    pub tags: Vec<String>,
    pub timestamp: u64,
}

pub struct MemoryIndex {
    memories: Vec<Memory>,
    tag_index: HashMap<String, Vec<usize>>,
}

impl MemoryIndex {
    pub fn new() -> Self { Self { memories: Vec::new(), tag_index: HashMap::new() } }

    pub fn add(&mut self, content: &str, tags: Vec<String>, ts: u64) -> u64 {
        let id = self.memories.len() as u64;
        for tag in &tags {
            self.tag_index.entry(tag.clone()).or_default().push(self.memories.len());
        }
        self.memories.push(Memory { id, content: content.into(), tags, timestamp: ts });
        id
    }

    pub fn by_tag(&self, tag: &str) -> Vec<&Memory> {
        self.tag_index.get(tag)
            .map(|indices| indices.iter().filter_map(|&i| self.memories.get(i)).collect())
            .unwrap_or_default()
    }

    pub fn count(&self) -> usize { self.memories.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_index() {
        let mut mi = MemoryIndex::new();
        mi.add("Kernel boot complete", vec!["kernel".into()], 100);
        mi.add("Block 42 mined", vec!["blockchain".into()], 200);
        assert_eq!(mi.count(), 2);
        assert_eq!(mi.by_tag("kernel").len(), 1);
    }
}
