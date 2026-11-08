---
spec_id: ATC-COMP-500
title: "Compute Architecture Specification"
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

# Compute Architecture Specification (ATC-COMP-500)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Architekturnorm der dezentralen Compute-Schicht: drei getrennte Workload-Klassen mit einer gemeinsamen Verifikations- und Abrechnungsschicht.

## 2. Scope (gilt für)

- Deterministic Compute (ATC-VM-/Contract-Jobs)
- AI Compute (Inferenz/Model-Execution via Aurora)
- Engine Compute (Genesis-Simulation)
- Gemeinsames Job-/Attestation-Fundament

## 3. Normative Anforderungen (MUST)

- **REQ-COMP-001:** Die drei Workload-Klassen sind getrennte Laufzeitpfade; ATC bleibt alleinige Wahrheitsschicht — Aurora/Genesis optimieren, normieren aber keinen Chain-State — *Nachweis: architecture-review*
- **REQ-COMP-002:** AI-Compute-Ergebnisse sind niemals Konsens-Input; sie laufen ausschließlich über Attestation + On-Chain-Verifikationspflicht (ATC-COMP-505/506) — *Nachweis: architecture+negative*
- **REQ-COMP-003:** Engine-Compute (Genesis) ist headless-fähig (Server/CI ohne GPU) — Rendering ist aus der Simulations-Runtime ausgeschlossen — *Nachweis: architecture*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Compute darf Chain-State nur über explizite, verifizierte Ergebnisse beeinflussen

## 6. Conformance-Tests (Mindestkategorien)

- architecture-conformance (Modul-/Pfad-Trennung statisch prüfbar)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1: AI-/Blockchain-Compute trennen)
- aurora-ai ARCHITECTURE (Aurora may request, policy decides, kernel executes)
