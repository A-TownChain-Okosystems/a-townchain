// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Command registry — built-in commands
use std::collections::HashMap;

pub struct CommandRegistry {
    commands: HashMap<String, fn(&[&str]) -> String>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut reg = Self { commands: HashMap::new() };
        reg.commands.insert("echo".into(), Self::cmd_echo);
        reg.commands.insert("help".into(), Self::cmd_help);
        reg.commands.insert("ls".into(), Self::cmd_ls);
        reg.commands.insert("pwd".into(), Self::cmd_pwd);
        reg.commands.insert("whoami".into(), Self::cmd_whoami);
        reg.commands.insert("clear".into(), Self::cmd_clear);
        reg.commands.insert("version".into(), Self::cmd_version);
        reg
    }

    pub fn run(&self, cmd: &str, args: &[&str]) -> String {
        match self.commands.get(cmd) {
            Some(handler) => handler(args),
            None => format!("Unknown command: {}", cmd),
        }
    }

    fn cmd_echo(args: &[&str]) -> String { args.join(" ") }
    fn cmd_help(_args: &[&str]) -> String {
        "Available: echo help ls pwd whoami clear version".into()
    }
    fn cmd_ls(_args: &[&str]) -> String { "atcfs/  blockchain/  kernel/  modules/".into() }
    fn cmd_pwd(_args: &[&str]) -> String { "/".into() }
    fn cmd_whoami(_args: &[&str]) -> String { "shiva".into() }
    fn cmd_clear(_args: &[&str]) -> String { String::new() }
    fn cmd_version(_args: &[&str]) -> String { "GlobusOS Shell v1.0.0".into() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_commands() {
        let reg = CommandRegistry::new();
        assert_eq!(reg.run("echo", &["hello"]), "hello");
        assert!(reg.run("help", &[]).contains("echo"));
        assert!(reg.run("unknown", &[]).contains("Unknown"));
    }
}
