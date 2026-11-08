---
spec_id: ATC-INTEROP-002
title: "Relayer = Untrusted Transport"
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

# Relayer = Untrusted Transport (ATC-INTEROP-002)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Der Relayer ist vollständig untrusted.

## 2. Scope
- relayer, Architektur

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Relayer darf nie: State autorisieren, Finality behaupten, Mint/Burn autorisieren, Proofs validieren, Quorum bestimmen, Governance entscheiden **[Nachweis: audit+negative]**
- **REQ-002:** Alle Autorität liegt in der Proof-Verificationsschicht **[Nachweis: design]**

## 4. Invarianten
- Kompromittierter Relayer → max. Verzögerung, nie falsche Ausführung

## 5. Conformance-Tests (Mindestkategorien)
- Malicious-Relayer-Suite

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INTEROP P0-2

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
