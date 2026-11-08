# AI Agent Instructions — Aurora AI

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
document_id: ATC-DOC-AI-008
title: Aurora AI Agent Instructions
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-09
updated: 2026-09-09
standard: ATC-STD-MD-001
---

## Identity & Governance
Dieser Bereich definiert die Regeln für automatische AI-Agenten im Repository `aurora-ai`.
Maßgebliche Standards sind `ATC-STD-README-001`, `ATC-STD-MD-001` und `ATC-STD-201`.

## Entry Point
Agenten SOLLEN bei der Navigation folgende Reihenfolge einhalten:
1. [README.md](README.md)
2. [STATUS.md](STATUS.md)
3. [ARCHITECTURE.md](ARCHITECTURE.md)
4. [ROADMAP.md](ROADMAP.md)

## Required Workflow
1. **Status prüfen:** Konsistenz zwischen Code und Dokumentation evaluieren.
2. **Standards lesen:** Relevante ATC-STD Normen konsultieren.
3. **Architektur inspizieren:** Betroffene Module identifizieren.
4. **Aufgabe identifizieren & implementieren:** Änderungen isoliert und testgestützt vornehmen.
5. **Validieren:** Testsuites (`cargo test`, `pytest`) und Validator-Scripts ausführen.
6. **Dokumentieren:** `CHANGELOG.md` aktualisieren und Metadaten/Status pflegen.
7. **Konsistenz prüfen:** Documentation Drift verhindern.

## Commit-Format (ATC-STD-AI-DEV-007 §1, normativ)

Agenten-Commits MÜSSEN einen Trailer-Block tragen (maschinenlesbar):

```text
Agent-ID: ATC-AI-ARCH-001
Task-ID: ATC-TASK-NNNN
AI-Role: software-development
Validation: PASS|FAIL|PENDING
```

Conventional-Commit-Typen: feat|fix|docs|test|refactor|security|build|ci|chore|spec.
