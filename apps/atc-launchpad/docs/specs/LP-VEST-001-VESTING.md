---
spec_id: LP-VEST-001
title: "Vesting Specification"
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

# Vesting Specification (LP-VEST-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Mathematisch eindeutige Vesting-Regeln mit Claim-Invarianten.

## 2. Scope (gilt für)

- Parameter (cliff, start, duration, interval, total_allocation)
- Claim-Berechnung
- Replay- & Overflow-Schutz

## 3. Normative Anforderungen (MUST)

- **REQ-LPV-001:** released_at(t) = 0 für t < cliff; sonst proportional: floor(total * (t - start) / duration), diskretisiert auf Intervalle — Integer-only, checked — *Nachweis: unit+vector*
- **REQ-LPV-002:** Invarianten: released ≤ allocation; claimable ≤ allocation - released; Double-Claim unmöglich (nonce je Beneficiary) — *Nachweis: property+negative*
- **REQ-LPV-003:** Timestamp-Bypass verboten: Zeit ausschließlich über Chain-Height/Slot;伪造 früherer Claim ⇒ Reject — *Nachweis: negative*
- **REQ-LPV-004:** Unautorisierte Freigabe (nicht Beneficiary) ⇒ Reject (Signatur-Pflicht) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- released + remaining = allocation — exakt, in jedem Zustand

## 6. Conformance-Tests (Mindestkategorien)

- vesting_math.json (cliff/interval-Edge-Fälle)
- vest_invariants.json
- double_claim ⇒ Reject
- overflow_claim ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (M3 Vesting)
