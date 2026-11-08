use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Locale(String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocaleError;

impl Locale {
    pub fn as_str(&self) -> &str { &self.0 }
    pub fn language(&self) -> &str { self.0.split('-').next().unwrap_or(&self.0) }
}
impl FromStr for Locale {
    type Err = LocaleError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        if value.is_empty() || value.len() > 35 || !value.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') { return Err(LocaleError); }
        let mut parts = value.split('-');
        let language = parts.next().unwrap_or_default();
        if language.len() < 2 || language.len() > 8 || !language.bytes().all(|b| b.is_ascii_alphabetic()) { return Err(LocaleError); }
        if parts.any(|p| p.is_empty() || p.len() > 8 || !p.bytes().all(|b| b.is_ascii_alphanumeric())) { return Err(LocaleError); }
        Ok(Self(value.replace('_', "-")))
    }
}
impl TryFrom<&str> for Locale {
    type Error = LocaleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> { value.parse() }
}
impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}
