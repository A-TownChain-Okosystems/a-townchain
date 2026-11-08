use std::collections::BTreeMap;
use crate::Locale;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalizedString { pub locale: Locale, pub text: String }
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatalogEntry {
    pub id: String,
    pub source: String,
    pub context: Option<String>,
    pub translations: BTreeMap<Locale, String>,
}
#[derive(Clone, Debug, Default)]
pub struct Catalog { entries: BTreeMap<String, CatalogEntry> }

impl Catalog {
    pub fn insert(&mut self, entry: CatalogEntry) -> Option<CatalogEntry> { self.entries.insert(entry.id.clone(), entry) }
    pub fn get(&self, id: &str) -> Option<&CatalogEntry> { self.entries.get(id) }
    pub fn resolve(&self, id: &str, locale: &Locale) -> Option<LocalizedString> {
        let entry = self.entries.get(id)?;
        let text = entry.translations.get(locale)
            .or_else(|| entry.translations.iter().find(|(l, _)| l.language() == locale.language()).map(|(_, t)| t))?;
        Some(LocalizedString { locale: locale.clone(), text: text.clone() })
    }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolves_exact_then_language_fallback() {
        let de = Locale::try_from("de-DE").unwrap();
        let de_ch = Locale::try_from("de-CH").unwrap();
        let mut translations = BTreeMap::new();
        translations.insert(de.clone(), "Hallo".into());
        let mut catalog = Catalog::default();
        catalog.insert(CatalogEntry { id: "hello".into(), source: "Hello".into(), context: None, translations });
        assert_eq!(catalog.resolve("hello", &de).unwrap().text, "Hallo");
        assert_eq!(catalog.resolve("hello", &de_ch).unwrap().text, "Hallo");
    }
}
