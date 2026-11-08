//! atc-maintenance — operative Umsetzung der Maintenance-Standards (SCR-0123/SCR-0124).
//!
//! MAINT-001: Klassifizierung jeder Maintenance-Aufgabe in M0–M3 (fail-closed,
//! Im-Zweifel-höhere-Klasse, Gates + Separation of Duties).
//! MAINT-000 §7.2: Readiness-Gate — "No honest PASS = No Release".

pub mod audit;
pub mod classification;
pub mod engineering;
pub mod readiness;
pub mod repair;

pub use audit::{audit_repository, AuditReport, Finding, FindingKind};
pub use classification::{Gate, MaintenanceClass};
pub use engineering::{
    engineer_until_clean, EngineeringPhase, EngineeringPolicy, EngineeringResult, FindingRecord,
};
pub use readiness::{evaluate_coverage, evaluate_record, GateOutcome, ReadinessRecord};
pub use repair::{repair_until_stable, RepairAction, RepairEvent, RepairRun};
