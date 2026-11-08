---
document_id: ATC-DOC-STD-COMP-001
title: "Org-Standards-Umsetzungscheck 2026-09-10 (SCR-0074)"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
standard: ATC-STD-MD-001
scr: SCR-0074
---

# Org-Standards-Umsetzungscheck — 2026-09-10 (SCR-0074)

> Owner-Direktive: »Prüfe jedes Repository ob die Standards umgesetzt wurden.«
> Maschinenlesbarer Scan: `docs/compliance/ORG-COMPLIANCE-SCAN-2026-09-10.json`

## Ergebnis (nach Fix-Runde im selben SCR)

- **MUST-Artefakte (ATC-STD-201):** 26/26 Repos vollständig (4 fehlende STATUS.md nachgezogen)
- **Governance-CI:** 26/26 Repos mit Workflow
- **Compliance-Badge:** 26/26 (Textform »ATC COMPLIANCE« je README)
- **Lizenz:** 26/26 konsistent Apache-2.0 (3 versteckte package.json-Reste bereinigt)
- **Ehrliche Status-Claims:** 26/26 (letzte 2 unbelegte PASS-Claims beseitigt)
- **Spec-Pakete:** 24/26 Produkt-Repos (atc-standards = selbst Standards-SSOT, a-townchain-os-docs = Dok-Hub — beide N/A)

## Scan-Matrix (Vor-Fix-Stand, Fixes siehe F-088..F-090)

| Repository | MUST 11/11 | Governance-CI | Badge | Lizenz | PASS ehrlich | Specs |
|---|---|---|---|---|---|---|
| .github | *EXEMPT* | — | — | — | — | — |
| a-townchain | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| a-townchain-os | 11/11 | ✅ | ❌ | ✅ | ❌ | ✅ |
| a-townchain-os-docs | 11/11 | ✅ | ❌ | ❌ | ✅ | — |
| atc-algorithm | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-compute | 10/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-contracts | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-explorer | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-indexer | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-interop | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-launchpad | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-marketplace | 10/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-mining | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-node | 10/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-oracle | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-sdk | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-shivacore | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-standards | 11/11 | ✅ | ❌ | ✅ | ✅ | — |
| atc-storage | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-vm | 11/11 | ✅ | ❌ | ✅ | ❌ | ✅ |
| atc-wallet | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atc-zkp | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| atclang | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| aurora-ai | 11/11 | ✅ | ❌ | ❌ | ✅ | ✅ |
| genesis-chronicles | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| genesis-engine | 11/11 | ✅ | ❌ | ✅ | ✅ | ✅ |
| globus-os | 10/11 | ✅ | ❌ | ✅ | ✅ | ✅ |

## Offen (F-091)

Der Scan lief ad hoc als Sandbox-Skript. Empfehlung: als wiederverwendbares
Organ-Tool `tools/org_compliance_scan.py` im .github-Hub verankern und je
SCR-Lauf bzw. wöchentlich ausführen (Badge-Regex: `ATC[- ]COMPLIANCE`).
