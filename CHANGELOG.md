# Changelog — atc-algorithm

Alle nennenswerten Aenderungen werden hier dokumentiert (ATC-STD-201 §Doku-Pflichten).

## [Unreleased]
### Fixed
- PoH-Slotarithmetik gegen `u64`-Overflow gehärtet: `tick()` und `verify()` lehnen den Übergang von `u64::MAX` auf `0` mit `SlotOverflow` ab.
- Regressionstest `slot_overflow_is_rejected` ergänzt, damit der Overflow-Fix dauerhaft verifiziert wird.
- PoH behandelt einen unerwartet leeren Tick-Speicher jetzt mit `MissingGenesis` statt eines Panics; Regressionstest `missing_genesis_is_rejected` ergänzt.

## [0.1.0] — 2026-09-07
### Added
- Repo-Gruendung per AD-044 (ATC-Algorithmus — Hybrid Consensus als eigenstaendiges Repo).
- ATC-STD-201 R1-Skeleton: .atc-Metadaten, README, SECURITY, CODEOWNERS, LICENSE.
- Abgrenzung zu a-townchain (Chain), atc-shivacore (Kernel), atc-vm (Vertraege).
