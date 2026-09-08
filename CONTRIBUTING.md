---
document_id: ATC-DOC-ALG-001
title: "Contributing Guidelines — ATC Algorithm"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-09-08
standard: ATC-STD-MD-001
---

# Contributing Guidelines — ATC Algorithm

Vielen Dank für das Interesse, an `atc-algorithm` mitzuwirken!

## Governance & Standards

Dieses Repository ist Teil des A-TownChain-Ökosystems und unterliegt den zentralen Governance-Regeln:
- **ATC-STD-000** (Enterprise Governance Framework)
- **ATC-STD-README-001** & **ATC-STD-MD-001** (Dokumentationsstandards)
- **ATC-STD-201** bis **204** (Software & Security Standards)

## Entwicklungs-Workflow

1. **Issues & Diskussion:**
   Jede größere Änderung oder Architektur-Anpassung sollte vorab als Issue oder Specification Change Request (SCR) besprochen werden.

2. **Branching & Commits:**
   - Erstelle Feature-Branches von `main`.
   - Verwende Conventional Commits (z.B. `feat: ...`, `fix: ...`, `docs: ...`).
   - Füge AI-Agenten-Signaturen in den Commit-Footer ein.

3. **Code Quality & Testing:**
   - Alle Tests müssen lokal vor dem Push erfolgreich laufen (`cargo test` / `pytest`).
   - Sicherheit und Konsens-Korrektheit stehen an erster Stelle (Konsens-Code ist S4-kritisch).

4. **Pull Requests:**
   - Pull Requests erfordern Review und Freigabe durch die Maintainer.
   - Alle CI-Gates müssen grün sein.

## Security

Sicherheitsrelevante Schwachstellen dürfen NICHT über öffentliche GitHub Issues gemeldet werden. Siehe [SECURITY.md](SECURITY.md) für den vertraulichen Meldeweg nach ATC-STD-203.
