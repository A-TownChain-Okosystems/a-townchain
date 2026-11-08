use crate::{Catalog, Locale, Terminology, TranslationMemory, validate_placeholders};

pub trait TranslationProvider {
    fn translate(&self, source: &str, source_locale: &Locale, target_locale: &Locale, context: Option<&str>) -> Result<String, String>;
}
pub struct LocalizationEngine<P> {
    provider: P,
    pub catalog: Catalog,
    pub memory: TranslationMemory,
    pub terminology: Terminology,
}
impl<P: TranslationProvider> LocalizationEngine<P> {
    pub fn new(provider: P) -> Self {
        Self { provider, catalog: Catalog::default(), memory: TranslationMemory::default(), terminology: Terminology::default() }
    }
    pub fn localize(&mut self, id: &str, source: &str, source_locale: &Locale, target_locale: &Locale, context: Option<&str>) -> Result<String, String> {
        if let Some(value) = self.catalog.resolve(id, target_locale) {
            validate_placeholders(source, &value.text).map_err(|e| format!("placeholder validation failed: {e:?}"))?;
            return Ok(value.text);
        }
        if let Some(value) = self.memory.get(source, target_locale) {
            validate_placeholders(source, value).map_err(|e| format!("placeholder validation failed: {e:?}"))?;
            return Ok(value.to_owned());
        }
        let translated = self.provider.translate(source, source_locale, target_locale, context)?;
        validate_placeholders(source, &translated).map_err(|e| format!("placeholder validation failed: {e:?}"))?;
        self.memory.insert(source.to_owned(), target_locale.clone(), translated.clone());
        Ok(translated)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Echo;
    impl TranslationProvider for Echo {
        fn translate(&self, source: &str, _: &Locale, _: &Locale, _: Option<&str>) -> Result<String, String> {
            Ok(source.replace("Hello", "Hallo"))
        }
    }
    #[test]
    fn uses_provider_and_preserves_placeholders() {
        let de = Locale::try_from("de-DE").unwrap();
        let en = Locale::try_from("en-US").unwrap();
        let mut engine = LocalizationEngine::new(Echo);
        let result = engine.localize("greeting", "Hello {player}", &en, &de, Some("ui")).unwrap();
        assert_eq!(result, "Hallo {player}");
        assert_eq!(engine.memory.get("Hello {player}", &de), Some("Hallo {player}"));
    }
}
