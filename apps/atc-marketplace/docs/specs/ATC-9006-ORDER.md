---
spec_id: ATC-9006
title: "Marketplace Order Specification"
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

# Marketplace Order Specification (ATC-9006)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Order-Objekt und Order-Lifecycle (Listings/Bids) mit Nonce-Schutz.

## 2. Scope (gilt für)

- Order-Objekt & -Zustände
- Nonce/Expiry/Replay
- Partial Fills & Cancellation

## 3. Normative Anforderungen (MUST)

- **REQ-A96-001:** Order: {order_id, asset_id, seller, price, expiry_height, nonce, signature} — Zustände LISTED→FILLED/CANCELLED/EXPIRED (endliche Maschine) — *Nachweis: unit+vector*
- **REQ-A96-002:** Duplicate Execution ist unmöglich: (seller, nonce) verbraucht; identische Order-ID ⇒ idempotenter Reject — *Nachweis: adversarial+negative*
- **REQ-A96-003:** Expiry: Order nach expiry_height ungültig; Ausführung danach ⇒ Reject (kein Timestamp-Trust) — *Nachweis: negative*
- **REQ-A96-004:** Partial Fills nur wenn deklariert (fill_policy); sonst ganzzahlige Ausführung — *Nachweis: unit*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Keine Order ist nach.FILL/CANCEL/EXPIRY erneut ausführbar

## 6. Conformance-Tests (Mindestkategorien)

- order_lifecycle.json
- duplicate_execution.json
- expiry_bypass ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Orders: nonce/replay, expiry, partial fills)
