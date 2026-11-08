use std::string::String;

use globus_diagnostics::{CrashEvent, ErrorCode, EventLog, Severity};

/// Userspace-facing, immutable presentation of a kernel/service fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemErrorView {
    pub code: &'static str,
    pub severity: Severity,
    pub title: &'static str,
    pub message: String,
    pub driver: String,
    pub device_id: u64,
    pub recovered: bool,
}

pub fn present(event: &CrashEvent) -> SystemErrorView {
    let (title, message) = match event.code {
        ErrorCode::DriverCrashed => (
            "Driver crashed",
            format!(
                "The driver '{}' crashed and was isolated to protect the system.",
                event.driver
            ),
        ),
        ErrorCode::DriverIsolated => (
            "Driver isolated",
            format!(
                "The driver '{}' was isolated after a fault; the rest of the system remains running.",
                event.driver
            ),
        ),
        ErrorCode::DriverRecoveryFailed => (
            "Driver recovery failed",
            format!(
                "The driver '{}' could not be recovered and remains offline.",
                event.driver
            ),
        ),
    };
    SystemErrorView {
        code: event.code.as_str(),
        severity: event.severity,
        title,
        message,
        driver: event.driver.clone(),
        device_id: event.device_id,
        recovered: event.recovered,
    }
}

pub fn latest_error(log: &EventLog) -> Option<SystemErrorView> {
    log.latest().map(present)
}
