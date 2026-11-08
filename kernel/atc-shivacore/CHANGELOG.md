---
document_id: ATC-DOC-SHIVACORE-CHANGELOG-001
title: Changelog - atc-shivacore
version: 1.0.0
status: active
standard: ATC-STD-MD-001
created: 2026-09-08
updated: 2026-09-08
---

# Changelog — atc-shivacore

All notable changes to `atc-shivacore` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-08

### Added
- **K14-Upgrade: P2P v1.0.0 (ATC-PROTO-P2P-001 implementiert, SCR-0028).**
  NEU `kernel/src/p2p_secure.rs` (~1250 Zeilen inkl. 29 Unit-Tests):
  - Envelope 9+1 Pflichtfelder mit kanonischer Byte-Serialisierung (§3.1)
  - Message-Types 10..13 für 6-Phasen-Handshake (§8)
  - Peer-Lifecycle, Anti-Replay, TokenBucket Rate-Limiting (§14/§15)
- Baseline-Dokumentation gemäß ATC-STD-README-001 und ATC-STD-MD-001.

### Changed
- Version-Baseline 0.1.0 für kernel/boot/service_space vereinheitlicht.
- Lizenz-Metadatum korrigiert: Apache-2.0 (SPDX) (SCR-0036).

### Fixed
- Maintenance-Befunde Zyklus 1 (ATC-STD-REPO-MAINT-001).
- 19 TODO/FIXME inventarisiert.
