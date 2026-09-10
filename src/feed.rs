// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Preis-/Daten-Feed-Aggregation: Median + Abweichungs-Gate (ORACLE-001..006, MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub source: u64,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedError {
    TooFewSources { have: usize, need: usize },
}

/// Median der Quellen; Ablehnung unter min_sources (Untrusted-Quellen-Modell).
pub fn aggregate_median(reports: &[Report], min_sources: usize) -> Result<u64, FeedError> {
    if reports.len() < min_sources {
        return Err(FeedError { have: reports.len(), need: min_sources });
    }
    let mut vals: Vec<u64> = reports.iter().map(|r| r.value).collect();
    vals.sort_unstable();
    Ok(vals[vals.len() / 2])
}

/// Jeder Wert darf vom Median hoechstens max_dev abweichen.
pub fn check_deviation(reports: &[Report], max_dev: u64) -> bool {
    match aggregate_median(reports, 1) {
        Ok(median) => reports.iter().all(|r| r.value.abs_diff(median) <= max_dev),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(s: u64, v: u64) -> Report { Report { source: s, value: v } }

    #[test]
    fn median_und_min_sources() {
        let rs = vec![r(1, 10), r(2, 20), r(3, 30)];
        assert_eq!(aggregate_median(&rs, 2), Ok(20));
        assert_eq!(aggregate_median(&rs, 4), Err(FeedError { have: 3, need: 4 }));
    }

    #[test]
    fn deviation_gate() {
        let ok = vec![r(1, 100), r(2, 102), r(3, 104)];
        assert!(check_deviation(&ok, 5));
        let bad = vec![r(1, 100), r(2, 102), r(3, 999)];
        assert!(!check_deviation(&bad, 5));
    }
}
