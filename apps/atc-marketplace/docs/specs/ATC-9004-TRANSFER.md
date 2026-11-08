---
spec_id: ATC-9004
title: "Asset Transfer Specification"
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

# Asset Transfer Specification (ATC-9004)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Transfer-Regeln samt Autorisierung und Replay-Schutz.

## 2. Scope (gilt für)

- Autorisierung (Owner-Signatur)
- Replay-Schutz
- Atomicity

## 3. Normative Anforderungen (MUST)

- **REQ-A94-001:** Transfer erfordert Owner-Signatur über kanonische Transfer-Nachricht (Domain-Separation „atc-asset.transfer.v1“) — unautorisierte Transfers sind unmöglich — *Nachweis: unit+negative*
- **REQ-A94-002:** Replay: (asset_id, nonce) ist eindeutig verbraucht; Wiederholung ⇒ Reject — *Nachweis: negative*
- **REQ-A94-003:** Transfer ist atomar (Owner-Wechsel + Registry-Update + Event in einer State-Transition) — kein Zwischenzustand — *Nachweis: property*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- (in diesem Grundgerüst noch offen)

## 6. Conformance-Tests (Mindestkategorien)

- transfer_vectors.json
- unauthorized_transfer ⇒ Reject
- transfer_replay ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- WAL-REPLAY-001 (analog)
