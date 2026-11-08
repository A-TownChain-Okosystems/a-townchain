# ATC-AI-LOC-001 — Aurora Localization Contract

Status: IMPLEMENTED (core); model-provider integration: PENDING

Canonical module: modules/atc-aurora-localization

## Contract
1. Localization is an Aurora AI service and remains outside the ShivaCore TCB.
2. Locale identifiers are validated before use.
3. Exact locale resolution precedes language fallback.
4. Translation Memory is consulted before model inference.
5. Terminology is explicit and available to provider integrations.
6. Source placeholders must be preserved exactly as a set.
7. Model inference is provider-independent through TranslationProvider.
8. No provider/API key is embedded in the core.
9. The core remains deterministic and testable without network access.

## Roadmap
- v0.1 deterministic core — implemented.
- v0.2 .loc/.locpack serialization.
- v0.3 provider adapters and confidence metadata.
- v0.4 asset/UI/voice localization services.
