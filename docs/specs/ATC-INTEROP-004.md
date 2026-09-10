---
spec_id: ATC-INTEROP-004
title: "Threshold & Validator-Set"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-interop
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# Threshold & Validator-Set (ATC-INTEROP-004)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
N/M-Threshold-Signing mit Lifecycle.

## 2. Scope
- validators, contracts

## 3. Normative Anforderungen (MUST)
- **REQ-001:** N/M (z. B. 7/5) konfiguriert und genesis-gelockt; Keygen, Rotation, Registration, Revocation spezifiziert **[Nachweis: design+config]**
- **REQ-002:** Aggregation deterministisch: kanonische Reihenfolge, Duplikate zählen einfach, Byzantine-Handling definiert **[Nachweis: unit+negative]**
- **REQ-003:** Validator-Set-Updates sind Governance-geschützt; Relayer kann Set NIE ändern **[Nachweis: audit]**

## 4. Invarianten
- Quorum-1 wird nie akzeptiert

## 5. Conformance-Tests (Mindestkategorien)
- quorum-1 → reject
- Duplicate signature → single count
- Epoch-Wechsel

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INTEROP P1-7

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
