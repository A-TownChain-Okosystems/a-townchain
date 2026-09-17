---
spec_id: WAL-VERIFY-001
title: "Signature & Transaction Verification Specification"
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

# Signature & Transaction Verification Specification (WAL-VERIFY-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verifikationsregeln inkl. aller Malformed-Rejections.

## 2. Scope (gilt für)

- Signatur-Verifikation
- Malformed-Input-Rejects
- Fehlercodes

## 3. Normative Anforderungen (MUST)

- **REQ-WVER-001:** Verify MUSS prüfen: Adresse↔PubKey (ableitbar), Signaturform (65 Byte, 0≤recid≤3), Low-S, Nonce-Konsistenz — jede Abweichung ⇒ eindeutiger Fehlercode — *Nachweis: unit+negative*
- **REQ-WVER-002:** Fehlerkategorien mit Codes: WVER-001 invalid signature, WVER-002 wrong chain, WVER-003 wrong nonce, WVER-004 malformed tx, WVER-005 truncated, WVER-006 overflow — keine Catch-All-Feinstergebnisse — *Nachweis: unit+vector*
- **REQ-WVER-003:** Verifikation ist unabhängig implementierbar (Publikation der Vektoren WAL-SIGN/VERIFY vectors) — *Nachweis: differential*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Verify ist reine Funktion ohne Zustandszugriff außer Nonce-Lookup

## 6. Conformance-Tests (Mindestkategorien)

- verify_negative.json (alle 7 Reject-Kategorien des Owner-Audits: wrong chain/nonce, modified payload/recipient/amount/signature)
- corrupted_signature.json
- truncated_payload.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Negative-Test-Matrix)
