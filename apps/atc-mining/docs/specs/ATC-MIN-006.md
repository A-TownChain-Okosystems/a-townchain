---
spec_id: ATC-MIN-006
title: "AI-Telemetry-Boundary"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-mining
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# AI-Telemetry-Boundary (ATC-MIN-006)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
MinerWatcherGPT beobachtet, autorisiert nie.

## 2. Scope
- telemetry, aurora-ai

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Input: Observations/Events/Metriken; Output: Analysen/Alerts **[Nachweis: design]**
- **REQ-002:** Verboten: Konsens-, Reward-, Block-Autorisierung, Regel-Änderung (NO CONSENSUS, NO REWARD AUTH, NO BLOCK AUTH) **[Nachweis: audit+negative]**

## 4. Invarianten
- KI-Output nie autoritativer Eingang in Konsens/Reward

## 5. Conformance-Tests (Mindestkategorien)
- AI-Empfehlung → nur Audit-Event

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit · ATC-AI-TB-001

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
