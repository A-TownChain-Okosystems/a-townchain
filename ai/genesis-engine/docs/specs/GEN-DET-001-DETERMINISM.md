---
spec_id: GEN-DET-001
title: "Genesis Determinism Specification"
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

# Genesis Determinism Specification (GEN-DET-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Simulations-Determinismus als Fundament für Multiplayer, Replay, Anti-Cheat und ZKP-State-Verifikation.

## 2. Scope (gilt für)

- Seed + Generator-Version + Parameter
- Replay & Synchronisation
- State-Hash-Kette

## 3. Normative Anforderungen (MUST)

- **REQ-GD-001:** Reproduzierbar: same seed + same input + same engine_version + same generator_version ⇒ same world (bit-identischer State-Hash je Tick) — *Nachweis: property+vector*
- **REQ-GD-002:** Generator-Versionswechsel erzeugt neue Welt (kein stilles Mischen von Terrain-Versionen) — *Nachweis: negative*
- **REQ-GD-003:** GenesisStateHash(t) ist kumulativ verkettet (Hash(t) = SHA-256(Hash(t-1) || canonical(diff(t)))) — Replay- und ZKP-fähig — *Nachweis: unit+vector*
- **REQ-GD-004:** Kein Float im Simulations-Pfad (Fixpunkt-Integer-Physik); Rendering ist außen vor (headless-Simulation Pflicht) — *Nachweis: architecture+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Determinismus gilt unabhängig von Plattform/Thread-Scheduling

## 6. Conformance-Tests (Mindestkategorien)

- determinism_replay.json
- seed_reproducibility.json
- cross_platform_differential.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Determinismus, Multiplayer/ZKP-Vorbereitung)
