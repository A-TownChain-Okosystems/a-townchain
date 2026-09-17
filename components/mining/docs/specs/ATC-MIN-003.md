---
spec_id: ATC-MIN-003
title: "Mining-Manager-Zustandsmaschine"
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

# Mining-Manager-Zustandsmaschine (ATC-MIN-003)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Deterministischer Manager-Automat.

## 2. Scope
- mining-manager

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Zustände: IDLE→CONNECTING→SYNCING→READY→JOB_RECEIVED→MINING(SHARE_FOUND|BLOCK_FOUND|JOB_STALE|TARGET_CHANGED)→SUBMIT→ACCEPTED/REJECTED **[Nachweis: design+unit]**
- **REQ-002:** Stale-Job-Protection: neuer Block → alte Jobs sofort invalid **[Nachweis: negative]**

## 4. Invarianten
- Kein Proof gegen stale Job

## 5. Conformance-Tests (Mindestkategorien)
- Job stale → verwerfen
- Target-Change mid-job
- Reconnect

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit MIN-P1 (Manager)

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
