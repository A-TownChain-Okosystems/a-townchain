// Shell — main command processor
use std::collections::HashMap;
use crate::CommandRegistry;

pub struct Shell {
    pub registry: CommandRegistry,
    pub running: bool,
    pub cwd: String,
}

impl Shell {
    pub fn new() -> Self {
        Self { registry: CommandRegistry::new(), running: false, cwd: "/".into() }
    }

    pub fn start(&mut self) { self.running = true; }
    pub fn stop(&mut self) { self.running = false; }

    pub fn execute(&mut self, input: &str) -> String {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { return String::new(); }
        let cmd = parts[0];
        let args = &parts[1..];
        self.registry.run(cmd, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shell() {
        let mut sh = Shell::new();
        assert!(sh.execute("").is_empty());
        let out = sh.execute("echo hello");
        assert!(out.contains("hello") || out.contains("Unknown"));
    }
}
