---
spec_id: GEN-ECS-001
title: "ECS Specification"
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

# ECS Specification (GEN-ECS-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Struktur-Norm des Entity-Component-Systems (Component-Storage statt Alles-am-Entity).

## 2. Scope (gilt für)

- Entity-Registry
- Component-Storage (SoA)
- Systeme & Event-Bus

## 3. Normative Anforderungen (MUST)

- **REQ-GE-001:** Entity = ID (u64, generiert, nie recycled in derselben Welt); Komponenten liegen in getrennten Component-Storage-Tabellen (kein AoS-Monolith) — *Nachweis: unit+architecture*
- **REQ-GE-002:** Systeme deklarieren Read/Write-Komponenten-Zugriff statisch (Determinismus der Ausführungsreihenfolge) — *Nachweis: architecture+unit*
- **REQ-GE-003:** Event-Bus ist determinismus-sicher (geordnete Zustellung, keine Thread-Zufälligkeit) — *Nachweis: property*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Komponenten-Zugriffe sind deklarativ geprüfbar (Data-Race-frei per Design)

## 6. Conformance-Tests (Mindestkategorien)

- ecs_storage.json
- entity_lifecycle.json
- system_order.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P2: SoA vs AoS)
