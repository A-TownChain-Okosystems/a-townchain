use std::collections::BTreeMap;
use crate::Locale;

#[derive(Clone, Debug, Default)]
pub struct TranslationMemory { entries: BTreeMap<(String, Locale), String> }
impl TranslationMemory {
    pub fn insert(&mut self, source: impl Into<String>, locale: Locale, target: impl Into<String>) {
        self.entries.insert((source.into(), locale), target.into());
    }
    pub fn get(&self, source: &str, locale: &Locale) -> Option<&str> {
        self.entries.get(&(source.to_string(), locale.clone())).map(String::as_str)
            .or_else(|| self.entries.iter().find(|((s, l), _)| s == source && l.language() == locale.language()).map(|(_, v)| v.as_str()))
    }
}
