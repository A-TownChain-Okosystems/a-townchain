//! SemVer-Subset-Helfer (deterministisch, abhängigkeitsfrei).
//!
//! Phase 0/1: numerische `x.y.z`-Schlüssel für Pinning-Prüfungen (SPEC-001 §25).
//! Pre-Release-Suffixe werden nicht unterstützt — fail-closed via `is_semver`.

/// Numerischer Vergleichsschlüssel; non-numerische Segmente zählen als 0.
pub fn semver_key(v: &str) -> (u64, u64, u64) {
    let mut parts = [0u64; 3];
    for (i, p) in v.split('.').take(3).enumerate() {
        parts[i] = p
            .split(|c: char| !c.is_ascii_digit())
            .next()
            .and_then(|d| d.parse().ok())
            .unwrap_or(0);
    }
    (parts[0], parts[1], parts[2])
}

/// Strenges numerisches `x.y.z` (kein Wildcard, kein Pre-Release).
pub fn is_semver(v: &str) -> bool {
    let segs: Vec<&str> = v.split('.').collect();
    segs.len() == 3
        && segs.iter().all(|s| {
            !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) && !s.starts_with('0')
                || *s == "0"
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        assert!(semver_key("1.3.0") < semver_key("1.10.0"));
        assert!(semver_key("2.0.0") > semver_key("1.9.9"));
    }

    #[test]
    fn strict_semver_check() {
        assert!(is_semver("1.0.0"));
        assert!(!is_semver("1.0"));
        assert!(!is_semver("1.x"));
        assert!(!is_semver("*"));
        assert!(is_semver("1.0.10"));
    }
}
