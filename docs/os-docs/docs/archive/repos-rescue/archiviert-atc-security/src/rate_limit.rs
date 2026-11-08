// Rate limiting middleware
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    limits: HashMap<String, (u32, Duration)>,
    counts: HashMap<String, (u32, Instant)>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self { limits: HashMap::new(), counts: HashMap::new() }
    }

    pub fn set_limit(&mut self, key: &str, max: u32, window: Duration) {
        self.limits.insert(key.to_string(), (max, window));
    }

    pub fn check(&mut self, key: &str) -> bool {
        if let Some(&(max, window)) = self.limits.get(key) {
            let now = Instant::now();
            let should_reset = match self.counts.get(key) {
                Some(&(_, last)) => now.duration_since(last) > window,
                None => true,
            };
            if should_reset {
                self.counts.insert(key.to_string(), (1, now));
                true
            } else {
                let entry = self.counts.get_mut(key).unwrap();
                if entry.0 < max {
                    entry.0 += 1;
                    true
                } else {
                    false
                }
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_rate_limit() {
        let mut rl = RateLimiter::new();
        rl.set_limit("api", 3, Duration::from_secs(60));
        assert!(rl.check("api"));
        assert!(rl.check("api"));
        assert!(rl.check("api"));
        assert!(!rl.check("api"));
    }

    #[test]
    fn test_no_limit() {
        let mut rl = RateLimiter::new();
        assert!(rl.check("unknown"));
    }
}
