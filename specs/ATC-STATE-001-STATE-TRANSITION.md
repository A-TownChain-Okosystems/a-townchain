---
spec_id: ATC-STATE-001
title: "State Transition Specification (Invariants)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain
layer: L2-Blockchain-Core
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends:
  - ATC-CONSENSUS-309
  - ATC-VM-001
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

- **REQ-STT-001:** Apply ist deterministisch geordnet: Consensus validation (ATC-CONSENSUS-309: proposal → attestation/finality/fork-choice) → Tx-Filter (kanonisch, geordnet) → sequenzielle Tx-Ausführung (ATC-VM-001) → validator/consensus metadata update → State-Commit + Root-Update — *Nachweis: unit+property*
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


## 10. Consensus State-Machine Binding

- **REQ-STT-004:** State transition MUST consume only a proposal accepted by ATC-CONSENSUS-309; legacy consensus simulators MUST NOT define block validity.
- **REQ-STT-005:** The committed state MUST include or deterministically derive the canonical state root, validator-state changes, and consensus/finality metadata required by the frozen protocol.
- **REQ-STT-006:** Block application MUST be atomic: validation or execution failure discards the complete staged transition.
- **REQ-STT-007:** Economic amounts MUST use the canonical u128 representation; counter fields retain their contract-defined u64 representation.
- **REQ-STT-008:** The transition result MUST be reproducible from canonical serialized inputs without host-local or external mutable state.
