//! Architecture-neutral interrupt routing and acknowledgement contract.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InterruptVector(pub u16);

impl InterruptVector {
    pub fn valid(self) -> bool {
        (32..=255).contains(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerMode {
    Edge,
    Level,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    ActiveHigh,
    ActiveLow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptRoute {
    pub vector: InterruptVector,
    pub destination_cpu: u32,
    pub trigger: TriggerMode,
    pub polarity: Polarity,
    pub masked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptError {
    InvalidVector,
    InvalidCpu,
    AlreadyRouted,
    NotRouted,
}

#[derive(Debug, Default)]
pub struct InterruptController {
    routes: Vec<InterruptRoute>,
}

impl InterruptController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn route(&mut self, route: InterruptRoute) -> Result<(), InterruptError> {
        if !route.vector.valid() {
            return Err(InterruptError::InvalidVector);
        }
        if route.destination_cpu == u32::MAX {
            return Err(InterruptError::InvalidCpu);
        }
        if self.routes.iter().any(|r| r.vector == route.vector) {
            return Err(InterruptError::AlreadyRouted);
        }
        self.routes.push(route);
        self.routes.sort_by_key(|r| r.vector);
        Ok(())
    }

    pub fn mask(&mut self, vector: InterruptVector, masked: bool) -> Result<(), InterruptError> {
        let route = self
            .routes
            .iter_mut()
            .find(|r| r.vector == vector)
            .ok_or(InterruptError::NotRouted)?;
        route.masked = masked;
        Ok(())
    }

    pub fn route_for(&self, vector: InterruptVector) -> Option<InterruptRoute> {
        self.routes.iter().find(|r| r.vector == vector).copied()
    }

    pub fn routes(&self) -> &[InterruptRoute] {
        &self.routes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routes_are_deterministic_and_maskable() {
        let mut c = InterruptController::new();
        c.route(InterruptRoute {
            vector: InterruptVector(40),
            destination_cpu: 1,
            trigger: TriggerMode::Level,
            polarity: Polarity::ActiveHigh,
            masked: false,
        })
        .unwrap();
        assert_eq!(c.route_for(InterruptVector(40)).unwrap().destination_cpu, 1);
        c.mask(InterruptVector(40), true).unwrap();
        assert!(c.route_for(InterruptVector(40)).unwrap().masked);
    }
}
