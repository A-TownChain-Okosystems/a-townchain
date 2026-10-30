---
document_id: ATC-DOC-ARC-ATCH-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-13
updated: 2026-09-13
standard: ATC-STD-MD-001
---

# Architecture Specification — a-townchain

## Übersicht

`a-townchain` ist die Chain-Protokoll-Bibliothek (Layer L3) der A-TownChain: Chain-ID 658467, Hybrid-Konsens (PoW+PoS+PoH), ZKP-Verifikation, On-Chain-Governance und Chain-DNS. Nach AD-012 ist die Chain ein Kernel-System-Service des KAI-OS.

## Subsysteme

1. **Chain Protocol Core:** Block-/Transaktionsmodell, Chain-ID 658467, Validierungsregeln (`modules/`).
2. **Konsens-Implementierungsgrenze:** Der migrierte Quellbestand liegt in `components/algorithm`; formelle Capability-Ownership und normative Protokollautorität bleiben bis zur SCR-0131-/§9-Entscheidung offen.
3. **ZKP-Integration:** Proof-Verifikation — kryptografische Primitive kanonisch in `atc-zkp`.
4. **On-Chain-Governance:** Abstimmungs- und Parameter-Änderungsmechanismen.
5. **Chain-DNS:** Namensauflösung auf der Chain.
6. **Kernel-Service (AD-012):** Dienste-Integration in das KAI-OS.

## Verantwortungsgrenzen

- `atc-node` betreibt das Protokoll (Full-Node-Binary/Runtime) — hier wird es definiert.
- `components/algorithm` enthält den migrierten Konsens-Prototyp; `a-townchain` integriert ihn. Die formelle Ownership bleibt bis zur genehmigten SCR-0131-Entscheidung offen.
- `atc-vm` führt Contracts aus — das Protokoll definiert die Ausführungssemantik.

## Registry-Einordnung

| Property | Value |
|---|---|
| Layer | L3 |
| Criticality | C1 |
| Security-Klasse | S4 |
| Maturity | R-Level laut `.atc/repository.yaml` · Statusleiter in `.atc/evidence/evidence.yaml` (SCR-0080) |
| Canonical | a-townchain (Chain-Protokoll, AD-012 Kernel-Service) |
| Domäne | domaene laut registry/repositories.yaml |

> Ehrlichkeitsregel: CLAIMED ≠ PASS · IMPLEMENTED ≠ VERIFIED — der verbindliche Implementierungsstand
> liegt ausschließlich in `.atc/evidence/evidence.yaml`, nicht in dieser Spezifikation.
