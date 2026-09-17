---
spec_id: SPEC-OVERVIEW-atc-node
title: "Specification Overview & Gap-Inventur (atc-node)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — Inventur, keine Implementierungsbehauptung
repository: atc-node
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
---

# Specification Overview — atc-node

> **Ehrlicher Status:** Inventur-Dokument (SCR-0071). „No status without
> evidence" — hier wird nichts als implementiert behauptet.

## 1. Rolle & Zweck

**Full Node (L5).** RPC-/P2P-Client der A-TownChain: Synchronisation, Mempool-Relay, State-Follow.

## 2. Normative Bindungen (bereits verbindlich bzw. in Spezifikation)

ATC-CONSENSUS-305/306 (Fork/Finality-Anbindung), ATC-STATE-001 (Apply), ATC-NETWORK-ID-001

## 3. Bekannte Spezifikations-Gaps (Backlog, folgt via SCR)

Node-Sync-/Mempool-Verhalten, RPC-API-Spezifikation, Peer-Downgrade-Regeln — folgt via SCR, sobald Implementierungsbeginn.

## 4. Status-Gates

- [ ] Detail-Spezifikation je Gap (SCR je Bereich)
- [ ] Implementierung mit je-Anforderung-Nachweis
- [ ] Conformance-/CI-Evidence

## 5. Referenzen

- atc-standards/registry/framework.yaml (Katalog)
- Owner-Audit-Welle 10.09.2026 (SCR-0069/0070/0071)
