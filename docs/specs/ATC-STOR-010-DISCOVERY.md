---
spec_id: ATC-STOR-010
title: "Storage Node Discovery Specification"
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

# Storage Node Discovery Specification (ATC-STOR-010)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Grundgerüst der Storage-Node-Erkundung (an P2P-Layer angeschlossen).

## 2. Scope (gilt für)

- Registry-Beitritt
- Ankündigung von Held-Objects
- Eclipse-Schutz

## 3. Normative Anforderungen (MUST)

- **REQ-ST10-001:** Discovery läuft über den ShivaCore-P2P-Layer (PROTOCOL-001/ATC-PROTO-P2P-001) mit Storage-Domain-Messages; kein eigenes Overlay-Protokoll — *Nachweis: architecture*
- **REQ-ST10-002:** Node kündigt gehaltene CIDs kompakt (Bloom-Filter-artig) an; Ankündigungen sind signiert (DID) — *Nachweis: unit+negative*
- **REQ-ST10-003:** Auswahl von Zielen für PUT/GET ist diversifiziert (kein einzelner Peer beliefert alles) — Eclipse-Regeln aus P2P-001 gelten — *Nachweis: adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- (in diesem Grundgerüst noch offen)

## 6. Conformance-Tests (Mindestkategorien)

- discovery_announce.json
- announcement_forged ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- ATC-PROTO-P2P-001 (Discovery/Eclipse-Regeln)
