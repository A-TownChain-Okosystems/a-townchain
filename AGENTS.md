---
document_id: ATC-DOC-MIN-AGE-001
title: AI Agent Instructions — atc-mining
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# AGENTS.md — atc-mining

## Identität & Standards

Zuständiger Agent: Aurora Superagent (`6a2756186106d6f0fbb105b5`).
Maßgebliche Standards: ATC-STD-000, ATC-STD-201, ATC-STD-202, ATC-STD-203, ATC-STD-README-001, ATC-STD-MD-001, ATC-STD-SC-018.

## Entry Point

Der verbindliche Dokumentationseinstieg folgt dieser Reihenfolge:
`README.md` -> `STATUS.md` -> `ARCHITECTURE.md` -> `ROADMAP.md` -> `CHANGELOG.md`.

## Required Workflow

1. **Status prüfen**: `STATUS.md` und CI-Gate-Status einsehen.
2. **Standards lesen**: Vorgaben aus `atc-standards` beachten.
3. **Architektur inspizieren**: `ARCHITECTURE.md` konsultieren vor Code-Änderungen.
4. **Implementieren**: Code-Änderungen in Rust/Python vornehmen.
5. **Testen**: Unit- und Integrationstests ausführen (`cargo test`).
6. **Dokumentieren**: README/STATUS/CHANGELOG aktualisieren bei Drift.
7. **Commit**: Conventional Commit mit Tag `[agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5]`.
