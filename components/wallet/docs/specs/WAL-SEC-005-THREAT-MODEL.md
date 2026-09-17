---
spec_id: WAL-SEC-005
title: "Wallet Threat Model Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-wallet
layer: L5-Wallet
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Wallet Threat Model Specification (WAL-SEC-005)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Verbindliche Bedrohungsliste und Mapping auf Controls.

## 2. Scope (gilt für)

- Bedrohungen (Leakage, Malformed-Input, Replay, Supply-Chain, Phishing-Delegation)
- Kontrolle je Bedrohung
- Review-Kadenz

## 3. Normative Anforderungen (MUST)

- **REQ-WS5-001:** Bedrohungen sind katalogisiert (IDs WTM-01..): mit je zugeordnetem Control (WAL-SEC-001/002, WAL-REPLAY-001, Dependency-Policy) und Test-Referenz — *Nachweis: review*
- **REQ-WS5-002:** Supply-Chain: Dependency-Änderungen am Trusted Core brauchen Review (analog SHIVA-DEP-001); keine unauditierten crypto-Kisten — *Nachweis: governance*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- (in diesem Grundgerüst noch offen)

## 6. Conformance-Tests (Mindestkategorien)

- threat-to-control-matrix ist vollständig (je Bedrohung ≥1 Test)

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P2 Threat Model)
