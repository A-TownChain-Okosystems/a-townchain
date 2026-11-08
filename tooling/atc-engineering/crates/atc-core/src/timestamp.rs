//! Timestamps der Domain (deterministisch: keine Systemuhr im Kern).
//!
//! `atc-core` liest NIE die Systemzeit (Determinismus, SPEC-001-Philosophie).
//! Timestamps werden als Werte hereingegeben und validiert.

use crate::error::{CoreError, Result};
use std::fmt;

/// Unix-Timestamp in Sekunden, positiv, aus Evidenz-Inputs übernommen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn from_unix(secs: u64) -> Self {
        Self(secs)
    }

    pub fn as_unix(&self) -> u64 {
        self.0
    }

    /// ISO-8601-ähnliche, deterministische Formatierung (UTC, ohne Zeitzonen).
    /// Vollständige RFC-3339-Konvertierung folgt mit Phase-0-Konventionen.
    pub fn to_iso8601_date(&self) -> Result<String> {
        // Zivile Datumsumrechnung nach Howard Hinnant (deterministisch, keine externen Krates).
        let days = (self.0 / 86_400) as i64;
        let secs_of_day = self.0 % 86_400;
        let z = days + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        if y < 1 {
            return Err(CoreError::Unsupported(
                "timestamp before epoch-0 date".into(),
            ));
        }
        let hh = secs_of_day / 3600;
        let mm = (secs_of_day % 3600) / 60;
        let ss = secs_of_day % 60;
        Ok(format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z"))
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_instants_format_correctly() {
        // 2026-09-14T00:00:00Z
        let ts = Timestamp::from_unix(1_789_344_000);
        assert_eq!(ts.to_iso8601_date().unwrap(), "2026-09-14T00:00:00Z");
        assert_eq!(
            Timestamp::from_unix(0).to_iso8601_date().unwrap(),
            "1970-01-01T00:00:00Z"
        );
    }

    #[test]
    fn ordering_and_access() {
        assert!(Timestamp::from_unix(10) > Timestamp::from_unix(9));
        assert_eq!(Timestamp::from_unix(42).as_unix(), 42);
    }
}
