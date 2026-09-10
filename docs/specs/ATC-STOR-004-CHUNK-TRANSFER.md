---
spec_id: ATC-STOR-004
title: "Chunk Transfer Protocol Specification"
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

# Chunk Transfer Protocol Specification (ATC-STOR-004)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindliche Chunk-Größen, -Ordnung und Reassembly.

## 2. Scope (gilt für)

- Chunk-Größe (Default 1 MiB, genesis-locked)
- Ordering & Reassembly
- Deduplizierung

## 3. Normative Anforderungen (MUST)

- **REQ-ST4-001:** chunk_size default 1 MiB (konfigurierbar je Objekt innerhalb [64 KiB, 4 MiB]); Werte außerhalb ⇒ Reject — *Nachweis: unit+vector*
- **REQ-ST4-002:** Chunk-IDs folgen ATC-STOR-002; Reassembly verifiziert Root-Hash (ATC-STOR-008) vor COMMIT — Divergenz ⇒ ganzer Transfer ungültig — *Nachweis: integration+negative*
- **REQ-ST4-003:** Identische Chunks (gleiche CID) werden dedupliziert; Zählpflicht je Objekt bleibt (Löschbarkeit über Capability) — *Nachweis: property*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Ein erfolgreiches PUT liefert atomar ein Objekt — kein halb zugesetzter Zustand

## 6. Conformance-Tests (Mindestkategorien)

- chunk_transfer.json
- corrupt_chunk ⇒ Reject
- dedup_property.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Chunk Specification)
