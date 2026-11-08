---
spec_id: ATC-COMP-506
title: "Compute Verification Specification"
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

# Compute Verification Specification (ATC-COMP-506)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Was ein „korrektes“ Compute-Ergebnis bedeutet — je Workload-Klasse unterschiedlich — und wie Verifikation entscheidet.

## 2. Scope (gilt für)

- Deterministic-Verifikation (Re-Execution/Vergleich)
- AI-Verifikation (Sampled Consensus + Attestation-Quorum)
- Engine-Verifikation (State-Hash-Vergleich)
- FAIL-CLOSED

## 3. Normative Anforderungen (MUST)

- **REQ-VER-001:** Deterministic-Klasse: Verifikator re-exekutiert (Voll- oder Sampling) und vergleicht result_hash — Divergenz ⇒ REJECT + Worker-Score-Abzug — *Nachweis: integration+property*
- **REQ-VER-002:** AI-Klasse: Ergebnis gilt als angenommen bei ≥ QUORUM (genesis-locked, z. B. 2-of-3) übereinstimmenden, attestierten Ergebnissen unabhängiger Worker; darunter FAIL-CLOSED — *Nachweis: adversarial+integration*
- **REQ-VER-003:** Engine-Klasse: Verifikation über deterministischen State-Hash (GEN-DET-001) am Tick-Ende — *Nachweis: integration*
- **REQ-VER-004:** Jede Verifikationsentscheidung ist PASS/REJECT — niemals „unbekannt akzeptiert“ — *Nachweis: negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Kein Attestation-Only-Ergebnis der AI-Klasse ohne Quorum wird chain-wirksam

## 6. Conformance-Tests (Mindestkategorien)

- verification_pass.json
- byzantine_worker.json (T12: 1-of-3 lügt ⇒ REJECT)
- invalid_result_rejection.json (T10)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (T08-T10, T12; „Was bedeutet ein korrektes Ergebnis?«)
