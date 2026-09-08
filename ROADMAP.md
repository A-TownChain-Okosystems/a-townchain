---
document_id: ATC-DOC-ALG-003
title: "Roadmap — ATC Algorithm"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-09-08
standard: ATC-STD-MD-001
---

# Roadmap — ATC Algorithm

## Meilensteine & Phase

- **Phase 1 (R1-Skeleton):** Grundeinrichtung, Governance-Verankerung, Metadaten und Dokumentations-Konformität (ATC-STD-README-001 & MD-001). [Abgeschlossen]
- **Phase 2 (Spezifikation):** Konsolidierung der PoH-Zeitstempel-Spezifikation (SHA-256 Hashkette nach AD-001) und formale Definition der Hybrid-Auswahlregeln (PoH/PoS/PoW). [In Arbeit]
- **Phase 3 (Rust-Implementierung):** Kanonische Rust-Kette für PoH-Ticks, Validator-Auswahl und Fork-Choice-Rule (AD-021 Rust-first).
- **Phase 4 (Integration & Fuzzing):** Anbindung an `a-townchain` Node, Fuzzing-Tests und Konsistenzprüfung.
- **Phase 5 (Audit & Freeze):** Security-Audit (S4-kritisch, G18-Gate nach AD-023).

Die primäre Entwicklungssteuerung erfolgt über das A-TownChain Development Management und GitHub Issues/Projects.
