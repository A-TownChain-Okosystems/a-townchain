---
spec_id: ATC-9008
title: "DEX/AMM Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-marketplace
layer: L5-Marketplace
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# DEX/AMM Specification (ATC-9008)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

AMM-Invarianten, Rundung und Order-Ausführung der DEX-Komponente (atc-dex).

## 2. Scope (gilt für)

- Konstante-Produkt-AMM
- Reserve-Konsistenz
- Rounding & Slippage
- Liquidity-Accounting

## 3. Normative Anforderungen (MUST)

- **REQ-A98-001:** AMM-Invariante: k = reserve_a * reserve_b; jede Swap-Operation MUSS k (exklusives Fee) erhalten oder erhöhen — Reserve-Divergenz ⇒ Reject (Invariant-Check in jedem Swap) — *Nachweis: property+unit*
- **REQ-A98-002:** Rundungsrichtung: immer zulasten des Traders (erhält nie mehr als mathematisch exakt); min_output/slippage_limit wird geprüft und unterschritten ⇒ Reject (kein silent Slippage) — *Nachweis: unit+negative+vector*
- **REQ-A98-003:** Liquidity: mint/burn rechnen mit denselben Formeln konservativ (floor für Trader-Anteile); Overflow-safe — *Nachweis: unit+vector*
- **REQ-A98-004:** Pool-Erstellung und Fee-Buchung (fee_bps) mit exakter Aufteilung auf Reserven; Fee-Extraktion verletzt k+fee nie negativ — *Nachweis: unit*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- k ist über alle gültigen Swaps monoton nicht fallend (exkl. Fees)

## 6. Conformance-Tests (Mindestkategorien)

- amm_invariants.json (Property: k-erhalt über randomisierte Swaps)
- rounding_direction.json
- slippage_rejection.json
- overflow_underflow.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (DEX Security Model: AMM invariant preservation)
