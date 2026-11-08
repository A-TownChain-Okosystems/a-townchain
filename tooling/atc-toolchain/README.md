# ATC Toolchain

> **ATC COMPLIANCE: R1** — initialer Audit via governance-ci.yml (ATC-STD-201/202/203), R-Level aus `.atc/repository.yaml`.

> Eigenständige Entwicklungs-, Build- und Governance-Werkzeuge für das A-TownChain-Ökosystem. Die Toolchain bündelt CLI, Build-Orchestrierung, Codegenerierung und Audit-Runner als from-scratch Eigenentwicklung (AD-Mandat: kein POSIX-Klon, keine externen Forks).

**Project:** atc-toolchain  
**Organization:** A-TownChain-Okosystems  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0 — A-TownChain-Okosystems`  
**Standard:** `ATC-STD-README-001`  
**Maintainer:** A-TownChain-Okosystems (ShivaCoreDev)

## Overview

`atc-toolchain` ist die kanonische Werkzeugkette des A-TownChain-Ökosystems. Sie verbindet die verteilten Per-Repo-Werkzeuge (Validatoren, Audit-Runner, Generatoren, Sync-Utilities) hinter einer einheitlichen CLI und Build-Orchestrierung.

Die Toolchain unterstützt die Schichten der Bauhierarchie:

- **atclang (L0):** Compiler-Treiber, Format-Checks, Gate-Runner.
- **atc-shivacore (L1):** Kernel-Build-Orchestrierung, Spec-Gates (SC-001ff).
- **a-townchain / Services (L3/L5):** Komponenten-Builds, Sync-Skripte, Release-Gates.
- **atc-standards:** Audit-Runner-Integration (atc-repo-audit), Standard-Validatoren.

## Purpose

ATC Toolchain provides the canonical developer tooling within the A-TownChain ecosystem. It is responsible for:

- Build-Orchestrierung über die Repo-Grenzen hinweg (Layer-Hierarchie).
- Einheitliche CLI für gängige Entwicklungs- und Governance-Operationen.
- Audit- und Validator-Runner-Integration (Registry-geprüft).
- Codegenerierung und Scaffolding nach AD-036-Naming.

## Scope

- **Gilt für:** Werkzeuge, Runner, Generatoren und CLI des Ökosystems.
- **Nicht-Gilt für:** Die Ziel-Software selbst — Sprache, Kernel, Chain und Services bleiben in ihren kanonischen Repos (Produkt-Repo-Regel AD-017).

## Status

**Status:** `development` — R1-Skelett. Struktur und Governance sind definiert; die Implementierung folgt den Gates der Lauffähigkeits-Roadmap.

Kein Production- oder Mainnet-Claim wird allein aus README-Status abgeleitet (SCR-0080: CLAIMED != PASS).

## Architecture

```text
atc CLI (atc-tc)
     │
     ├── build        # Build-Orchestrierung (Layer-geprüft)
     ├── audit        # atc-repo-audit / Standard-Validatoren
     ├── gen          # Scaffolding / Generatoren (AD-036-Naming)
     └── sync         # Modul-Sync (AD-017-Regelwerk)
```

### Components

- `CLI`: Einheitliche Einstiegsfläche (geplant, R1-Skelett).
- `Audit-Runner-Integration`: Anbindung an atc-repo-audit und Standard-Validatoren.
- `Sync-Utilities`: Modul-Sync nach AD-017-Regelwerk.

### Dependencies

| Component | Purpose | Required |
|---|---|---|
| `atc-standards` | Governance, Standards, atc-repo-audit | Yes |
| `atclang` | L0-Ziel der Build-Orchestrierung | No |
| `a-townchain` | Integration Sync-Ziel | No |

## Features

- Einheitliche CLI (geplant, R1).
- Governance-CI ab erster Stunde (AD-039-Sweep-Lektion).
- Registry-Anbindung an `atc-standards/registry/repositories.yaml`.

## Repository Structure

```text
.
├── docs/                # Dokumentation und Standard-Referenz
├── .atc/                # Repository-Metadaten + Evidence (SSOT)
└── src/                 # Werkzeug-Quellcode (folgt)
```

## Requirements

- Python >= 3.11 (stdlib-first, wie atc-repo-audit)
- Git >= 2.30

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-toolchain.git
```

## Usage

R1-Skelett — CLI folgt. Governance-Audit läuft automatisch via `governance-ci.yml` bei jedem Push.

## Development

Entwicklung erfolgt stdlib-first (Python) mit Conventional Commits. Werkzeuge greifen nur über definierte Schnittstellen auf Ziel-Repos zu; Mutationsrechte bleiben bei den Ziel-Repos.

## Testing

Tests folgen mit der Implementierung; kein Test-Claim ohne Evidence-Bundle (ATC-STD-MILESTONE-001).

## Security

Security issues werden gemäß ATC-STD-203 und dem offiziellen ATC-Security-Reporting-Prozess behandelt, nicht über öffentliche GitHub Issues.

## Documentation

- `docs/REPOSITORY_STANDARD.md` — Repo-spezifischer Standard
- `a-townchain-os-docs` — zentrale technische Dokumentation

## Governance

Dieses Repository unterliegt `ATC-STD-000` und den jeweils geltenden A-TownChain-Standards. Standard-IDs werden ausschließlich über Registry und Governance vergeben.

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.2.0 | ✅ |
| ATC-STD-203 | 1.0.1 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |

## Roadmap

Siehe GitHub Issues/Projects und die kanonischen Dokumentationsquellen.

## Contributing

Beiträge erfolgen gemäß den Governance-Regeln von `ATC-STD-000`.

## License

Apache-2.0 — A-TownChain-Okosystems. Siehe `LICENSE`.

## Maintainers

**Organization:** A-TownChain-Okosystems  
**Maintainer:** ShivaCoreDev / Aurora Superagent

## Changelog

Initial: R1-Skelett (05.10.2026).

## Repository Metadata

<!-- atc metadata block (ATC-STD-README-001 §14) -->
<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-TOOL-001
  name: atc-toolchain
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Python
governance:
  security_class: S2 — Developer Tools
  criticality: C3
-->
