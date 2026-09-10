---
spec_id: ATC-CONSENSUS-303
title: "Proof-of-Work (PoW) Specification — Difficulty & Work-Beitrag"
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

# Proof-of-Work (PoW) Specification — Difficulty & Work-Beitrag (ATC-CONSENSUS-303)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Definition des PoW-Beitrags im hybriden Konsens (Ergänzung, nicht Ersatz von PoS) samt Difficulty-Adjustment und Anti-Grinding.

## 2. Scope (gilt für)

- Difficulty/Target-Encoding
- Epochen-basiertes Difficulty-Adjustment
- Work-Beitrag zur Hybrid-Selection
- Anti-Grinding

## 3. Normative Anforderungen (MUST)

- **REQ-POW-001:** Target-Encoding: kompakte Big-Endian-Darstellung (analog Bitcoin-Compact); Block ist gültig, wenn SHA-256(header) numerisch < target — *Nachweis: unit+vector*
- **REQ-POW-002:** Difficulty-Adjustment je Epoche als EMA über die letzten E Epochendauern mit geklemmten Faktorgrenzen (max ±25 % je Anpassung); Zielblockzeit genesis-locked — *Nachweis: unit+property*
- **REQ-POW-003:** Work-Beitrag eines Blocks: floor(2^256 / (target+1)) als u128-Arithmetik; Overflow ⇒ ungültig — *Nachweis: unit+negative*
- **REQ-POW-004:** Anti-Grinding: Der verwendbare PoW-Nonce-Raum ist an den PoH-Slot gebunden (grinding lohnt nicht, weil die Selection ATC-CONSENSUS-304 den PoH-Seed nutzt) — *Nachweis: property+adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Difficulty ist pro Epoche konstant und für alle Knoten identisch
- Kein Vorteil durch Timestamp-Manipulation (Zielzeit aus verketteten PoH-Slots, nicht Wallclock)

## 6. Conformance-Tests (Mindestkategorien)

- pow_difficulty.json (Grenz-/Edge-Fälle)
- adjustment_clamp.json (±25 %-Klemme)
- malformed_target ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0-03 PoW)
- ATC-CONSENSUS-301
