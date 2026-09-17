---
spec_id: ATC-CONSENSUS-306
title: "Block Finality Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-algorithm
layer: L3-Konsens
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Block Finality Specification (ATC-CONSENSUS-306)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindliche Definition, wann ein Block als final gilt und welche Beweise Finalität belegen.

## 2. Scope (gilt für)

- Finality-Threshold
- Attestations
- Equivocation & Slashing
- Kein Rollback über Finalität

## 3. Normative Anforderungen (MUST)

- **REQ-FIN-001:** Ein Block gilt als FINAL, wenn ≥ 2/3 der aktiven Validator-Stake-Gewichte (bezogen auf den letzten finalisierten Validator-Set-Hash) attestiert haben — *Nachweis: unit+integration*
- **REQ-FIN-002:** Attestations sind signierte Nachrichten über (block_hash, height, source=finalisierte Höhe); ungültige Signaturen zählen nicht — *Nachweis: negative*
- **REQ-FIN-003:** Equivocation (zwei konfliktierende Attestations desselben Validators) ist ein Slashing-Tatbestand mit Beweis-Nachricht (ATC-CONSENSUS-302) — *Nachweis: adversarial*
- **REQ-FIN-004:** Nach Finalität verworfene Blöcke derselben Höhe sind Protocol-Violations; der Zustand wird NIE über finalisierte Höhe hinaus zurückgerollt — *Nachweis: property+adversarial*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- finalität ist monoton: height_final ist nicht fallend
- 2/3-Quorum bei BFT-Annahme (≤1/3 byzantinisch) garantiert Sicherheit

## 6. Conformance-Tests (Mindestkategorien)

- finality.json (Quorum-Grenzen: 2/3±1)
- equivocation_slash.json
- rollback_attempt ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0-03 Finality: »Wann gilt ein Block als final?«)
