//! MAINT-001 — Maintenance Classification Standard (freigegeben SCR-0124).
//!
//! REQ-MAINT-016: Klasse ist ab Anlage pflichtig. REQ-MAINT-017: Im Zweifel die
//! höhere Klasse. REQ-MAINT-018/019: M2/M3-Regeln. REQ-MAINT-020: M2/M3
//! erzeugen Separation-of-Duties-Pflicht.

use atc_core::{CoreError, Result};
use std::fmt;

/// Die vier verbindlichen Klassen (MAINT-001 §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaintenanceClass {
    M0,
    M1,
    M2,
    M3,
}

impl MaintenanceClass {
    pub const ALL: [MaintenanceClass; 4] = [
        MaintenanceClass::M0,
        MaintenanceClass::M1,
        MaintenanceClass::M2,
        MaintenanceClass::M3,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            MaintenanceClass::M0 => "M0-Routine",
            MaintenanceClass::M1 => "M1-Operational",
            MaintenanceClass::M2 => "M2-Security",
            MaintenanceClass::M3 => "M3-Critical",
        }
    }

    /// Fail-closed Parsing (REQ-MAINT-016).
    pub fn parse(s: &str) -> Result<Self> {
        let raw = s.trim();
        let t = raw
            .strip_prefix('M')
            .or_else(|| raw.strip_prefix('m'))
            .unwrap_or(raw);
        match t {
            "0" | "0-Routine" | "0-routine" => Ok(MaintenanceClass::M0),
            "1" | "1-Operational" | "1-operational" => Ok(MaintenanceClass::M1),
            "2" | "2-Security" | "2-security" => Ok(MaintenanceClass::M2),
            "3" | "3-Critical" | "3-critical" => Ok(MaintenanceClass::M3),
            other => Err(CoreError::InvalidState(format!(
                "unknown maintenance class: '{other}' (expected M0..M3)"
            ))),
        }
    }

    /// REQ-MAINT-017: Im Zweifel die höhere Klasse (deterministisch per Ord).
    pub fn in_doubt(a: MaintenanceClass, b: MaintenanceClass) -> MaintenanceClass {
        a.max(b)
    }

    /// Gates je Klasse (MAINT-001 §3).
    pub fn gate(&self) -> Gate {
        match self {
            MaintenanceClass::M0 => Gate::StandardReview,
            MaintenanceClass::M1 => Gate::OperationalReview,
            MaintenanceClass::M2 => Gate::SecurityReview,
            MaintenanceClass::M3 => Gate::EmergencyGovernance,
        }
    }

    /// REQ-MAINT-020: M2/M3 erzeugen Separation-of-Duties-Pflicht.
    pub fn requires_separation_of_duties(&self) -> bool {
        matches!(self, MaintenanceClass::M2 | MaintenanceClass::M3)
    }
}

impl fmt::Display for MaintenanceClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for MaintenanceClass {
    type Err = CoreError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        MaintenanceClass::parse(s)
    }
}

/// Review-Gate je Klasse (MAINT-001 §3, MAINT-000 §9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gate {
    StandardReview,
    OperationalReview,
    SecurityReview,
    EmergencyGovernance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_classes() {
        for c in MaintenanceClass::ALL {
            assert_eq!(
                c.as_str().parse::<MaintenanceClass>().unwrap(),
                c,
                "roundtrip {}",
                c.as_str()
            );
        }
        assert_eq!(
            "M2".parse::<MaintenanceClass>().unwrap(),
            MaintenanceClass::M2
        );
        assert_eq!(
            "m2".parse::<MaintenanceClass>().unwrap(),
            MaintenanceClass::M2
        );
    }

    #[test]
    fn fails_closed_on_unknown_class() {
        assert!("M4".parse::<MaintenanceClass>().is_err());
        assert!("".parse::<MaintenanceClass>().is_err());
        assert!("routine".parse::<MaintenanceClass>().is_err());
    }

    #[test]
    fn in_doubt_takes_higher_class() {
        // REQ-MAINT-017
        let d = MaintenanceClass::in_doubt(MaintenanceClass::M0, MaintenanceClass::M2);
        assert_eq!(d, MaintenanceClass::M2);
        assert_eq!(
            MaintenanceClass::in_doubt(MaintenanceClass::M3, MaintenanceClass::M1),
            MaintenanceClass::M3
        );
    }

    #[test]
    fn gates_follow_class() {
        assert_eq!(MaintenanceClass::M0.gate(), Gate::StandardReview);
        assert_eq!(MaintenanceClass::M3.gate(), Gate::EmergencyGovernance);
    }

    #[test]
    fn separation_of_duties_only_for_m2_m3() {
        // REQ-MAINT-020
        assert!(!MaintenanceClass::M0.requires_separation_of_duties());
        assert!(!MaintenanceClass::M1.requires_separation_of_duties());
        assert!(MaintenanceClass::M2.requires_separation_of_duties());
        assert!(MaintenanceClass::M3.requires_separation_of_duties());
    }
}
