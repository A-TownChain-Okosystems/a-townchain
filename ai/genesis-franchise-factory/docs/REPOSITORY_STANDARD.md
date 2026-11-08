# Repository Standard — genesis-franchise-factory

**Klassifizierung:** GAME · **Maturity:** R1 · **Layer:** L6 · **Domain:** franchise-factory
**Registry-ID:** ATC-REPO-GFF-001 · **Tier:** T3 · **Criticality/Security:** C3/S2

Dieses Repository folgt den ATC-Repository-Standards (kanonisch im
[atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards)):

- **ATC-STD-201** Repository Structure (Metadaten, Struktur, Compliance-Matrix)
- **ATC-STD-202** Repository Naming & Classification (R0-R4, S0-S4, Namensregeln ATC-STD-000 §7)
- **ATC-STD-203** Repository Security & Release (Branching, Conventional Commits, Release-Gates)

## Zwei-Ebenen-Entscheidungsmodell (AD-029)

1. **Zentral:** docs/DECISIONS_REGISTER.md im Docs-Hub — AD-Nummern sind
   verbindlich und haben Vorrang (ATC-STD-000 §31: hoechste Governance-Prioritaet).
2. **Lokal:** repo-spezifische Entscheidungen in docs/decisions/ (ADR-NNN-Form,
   ATC-STD-000 §7.2).

## Struktur

```
genesis-franchise-factory/
├── .atc/            Metadaten (repository.yaml, evidence/evidence.yaml)
├── .github/         CI (governance-ci.yml, test-suite.yml, codeql, dependency-review)
├── docs/            REPOSITORY_STANDARD.md
├── specs/           Kanonische Factory-Specs (AD-20..43, ATC-9900, Contracts)
├── gff/             Referenz-Implementierung (core, dao, lifecycle, spec_loader)
├── tools/           validate_specs.py (CI-Spec-Gate)
├── tests/           pytest-Suite (33 Tests)
├── README.md · ARCHITECTURE.md · LICENSE
```

## Plattform-Trennung (CI-erzwungen)

GFF importiert niemals Genesis Chronicles — `tools/validate_specs.py`
blockt `import *chronicles*` in allen `.atc`-Dateien (Quality-Gate,
verbindlich in test-suite.yml).

## Evidence (SCR-0080)

`.atc/evidence/evidence.yaml` ist die ehrliche SSOT: implementation partial,
tests implemented (33 PASS), security not_audited, bound_commit gesetzt.
