---
spec_id: ATC-STATE-001
title: "State Transition Specification (Invariants)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain
layer: L3-Chain
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# State Transition Specification (Invariants) (ATC-STATE-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindliche Invarianten des Chain-State-Übergangs (Block-Apply).

## 2. Scope (gilt für)

- Apply-Sequenz (deterministisch)
- State-Invarianten
- Ungültiger-Block-Behandlung

## 3. Normative Anforderungen (MUST)

- **REQ-STT-001:** Apply ist deterministisch geordnet: Header-Check (ATC-CONSENSUS-305/306) → Tx-Filter (kanonisch, geordnet) → sequenzielle Tx-Ausführung (ATC-VM-001) → State-Commit + Root-Update — *Nachweis: unit+property*
- **REQ-STT-002:** Invarianten je Block: value-flow konserviert (Inputs = Outputs + Fees), Nonces strikt monoton, State-Root verifizierbar (Merkle), keine Tx-Halbwirkung (Atomicity je Tx) — *Nachweis: property+vector*
- **REQ-STT-003:** Ein Block, der irgendeine Invariante verletzt, wird als Ganzes verworfen — nie partiell akzeptiert — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- State-Root ist nach jedem Block verifizierbar aus den Journal-Daten

## 6. Conformance-Tests (Mindestkategorien)

- state_transition_vectors.json
- invalid_block ⇒ Reject (ganz)
- fee_conservation.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0: State-transition invariants)
