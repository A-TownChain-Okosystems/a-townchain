---
spec_id: GEN-AI-001
title: "Creature AI Specification (Intent-Policy-Trennung)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: genesis-engine
layer: L6-Engine
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Creature AI Specification (Intent-Policy-Trennung) (GEN-AI-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

KI-Verhalten mit Sicherheitsarchitektur: KI erkennt → Policy validiert → System führt aus.

## 2. Scope (gilt für)

- Creature-Datenmodell (Identity, Stats, Perception, Memory, Decision, Behaviour-Tree, Action)
- Intent-Schnittstelle
- Policy-Validierung

## 3. Normative Anforderungen (MUST)

- **REQ-GA-001:** KI gibt ausschließlich Intents ab (strukturierte, validierbare Absichten); direkte State-Mutation durch KI ist verboten (statisch prüfbar) — *Nachweis: architecture+negative*
- **REQ-GA-002:** Policy-Validierung prüft jeden Intent gegen Regeln (Scope, Kosten, Legitimität); ungültige Intents werden verworfen und geloggt — *Nachweis: unit+negative*
- **REQ-GA-003:** Behaviour-Tree-Auswertung ist deterministisch (geordnete Traversierung, keine Randomness ohne deklarierten Seed) — *Nachweis: property*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Keine KI-Komponente ist Teil der autoritativen State-Mutation

## 6. Conformance-Tests (Mindestkategorien)

- intent_validation.json
- illegal_mutation_static_check
- behavior_tree_determinism.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P2 Creature AI: „KI darf nicht direkt den Engine-State manipulieren«)
- aurora-ai ATC-AI-TB-001 (gleiche Architektur)
