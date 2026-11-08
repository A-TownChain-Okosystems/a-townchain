# ATC Aurora Localization

Provider-independent localization core for Aurora AI.

## Scope
- Locale validation and language fallback.
- Translation Memory.
- Terminology glossary.
- Placeholder integrity validation.
- Provider abstraction for AI/LLM translation.
- Catalog resolution before model inference.

The crate does not claim to be an AI model. Model inference is supplied through TranslationProvider.

## Status
IMPLEMENTED: deterministic core.
PENDING: model/provider adapters and .loc/.locpack serialization.
