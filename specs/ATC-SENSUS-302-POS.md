---
spec_id: ATC-CONSENSUS-302
title: "Proof-of-Stake (PoS) Specification — Validator-Selection & Gewichtung"
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

# Proof-of-Stake (PoS) Specification — Validator-Selection & Gewichtung (ATC-CONSENSUS-302)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Stake-basierte, deterministische Validator-Auswahl als PoS-Komponente des hybriden ShivaConsensus.

## 2. Scope (gilt für)

- Validator-Registration & -Lifecycle (Details: ATC-CONSENSUS-307)
- Stake-Berechnung und Gewichtungsfunktion
- Eligibility & Selection-Algorithmus
- Tie-Breaking

## 3. Normative Anforderungen (MUST)

- **REQ-POS-001:** Gewichtungsfunktion: weight(v) = floor(effective_stake(v) / WEIGHT_UNIT); WEIGHT_UNIT genesis-locked, reine u64-Division — *Nachweis: unit+vector*
- **REQ-POS-002:** Selection: aus dem eligible-Set wird deterministisch aus dem PoH-Seed des Slots abgeleitet (kein Global-Random, kein Timestamp) — *Nachweis: unit+property*
- **REQ-POS-003:** Tie-Breaking: bei identischem Score gewinnt die byte-lexikografisch kleinste Validator-Pubkey (kein weiterer Zufall) — *Nachweis: vector*
- **REQ-POS-004:** effective_stake = own_stake + sum(delegated) − slashed − pending_unbond; alle Abbuchungen checked-arithmetic (Overflow ⇒ Reject) — *Nachweis: unit+negative*
- **REQ-POS-005:** Mindest-Stake (genesis-locked) verhindert Sybil-Spam; darunter ist Registration ungültig — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Gleicher Validator-Set-Zustand + gleicher PoH-Seed ⇒ identischer gewählter Validator (übergreifend implementierbar)
- weight ist nie negativ; slashing reduziert weight sofort ab aktivierter Epoche

## 6. Conformance-Tests (Mindestkategorien)

- pos_selection.json (deterministische Auswahl)
- pos_tie.json (Tie-Breaking)
- stake_overflow.json (u64-Grenzen)
- duplicate_validator.json ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0-03 PoS)
- ATC-CONSENSUS-301 (PoH-Seed)
