---
spec_id: ATC-CONSENSUS-305
title: "Fork-Choice Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-algorithm
layer: L2-Blockchain-Core
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Fork-Choice Specification (ATC-CONSENSUS-305)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Deterministische Auswahl der kanonischen Kette bei konkurrierenden Blöcken.

## 2. Scope (gilt für)

- Kanonische Kette
- Reorg-Regeln
- Gleichheits-Fälle

## 3. Normative Anforderungen (MUST)

- **REQ-FC-001:** Fork-Choice MUSS eine vollständig definierte totale Ordnung über valide Kandidaten verwenden. Die konkrete Prioritätsreihenfolge zwischen Finality, Chain-Weight/PoH und Hash-Tie-Breaking ist bis zum Spec-Freeze festzulegen; keine implizite Priorität — *Nachweis: unit+vector+property*
- **REQ-FC-002:** Reorgs über die Finality-Grenze hinaus sind verboten (siehe ATC-CONSENSUS-306) und MÜSSEN als Protocol-Violation gewertet werden — *Nachweis: negative+adversarial*
- **REQ-FC-003:** Bei Gleichstand muss die spätere, eingefrorene Totalordnung einen deterministischen Byte-Tie-Breaker definieren; kein Node-Local-Random, kein Timestamp und keine Implementierungsreihenfolge dürfen verwendet werden — *Nachweis: vector*
- **REQ-FC-004:** Fork-Choice MUST consume only candidates that passed ATC-CONSENSUS-309 proposal validation and MUST reject any candidate that conflicts with the finalized prefix — *Nachweis: negative+adversarial*
- **REQ-FC-005:** PoH, PoS, PoW and PoI contributions may affect Fork-Choice only through explicitly frozen canonical chain-weight/selection semantics; no contribution is implicitly promoted to a fork-choice rule — *Nachweis: vector*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Fork-Choice ist eine totale Ordnung über alle validen Ketten — für jedes Kettenpaar existiert genau einen Gewinner

## 6. Conformance-Tests (Mindestkategorien)

- fork_choice.json (2-Node/3-Node-Szenarien)
- fork_conflict.json (adversarial)
- reorg_past_finality ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P0-03 Fork-Choice)
