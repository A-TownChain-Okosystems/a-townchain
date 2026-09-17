---
spec_id: ATC-ORACLE-001
title: "Source Trust Model"
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

# Source Trust Model (ATC-ORACLE-001)

> **Ehrlicher Status:** SCR-0072 / Owner-Audit-Welle 3 (10.09.2026).
> Implementierung, Tests und Evidence PENDING — Reihenfolge:
> Spec-Freeze (Owner §9) → Implementierung → Conformance-Evidence.

## 1. Zweck
Jede Observation ist signiert, versioniert und auf Frische prüfbar; AI ist niemals Truth Source.

## 2. Scope
- Oracle-Adapter, Feeds, Aggregationseingang

## 3. Normative Anforderungen (MUST)
- **REQ-001:** Pflichtfelder je Observation: source_id, provider, endpoint, asset/data_type, timestamp, value, confidence, signature, provenance, freshness, status **[Nachweis: schema+unit]**
- **REQ-002:** Signaturen gegen registrierte Source-Keys; unbekannte Quelle → REJECT **[Nachweis: negative]**
- **REQ-003:** AI-Werte sind Observationen mit model_id/model_version/input_hash/output_hash — Pfad: AI→Observation→Validation→Attestation→Policy→Oracle-Konsens→ATVM **[Nachweis: design+audit]**

## 4. Invarianten
- Kein unsignierter Wert erreicht die Aggregation
- AI ist nie autoritativ

## 5. Conformance-Tests (Mindestkategorien)
- Forged signature → reject
- Unknown source → reject
- Stale → reject

## 6. Abhängigkeiten & Kompatibilität
ATC-STD-SC-016 (Contract-Seite).

## 7. Referenzen
- Owner-Audit P1-ORACLE-001/002

## 8. Status-Gates
- [ ] Spec-Freeze (Owner-Review, §9)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence)
- [ ] Security-Review
