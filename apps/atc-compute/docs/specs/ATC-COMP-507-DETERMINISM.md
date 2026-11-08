---
spec_id: ATC-COMP-507
title: "Deterministic Execution Specification (Compute)"
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

# Deterministic Execution Specification (Compute) (ATC-COMP-507)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Determinismus-Anforderungen für die Deterministic-Klasse und reproduzierbare Simulation (Engine-Klasse).

## 2. Scope (gilt für)

- Laufzeitumgebung-Versionsbindung
- Sandbox-Isolation
- Replay-Pflicht

## 3. Normative Anforderungen (MUST)

- **REQ-CDET-001:** Deterministische Jobs binden runtime_version + env_hash; jede Ergebnis-Attestation ohne Versionsbindung ist ungültig — *Nachweis: unit+negative*
- **REQ-CDET-002:** Sandbox-Isolation: kein Netzwerkzugang (außer deklarierter Input-Fetch), kein Host-FS, keine Umgebungsvariablen außer deklariertem Env-Set — *Nachweis: adversarial*
- **REQ-CDET-003:** Replay ist für die Deterministic-Klasse MÜSSEN-Pflicht: gleicher Job ⇒ bit-identisches Ergebnis über unabhängige Worker (T17) — *Nachweis: property*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Determinism-Klasse: kein Float, kein Wallclock, kein Zufall im Job-Pfad

## 6. Conformance-Tests (Mindestkategorien)

- deterministic_replay.json (T17)
- env_leak ⇒ Reject (T06)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- ATC-CONSENSUS-DET (Kernel-Determinismus-Contract, analog)
