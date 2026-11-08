//! Monotonic timer and scheduler-tick contract.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerFrequency {
    pub hz: u64,
}

impl TimerFrequency {
    pub fn valid(self) -> bool {
        self.hz > 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerTick {
    pub sequence: u64,
    pub elapsed_ns: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerError {
    InvalidFrequency,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonotonicTimer {
    frequency: TimerFrequency,
    ticks: u64,
}

impl MonotonicTimer {
    pub fn new(frequency: TimerFrequency) -> Result<Self, TimerError> {
        if !frequency.valid() {
            return Err(TimerError::InvalidFrequency);
        }
        Ok(Self {
            frequency,
            ticks: 0,
        })
    }

    pub fn frequency(&self) -> TimerFrequency {
        self.frequency
    }

    pub fn tick(&mut self) -> Result<TimerTick, TimerError> {
        self.ticks = self.ticks.checked_add(1).ok_or(TimerError::Overflow)?;
        let elapsed_ns = self
            .ticks
            .checked_mul(1_000_000_000 / self.frequency.hz)
            .ok_or(TimerError::Overflow)?;
        Ok(TimerTick {
            sequence: self.ticks,
            elapsed_ns,
        })
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timer_is_monotonic() {
        let mut timer = MonotonicTimer::new(TimerFrequency { hz: 1000 }).unwrap();
        assert_eq!(timer.tick().unwrap().sequence, 1);
        assert_eq!(timer.tick().unwrap().sequence, 2);
        assert!(timer.tick().unwrap().elapsed_ns > 0);
    }
    #[test]
    fn rejects_zero_frequency() {
        assert_eq!(
            MonotonicTimer::new(TimerFrequency { hz: 0 }),
            Err(TimerError::InvalidFrequency)
        );
    }
}
