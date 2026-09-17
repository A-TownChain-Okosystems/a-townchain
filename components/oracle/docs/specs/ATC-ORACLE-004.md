---
spec_id: ATC-ORACLE-004
title: "Replay-Schutz"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-oracle
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# Replay-Schutz (ATC-ORACLE-004)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Gültig signierte alte Werte gelten nie als aktuell.

## 2. Scope
- Verifikation, Feeds

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Felder: nonce/sequence, timestamp, source_epoch, feed_version, chain_id (658467), oracle_id, observation_hash **[Nachweis: unit+negative]**
- **REQ-002:** Persistenter Replay-Store; verarbeitete IDs → REJECT **[Nachweis: unit]**

## 4. Invarianten
- Kein Wert wird zweimal akzeptiert

## 5. Conformance-Tests (Mindestkategorien)
- Replay → reject
- Wrong chain_id → reject
- Timestamp-Manipulation → reject

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit P1-ORACLE-004

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
