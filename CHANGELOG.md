# Changelog — a-townchain

## [Unreleased] — 2026-11-11

### Changed (Doppelungsaufloesung der Kernel-Ebene)
- `kernel/globus-os` entfernt: 482 Dateien, 54 davon abweichend vom Kanon `globus-os`,
  18 Dateien, die dort nicht existieren (14 davon byte-gleich zu
  `globus-os/modules/atc-shivacore/legacy/kernel-src-uncompiled`), 58 kanonische
  Dateien fehlten. Die Kopie war nicht baubar (nicht im Workspace, seit dem
  07.11. ohne `init_program`, ohne die integrierten Treiber) und wurde von
  `tools/ecosystem.sh` dennoch still gebaut. Kanonisch ist allein das Repo `globus-os`.
- `kernel/kai-os` entfernt: 81 Dateien, bis auf `README.md` datei-identisch zu
  `a-townchain-os/kai-os`. Kanonisch ist allein das Repo `a-townchain-os`.
- `Cargo.toml`: die beiden `exclude`-Eintraege entfallen, weil die Pfade nicht mehr
  existieren; `kernel/atc-shivacore` bleibt Workspace-Mitglied (einziger Kernel-Pfad,
  der hier gebaut wird).
- `tools/consolidate_monorepo.py`: beide Repos in `NOT_CONSOLIDATED` mit Begruendung,
  damit die Kopien nicht durch einen Lauf wiederkehren.
- `tools/ecosystem.sh`: Kernel-Bau und -Boot laufen ueber `GLOBUS_OS_DIR`
  (Vorgabe `../globus-os`) und melden `uebersprungen`, wenn kein Auschecken da ist,
  statt aus einer Kopie zu bauen.

### Verified
- Keine Datei mit eigenem Inhalt verloren: die 14 Kernel-Legacy-Dateien sind
  byte-gleich im Kanon vorhanden, `system/media/src/sync.rs` war die am 07.11.
  bewusst geloeschte Dublette, `service_space/src/ai.rs` der am 11.11. nach
  `atc-aurora-tensor` extrahierte Tensor-Kern. Die Monorepo-Historie behaelt alles.

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
