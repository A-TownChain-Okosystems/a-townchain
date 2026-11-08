// Command history
pub struct ShellHistory { commands: Vec<String>, max: usize }
impl ShellHistory {
    pub fn new(max: usize) -> Self { Self { commands: Vec::new(), max } }
    pub fn add(&mut self, cmd: &str) {
        if self.commands.len() >= self.max { self.commands.remove(0); }
        self.commands.push(cmd.into());
    }
    pub fn last(&self, n: usize) -> Vec<&String> {
        self.commands.iter().rev().take(n).collect()
    }
    pub fn search(&self, pattern: &str) -> Vec<&String> {
        self.commands.iter().filter(|c| c.contains(pattern)).collect()
    }
    pub fn clear(&mut self) { self.commands.clear(); }
    pub fn count(&self) -> usize { self.commands.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_history() {
        let mut h = ShellHistory::new(3);
        h.add("ls"); h.add("pwd"); h.add("echo hi"); h.add("help");
        assert_eq!(h.count(), 3);
        assert_eq!(h.search("echo").len(), 1);
    }
}
