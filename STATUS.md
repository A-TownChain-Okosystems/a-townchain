---
document_id: ATC-DOC-TOWNCHAIN-004
title: "Project Status"
version: 1.1.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-09-14
standard: ATC-STD-MD-001
---

# Project Status — ATC A-TownChain Core

| Property | Value |
|---|---|
| Repository | a-townchain |
| Version | 0.1.0 |
| Status | development |
| Implementation | partial / prototype |
| Build | PASS with evidence |
| Tests | PASS WITH EVIDENCE |
| Security | NOT AUDITED |
| Conformance | NOT VERIFIED |
| Production Readiness | NO-GO |
| Mainnet Date | NONE APPROVED |
| Last Evidence Update | 2026-09-12 |

## Status Summary

Das Repository `a-townchain` befindet sich im Status `development`. Die Evidence-SSOT weist ausdrücklich darauf hin, dass der Blockchain-Kern nur teilweise implementiert ist und **nicht production-ready** ist.

`a-townchain` ist die Orchestrierungs- und Integrationsschicht. Die kanonische Konsenslogik liegt in `atc-algorithm`. Der aktuelle M4-Integrationsstand ist kein Mainnet-Nachweis.

## Release Gate

Ein Release über `development` hinaus erfordert mindestens:

1. kanonische Konsens-Spezifikation und Freeze,
2. vollständige Konsens-Implementierung in `atc-algorithm`,
3. ATVM-Determinismus und End-to-End-Conformance,
4. kryptografische Spezifikation und unabhängige Prüfung,
5. reproduzierbaren Build,
6. Security-Audit,
7. vollständiges Evidence-Bundle und Audit-Freigabe.

**Mainnet: NO-GO.** Ein Mainnet-Termin ist derzeit nicht freigegeben.
