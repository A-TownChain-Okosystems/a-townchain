---
document_id: ATC-DOC-STORAGE-AGENTS-001
title: AI Agent Instructions - atc-storage
version: 1.0.0
status: active
standard: ATC-STD-MD-001
created: 2026-09-08
updated: 2026-09-08
---

# AI Agent Instructions — atc-storage

## Identity & Normative Standards

This repository adheres strictly to:
- **ATC-STD-000:** Verfassung & Change Control
- **ATC-STD-README-001:** README Standard
- **ATC-STD-MD-001:** Documentation & Markdown Standard
- **ATC-STD-201 / 202 / 203:** Repository Governance, Layout, and Security

## Entry Point Sequence

When working on this repository, AI agents MUST follow this sequence:
1. `README.md` — Entry point & high-level overview
2. `STATUS.md` — Machine-readable build and test status
3. `ARCHITECTURE.md` — Storage architecture & design rules
4. `ROADMAP.md` — Active development goals
5. `CHANGELOG.md` — Historical record of changes

## Required Workflow

1. Check repository status in `STATUS.md`.
2. Inspect relevant architecture specifications in `ARCHITECTURE.md`.
3. Perform implementation or bug fixes.
4. Execute test suite: `cargo test`.
5. Update `CHANGELOG.md` following Keep a Changelog syntax.
6. Verify documentation consistency using `check_readme.py` and `check_md.py`.
