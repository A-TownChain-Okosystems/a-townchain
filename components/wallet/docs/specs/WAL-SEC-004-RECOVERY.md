---
spec_id: WAL-SEC-004
title: "Recovery Specification"
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

# Recovery Specification (WAL-SEC-004)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Wiederherstellung von Wallet und Accounts (Seed-basiert).

## 2. Scope (gilt für)

- Seed-Restore
- Account-Rescan (chain)
- Recovery-Testvektoren

## 3. Normative Anforderungen (MUST)

- **REQ-WS4-001:** Restore aus Seed/Mnemonik reproduziert deterministisch alle abgeleiteten Accounts (WAL-KEY-002) bis zu einem konfigurierten Scan-Horizont — *Nachweis: unit+integration*
- **REQ-WS4-002:** Recovery-Vektoren sind kanonisch publiziert (Seed → Adressliste → erwartete Nonce-Stände) für unabhängige Implementierungen — *Nachweis: differential*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Restore ist pure Funktion aus (seed, chainstate)

## 6. Conformance-Tests (Mindestkategorien)

- recovery_vectors.json
- rescan_horizon.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (WAL-SEC-004)
