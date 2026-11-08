use std::collections::BTreeMap;
use crate::Locale;

#[derive(Clone, Debug, Default)]
pub struct Terminology { terms: BTreeMap<String, BTreeMap<Locale, String>> }
impl Terminology {
    pub fn set(&mut self, source: impl Into<String>, locale: Locale, target: impl Into<String>) {
        self.terms.entry(source.into()).or_default().insert(locale, target.into());
    }
    pub fn resolve(&self, source: &str, locale: &Locale) -> Option<&str> {
        let targets = self.terms.get(source)?;
        targets.get(locale)
            .or_else(|| targets.iter().find(|(l, _)| l.language() == locale.language()).map(|(_, v)| v))
            .map(String::as_str)
    }
}
