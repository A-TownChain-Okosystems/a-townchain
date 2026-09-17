---
spec_id: ATC-MIN-001
title: "PoW-Algorithmus (SHA3-ATC)"
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

# PoW-Algorithmus (SHA3-ATC) (ATC-MIN-001)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Normative PoW-Spezifikation — konsens-gebunden an ATC-CONSENSUS-303.

## 2. Scope
- Hashing, Nonce-Suche, Proofs

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Kanonisches Header-Encoding: Feldfolge, little-endian, algorithm_id versioniert **[Nachweis: vector+unit]**
- **REQ-002:** H_pow = SHA3-ATC(domain_tag || canonical_header || nonce); nonce 64-bit, extraNonce 32-bit **[Nachweis: vector]**
- **REQ-003:** difficulty→target und Target-Vergleich deterministisch (u64, kein Float); kanonische Test-Vektoren **[Nachweis: vector]**

## 4. Invarianten
- Proof nur gültig gegen kanonischen Header und Target des Konsens

## 5. Conformance-Tests (Mindestkategorien)
- Valid/Invalid-Proof-Vektoren
- Header-Manipulation → invalid

## 6. Abhängigkeiten & Kompatibilität
ATC-CONSENSUS-303 ist regelgebend.

## 7. Referenzen
- Owner-Audit MIN-P1-001..004

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
