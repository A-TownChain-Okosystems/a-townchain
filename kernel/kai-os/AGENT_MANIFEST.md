# AGENT_MANIFEST.md

> **Generated-state declaration:** This manifest is a governance pointer, not a second standards registry. The authoritative standards SSOT is `A-TownChain-Okosystems/atc-standards/registry/standards.yaml`; implementation status is authoritative in `registry/standard-implementation.yaml`.
>
> **Registry snapshot:** 505 standards · 75 families · implementation matrix dated 2026-09-14. Regenerate this manifest when the registry changes.

## Normative mandate

The responsible agent MUST comply with all standards applicable to this repository under `ATC-STD-IMPLEMENTATION-001`:

- `MANDATORY`: MUST be implemented and enforced.
- `CONDITIONAL`: MUST be implemented when applicable.
- `REFERENCE`: informative unless another normative rule makes it applicable.
- `NOT_APPLICABLE`: requires an explicit recorded justification.

`ATC-STD-000 v1.3.0` is the governance authority. The standards registry is the source of truth; this manifest MUST NOT contain a copied static list that can drift from the registry.

## Required bindings

This repository MUST maintain the applicable ATC-STD-201 maturity metadata and agent-governance artifacts, including:

- `.atc/repository.yaml`
- `.atc/ownership.yaml`
- `.atc/lifecycle.yaml`
- `.atc/compliance.yaml`
- `.github/ai/agent.yaml`
- `AGENTS.md`
- `AGENT_MANIFEST.md`
- `.github/workflows/` with governance and product validation gates

## Enforcement

`DISCOVER → CLASSIFY → DOCUMENT → IMPLEMENT/FIX → TEST → RE-AUDIT → VERIFY → DOCUMENT STATE`

Compliance claims require current evidence. README/status claims do not replace validator, source, or runtime evidence.

The organization-wide offline standards auditor is:

`atc-engineering/scripts/standards_enforcement_audit.py`

Historical documentation MUST NOT override current normative artifacts.

## Current audit state

The organization-wide audit dated 2026-09-16 remains **IN PROGRESS**. Known open findings include the GlobusOS LKM dependency/symbol-resolution issues. They remain open until source changes and executable verification evidence are available.
