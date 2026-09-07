# AI Agent Instructions

## Identity
Dieser Bereich definiert die Regeln für KI-Agenten in `atc-vm`. Maßgebliche Standards: ATC-STD-000, ATC-STD-README-001, ATC-STD-MD-001, ATC-STD-203.

## Entry Point
1. [README.md](README.md) — Systemidentität und Übersicht
2. [STATUS.md](STATUS.md) — Systemstatus und Gates
3. [ARCHITECTURE.md](ARCHITECTURE.md) — ATVM Architektur
4. [ROADMAP.md](ROADMAP.md) — Entwicklungsphasen
5. [CHANGELOG.md](CHANGELOG.md) — Historie

## Required Workflow
1. **Status prüfen:** Read `STATUS.md` and active tickets.
2. **Standards lesen:** Verify applicable standards in `atc-standards`.
3. **Architektur beachten:** ATVM ist Trust Boundary (S4). Strikte Validierung einhalten.
4. **Rust-first Implementierung:** Execute updates with tests (`cargo test`).
5. **Doku & Changelog:** Maintain documentation and update `CHANGELOG.md`.
6. **Validators:** Run `check_readme.py` and `check_md.py`.
