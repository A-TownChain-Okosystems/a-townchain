# AI Agent Instructions — ATC Launchpad

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
document_id: ATC-DOC-LPD-AGENTS-001
title: AI Agent Instructions — ATC Launchpad
status: active
standard: ATC-STD-MD-001
owner: A-TownChain-Okosystems
version: 1.0.0
created: 2026-09-08
updated: 2026-09-08
---

## Identity & Mandat
Dieses Repository gehört zum A-TownChain-Ökosystem (Layer L5) und folgt strikt den normativen Standards `ATC-STD-README-001` und `ATC-STD-MD-001`.

## Entry Point Sequence
1. `README.md` — Einstiegspunkt und Repository-Überblick
2. `STATUS.md` — Aktueller System- und Audit-Status
3. `ARCHITECTURE.md` — Technische Architektur und Modulstruktur
4. `ROADMAP.md` — Geplante Meilensteine und Entwicklungsphasen
5. `CHANGELOG.md` — Historiendokumentation aller Releasestände

## Required Workflow
1. Status prüfen (`STATUS.md`)
2. Einschlägige Standards lesen (`ATC-STD-000`, `ATC-STD-README-001`, `ATC-STD-MD-001`)
3. Architektur inspizieren (`ARCHITECTURE.md`)
4. Aufgabe identifizieren & implementieren
5. Tests ausführen (`cargo test` / `pytest`)
6. Dokumentation aktualisieren & Drift verhindern
7. Changelog fortschreiben (`CHANGELOG.md`)

## Commit-Format (ATC-STD-AI-DEV-007 §1, normativ)

Agenten-Commits MUESSEN einen Trailer-Block tragen:

Agent-ID: ATC-AI-ARCH-001
Task-ID: ATC-TASK-NNNN
AI-Role: software-development
Validation: PASS|FAIL|PENDING

Conventional-Commit-Typen: feat|fix|docs|test|refactor|security|build|ci|chore|spec. Ohne Trailer gilt ein Commit als menschlicher Commit.
