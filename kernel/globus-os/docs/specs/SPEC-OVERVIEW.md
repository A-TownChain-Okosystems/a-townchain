---
spec_id: SPEC-OVERVIEW-globus-os
title: "Specification Overview & Gap-Inventur (globus-os)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — Inventur, keine Implementierungsbehauptung
repository: globus-os
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
---

# Specification Overview — globus-os

> **Ehrlicher Status:** Inventur-Dokument (SCR-0071). „No status without
> evidence" — hier wird nichts als implementiert behauptet.

## 1. Rolle & Zweck

**Globus OS Userspace (L4).** Userspace des Ökosystems auf ShivaCore — Desktop-Clients (Windows/Linux, Rust std + egui) sind Clients, kein alternatives OS.

## 2. Normative Bindungen (bereits verbindlich bzw. in Spezifikation)

ShivaCore HAL/Syscall-Interface (K9), ATC-AI-TB-001 (Kernel-Grenze)

## 3. Bekannte Spezifikations-Gaps (Backlog, folgt via SCR)

Runtime-/Driver-API-Spezifikation (IFC-0008), Service-Interface-Katalog, Userspace-Security-Modell — folgt mit Implementierung.

## 4. Status-Gates

- [ ] Detail-Spezifikation je Gap (SCR je Bereich)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-/CI-Evidence

## 5. Referenzen

- atc-standards/registry/framework.yaml (Katalog)
- Owner-Audit-Welle 10.09.2026 (SCR-0069/0070/0071)
