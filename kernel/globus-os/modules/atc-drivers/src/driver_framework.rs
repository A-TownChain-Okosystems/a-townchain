//! Deterministic driver supervision and fail-closed fault reporting.
extern crate alloc;
use alloc::string::{String, ToString};
use globus_diagnostics::EventLog;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Registered,
    Active,
    Faulted,
    Isolated,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    InitializationFailed,
    RuntimeFault,
    DeviceUnavailable,
}

pub trait Driver {
    fn name(&self) -> &str;
    fn start(&mut self) -> Result<(), DriverError>;
    fn stop(&mut self);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverRecord {
    pub name: String,
    pub device_id: u64,
    pub state: DriverState,
}

pub struct DriverSupervisor {
    records: alloc::vec::Vec<DriverRecord>,
    pub events: EventLog,
}
impl Default for DriverSupervisor {
    fn default() -> Self {
        Self::new()
    }
}
impl DriverSupervisor {
    pub const fn new() -> Self {
        Self {
            records: alloc::vec::Vec::new(),
            events: EventLog::new(),
        }
    }
    pub fn register(&mut self, name: &str, device_id: u64) -> bool {
        if name.is_empty() || self.records.iter().any(|r| r.device_id == device_id) {
            return false;
        }
        self.records.push(DriverRecord {
            name: name.to_string(),
            device_id,
            state: DriverState::Registered,
        });
        self.records.sort_by_key(|r| r.device_id);
        true
    }
    pub fn mark_active(&mut self, device_id: u64) -> bool {
        let Some(record) = self.records.iter_mut().find(|r| r.device_id == device_id) else {
            return false;
        };
        if record.state != DriverState::Registered {
            return false;
        }
        record.state = DriverState::Active;
        true
    }
    /// Record a fatal driver fault and immediately isolate the driver.
    pub fn report_fault(&mut self, device_id: u64, timestamp_ns: u64) -> bool {
        let Some(record) = self.records.iter_mut().find(|r| r.device_id == device_id) else {
            return false;
        };
        if record.state != DriverState::Active {
            return false;
        }
        let sequence = self
            .events
            .record_driver_crash(&record.name, device_id, timestamp_ns);
        if self.events.mark_recovered(sequence) {
            record.state = DriverState::Isolated;
            true
        } else {
            record.state = DriverState::Faulted;
            false
        }
    }
    pub fn get(&self, device_id: u64) -> Option<&DriverRecord> {
        self.records.iter().find(|r| r.device_id == device_id)
    }
    pub fn records(&self) -> &[DriverRecord] {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fatal_driver_fault_is_isolated_and_logged() {
        let mut supervisor = DriverSupervisor::new();
        assert!(supervisor.register("e1000", 42));
        assert!(supervisor.mark_active(42));
        assert!(supervisor.report_fault(42, 123));
        assert_eq!(supervisor.get(42).unwrap().state, DriverState::Isolated);
        let event = supervisor.events.latest().unwrap();
        assert_eq!(event.code.as_str(), "SC-DRV-0008");
        assert!(event.recovered);
    }
}
