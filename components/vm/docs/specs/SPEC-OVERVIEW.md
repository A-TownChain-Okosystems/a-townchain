---
spec_id: SPEC-OVERVIEW-atc-vm
title: "Specification Overview & Gap-Inventur (atc-vm)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — Inventur, keine Implementierungsbehauptung
repository: atc-vm
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
---

# Specification Overview — atc-vm

> **Ehrlicher Status:** Inventur-Dokument (SCR-0071). „No status without
> evidence" — hier wird nichts als implementiert behauptet.

## 1. Rolle & Zweck

**ATVM (L5, Contract-Ausführung).** Implementierung der ATVM — normative Quelle ist ATC-VM-001 (atclang/specs/vm).

## 2. Normative Bindungen (bereits verbindlich bzw. in Spezifikation)

ATC-VM-001, ATC-BC-001, ATC-VM-CONF-001, IFC-0009 (ATVM Execution-API, derzeit ohne Consumer)

## 3. Bekannte Spezifikations-Gaps

### Gas-Kosten-Registry

Die kanonische Rust-VM erzwingt jetzt Gasabrechnung vor jedem Opcode-Effekt. Der bestehende ATC/ShivaCore-Baseline-Satz ist im `Op::gas_cost()` zentralisiert; Out-of-Gas beendet die Ausführung fail-closed und verhindert den betreffenden Opcode-Effekt.

**Implementation commit:** `473f6411d5ce2c5764800fb14e9ad815b33a5eda`

**Wichtig:** Der Baseline-Satz ist konsensrelevant und darf nicht stillschweigend geändert werden. Die normative ATC-VM-001-Gas-Registry muss diesen Satz explizit einfrieren bzw. eine versionierte Änderung definieren. Bis zum Exact-SHA-Conformance-Lauf bleibt dieser Punkt `IMPLEMENTED / CI_PENDING`, nicht `VERIFIED`.

## 4. Status-Gates

- [ ] Detail-Spezifikation je Gap (SCR je Bereich)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-/CI-Evidence

## 5. Referenzen

- atc-standards/registry/framework.yaml (Katalog)
- Owner-Audit-Welle 10.09.2026 (SCR-0069/0070/0071)
