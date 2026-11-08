//! Deterministic service lifecycle and dependency model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Defined,
    Starting,
    Ready,
    Failed,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceSpec {
    pub name: String,
    pub dependencies: Vec<String>,
    pub critical: bool,
}

pub fn boot_order() -> [&'static str; 8] {
    [
        "security", "devices", "storage", "network", "runtime", "graphics", "audio", "aurora",
    ]
}
