// Game engine core loop
pub struct GameEngine {
    tick: u64,
    running: bool,
}

impl GameEngine {
    pub fn new() -> Self { Self { tick: 0, running: false } }
    pub fn start(&mut self) { self.running = true; }
    pub fn stop(&mut self) { self.running = false; }
    pub fn update(&mut self) -> u64 {
        if self.running { self.tick += 1; }
        self.tick
    }
    pub fn is_running(&self) -> bool { self.running }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine() {
        let mut e = GameEngine::new();
        e.start();
        assert!(e.is_running());
        assert_eq!(e.update(), 1);
        assert_eq!(e.update(), 2);
        e.stop();
        assert!(!e.is_running());
    }
}
