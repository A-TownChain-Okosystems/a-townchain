// Context window — manages conversation context
pub struct ContextWindow {
    messages: Vec<(String, String)>,
    max_size: usize,
}

impl ContextWindow {
    pub fn new(max_size: usize) -> Self { Self { messages: Vec::new(), max_size } }

    pub fn add(&mut self, role: &str, content: &str) {
        if self.messages.len() >= self.max_size {
            self.messages.remove(0);
        }
        self.messages.push((role.into(), content.into()));
    }

    pub fn get_context(&self) -> String {
        self.messages.iter()
            .map(|(r, c)| format!("{}: {}", r, c))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn clear(&mut self) { self.messages.clear(); }
    pub fn len(&self) -> usize { self.messages.len() }
    pub fn is_full(&self) -> bool { self.messages.len() >= self.max_size }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_context() {
        let mut cw = ContextWindow::new(3);
        cw.add("user", "hello");
        cw.add("assistant", "hi");
        cw.add("user", "bye");
        cw.add("assistant", "goodbye");
        assert_eq!(cw.len(), 3);
        assert!(cw.get_context().contains("goodbye"));
        assert!(!cw.get_context().contains("hello"));
    }
}
