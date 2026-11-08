---
spec_id: ATC-COMP-505
title: "Result Attestation Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-compute
layer: L5-Compute
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Result Attestation Specification (ATC-COMP-505)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Maschinenlesbarer Nachweis über Ausführung und Ergebnis eines Jobs — die Vertrauenskette von Job zu ATC.

## 2. Scope (gilt für)

- Attestation-Felder & Signatur
- Input-/Result-Hashes
- Runtime-/Model-/Env-Hashes

## 3. Normative Anforderungen (MUST)

- **REQ-ATT-001:** Attestation = signiertes Objekt {job_hash, input_hash, result_hash, runtime_version, model_hash?, env_hash, resource_record, worker_pubkey, nonce, executed_at_slot} — *Nachweis: unit+vector*
- **REQ-ATT-002:** result_hash = SHA-256(canonical(result_payload)) — für Deterministic-Classes bit-identisch reproduzierbar — *Nachweis: property*
- **REQ-ATT-003:** Nonce + Slot binden die Attestation gegen Replay; Wiederverwendung derselben Attestation für einen anderen Job ist verboten — *Nachweis: negative*
- **REQ-ATT-004:** Verifikation der Attestation-Signatur folgt ATC-CRYPTO-001 (Domain-Separation „atc-compute.attestation.v1“) — *Nachweis: unit*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Attestation ohne vollen Hash-Satz (input/result/runtime/env) ist ungültig

## 6. Conformance-Tests (Mindestkategorien)

- attestation_format.json (T08)
- attestation_replay ⇒ Reject (T13)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1 Determinismus & Verifikation: Vertrauenskette)
