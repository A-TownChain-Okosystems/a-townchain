// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Pipe system — command chaining
pub struct PipeSystem;
impl PipeSystem {
    pub fn split_pipes(input: &str) -> Vec<String> {
        input.split('|').map(|s| s.trim().to_string()).collect()
    }
    pub fn chain(commands: &[String], executor: impl Fn(&str) -> String) -> String {
        let mut output = String::new();
        for cmd in commands {
            output = if output.is_empty() {
                executor(cmd)
            } else {
                executor(&format!("{} {}", cmd, output))
            };
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pipe() {
        let parts = PipeSystem::split_pipes("echo hi | wc");
        assert_eq!(parts.len(), 2);
        let result = PipeSystem::chain(&["echo hello".into()], |c| format!("[{}]", c));
        assert!(result.contains("hello"));
    }
}
