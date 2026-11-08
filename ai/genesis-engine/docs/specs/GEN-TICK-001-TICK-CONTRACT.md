---
spec_id: GEN-TICK-001
title: "Engine Tick Contract Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: genesis-engine
layer: L6-Engine
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Engine Tick Contract Specification (GEN-TICK-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Der verbindliche, deterministische Engine-Loop (Ordnung der Phasen ist normativ).

## 2. Scope (gilt für)

- Phasenfolge je Tick
- Pre-/Post-Verpflichtungen
- State Commit & Hash

## 3. Normative Anforderungen (MUST)

- **REQ-GT-001:** Phasenfolge fixiert: Input-Validation → Pre-Tick → Physics → AI (Intents) → World → Gameplay → Post-Tick → State-Commit → State-Hash — keine Phase darf umsortiert oder übersprungen werden (Determinismus) — *Nachweis: unit+property*
- **REQ-GT-002:** State-Commit ist atomar: Zwischenzustände sind nicht beobachtbar; State-Hash = SHA-256(canonical(world_state)) nach Commit — *Nachweis: unit+vector*
- **REQ-GT-003:** Fehler in einer Phase ⇒ Tick-Reject mit deterministischem Rollback auf letzten Commit (kein halber Tick) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Gleicher Input + gleicher Stand + gleiche Version ⇒ identischer State-Hash

## 6. Conformance-Tests (Mindestkategorien)

- tick_order.json
- tick_rollback.json
- state_hash_determinism.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P2 Engine Tick Contract)
