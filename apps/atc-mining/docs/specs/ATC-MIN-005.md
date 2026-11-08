---
spec_id: ATC-MIN-005
title: "HAL & Mining-Protokoll"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-mining
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# HAL & Mining-Protokoll (ATC-MIN-005)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Backend-Abstraktion und node-seitiges Protokoll.

## 2. Scope
- backends, network

## 3. Normative Anforderungen (MUST)
- **REQ-001:** MinerBackend-Trait: submit_job, start, pause, stop, get_hashrate, get_temperature, get_power, get_capabilities **[Nachweis: design+unit]**
- **REQ-002:** ATC-MIN-PROTO-001: job request/response, block template, difficulty update, proof/share submission, Auth, Replay-Schutz, Timeouts, Versionierung **[Nachweis: design]**
- **REQ-003:** Kein Ethereum-JSON-RPC als Annahme; localhost:8545-Config ist als Kompatibilitätsartefakt zu prüfen **[Nachweis: review]**

## 4. Invarianten
- Backends austauschbar ohne Protokolländerung

## 5. Conformance-Tests (Mindestkategorien)
- Backend-Failover
- Malformed job → reject

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit MIN-P1-008/009

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
