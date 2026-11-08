---
spec_id: ATC-COMP-509
title: "Compute Economics Specification"
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

# Compute Economics Specification (ATC-COMP-509)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Abrechnungs- und Anreizmodell (Grundgerüst — endgültige Parameter sind Owner-Entscheidung).

## 2. Scope (gilt für)

- Job-Gebühr (Budget-basiert)
- Worker-Vergütung & Score
- Slashing-Analogon (Score-Abzug)

## 3. Normative Anforderungen (MUST)

- **REQ-CEC-001:** Gebühr = Ressourcen-Record × Klassentarif (Integer micro-ATC, checked ops); Tarife sind genesis-locked Parameter — *Nachweis: unit+vector*
- **REQ-CEC-002:** Vergütung erfolgt nur bei PASS-Verifikation; REJECT ⇒ keine Vergütung + Score-Abzug (deterministische Straffunktion) — *Nachweis: integration*
- **REQ-CEC-003:** Parameter-Änderungen (Tarife, Quoren, Limits) nur via MAJOR + Owner-Freigabe (COMPAT-001) — *Nachweis: governance*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Abgerechnete Summe ≤ Job-Budget; Differenz (Restbudget) geht an Job-Owner zurück

## 6. Conformance-Tests (Mindestkategorien)

- fee_rounding.json
- payout_on_pass_only.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (Compute Economics)
