# ShivaCore Kernel — aktuelle Architekturgrenze

> **Aktualisiert:** 2026-10-09  
> **Status:** Architektur-Referenz; kein Architektur-Freeze- oder Produktionsfreigabenachweis.

## Source of truth

Der aktive kanonische Kernel-Quellbaum liegt in [`globus-os/modules/atc-shivacore/kernel/`](https://github.com/A-TownChain-Okosystems/globus-os/tree/main/modules/atc-shivacore/kernel). Das Repository [`atc-shivacore`](https://github.com/A-TownChain-Okosystems/atc-shivacore) hält unterstützende Spezifikationen, Governance und Support-Material; es ist kein zweiter kanonischer Kernel.

## Sicherheitsmodell

ShivaCore ist als Rust/no_std-capability-Microkernel und Trusted Computing Base (TCB) vorgesehen. Der Kernel erzwingt Isolation und Rechte; er trifft keine KI-Entscheidungen und enthält keine Chain- oder Vertragsautorität.

```text
Hardware / Firmware
       ↓
HAL + Boot boundary
       ↓
ShivaCore kernel / TCB
  ├─ capability enforcement
  ├─ address-space / memory primitives
  ├─ scheduler / threads
  ├─ IPC / endpoints
  └─ traps / interrupts / timers
       ↓
GlobusOS service space
  ├─ networking / storage / drivers
  ├─ system identity and wallet services
  ├─ ATC chain / node services
  └─ Aurora AI runtime and agents
       ↓
Applications
```

Diese Skizze beschreibt Zielgrenzen und ist kein Beleg dafür, dass jede Subsystem- oder Hardwareintegration vollständig umgesetzt wurde.

## Kernel vs. Service Space

- Der TCB bleibt minimal und capability-basiert; keine ambient authority.
- Netzwerkprotokolle, komplexe Dateisysteme, GPU-/Audio-/USB-Dienste, Blockchain-Runtime, Vertragsausführung und Aurora AI gehören in Service Space, nicht in den Kernel.
- Prozesse und Dienste erhalten nur explizit zugewiesene Capabilities; privilegierte Aktionen müssen an der zuständigen Grenze geprüft werden.
- Ein Service darf nicht allein aufgrund seiner Integration in GlobusOS als sicherheitsgeprüft gelten.

## Status und offene Evidenz

Der Entwicklungsstatus von GlobusOS/ShivaCore ist **IMPLEMENTED / BLOCKED — Development Kernel**, nicht final, nicht vollständig verifiziert und nicht produktionsreif. Zuletzt dokumentierte offene Bereiche umfassen die TCB-Architekturabweichung, VMM Page-Table-Mutation/Rollback, Mapping-/Object-Lifetime sowie Rust-/Cargo- und UEFI Secure-Boot-/ACPI-Evidenz. Der aktuelle Stand dieser Blocker muss im kanonischen `globus-os`-Status und den aktuellen Workflows überprüft werden.

## Verifikation

`IMPLEMENTED` beschreibt vorhandene Implementierung. `VERIFIED` erfordert relevante Tests und Evidenz auf dem exakten SHA; Kernel-Security-Audit, Hardware-Boot und Release-Readiness sind getrennte Gates. Historische Testzahlen und Architekturentscheidungen ersetzen diese Evidenz nicht.
