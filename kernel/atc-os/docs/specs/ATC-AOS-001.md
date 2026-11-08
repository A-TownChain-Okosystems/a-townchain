---
spec_id: ATC-AOS-001
title: "Integration-Evidence-Standard"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: a-townchain-os
layer: L7
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# Integration-Evidence-Standard (ATC-AOS-001)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Maschinenlesbare Evidence-Records für Build-/Test-Claims.

## 2. Scope
- CI/CD, Status

## 3. Normative Anforderungen (MUST)
- **REQ-001:** .evidence/: latest-test-run.json, workspace-tests.json, kernel-tests.json, governance-gate.json, build-manifest.json mit commit, timestamp, toolchain, counts, result **[Nachweis: ci+unit]**
- **REQ-002:** STATUS/README-Zahlen (731 Workspace/674 Kernel) nur aus .evidence/ generiert/referenziert; manuelle Zahlen verboten **[Nachweis: audit+negative]**

## 4. Invarianten
- Kein PASS-Claim ohne Evidence-Record mit SHA und Run-ID

## 5. Conformance-Tests (Mindestkategorien)
- Evidence ↔ Status-Claim
- Fehlende Evidence → kein Status

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit AOS P0-004/P1-001

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
