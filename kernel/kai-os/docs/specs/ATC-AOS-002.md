---
spec_id: ATC-AOS-002
title: "Module-Provenance-Gate"
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

# Module-Provenance-Gate (ATC-AOS-002)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Verifizierbare Herkunft jeder synchronisierten Datei.

## 2. Scope
- sync_modules.py, workspace

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Provenanz je Datei: Quell-Repo, Commit-SHA, Version, Checksum, Manifest — nach Sync verifizierbar **[Nachweis: unit+integration]**
- **REQ-002:** Dirty-State-Detection: manuelle Edits im Integration-Tree → Alarm (AD-017: Produkt gewinnt) **[Nachweis: negative]**
- **REQ-003:** a-townchain-os ist niemals zweite Source of Truth **[Nachweis: audit]**

## 4. Invarianten
- Jede Datei rückführbar auf (Repo, SHA, Version, Hash)

## 5. Conformance-Tests (Mindestkategorien)
- Sync 2× → identisch
- Dirty-Edit → Alarm
- Mismatch → Block

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit AOS P1-002/003 · AD-017

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
