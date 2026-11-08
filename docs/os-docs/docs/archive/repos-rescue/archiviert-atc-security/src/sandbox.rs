// Sandbox isolation
pub struct Sandbox {
    id: u64,
    memory_limit: usize,
    cpu_limit_ms: u64,
    active: bool,
}

impl Sandbox {
    pub fn new(id: u64, memory_limit: usize, cpu_limit_ms: u64) -> Self {
        Self { id, memory_limit, cpu_limit_ms, active: false }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.active { return Err("Sandbox already active".into()); }
        self.active = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if !self.active { return Err("Sandbox not active".into()); }
        self.active = false;
        Ok(())
    }

    pub fn is_active(&self) -> bool { self.active }
    pub fn memory_limit(&self) -> usize { self.memory_limit }
    pub fn cpu_limit_ms(&self) -> u64 { self.cpu_limit_ms }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_lifecycle() {
        let mut sb = Sandbox::new(1, 4096, 1000);
        assert!(!sb.is_active());
        assert!(sb.start().is_ok());
        assert!(sb.is_active());
        assert!(sb.start().is_err());
        assert!(sb.stop().is_ok());
        assert!(!sb.is_active());
    }
}
