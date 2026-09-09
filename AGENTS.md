# AGENTS.md — atc-mining

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
document_id: ATC-DOC-MIN-AGE-001
title: AI Agent Instructions — atc-mining
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

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