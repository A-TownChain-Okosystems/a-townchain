---
spec_id: LP-WL-001
title: "Whitelist Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-launchpad
layer: L5-Launchpad
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Whitelist Specification (LP-WL-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Allowlist-Verfahren (Merkle-basiert) mit Tiers und Revocation.

## 2. Scope (gilt für)

- Merkle-Proof-Verfahren
- Tier-/Limit-Logik
- Ablauf & Widerruf
- Replay-Schutz

## 3. Normative Anforderungen (MUST)

- **REQ-LPW-001:** Whitelist-Einträge sind Merkle-Blätter (address, tier, limit, expiry); Gültigkeitsprüfung = Proof + Root (genesis/sale-gebunden) — *Nachweis: unit+vector*
- **REQ-LPW-002:** InvalidProof ⇒ Reject (kein Fallback, keine Ausnahme-Blätter) — *Nachweis: negative*
- **REQ-LPW-003:** Revocation: Ein-Widerruf-Liste je Sale; widerrufene Adressen scheitern auch mit gültigem Proof — *Nachweis: adversarial*
- **REQ-LPW-004:** Expiry je Eintrag; nach Ablauf gilt der Eintrag als ungültig (Fail-Closed) — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Ein Merkle-Root je Sale — nach Sale-Start unveränderbar

## 6. Conformance-Tests (Mindestkategorien)

- whitelist_proofs.json
- invalid_proof ⇒ Reject
- revoked_entry ⇒ Reject
- expiry.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (M4 Whitelist)
