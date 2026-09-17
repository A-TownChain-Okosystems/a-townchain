---
spec_id: ATC-ORACLE-003
title: "Attestation-Schema"
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

# Attestation-Schema (ATC-ORACLE-003)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Maschinenlesbares Attestation-Schema inkl. AI-Attestations.

## 2. Scope
- Attestation-Erzeugung/-Verifikation

## 3. Normative Anforderungen (MUST)
- **REQ-001:** 14 Pflichtfelder: attestation_id, oracle_id, source_id, subject, observation, timestamp, model_id, model_version, input_hash, output_hash, evidence_hash, confidence, signature, policy_version **[Nachweis: schema+unit]**
- **REQ-002:** Attestations sind einer Policy-Version zugeordnet; Wechsel versioniert, invalidiert nicht rückwirkend **[Nachweis: design]**

## 4. Invarianten
- Kein Attestation-Claim ohne verifizierbare Signaturen/Hashes

## 5. Conformance-Tests (Mindestkategorien)
- Round-trip
- Fehlendes Feld → reject
- Hash-Mismatch → reject

## 6. Abhängigkeiten & Kompatibilität
ATC-AI-CAP-001 (aurora-ai).

## 7. Referenzen
- Owner-Audit P1-ORACLE-003

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
