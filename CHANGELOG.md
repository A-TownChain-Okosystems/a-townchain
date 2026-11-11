# Changelog — a-townchain

## [Unreleased] — 2026-11-11

### Added (Integrationspfad: Quelle -> Artefakt -> Freigabe -> VM -> Kette)
- Neues Crate `components/system` (Paket `atc-system`, Workspace-Mitglied): verbindet die
  bisher getrennten Stufen zu **einem** Pfad und ist ausdruecklich Integrations- und
  Kontrollschicht, keine zweite VM und kein zweiter Compiler.
  - `backend.rs` — Uebersetzer von ATCLang-Bytecode (`atc-core`) auf ATCB-Operationen
    (`atc-vm`). Sprungziele werden absolut umgerechnet, die PC-Zuordnung wird
    mitgeliefert. Alles, was nicht exakt abbildbar ist, wird abgelehnt: `call` (die VM
    hat keine Aufrufkonvention), `Neg`, `Pop`, negative Konstanten, Sprungziele
    ausserhalb des Programms, Programme ueber 65 536 Operationen.
  - `seal.rs` — Freigabe mit echter Signatur: secp256k1-ECDSA ueber den SHA-256 der
    kanonischen Policy-Bytes, Pruefung gegen den oeffentlichen Schluessel. Eine gueltige
    Signatur bleibt von der Autorisierung des Signierers getrennt.
  - `state.rs` — Vertragszustand und Zustandswurzel (Domain, Code-Hash, Woerter).
  - `system.rs` — Deployment (kompilieren, abbilden, Artefakt, Freigabe, Registrierung)
    und Ausfuehrung (Artefakt aus den Bytes, Freigabe, Zustand aus der Kette, Gas,
    Blockbuchung). Faellt eine Stufe aus, entsteht kein Block und keine
    Zustandsaenderung.
  - `main.rs` — CLI `atc-system demo | run <datei.atc> [--gas N]`.
- `components/vm/src/vm.rs`: `Vm::from_artifact_with_context(artifact, caller, storage)`.
  Die Kette liefert den Zustand, die Generation kommt weiterhin ausschliesslich aus dem
  Artefakt; ein eigener Test haelt beides gleichzeitig fest.
- Evidenz: `docs/audits/SYSTEM-INTEGRATION-2026-11-11.md` (Gates A-E, Messwerte,
  Evidenzstufen, offene Punkte) und `docs/decisions/VM-INTEGER-SEMANTICS-DRAFT.md`.

### Verified (lokal, 11.11.2026 — Stufe TESTED, nicht CI_VERIFIED)
- `cargo test --workspace`: 662 bestanden, 0 fehlgeschlagen; davon 46 in `atc-system`
  (13 Striktheit, 11 Differential, 11 E2E, 11 Signatur).
- `cargo test -p atc-vm`: 132 Tests in fuenf Suiten.
- `cargo clippy -p atc-system --all-targets --no-deps -- -D warnings`: 0;
  `rustfmt --check`: 0.
- `atc-system demo`: 19 ATCLang-Instruktionen -> 20 ATCB-Operationen, 138-Byte-Artefakt,
  Ergebnis 10, Blockhoehe 2, `Chain::verify` gruen.

### Known limits (nicht behoben, bewusst benannt)
- **Ueberlauf und Vorzeichen**: der Referenzweg rechnet `i64` geprueft, die VM `u64`
  umlaufend. Der Entwurf fuer die verbindliche Semantik liegt als
  `docs/decisions/VM-INTEGER-SEMANTICS-DRAFT.md`; nichts davon ist umgesetzt, beide
  Faelle sind als Test festgehalten und maschinenlesbar als Divergenz markiert.
- Keine Aufrufkonvention: nur die Einstiegsfunktion wird uebersetzt, weitere Funktionen
  sind unerreichbar.
- Lokale werden Speicherplaetze (offenes Speichermodell), kein Nonce, kein
  Gasverbrauch im Beleg, keine persistente Authority.
- **Keine CI-Evidenz**: `git push` scheitert an der unbetaetigten Committer-Mail,
  Uebertragung ueber die API; `CI_VERIFIED` bleibt offen.

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
