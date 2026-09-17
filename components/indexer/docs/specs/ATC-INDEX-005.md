---
spec_id: ATC-INDEX-005
title: "SLO- & Security-Baseline"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-indexer
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# SLO- & Security-Baseline (ATC-INDEX-005)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Messbare SLOs; Derived-Data-Regel.

## 2. Scope
- Gesamt, analytics

## 3. Normative Anforderungen (MUST)
- **REQ-001:** SLO-Katalog: Throughput, Latenz P50/95/99, Recovery-Zeit, Reorg-Recovery — mit CI-Benchmark-Harness **[Nachweis: benchmark]**
- **REQ-002:** Derived-Data-Regel: Indexer-Daten nie ungeprüft als Chain-Truth zurück in Konsens/Execution; Analytics ist Consumer, nie Ersatz **[Nachweis: audit+negative]**
- **REQ-003:** Threat-Katalog: malicious node, oversized events, injection, rate limiting, exhaustion, replay, index corruption **[Nachweis: design]**

## 4. Invarianten
- High-Performance erst nach SLO-Nachweis eine Aussage

## 5. Conformance-Tests (Mindestkategorien)
- Bench-Baseline
- Injection/Fuzz → reject

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INDEX P1-13/15

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
