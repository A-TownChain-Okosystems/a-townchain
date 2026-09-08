---
document_id: ATC-DOC-INDEXER-007
title: "AI Agent Instructions"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# AI Agent Instructions — ATC Chain Indexer & Analytics

## Identity & Standards

- **ATC-STD-README-001** (README Standard)
- **ATC-STD-MD-001** (Markdown Standard)

## Entry Point

1. Read `README.md`
2. Read `STATUS.md`
3. Read `ARCHITECTURE.md`

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
