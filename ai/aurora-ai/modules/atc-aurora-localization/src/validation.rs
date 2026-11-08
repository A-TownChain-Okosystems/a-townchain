use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationError { PlaceholderMismatch { expected: Vec<String>, actual: Vec<String> } }

fn placeholders(text: &str) -> Vec<String> {
    let mut found = BTreeSet::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 2 <= bytes.len() {
        if bytes[i] == b'{' {
            if let Some(end) = text[i + 1..].find('}') {
                let end = i + 1 + end;
                let name = &text[i + 1..end];
                if !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') { found.insert(name.to_string()); }
                i = end + 1;
                continue;
            }
        }
        i += 1;
    }
    found.into_iter().collect()
}
pub fn validate_placeholders(source: &str, translated: &str) -> Result<(), ValidationError> {
    let expected = placeholders(source);
    let actual = placeholders(translated);
    if expected == actual { Ok(()) } else { Err(ValidationError::PlaceholderMismatch { expected, actual }) }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_reordered_placeholders() { assert!(validate_placeholders("Hello {player} — {count}", "{count} x für {player}").is_ok()); }
    #[test]
    fn rejects_missing_placeholder() { assert!(validate_placeholders("Hello {player}", "Hallo").is_err()); }
}
