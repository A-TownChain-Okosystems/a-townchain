---
spec_id: LP-SEC-001
title: "Launchpad Security Specification (Criticality-Uplift)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-launchpad
layer: L5-Launchpad
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Launchpad Security Specification (Criticality-Uplift) (LP-SEC-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Sicherheitsnorm für ein System, das reale Kapital-/Assetflüsse verarbeitet — Criticality-Anhebung von S1/low auf S3/financial (Empfehlung des Owner-Audits).

## 2. Scope (gilt für)

- Kritikalitäts-Klassifizierung
- Bedrohungen (Kapitalfluss: Overflow, Replay, Refund-Angriffe, Whitelist-Bypass)
- Invarianten-Schutz (SALE-INV/VEST-INV/WL-INV/AUTH-INV)

## 3. Normative Anforderungen (MUST)

- **REQ-LPS1-001:** Repository-Klassifizierung wird auf criticality: high (S3) angehoben; .atc/repository.yaml + README synchronisieren (offene Governance-Änderung, Owner-Review) — *Nachweis: governance*
- **REQ-LPS1-002:** Invarianten-Katalog verbindlich: SALE-INV-001 (Allocated≤Supply), SALE-INV-002 (Claimed≤Purchased), SALE-INV-003 (Refunded≤Contribution), VEST-INV-001 (Released≤Allocation), WL-INV-001 (InvalidProof⇒Reject), AUTH-INV-001 (Unauthorized⇒Reject) — je mit Property-/Adversarial-Test — *Nachweis: property+adversarial*
- **REQ-LPS1-003:** Refund-Engine: Refund ist deterministisch aus Contribution-Ledger abgeleitet; Double-Refund unmöglich (Nonce) — *Nachweis: unit+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Kein Kapitalfluss ohne zugeordnete Invariante

## 6. Conformance-Tests (Mindestkategorien)

- inv_matrix.json (alle 6 Invarianten als Property-Tests)
- refund_double ⇒ Reject

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1: criticality low ist für Financial-System nicht haltbar; Invarianten-Katalog)
