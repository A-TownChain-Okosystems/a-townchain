@@STAMP@@
# ATC Standards

[![ATC COMPLIANCE](https://img.shields.io/badge/ATC-COMPLIANCE-green) ](README.md) <!-- formal: PASS, R12-enforced (SCR-0096; Scan-Konvention ATC-COMPLIANCE) -->

## State (GENERIERT — Quelle: `registry/standards.yaml`, SCR-0090/@@STD@@)

```yaml
state:
  id: ATC-STATE-@@DATE@@-@@SHA8@@
  generated_at: "@@NOW@@"
  registry_version: "@@REGVER@@"
  registry_sha256: "@@SHA@@"
  standards_total: @@TOTAL@@
  standards_approved: @@APPROVED@@
  standards_candidate: @@CANDIDATE@@
  standards_other: @@OTHER@@
  standard_files: @@FILES@@
  families: @@FAMS@@
```

**Diese Zahlen sind die EINZIG maschinenverbindliche Auskunft** (@@STD@@ §2 Ein-Zahl-Regel).
Historische Zahlenstände: ausschließlich `STATUS.md` / `CHANGELOG.md` / `audits/`.

**Implementierungs-KPI:** @@TOTAL@@ Standards normativ definiert — @@ENF@@ enforced, @@IMPL@@ implemented, @@SPEC@@ specification-only. Die Aussage „@@TOTAL@@ Standards implementiert“ ist unzulässig (@@STD@@ §8: APPROVED ≠ IMPLEMENTED).

**FORMALE COMPLIANCE: @@FCOMP@@** · **IMPLEMENTATION: @@ICOMP@@** · **PRODUCTION READINESS: @@PRD@@** — ein Zustand behauptet nie den anderen.

---

## Status

**Status:** `release-candidate` — aktueller normativer Bestand und Umsetzungsgrad ausschließlich aus dem State-Block oben.

## Purpose

ATC Standards is the canonical normative governance layer of the A-TownChain ecosystem. It maintains:

- **Registry:** SSOT für alle @@TOTAL@@ Standards mit Versionierung, Dependencies (DAG) und Findings
- **Verfassung:** ATC-STD-000 v@@VER000@@, @@ST000@@
- **Validator-Suite:** atc-std-validator, atc-repo-audit, atc-readme-validator
- **Governance Framework:** @@FAMS@@ Familien
- **Governance-Determinismus:** @@STD@@ (SSOT-Matrix, State-ID, Ein-Zahl-Regel)

**Organisationsbestand:** @@REPONUM@@ Repository-Einträge im Repository-Registry-SSOT. Governance-Status und Ausnahmen werden ausschließlich aus `registry/repositories.yaml` bzw. den daraus generierten Compliance-Views abgeleitet.

## Scope

Governance-Root der A-TownChain-Organisation: Registry, Standards, Schemata, Validatoren, Audits und Change-Requests. In Scope: normative Standards (@@FILES@@ Dateien, @@FAMS@@ Familien) und ihre Metadaten/Validierung. Out of Scope: Implementierung der Standards in den Repository-Einträgen der Organisation und deren CI-Gates.

## Features

- Registry-SSOT (`standards.yaml`, SHA-Lock) mit schema-validierten Metadaten
- Validatoren und Repository-Audit
- Implementierungs-Matrix — kein Status ohne Evidence
- Governance-CI
- Discovery-Familie für neue Inhalte

## Architecture

### Core Components
- `registry/` — SSOT
- `standards/` — @@FILES@@ Standard-Dateien in @@FAMS@@ Familien
- `profiles/` — @@REPONUM@@ verbindliche Standards-Profile je Repository
- `schemas/` — JSON/YAML-Schemata
- `tools/` — Validators, Auditors, Generators
- `governance/`, `approval/`, `change-requests/` — Verfassung, Freigaben und SCR-System

### Usage

```bash
python3 tools/atc-std-validator/validate_all.py
python3 tools/atc-repo-audit/atc_repo_audit.py . --level R3
```

## Installation

Requirements: Python 3.11+ and PyYAML.

```bash
python3 -m pip install --disable-pip-version-check pyyaml
python3 tools/atc-std-validator/validate_all.py
```

## Governance Flow
Change Request → SCR → Owner-Freigabe (§9) → Registry-Eintrag → CI-Validierung → APPROVED → normativ in Kraft.

## Quick Start

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-standards.git
cd atc-standards
python3 -m pip install pyyaml
```

## Governance

This repository is governed according to ATC-STD-000 v@@VER000@@ (@@ST000@@):

- Changes to APPROVED Standards: via SCR only
- New Standards: Registry-First
- AI Agents: Executor/Auditor/Maintainer, never Approver
- CI Gates: Standards-Validierung, Repo-Audit, Agent-Manifest-Enforcement

See `CONTRIBUTING.md` and `governance/ATC-STD-000.md` for details.

## Standards & Compliance

| Standard | Version | Status |
|---|---|---|
@@COMPTABLE@@

See `registry/standards.yaml` for the complete registry.

## Development

Änderungen laufen über SCR. Registry-Änderungen nur via kontrolliertem Change-Request. Views sind GENERIERT.

## Testing

```bash
python3 tools/atc-std-validator/validate_all.py
python3 tools/atc-repo-audit/atc_repo_audit.py . --level R3
python3 tools/atc-readme-validator/check_readme.py .
```

## Roadmap

Governance-Hardening: Registry-Integritäts-Gate, Discovery→Audit→SCR-Kopplung, Evidence-L3 für kritische Standards und CI-Evidence-Kopplung der Implementierungs-Matrix.

## Repository Structure

```text
@@REPOSTRUCT@@
```

## Documentation

- Governance: `governance/`, `STATUS.md`, `ROADMAP.md`
- Audits: `docs/audits/`
- Change History: `change-requests/`, `CHANGELOG.md`
- Wiki: `a-townchain-os-docs`

## Security

Security issues must NOT be disclosed via GitHub Issues. Use GitHub Private Vulnerability Reporting or `SECURITY.md`.

## Maintainers

**Organization:** A-TownChain-Okosystems · **Owner:** See `governance/authority/authority-matrix.yaml` and the applicable §9 approval record. Historical approval documents may contain former identity metadata and are not by themselves the current authority SSOT.

## License

**Apache-2.0** (see `LICENSE`). Additional ATC usage terms are governed by the ATC-LICENSE family.

## Metadata

- **Project:** atc-standards
- **Organization:** A-TownChain-Okosystems
- **Status:** `release-candidate`
- **Version:** @@REPOVER@@
- **Registry ID:** ATC-REPO-GOV-001

Machine-readable metadata: State-Block oben; Registry-SSOT: `registry/standards.yaml`.

## Registry-vs-Dateien-Relation

Regel: Jeder Registry-Eintrag besitzt eine Datei in `standards/` ODER eine dokumentierte EXEMPT-Begründung. Gültigkeit und Zahlen werden je Lauf aus dem Registry-SSOT geprüft.
