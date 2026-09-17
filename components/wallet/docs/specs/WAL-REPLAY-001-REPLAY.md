---
spec_id: WAL-REPLAY-001
title: "Replay Protection Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-wallet
layer: L5-Wallet
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Replay Protection Specification (WAL-REPLAY-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindlicher Schutz gegen Transaktions-Replay innerhalb und zwischen Ketten.

## 2. Scope (gilt für)

- Chain-ID-Bindung
- Nonce-Accounting
- Cross-Chain-Rejects

## 3. Normative Anforderungen (MUST)

- **REQ-WRP-001:** chain_id ist Teil des signierten Payloads (EIP-155-Analog) — eine Tx mit chain_id≠658467 (bzw. Netz-ID) wird abgelehnt — *Nachweis: unit+negative*
- **REQ-WRP-002:** Nonce je Sender ist monoton strikt steigend (expected = last+1); Lücken oder Wiederholung ⇒ Reject — *Nachweis: unit+negative+vector*
- **REQ-WRP-003:** Replay derselben Tx (identischer Hash) auf derselben Kette ist idempotent abweisend (Bekannt-Set per tx_hash) — *Nachweis: negative*
- **REQ-WRP-004:** Cross-Chain-Reject-Matrix ist Teil der Vektoren (same TX/different chain ⇒ reject; wrong chain ID ⇒ reject) — *Nachweis: vector*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Eine bestätigte Tx kann nie zweimal angewandt werden

## 6. Conformance-Tests (Mindestkategorien)

- replay_matrix.json (alle 7 Fälle des Owner-Audits)
- nonce_gap.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1-3 Replay Protection)
