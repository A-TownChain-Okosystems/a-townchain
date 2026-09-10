---
spec_id: WAL-SEC-003
title: "Backup Specification"
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

# Backup Specification (WAL-SEC-003)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Sicheres Backup-Format und -Prozess.

## 2. Scope (gilt für)

- Backup-Format (verschlüsselt, versioniert)
- Recovery-Vektoren

## 3. Normative Anforderungen (MUST)

- **REQ-WS3-001:** Backup ist AES-256-GCM-verschlüsselt (gleiche KDF-Pflicht wie WAL-SEC-001) mit eigenem Format-Version-Feld — *Nachweis: unit+vector*
- **REQ-WS3-002:** Backup enthält NIE Klartext-Mnemonik im Backup-File; Export der Mnemonik ist nur bewusster, interaktiver UI-Vorgang (User-Confirmation) — *Nachweis: negative*
- **REQ-WS3-003:** Backup-Integrität: Auth-Tag-Fehler ⇒ Reject, nie Teileröffnung — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- (in diesem Grundgerüst noch offen)

## 6. Conformance-Tests (Mindestkategorien)

- backup_roundtrip.json
- tampered_backup ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (WAL-SEC-003)
