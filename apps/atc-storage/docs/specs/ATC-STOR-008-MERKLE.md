---
spec_id: ATC-STOR-008
title: "Storage Integrity & Merkle Proofs Specification"
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

# Storage Integrity & Merkle Proofs Specification (ATC-STOR-008)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Nachweisbare Integrität von Objekten und Chunks.

## 2. Scope (gilt für)

- Root-Hash-Konstruktion
- Proof-Format
- Verifikation

## 3. Normative Anforderungen (MUST)

- **REQ-ST8-001:** Objekt-Root = BLAKE3 über kanonische (chunk_index, chunk_cid)-Liste; Blatt- und Knoten-Encoding sind fest (Domain-Prefixe leaf/node) — *Nachweis: unit+vector*
- **REQ-ST8-002:** Merkle-Proof = inklusionsbeweis (Pfad der Sibling-Hashes) — Verifikation ist lokal ohne weitere Chunks — *Nachweis: unit+property*
- **REQ-ST8-003:** GET liefert optional (root, proofs[]); Divergenz zwischen Root und gelieferten Chunks ⇒ whole-object Reject — *Nachweis: integration+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Proof-Verifikation ist O(log n) und ohne Netzwerkzugang möglich

## 6. Conformance-Tests (Mindestkategorien)

- merkle_proofs.json
- proof_tampering ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Merkle Verification)
