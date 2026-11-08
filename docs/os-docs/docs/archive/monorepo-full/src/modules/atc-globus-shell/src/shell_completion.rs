// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Autocomplete
pub struct AutoComplete { commands: Vec<String> }
impl AutoComplete {
    pub fn new() -> Self {
        Self { commands: vec!["echo","help","ls","pwd","whoami","clear","version"].iter().map(|s| s.to_string()).collect() }
    }
    pub fn complete(&self, partial: &str) -> Vec<String> {
        self.commands.iter().filter(|c| c.starts_with(partial)).cloned().collect()
    }
    pub fn add(&mut self, cmd: &str) { self.commands.push(cmd.into()); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complete() {
        let ac = AutoComplete::new();
        let matches = ac.complete("e");
        assert!(matches.contains(&"echo".to_string()));
        let no_match = ac.complete("zzz");
        assert!(no_match.is_empty());
    }
}
