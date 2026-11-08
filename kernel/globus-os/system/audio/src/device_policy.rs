//! Audio-device routing and session policy boundary.

use crate::AudioSessionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioOutputKind {
    Speaker,
    Headphones,
    Hdmi,
    Usb,
    Bluetooth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioRoute {
    pub session: AudioSessionId,
    pub output: AudioOutputKind,
    pub exclusive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteError {
    DuplicateSession,
    ExclusiveConflict,
    MissingSession,
}

#[derive(Debug, Default)]
pub struct AudioRouteTable {
    routes: Vec<AudioRoute>,
}

impl AudioRouteTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn routes(&self) -> &[AudioRoute] {
        &self.routes
    }

    pub fn add(&mut self, route: AudioRoute) -> Result<(), RouteError> {
        if self.routes.iter().any(|r| r.session == route.session) {
            return Err(RouteError::DuplicateSession);
        }
        if route.exclusive && self.routes.iter().any(|r| r.output == route.output) {
            return Err(RouteError::ExclusiveConflict);
        }
        if self
            .routes
            .iter()
            .any(|r| r.output == route.output && r.exclusive)
        {
            return Err(RouteError::ExclusiveConflict);
        }
        self.routes.push(route);
        self.routes.sort_by_key(|r| r.session.0);
        Ok(())
    }

    pub fn remove(&mut self, session: AudioSessionId) -> Result<(), RouteError> {
        let before = self.routes.len();
        self.routes.retain(|r| r.session != session);
        if self.routes.len() == before {
            return Err(RouteError::MissingSession);
        }
        Ok(())
    }

    pub fn route_for(&self, session: AudioSessionId) -> Option<AudioRoute> {
        self.routes.iter().copied().find(|r| r.session == session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclusive_route_blocks_shared_output() {
        let mut table = AudioRouteTable::new();
        table
            .add(AudioRoute {
                session: AudioSessionId(1),
                output: AudioOutputKind::Hdmi,
                exclusive: true,
            })
            .unwrap();
        assert_eq!(
            table.add(AudioRoute {
                session: AudioSessionId(2),
                output: AudioOutputKind::Hdmi,
                exclusive: false
            }),
            Err(RouteError::ExclusiveConflict)
        );
    }
}
