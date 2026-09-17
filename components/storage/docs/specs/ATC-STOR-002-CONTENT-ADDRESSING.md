---
spec_id: ATC-STOR-002
title: "Content Addressing Specification (CID & BLAKE3)"
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

# Content Addressing Specification (CID & BLAKE3) (ATC-STOR-002)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Das ATC-CID-Format als einzige Adresse von Content.

## 2. Scope (gilt für)

- CID-Format
- Hash-Algorithmus (BLAKE3)
- Versionierbarkeit

## 3. Normative Anforderungen (MUST)

- **REQ-ST2-001:** CID = encode(version: u8 || algo_id: u8 || digest: [u8;32]) — version 1, algo_id 1 = BLAKE3; andere Kombinationen sind unbekannt ⇒ Reject — *Nachweis: unit+vector+negative*
- **REQ-ST2-002:** digest = BLAKE3(content) für Chunks; Objekt-Root = BLAKE3 über kanonische Chunk-Liste (Merkle-artig, s. ATC-STOR-008) — *Nachweis: unit*
- **REQ-ST2-003:** CID-Hex-Darstellung (Debug) lowercase; Wire-Format ist Ro Bytewert — *Nachweis: vector*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Gleicher Content ⇒ gleiche CID (global deduplizierbar)

## 6. Conformance-Tests (Mindestkategorien)

- cid_vectors.json
- unknown_algo ⇒ Reject
- cid_roundtrip.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Content Addressing: „BLAKE3 allein reicht nicht«)
