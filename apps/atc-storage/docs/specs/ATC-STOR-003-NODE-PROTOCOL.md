---
spec_id: ATC-STOR-003
title: "Storage Node Protocol Specification"
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

# Storage Node Protocol Specification (ATC-STOR-003)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Grundgerüst des Node-Protokolls (PUT/GET/HANDSHAKE) — Detail-Erweiterung beim Spec-Freeze.

## 2. Scope (gilt für)

- Operationen (PUT, GET, DELETE via Capability)
- Antwort-Envelope (9-Felder, PROTOCOL-001)
- Fehlercodes

## 3. Normative Anforderungen (MUST)

- **REQ-ST3-001:** Alle Nachrichten nutzen den ATC-Protokoll-Envelope (PROTOCOL-001, deterministische Serialisierung) mit Storage-Domain-Separation — *Nachweis: architecture*
- **REQ-ST3-002:** Fehlercodes STOR-001..019 (katalogisiert: not_found, forbidden, corrupt, quota, unreachable …) — *Nachweis: unit+negative*
- **REQ-ST3-003:** Jede Operation ist gegen den Capability-Layer autorisiert (ATC-STOR-007) — keine operation ohne Capability-Prüfung — *Nachweis: adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- (in diesem Grundgerüst noch offen)

## 6. Conformance-Tests (Mindestkategorien)

- protocol_roundtrip.json
- error_matrix.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- ATC-STD-PROTOCOL-001 (Dachnorm Envelope)
