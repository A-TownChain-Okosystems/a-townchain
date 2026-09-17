---
spec_id: ATC-INTEROP-001
title: "Message-Format & Replay-Schutz"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementation PENDING
repository: atc-interop
layer: L5
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0072
depends: [ATC-STD-000, ATC-STD-PROTOCOL-001]
---

# Message-Format & Replay-Schutz (ATC-INTEROP-001)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Kanonische Message-ID + persistenter Replay-Store.

## 2. Scope
- relayer, verifier, contracts

## 3. Normative Anforderungen (MUST)
- **REQ-001:** MSG-ID = H(source_chain_id || source_domain || source_block || source_tx || source_event || nonce || target_chain_id || target_contract || payload_hash) **[Nachweis: vector+unit]**
- **REQ-002:** processed_message[id]=true; zweite Ausführung → REJECT **[Nachweis: unit+negative]**
- **REQ-003:** Kanonische Serialisierung: Feldfolge, Endianness, Versionierung **[Nachweis: vector]**

## 4. Invarianten
- Keine gültige Message wird zweimal ausgeführt

## 5. Conformance-Tests (Mindestkategorien)
- Replay → reject
- Reordered → reject
- Malformed → reject

## 6. Abhängigkeiten & Kompatibilität
Siehe Frontmatter (depends).

## 7. Referenzen
- Owner-Audit INTEROP P0-3

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
