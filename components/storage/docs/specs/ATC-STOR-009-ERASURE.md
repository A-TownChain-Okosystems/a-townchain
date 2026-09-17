---
spec_id: ATC-STOR-009
title: "Erasure Coding Specification"
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

# Erasure Coding Specification (ATC-STOR-009)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Parameterisiertes Erasure Coding (k-of-n) als Replikations-Ergänzung.

## 2. Scope (gilt für)

- Parameter (k=4, m=2 Default)
- Fault-Tolerance-Ableitung
- Rebuild aus Shards

## 3. Normative Anforderungen (MUST)

- **REQ-ST9-001:** Default: k=4 Daten-Shards, m=2 Paritäts-Shards (n=6) über Reed-Solomon (kein Eigenbau-Codec; etablierte Library mit Review-Pflicht) — *Nachweis: unit+vector*
- **REQ-ST9-002:** Fault-Tolerance: lesbar bei Verlust von ≤ m Shards; Objekt gilt SAFE bei ≥ k+m gültigen verteilten Shards — *Nachweis: integration*
- **REQ-ST9-003:** Rebuild deterministisch: fehlende Shards werden aus demk gültigen rekonstruiert und re-verifiziert (Root-Hash) — *Nachweis: integration+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- k+m Shard-Set mit gültigem Root ⇒ Objekt garantiert rekonstruierbar

## 6. Conformance-Tests (Mindestkategorien)

- erasure_rebuild.json
- shard_corruption ⇒ Reject (Root-Verifikung)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Erasure Coding: k/m/n-Parameter)
