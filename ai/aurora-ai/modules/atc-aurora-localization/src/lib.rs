//! Aurora AI Localization Core.
mod catalog;
mod engine;
mod locale;
mod memory;
mod terminology;
mod validation;

pub use catalog::{Catalog, CatalogEntry, LocalizedString};
pub use engine::{LocalizationEngine, TranslationProvider};
pub use locale::{Locale, LocaleError};
pub use memory::TranslationMemory;
pub use terminology::Terminology;
pub use validation::{validate_placeholders, ValidationError};
