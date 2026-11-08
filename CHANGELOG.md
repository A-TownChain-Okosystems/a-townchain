# Changelog — a-townchain

## [Unreleased] — 2026-11-08

### Fixed
- Boot-Hash im Node-RPC war auf 64 Bit trunkniert (SEC-P0-007-Rest): `Genesis::boot_hash()`
  und `townhash_u64` entfernt, `devnet_boot` liefert den vollen 32-Byte-Bindungs-Hash,
  `BOOT_HASH`/`boot_hash` liefern 64-stelliges Hex. CLI-Client lehnt kurze oder nicht-hex
  Antworten ab, damit ein Rueckfall auf ein Traversal auffaellt.
- Contract-Registry zeigte auf nicht existierende Python-Pfade; 16 ATCLang-Quellen sind
  neu registriert und per `source_sha256` gebunden.

### Added
- `tools/atc-sc-validator/check_contract_registry.py`: Gate A1-A8 fuer die Contract-Registry
  (SC-019), inklusive ATCLang-Kompilierpruefung, 16 Regressionstests.

## [0.1.0] - 2026-09-08

### Added
- Rollout README-001/MD-001 Documentation Compliance.

### Changed
- Governance-Ueberarbeitung nach ATC-STD-201/202/203: .atc-Metadaten (.atc/), SECURITY.md, CODEOWNERS, docs/REPOSITORY_STANDARD.md.
