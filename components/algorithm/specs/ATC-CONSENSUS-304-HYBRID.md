---
spec_id: ATC-CONSENSUS-304
title: "Hybrid Selection Specification — ShivaConsensus Leader-Wahl"
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

# Hybrid Selection Specification — ShivaConsensus Leader-Wahl (ATC-CONSENSUS-304)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Mathematisch eindeutige Kombination von PoH, PoS-Gewicht und PoW-Beitrag zur Leader-Selection — der kritischste Teil der Spezifikation.

## 2. Scope (gilt für)

- Hybrid-Score
- Leader-Selection
- Parameter-Genesis-Lock
- Consensus-Version

## 3. Normative Anforderungen (MUST)

- **REQ-HYB-001:** Hybrid-Score pro Kandidat: score = W_S * pos_weight(v) + W_W * pow_work(block); W_S, W_W sind genesis-locked u64-Gewichte — *Nachweis: unit+vector*
- **REQ-HYB-002:** Leader = argmax(score) über alle validen Kandidaten des Slots; festgelegt durch PoH-Seed + Validator-Set-Hash (deterministisch) — *Nachweis: unit+property*
- **REQ-HYB-003:** Hybrid-Parameter (W_S, W_W, Zielblockzeit, Difficulty-Start) sind Teil der Network-Identity (ATC-NETWORK-ID-001) und nur per MAJOR + Owner-Freigabe änderbar — *Nachweis: vector+negative*
- **REQ-HYB-004:** Consensus-Version ist im Block-Header; inkompatible Versionen ⇒ Headers werden verworfen (kein silent upgrade) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Gleiche Kandidatenmenge + gleicher PoH-Zustand ⇒ identischer Leader — zwei unabhängige Implementierungen dürfen nie divergieren (ATC-CONSENSUS-DET)

## 6. Conformance-Tests (Mindestkategorien)

- hybrid_selection.json (Referenzvektoren)
- independent_impl_differential (Rust vs. Referenz-Oracle)
- version_mismatch ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0-03 Hybrid-Engine, mathematisch eindeutig)
- ATC-CONSENSUS-302/303
