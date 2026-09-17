---
spec_id: ATC-STOR-007
title: "ContentCap Integration Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-storage
layer: L5-Storage
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# ContentCap Integration Specification (ATC-STOR-007)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Capability-modellierter Zugriff auf Storage — ohne automatische Kernel-Autorität.

## 2. Scope (gilt für)

- Content-Cap-Felder
- Authorization-Kette
- Kernel-Abgrenzung

## 3. Normative Anforderungen (MUST)

- **REQ-ST7-001:** ContentCap = {capability_id, object_id, operations (read/write/delete), scope, expires_at, nonce, issuer_did, subject_did, signature} — Replay-geschützt über (nonce, expires) — *Nachweis: unit+negative*
- **REQ-ST7-002:** Authorization-Kette: Capability → Policy-Prüfung (Storage-Policy) → Execution; Storage erhält NIE automatisch Kernel-Authority (ShivaCore-Prinzip: Capability → Policy → Execution) — *Nachweis: architecture*
- **REQ-ST7-003:** Abgelaufene/revokierte Capabilities werden über eine Revocation-Liste (Chain-/Registry-Referenz) geprüft — kein Lokal-Cache als Quelle — *Nachweis: adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Jede Storage-Operation hat eine geprüfte Capability — keine Ausnahme

## 6. Conformance-Tests (Mindestkategorien)

- cap_authz.json
- cap_expired ⇒ Reject
- cap_replay ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (ContentCap: „Storage darf nicht Kernel-Autorität erhalten«)
- ShivaCore K3a Capability-Modell
