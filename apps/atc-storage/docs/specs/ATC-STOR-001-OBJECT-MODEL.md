---
spec_id: ATC-STOR-001
title: "Storage Object Model Specification"
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

# Storage Object Model Specification (ATC-STOR-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Das verbindliche Objektmodell des dezentralen Storage (Objekt, Chunks, Metadaten, Besitz/Capability).

## 2. Scope (gilt für)

- StorageObject-Struktur
- Chunk-Referenzen
- Metadaten (Größen, MIME, Richtlinien)
- Owner/Capability-Feld

## 3. Normative Anforderungen (MUST)

- **REQ-ST1-001:** StorageObject: {object_id (CID, s. ATC-STOR-002), content_hash, size, mime, chunk_size, chunk_count, encryption_mode, replication_policy, owner_did, capability_ref, metadata (Größen-gecappt)} — *Nachweis: unit+vector*
- **REQ-ST1-002:** object_id ist die CID über den kanonischen Objekt-Deskriptor; Änderung eines Feldes ⇒ neue CID (immutable Deskriptoren) — *Nachweis: unit*
- **REQ-ST1-003:** metadata ist größenlimitiert (Obergrenze genesis-locked); über große Metadaten ist ein separater Chunk-Verweis Pflicht — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Objekt-Deskriptoren sind immutable; Änderung erzeugt neue Objekt-Version

## 6. Conformance-Tests (Mindestkategorien)

- object_model_vectors.json
- metadata_limit.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Storage Object Model)
