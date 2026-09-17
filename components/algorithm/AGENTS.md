# AI Agent Instructions — ATC Algorithm

## Org-Regeln (vererbt — Pflicht für jeden Agenten in diesem Repo)

Dieses Repository unterliegt dem **ATC Org-weiten Agent-Governance-System** (SCR-0057):
[.github-Hub](https://github.com/A-TownChain-Okosystems/.github) — Org-AGENTS.md
(Arbeits-Sequenz + Hierarchie-Kaskade), agent-instructions/00-11,
ai/policies.yaml (**AP-001..016, normativ**), ai/capabilities.yaml (8 Rollen
ATC-AI-ARCH/AUDIT/SEC/CI/DOC/TEST/RELEASE/GOV-001), ai/agent.yaml.

Repo-spezifische Regeln ERGÄNZEN die Org-Regeln; keine höhere Security-,
Compliance- oder Governance-Regel darf stillschweigend ausgehebelt werden.
Kaskade: Org-Policy → AGENT_MANIFEST → Org-AGENTS.md → dieses Dokument → Task.

---
document_id: ATC-DOC-ALG-005
title: "AI Agent Instructions — ATC Algorithm"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-09-08
standard: ATC-STD-MD-001
---

## Identity & Standards

AI-Agenten, die in diesem Repository arbeiten, müssen strikt folgende Standards einhalten:
- **ATC-STD-000** (A-TownChain Enterprise Governance Framework)
- **ATC-STD-README-001** (README Standard)
- **ATC-STD-MD-001** (Markdown & Documentation Standard)
- **ATC-STD-201** bis **204** (Quality & Security Standards)

## Entry Point

1. Read `README.md`
2. Read `STATUS.md`
3. Read `ARCHITECTURE.md`
4. Read `ROADMAP.md`
5. Read `CONTRIBUTING.md`

## Required Workflow

1. **Status prüfen:** In `STATUS.md` den aktuellen Stand und offene Punkte ablesen.
2. **Standards lesen:** Relevante ATC-STD Normen konsultieren.
3. **Architektur inspizieren:** `ARCHITECTURE.md` und Code-Struktur analysieren.
4. **Aufgabe identifizieren & umsetzen:** Minimal-invasive, standardkonforme Änderungen durchführen.
5. **Testing & Validierung:** Lokale Tests ausführen und Validatoren prüfen (`check_readme.py`, `check_md.py`).
6. **Dokumentation & Changelog:** `CHANGELOG.md` aktualisieren und Konsistenz sicherstellen.
7. **Commit & Push:** Conventional Commit mit Agent-Signatur erstellen.

## Commit-Format (ATC-STD-AI-DEV-007 §1, normativ)

Agenten-Commits MUESSEN einen Trailer-Block tragen:

Agent-ID: ATC-AI-ARCH-001
Task-ID: ATC-TASK-NNNN
AI-Role: software-development
Validation: PASS|FAIL|PENDING

Conventional-Commit-Typen: feat|fix|docs|test|refactor|security|build|ci|chore|spec. Ohne Trailer gilt ein Commit als menschlicher Commit.
