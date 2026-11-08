---
document_id: ATC-DOC-ENGINE-CONTR-001
title: Contributing Guidelines — genesis-engine
version: 1.0.0
status: active
standard: ATC-STD-MD-001
date: 2026-09-07
---

# Contributing to ATC Genesis Engine

Vielen Dank für Ihr Interesse an Beiträgen zur Genesis Engine der A-TownChain-Ökosystems.

## Governance & Regeln

Beiträge unterliegen dem **A-TownChain Enterprise Governance Framework** (ATC-STD-000, ATC-ENT-001..015).

1. **Standard-Compliance:** Alle Änderungen müssen ATC-STD-000, ATC-STD-201, ATC-STD-202 und ATC-STD-203 einhalten.
2. **Branching & Commits:** Verwenden Sie Feature-Branches und Conventional Commits (`feat: ...`, `fix: ...`, `docs: ...`).
3. **Tests:** Jeder Pull Request muss bestehende Tests bestehen (`cargo test --workspace`) und neue Tests für geänderte Logik enthalten.
4. **Dokumentation:** Aktualisieren Sie bei relevanten Änderungen `ARCHITECTURE.md`, `CHANGELOG.md` und `STATUS.md`.

## Workflow

1. Forken oder erstellen Sie einen Feature-Branch.
2. Führen Sie Ihre Änderungen durch.
3. Testen Sie lokal: `cargo test --workspace` und prüfen Sie Markdown-Compliance via `check_readme.py` / `check_md.py`.
4. Reichen Sie einen Pull Request ein.
