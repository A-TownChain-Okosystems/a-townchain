---
spec_id: ATC-STOR-006
title: "Storage Encryption Specification"
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

# Storage Encryption Specification (ATC-STOR-006)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verschlüsselung at-rest mit Standard-Primitive — keine selbst entwickelte Kryptographie.

## 2. Scope (gilt für)

- AEAD (AES-256-GCM)
- Key-Derivation & -Referenz
- Nonce-Regeln
- Secure Deletion

## 3. Normative Anforderungen (MUST)

- **REQ-ST6-001:** encryption_mode: NONE | AES256GCM; bei AES256GCM: Objekt-Key abgeleitet aus (owner_secret, object_id) via HKDF-SHA256 (Domain-Separation „atc-storage.key.v1“) — *Nachweis: unit+vector*
- **REQ-ST6-002:** Nonce: 96-Bit eindeutig je (Key, Chunk) — Ableitung deterministisch aus (object_id, chunk_index, key_version); Wiederverwendung verboten — *Nachweis: negative+static*
- **REQ-ST6-003:** Key-Rotation über key_version; alte Versionen bleiben lesbar bis Explizite-Migration (Policy) — *Nachweis: unit+integration*
- **REQ-ST6-004:** Secure Deletion: DELETE entfernt Capability + Key-Referenz; ohne Key-Referenz ist Chiffre nicht mehr nutzbar (kryptographische Löschung), plus Best-Effort-Chunk-Deletion — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Kein Klartext-Content verlässt den Client für NONE-Modus-Nutzung ohne dokumentierte Entscheidung

## 6. Conformance-Tests (Mindestkategorien)

- encryption_vectors.json
- nonce_reuse_static_check
- rotation_migration.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Encryption Specification: keine Eigenbau-Kryptographie)
