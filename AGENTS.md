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

## Commit-Format (ATC-STD-AI-DEV-007 §1, normativ)

Agenten-Commits MUSSEN einen Trailer-Block tragen (maschinenlesbar):

```
Agent-ID: ATC-AI-ARCH-001
Task-ID: ATC-TASK-NNNN
AI-Role: software-development
Validation: PASS|FAIL|PENDING
```

Conventional-Commit-Typen: feat|fix|docs|test|refactor|security|build|ci|chore|spec.
Ohne Trailer gilt ein Commit als menschlicher Commit (Agentenarbeit wird zurueckgewiesen).
