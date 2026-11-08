# AI Agent Instructions

## Org-Regeln (vererbt — Pflicht für jeden Agenten in diesem Repo)

Dieses Repository unterliegt dem **ATC Org-weiten Agent-Governance-System** (SCR-0057):
[.github-Hub](https://github.com/A-TownChain-Okosystems/.github) — Org-AGENTS.md
(Arbeits-Sequenz + Hierarchie-Kaskade), agent-instructions/00-11,
ai/policies.yaml (**AP-001..016, normativ**), ai/capabilities.yaml (8 Rollen
ATC-AI-ARCH/AUDIT/SEC/CI/DOC/TEST/RELEASE/GOV-001), ai/agent.yaml.

Repo-spezifische Regeln ERGÄNZEN die Org-Regeln; keine höhere Security-,
Compliance- oder Governance-Regel darf stillschweigend ausgehebelt werden.
Kaskade: Org-Policy → AGENT_MANIFEST → Org-AGENTS.md → dieses Dokument → Task.

## Identity
Dieser Bereich definiert die Regeln und Workflows für KI-Agenten, die in `a-townchain-os` arbeiten. Maßgebliche Standards: ATC-STD-000, ATC-STD-README-001, ATC-STD-MD-001, ATC-AAS-001..025.

## Entry Point
1. [README.md](README.md) — Identität, Zweck und Übersicht
2. [STATUS.md](STATUS.md) — Aktueller System- und Gate-Status
3. [ARCHITECTURE.md](ARCHITECTURE.md) — Systemarchitektur L7
4. [ROADMAP.md](ROADMAP.md) — Meilensteine M1–M8
5. [CHANGELOG.md](CHANGELOG.md) — Änderungshistorie

## Required Workflow
1. **Status prüfen:** Inspect `STATUS.md` and active issues.
2. **Standards lesen:** Verify applicable standards in `atc-standards`.
3. **Architektur inspizieren:** Align with `ARCHITECTURE.md` and L7 boundaries.
4. **Aufgabe umsetzen:** Implement changes following Conventional Commits.
5. **Tests & Doku:** Run `cargo test --workspace` and update documentation.
6. **Konsistenz prüfen:** Execute `check_readme.py` and `check_md.py` validators.

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