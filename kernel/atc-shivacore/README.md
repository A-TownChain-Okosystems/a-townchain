<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-CORE-001
  name: atc-shivacore
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S4
  criticality: C1
-->

# ATC ShivaCore

> Capability-basierter Rust/no_std-Microkernel als wiederverwendbare sicherheitskritische Kernel-Basis für GlobusOS. Das aktive Kernel-Source-of-Truth liegt inzwischen in `globus-os`; dieses Repository enthält unterstützendes ShivaCore-Material.

**Project:** `atc-shivacore`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Version:** `0.1.0`  
**Production:** `NOT_READY`  
**License:** `Apache-2.0`

## Role and Scope

## Purpose

Provide reusable ShivaCore kernel contracts, HAL/ABI/boot specifications, governance and audit support. The active kernel implementation remains in `globus-os/modules/atc-shivacore/kernel/`.

ShivaCore stellt den wiederverwendbaren Kernel-/TCB-Vertrag des Ökosystems bereit. Der aktive kanonische Kernel-Quellbaum wird in `globus-os/modules/atc-shivacore/kernel/` gebaut und verifiziert. Dieses Repository ist kein zweiter kanonischer Kernel-Source-of-Truth.

**In scope:**

- Kernel-/TCB-Spezifikationen und Governance
- unterstützendes Kernel- und Boot-Material
- Wiederverwendungs-, HAL-, ABI- und Boot-Verträge
- Dokumentation und Audit-Nachweise

**Out of scope:**

- der kanonische aktive Kernel-Quellbaum (siehe `globus-os`)
- GlobusOS-Userspace und Systemdienste
- Aurora AI
- A-TownChain-Protokoll und Chain-State
- ATCLang-Contracts und ATC-VM
- Game-/GameFi-Anwendungen

## Features

Documented contract areas include capabilities/CSpace, memory and address spaces, scheduling and threads, IPC endpoints, timers, traps, HAL, ABI and boot integration. These topics describe the contract scope and do not independently prove implementation or production readiness.

## Architecture

```text
Firmware / Bootloader
        ↓
Architecture HAL
        ↓
ShivaCore Microkernel / TCB
  ├─ Capabilities / CSpace
  ├─ Memory / Address Spaces
  ├─ Scheduling / Threads
  ├─ IPC / Endpoints
  └─ Timers / Traps
        ↓
GlobusOS platform integration
```

Die Wiederverwendbarkeit wird durch den Kernel-, HAL-, ABI- und Boot-Vertrag definiert. Änderungen am kanonischen Kernel müssen im `globus-os` Repository erfolgen, damit dieselbe CI- und Integrationskette den TCB prüft.

## Current Implementation Boundary

Historische oder experimentelle Kernel-Bestandteile in diesem Repository sind nicht automatisch Teil des normativen Microkernel-Vertrags. OS-, Blockchain- oder AI-Semantik darf nicht ohne Governance und TCB-Review in den Kernvertrag aufgenommen werden.

## Requirements

- Rust für vorhandene Rust-Tools und unterstützende Komponenten
- Python nur für explizit dokumentierte Hilfsskripte
- Git
- Für kanonische Kernel-Builds gelten die Toolchain- und Target-Anforderungen von `globus-os`

## Installation

Dieses Repository ist primär ein Governance-/Dokumentations- und Support-Repository. Der kanonische Kernel-Build erfolgt im `globus-os` Repository:

```bash
git clone https://github.com/A-TownChain-Okosystems/globus-os.git
cd globus-os
cargo test --workspace
```

Vor einem Build sind die aktuellen `globus-os` Manifest-, Target- und CI-Vorgaben maßgeblich.

## Testing

Kanonische Kernel-Tests müssen aus dem aktuellen `globus-os` CI-Lauf stammen. Historische Testzahlen in diesem Repository gelten nicht als permanente Verifikation.

## Documentation

- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`STATUS.md`](STATUS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`SECURITY.md`](SECURITY.md)
- [`docs/specs/SHIVA-KERNEL-REUSE-001.md`](docs/specs/SHIVA-KERNEL-REUSE-001.md)
- [`docs/specs/SHIVA-HAL-001.md`](docs/specs/SHIVA-HAL-001.md)
- [`docs/specs/SHIVA-ABI-001.md`](docs/specs/SHIVA-ABI-001.md)
- [`docs/specs/SHIVA-BOOT-001.md`](docs/specs/SHIVA-BOOT-001.md)

## Security

ShivaCore ist sicherheitskritische Infrastruktur. Sicherheitslücken nicht öffentlich über GitHub Issues veröffentlichen; den in `SECURITY.md` definierten Disclosure-Prozess verwenden.

Die Verlagerung des kanonischen Kernel-Quellbaums in `globus-os` reduziert die Gefahr divergierender TCB-Implementierungen: Build, Tests, Lints und Integration werden an einer Stelle ausgeführt. Das ist eine Architekturkontrolle, aber kein Beweis vollständiger Angriffs- oder Malware-Immunität.

## Development and Governance

- Änderungen folgen `ATC-STD-000` und dem aktuellen ATC-Governance-Prozess.
- Architekturänderungen mit TCB-Auswirkung benötigen dokumentierte Governance-/Review-Evidence.
- Conventional Commits verwenden.
- Neue family-scoped Standard-IDs verwenden `ATC-STD-F{family}-{sequence}`.
- Der kanonische Kernel-Source-of-Truth darf nicht in diesem Repository dupliziert werden.

## Blockchain Boundary

```text
ATCLang → ATC-VM → A-TownChain
```

ShivaCore stellt keine Chain-Semantik und keine feste Chain-ID bereit.

## Roadmap

Die operative Kernel-Roadmap und Umsetzungsplanung werden am kanonischen Implementierungsort `globus-os` geführt. Dieses Repository dokumentiert nur ShivaCore-spezifische Verträge, Governance und Support-Material.

## Version

Die Repository-Version ist `0.1.0`. Änderungen am kanonischen Kernel werden über die Versions- und Release-Prozesse von `globus-os` nachgewiesen.

## Compliance

![ATC COMPLIANCE](https://img.shields.io/badge/ATC%20COMPLIANCE-R3%20%C2%B7%20ATC--STD--201%2F202%2F203-brightgreen)

Der aktuelle Repository-Zustand wird als **development / NOT_READY** geführt. `APPROVED`, `IMPLEMENTED`, `AUDITED` und `PRODUCTION_READY` sind unabhängige Zustände und dürfen nicht aus Dokumentation allein abgeleitet werden.

## License

Apache-2.0. Siehe [`LICENSE`](LICENSE).

## AI Agent Instructions

Vor Änderungen mindestens `AGENTS.md`, `AGENT_MANIFEST.md`, `ARCHITECTURE.md`, `STATUS.md` und `ROADMAP.md` prüfen. TCB-, Capability-, Boot- oder Sicherheitsgrenzen dürfen nicht gegen die Canonical-Source-Regel verschoben werden.

## ShivaCore relocation

The canonical ShivaCore kernel source is `A-TownChain-Okosystems/globus-os/modules/atc-shivacore/kernel/`. This repository retains supporting ShivaCore tooling, specifications and governance material.

## Historical template archive

The legacy template material has been retained in `docs/archive/README-template-legacy.txt`; it is not the canonical repository description.
