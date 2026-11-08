---
spec_id: ATC-INTEROP-006
title: "Fail-Closed-Norm"
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

# Fail-Closed-Norm (ATC-INTEROP-006)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Unsicherheit MUSS nie zur Ausführung führen.

## 2. Scope
- verifier, Gesamt

## 3. Normative Anforderungen (MUST)
- **REQ-001:** REJECT-Liste: invalid proof, unknown validator, insufficient quorum, unknown chain, unknown epoch, replay, finality uncertain, malformed, clock violation **[Nachweis: negative]**
- **REQ-002:** REJECT-Entscheidungen mit Grund geloggt **[Nachweis: audit]**

## 4. Invarianten
- Fail-Closed ist Default jeder Unsicherheit

## 5. Conformance-Tests (Mindestkategorien)
- Jede REJECT-Kategorie als Negativ-Test

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INTEROP P1-9

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
