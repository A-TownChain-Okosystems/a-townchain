//! Stable, machine-readable system fault reporting shared by ShivaCore and GlobusOS.

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// Severity level of a system fault or event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Severity {
    /// Informational diagnostic message.
    Info = 0,
    /// Warning condition that does not impair normal operation.
    Warning = 1,
    /// Error condition affecting functionality.
    Error = 2,
    /// Critical error requiring system or component recovery.
    Critical = 3,
}

/// System diagnostic error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ErrorCode {
    /// Indicates a driver crash fault.
    DriverCrashed = 0x0007,
    /// Indicates a driver has been isolated.
    DriverIsolated = 0x0008,
    /// Indicates driver recovery failed.
    DriverRecoveryFailed = 0x0009,
}

impl ErrorCode {
    /// Returns the string identifier corresponding to the error code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DriverCrashed => "SC-DRV-0007",
            Self::DriverIsolated => "SC-DRV-0008",
            Self::DriverRecoveryFailed => "SC-DRV-0009",
        }
    }
}

/// Subsystem component reporting a fault.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    /// Kernel subsystem component.
    Kernel,
    /// Driver subsystem component.
    Driver,
    /// Device subsystem component.
    Device,
    /// Service subsystem component.
    Service,
}

/// Details of a recorded crash fault event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashEvent {
    /// Monotonically increasing event sequence identifier.
    pub sequence: u64,
    /// Diagnostic error code associated with the event.
    pub code: ErrorCode,
    /// Severity level of the crash event.
    pub severity: Severity,
    /// Component that reported or experienced the crash.
    pub component: Component,
    /// Name of the affected driver.
    pub driver: String,
    /// Identifier of the target device.
    pub device_id: u64,
    /// Timestamp of the event in nanoseconds.
    pub timestamp_ns: u64,
    /// Indicates whether the crashed component was recovered.
    pub recovered: bool,
}

impl CrashEvent {
    /// Constructs a new `CrashEvent` representing a driver crash.
    pub fn driver_crash(sequence: u64, driver: &str, device_id: u64, timestamp_ns: u64) -> Self {
        Self {
            sequence,
            code: ErrorCode::DriverCrashed,
            severity: Severity::Critical,
            component: Component::Driver,
            driver: String::from(driver),
            device_id,
            timestamp_ns,
            recovered: false,
        }
    }
}

/// In-memory event log for tracking crash diagnostic events.
#[derive(Debug, Default)]
pub struct EventLog {
    next_sequence: u64,
    events: Vec<CrashEvent>,
}

impl EventLog {
    /// Maximum number of events retained in the log.
    pub const MAX_EVENTS: usize = 256;

    /// Creates an empty `EventLog`.
    pub const fn new() -> Self {
        Self {
            next_sequence: 1,
            events: Vec::new(),
        }
    }

    /// Records a new driver crash event in the log and returns its sequence number.
    pub fn record_driver_crash(&mut self, driver: &str, device_id: u64, timestamp_ns: u64) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        if self.events.len() == Self::MAX_EVENTS {
            self.events.remove(0);
        }
        self.events.push(CrashEvent::driver_crash(
            sequence,
            driver,
            device_id,
            timestamp_ns,
        ));
        sequence
    }

    /// Marks an event by sequence number as recovered and updates its status.
    pub fn mark_recovered(&mut self, sequence: u64) -> bool {
        let Some(event) = self.events.iter_mut().find(|e| e.sequence == sequence) else {
            return false;
        };
        event.recovered = true;
        event.code = ErrorCode::DriverIsolated;
        event.severity = Severity::Error;
        true
    }

    /// Returns a slice of all currently logged crash events.
    pub fn events(&self) -> &[CrashEvent] {
        &self.events
    }

    /// Returns a reference to the latest logged crash event, if any.
    pub fn latest(&self) -> Option<&CrashEvent> {
        self.events.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn driver_crash_has_stable_code() {
        let mut log = EventLog::new();
        let seq = log.record_driver_crash("e1000", 42, 100);
        assert_eq!(seq, 1);
        let event = log.latest().unwrap();
        assert_eq!(event.code.as_str(), "SC-DRV-0007");
        assert_eq!(event.driver, "e1000");
        assert!(!event.recovered);
    }

    #[test]
    fn recovery_changes_event_state() {
        let mut log = EventLog::new();
        let seq = log.record_driver_crash("nvme", 7, 100);
        assert!(log.mark_recovered(seq));
        let event = log.latest().unwrap();
        assert_eq!(event.code, ErrorCode::DriverIsolated);
        assert!(event.recovered);
    }
}
