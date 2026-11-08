---
spec_id: LP-SALE-001
title: "Sales Engine Specification (Token-/NFT-Sales)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-launchpad
layer: L5-Launchpad
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Sales Engine Specification (Token-/NFT-Sales) (LP-SALE-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Normativer Zustandsautomat und Kappen-Regeln für Token-/NFT-Sales.

## 2. Scope (gilt für)

- Zustandsmaschine DRAFT→CONFIGURED→WHITELIST_OPEN→PUBLIC_SALE→SOLD_OUT/ENDED→CLAIMABLE→SETTLED
- Hard/Soft Cap, Min-/Max-Contribution
- Zeitfenster (Slot-basiert)

## 3. Normative Anforderungen (MUST)

- **REQ-LPS-001:** Zustandsübergänge nur in der fixierten Reihenfolge; illegale Übergänge (z. B. PUBLIC_SALE→WHITELIST_OPEN) ⇒ Reject — *Nachweis: unit+negative*
- **REQ-LPS-002:** Invarianten je Sale: TotalAllocated ≤ TotalSupply (Hard Cap); Contribution je Adresse in [min, max]; Unterschreitung von Soft Cap am Ende ⇒ Refund-Phase (LP-SALE-002 analog) — *Nachweis: property+unit*
- **REQ-LPS-003:** Zeitfenster sind Slot-/Height-basiert (kein Wallclock-Trust); Start/End außerhalb ⇒ Contribution-Reject — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- TotalAllocated ≤ TotalSupply — verletzungsfrei in jedem Zwischenzustand (Overflow-checked)

## 6. Conformance-Tests (Mindestkategorien)

- sale_state_machine.json
- cap_invariants.json
- contribution_bounds.json
- timestamp_bypass ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (M2 Sales Engine)
