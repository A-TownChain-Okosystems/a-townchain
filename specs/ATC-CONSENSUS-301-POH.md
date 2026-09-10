---
spec_id: ATC-CONSENSUS-301
title: "Proof-of-History (PoH) Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-algorithm
layer: L3-Konsens
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Proof-of-History (PoH) Specification (ATC-CONSENSUS-301)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Voraussetzungslose, deterministische Zeit- und Ereignis-Sequenz für ShivaConsensus: PoH erzeugt eine verifizierbare Hash-Kette als Ordnungsreferenz für Proposal- und Voting-Ereignisse.

## 2. Scope (gilt für)

- Tick-Erzeugung (Leader/VDF-Pfad)
- Tick-Verifikation (jeder Knoten, ohne Vertrauen)
- Seed-/Genesis-Handling
- Checkpointing und Restart-Wiederherstellung

## 3. Normative Anforderungen (MUST)

- **REQ-POH-001:** Ein PoH-Tick ist definiert als SHA-256(prev_tick_hash || canonical_encode(tick_data)); canonical_encode ist bytewise deterministisch (ATC-CONSENSUS-ENC) — *Nachweis: unit+vector*
- **REQ-POH-002:** Der initiale Seed ist genesis-locked (Teil von ATC-NETWORK-ID-001) und darf sich nach Genesis nie ändern — *Nachweis: vector+negative*
- **REQ-POH-003:** Tick-Typen: TICK (reines Hashing) und EVENT (referenziert externe Daten per Hash, speichert nie Rohdaten) — *Nachweis: unit+negative*
- **REQ-POH-004:** Verifikation MUSS in O(n) über die Kette möglich sein; ein fehlerhafter Zwischentick macht die gesamte Suffix-Kette ungültig — *Nachweis: property+negative*
- **REQ-POH-005:** Nach Restart/Reorg MUSS die Tick-Kette vom letzten Checkpoint deterministisch rekonstruierbar sein (Checkpoint alle N=4096 Ticks, Parameter genesis-locked) — *Nachweis: unit+integration*

## 4. Datenmodelle & Schnittstellen

```struct PoHTick { type: TickType, slot: u64, seq: u64, prev_hash: [u8;32], data_hash: Option<[u8;32]>, hash: [u8;32] }```

## 5. Invarianten

- hash(tick_i) = SHA-256(hash(tick_{i-1}) || encode(data_i)) — keine Ausnahme
- Kein Float, keine Wandlungsabhängigkeit; nur u64-Arithmetik (checked, Overflow ⇒ invalid)

## 6. Conformance-Tests (Mindestkategorien)

- poh_basic.json (3 Ticks, Determinismus)
- poh_chain.json (10.000 Ticks, Verkettung)
- poh_invalid.json (korrupter Zwischentick ⇒ Suffix verworfen)
- Restart-Recovery vom Checkpoint

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0-03 PoH-Formalisierung)
- a-townchain README (ShivaConsensus)
- ATC-STD-000 v1.2.0
