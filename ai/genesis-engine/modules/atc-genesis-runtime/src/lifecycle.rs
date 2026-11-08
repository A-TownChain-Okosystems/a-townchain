//! Canonical engine lifecycle and fixed-timestep scheduling primitives.
//!
//! This module deliberately contains orchestration policy only. Concrete
//! subsystems remain owned by their respective workspace crates.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuntimeConfig {
    /// Fixed simulation step in seconds.
    pub fixed_dt: f64,
    /// Maximum simulation steps executed for one presented frame.
    pub max_steps_per_frame: u32,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            fixed_dt: 1.0 / 60.0,
            max_steps_per_frame: 8,
        }
    }
}

impl RuntimeConfig {
    pub fn validate(self) -> Result<Self, RuntimeConfigError> {
        if !self.fixed_dt.is_finite() || self.fixed_dt <= 0.0 {
            return Err(RuntimeConfigError::InvalidFixedDt);
        }
        if self.max_steps_per_frame == 0 {
            return Err(RuntimeConfigError::InvalidStepLimit);
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeConfigError {
    InvalidFixedDt,
    InvalidStepLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimePhase {
    Boot,
    Initialize,
    LoadProject,
    LoadAssets,
    CreateWorld,
    Simulate,
    PrepareRender,
    Render,
    Present,
    Shutdown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickPlan {
    pub simulation_steps: u32,
    pub interpolation_alpha: f64,
    pub dropped_time: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct FixedTimestep {
    accumulator: f64,
    config: RuntimeConfig,
}

impl FixedTimestep {
    pub fn new(config: RuntimeConfig) -> Result<Self, RuntimeConfigError> {
        Ok(Self {
            accumulator: 0.0,
            config: config.validate()?,
        })
    }

    pub fn advance(&mut self, frame_dt: f64) -> TickPlan {
        let frame_dt = if frame_dt.is_finite() && frame_dt > 0.0 {
            frame_dt
        } else {
            0.0
        };

        self.accumulator += frame_dt;
        let mut steps = 0;
        while self.accumulator >= self.config.fixed_dt && steps < self.config.max_steps_per_frame {
            self.accumulator -= self.config.fixed_dt;
            steps += 1;
        }

        let mut dropped_time = 0.0;
        if self.accumulator >= self.config.fixed_dt {
            dropped_time = self.accumulator - (self.config.fixed_dt - f64::EPSILON);
            self.accumulator = self.config.fixed_dt - f64::EPSILON;
        }

        TickPlan {
            simulation_steps: steps,
            interpolation_alpha: self.accumulator / self.config.fixed_dt,
            dropped_time,
        }
    }

    pub fn accumulator(&self) -> f64 {
        self.accumulator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_configuration() {
        assert_eq!(
            RuntimeConfig {
                fixed_dt: 0.0,
                ..Default::default()
            }
            .validate(),
            Err(RuntimeConfigError::InvalidFixedDt)
        );
        assert_eq!(
            RuntimeConfig {
                max_steps_per_frame: 0,
                ..Default::default()
            }
            .validate(),
            Err(RuntimeConfigError::InvalidStepLimit)
        );
    }

    #[test]
    fn produces_fixed_simulation_steps() {
        let mut clock = FixedTimestep::new(RuntimeConfig::default()).unwrap();
        let plan = clock.advance(1.0 / 30.0);
        assert_eq!(plan.simulation_steps, 2);
        assert!(plan.interpolation_alpha.abs() < 1e-12);
    }

    #[test]
    fn clamps_pathological_frame_delta() {
        let mut clock = FixedTimestep::new(RuntimeConfig::default()).unwrap();
        let plan = clock.advance(10.0);
        assert_eq!(plan.simulation_steps, 8);
        assert!(plan.dropped_time > 0.0);
        assert!(plan.interpolation_alpha >= 0.0);
        assert!(plan.interpolation_alpha < 1.0);
    }

    #[test]
    fn invalid_frame_delta_does_not_poison_clock() {
        let mut clock = FixedTimestep::new(RuntimeConfig::default()).unwrap();
        let plan = clock.advance(f64::NAN);
        assert_eq!(plan.simulation_steps, 0);
        assert_eq!(clock.accumulator(), 0.0);
    }
}
