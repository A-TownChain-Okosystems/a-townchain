---
spec_id: ATC-VM-CONF-001
title: "VM Determinism & Conformance Specification (Chain-Sicht)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain
layer: L3-Chain
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# VM Determinism & Conformance Specification (Chain-Sicht) (ATC-VM-CONF-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Chain-seitige Pflichten an die ATVM: Determinismus, Unabhängigkeit, Evidence.

## 2. Scope (gilt für)

- Determinismus-Anforderung (x86_64/aarch64)
- Referenz-Vektoren (Bytecode → Effects)
- Unabhängige Verifikation

## 3. Normative Anforderungen (MUST)

- **REQ-VCF-001:** VM-Implementierungen müssen auf x86_64 und aarch64 bit-identische Effekte produzieren (Vektor-Suite, Registry-publiziert) — *Nachweis: differential+property*
- **REQ-VCF-002:** VM ist nie gleichzeitig Spec und Orakel: Referenz-Vektoren stammen aus der Spec (ATC-VM-001) und werden von einer unabhängigen Implementierung erzeugt/geprüft — *Nachweis: process*
- **REQ-VCF-003:** Mainnet-Gate: kein Block mit VM-Version, deren Conformance-Evidence fehlt (Registry-Eintrag mit Run/Commit) — *Nachweis: governance*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Conformance-Evidence ist Voraussetzung für jede VM-Version im Netz

## 6. Conformance-Tests (Mindestkategorien)

- vm_conformance_suite.json (ATC-VM-001 + ATC-BC-001 Vektoren)
- cross_platform_matrix.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (9: ATVM determinism/conformance)
