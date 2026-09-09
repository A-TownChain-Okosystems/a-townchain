# AGENTS.md — a-townchain

## Org-Regeln (vererbt — Pflicht für jeden Agenten in diesem Repo)

Dieses Repository unterliegt dem **ATC Org-weiten Agent-Governance-System** (SCR-0057):
[.github-Hub](https://github.com/A-TownChain-Okosystems/.github) — Org-AGENTS.md
(Arbeits-Sequenz + Hierarchie-Kaskade), agent-instructions/00-11,
ai/policies.yaml (**AP-001..016, normativ**), ai/capabilities.yaml (8 Rollen
ATC-AI-ARCH/AUDIT/SEC/CI/DOC/TEST/RELEASE/GOV-001), ai/agent.yaml.

Repo-spezifische Regeln ERGÄNZEN die Org-Regeln; keine höhere Security-,
Compliance- oder Governance-Regel darf stillschweigend ausgehebelt werden.
Kaskade: Org-Policy → AGENT_MANIFEST → Org-AGENTS.md → dieses Dokument → Task.

> Repo-spezifische Agenten-Regeln sind noch nicht ausformuliert — bis dahin gilt der Org-Standard vollständig.


## Commit-Format (ATC-STD-AI-DEV-007 §1, normativ)

Agenten-Commits MUESSEN einen Trailer-Block tragen:

Agent-ID: ATC-AI-ARCH-001
Task-ID: ATC-TASK-NNNN
AI-Role: software-development
Validation: PASS|FAIL|PENDING

Conventional-Commit-Typen: feat|fix|docs|test|refactor|security|build|ci|chore|spec. Ohne Trailer gilt ein Commit als menschlicher Commit.
