---
spec_id: ATC-STOR-005
title: "Replication Specification"
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

# Replication Specification (ATC-STOR-005)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Replikations- und Reparatur-Regeln (crash-fault-Annahme; byzantinische Fälle über Erasure + Integrität).

## 2. Scope (gilt für)

- Replikationsfaktor & Mindest-Nodes
- Repair-Threshold
- Rebuild-Policy

## 3. Normative Anforderungen (MUST)

- **REQ-ST5-001:** Default-Policy: replication_factor R=3 (genesis-locked Parameter); Objekt gilt als SAFE wenn ≥ R gültige Kopien (hash-verifiziert) — *Nachweis: unit+integration*
- **REQ-ST5-002:** Repair-Threshold: sinkt die gültige Kopiezahl unter R, wird Rebuild getriggert (deterministische Zielauswahl aus dem Node-Set) — *Nachweis: integration*
- **REQ-ST5-003:** Node-Failure-Detection über Storage-Heartbeats; nach Schwelle gilt Node als DOWN und seine Replicas werden umgeplant — *Nachweis: integration*
- **REQ-ST5-004:** Byzantinische Nodes liefern korrupte Chunks — Abwehr ausschließlich über Hash-Verifikation (kein Mehrheits-Voting über Rohdaten) — *Nachweis: adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- SAFE-Zustand ist rein aus Hash-Verifikationen ableitbar (lokal prüfbar)

## 6. Conformance-Tests (Mindestkategorien)

- replication_repair.json
- corrupt_replica ⇒ Reject
- node_down_rebalance.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Replication: Fault Tolerance, Repair Threshold)
