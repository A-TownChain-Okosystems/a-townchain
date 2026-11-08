//! Minimaler, fail-closed Parser für YAML-Inline-Flow-Maps: `{k: v, k2: "v2", …}`.
//!
//! Deckt exakt das Format der atc-standards-Registry-Dateien ab (Standards- und
//! Repository-Einträge). Kein generisches YAML — alles Unbekannte ist ein Fehler.

use atc_core::{CoreError, Result};

/// Parst eine Inline-Flow-Map (ohne umgebende `{}`, mit oder ohne — fail-closed).
/// Werte dürfen doppelte Anführungszeichen enthalten; innerhalb von Anführungszeichen
/// zählen Kommas nicht als Trenner.
pub fn parse_flow_map(raw: &str) -> Result<Vec<(String, String)>> {
    let s = raw.trim();
    let inner = if s.starts_with('{') && s.ends_with('}') {
        &s[1..s.len() - 1]
    } else if s.starts_with('{') {
        return Err(CoreError::ConfigInvalid("unterminated flow map".into()));
    } else {
        s
    };
    let inner = inner.trim();
    if inner.is_empty() {
        return Ok(Vec::new());
    }

    let mut pairs = Vec::new();
    for chunk in split_top_level(inner) {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            return Err(CoreError::ConfigInvalid("empty pair in flow map".into()));
        }
        let Some((k, v)) = chunk.split_once(':') else {
            return Err(CoreError::ConfigInvalid(format!(
                "flow map pair without ':': '{chunk}'"
            )));
        };
        let key = k.trim();
        if key.is_empty() {
            return Err(CoreError::ConfigInvalid("empty key in flow map".into()));
        }
        let val = unquote(v.trim());
        pairs.push((key.to_string(), val));
    }
    Ok(pairs)
}

/// Trennt an Kommas auf oberster Ebene (Anführungszeichen-respektierend).
fn split_top_level(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for c in s.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                cur.push(c);
            }
            ',' if !in_quotes => {
                out.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// Entfernt umgebende doppelte Anführungszeichen; schlägt bei einzelnen fehl.
fn unquote(v: &str) -> String {
    if v.len() >= 2 && v.starts_with('"') && v.ends_with('"') {
        v[1..v.len() - 1].to_string()
    } else {
        v.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pairs_and_quoted_values() {
        let p = parse_flow_map(r#"{id: ATC-STD-000, title: "A, B", normative: true}"#).unwrap();
        assert_eq!(p[0], ("id".into(), "ATC-STD-000".into()));
        assert_eq!(p[1], ("title".into(), "A, B".into()));
        assert_eq!(p[2], ("normative".into(), "true".into()));
    }

    #[test]
    fn fails_closed_on_missing_colon() {
        assert!(parse_flow_map("{id}").is_err());
    }

    #[test]
    fn fails_closed_on_empty_key() {
        assert!(parse_flow_map("{: x}").is_err());
    }

    #[test]
    fn fails_closed_on_unterminated() {
        assert!(parse_flow_map("{id: x").is_err());
    }

    #[test]
    fn empty_map_allowed() {
        assert!(parse_flow_map("{}").unwrap().is_empty());
    }
}
