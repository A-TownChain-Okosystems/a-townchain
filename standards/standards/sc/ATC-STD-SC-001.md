---
standard:
  id: ATC-STD-SC-001
  title: "ATC-STD-SC-001 — Smart Contract General Standard"
  version: "1.0.0"
  status: approved
  category: sc
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  applies_to: "Alle Smart Contracts des A-TownChain-Oekosystems"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-SC-001 — Smart Contract General Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

ATC-STD-SC-001 ist der Hauptstandard des ATC Smart Contract Standards Framework: Vertragskategorien (SC-CORE..SC-SYSTEM), Source-of-Truth-Kette (Specification → Source → Tests → Audit → Deployment → Verification), Semantic Versioning und das verbindliche Compliance-Gate-System SC-G0..G13. Kein Gate — kein Mainnet.

## Scope

Gilt fuer alle Smart Contracts des Oekosystems (alle Ketten, alle Sprachen/Runtimes). Kategoriespezifische Vertiefungen: SC-002..SC-020. README/Dokumentation der Contract-Repos: ATC-STD-README-001/MD-001.

## 1. Contract-Kategorien (REQ-SC-001)

Jeder Contract MUSS genau einer Kategorie zugeordnet sein:

| ID | Kategorie | Zweck |
|---|---|---|
| SC-CORE | Core Contracts | Blockchain-Kernfunktionen |
| SC-TOKEN | Token | ATC-, Governance- und Utility-Token |
| SC-NFT | NFT | NFTs, Assets, Collections |
| SC-DEFI | DeFi | Staking, Liquidity, Swaps |
| SC-GOV | Governance | DAO, Voting, Proposals |
| SC-MARKET | Marketplace | NFT-/Asset-Handel |
| SC-GAME | GameFi | Shivamon, Items, Rewards |
| SC-MINING | Mining | Mining-Rewards und Emissionen |
| SC-BRIDGE | Bridge | Cross-Chain-Kommunikation |
| SC-IDENTITY | Identity | Identitaet/Reputation |
| SC-ORACLE | Oracle | externe Daten |
| SC-SYSTEM | System | Protokoll- und Netzwerkverwaltung |

## 2. Source-of-Truth-Kette (REQ-SC-002)

Fuer jeden Smart Contract gilt die kanonische Kette:

```text
ATC Standard → Specification → Implementation → Unit Tests → Integration Tests
→ Security Analysis → Independent Audit → Deployment Candidate → Testnet
→ Mainnet → Verification
```

Manuelle Aenderungen direkt auf Production sind UNZULAESSIG.

## 3. Versionierung (REQ-SC-003)

Contracts verwenden Semantic Versioning (ATC-STD-000 §13): MAJOR = Breaking Change, MINOR = neue kompatible Funktion, PATCH = Bugfix/Security-Fix ohne API-Break.

## 4. Compliance-Gate-System (REQ-SC-004)

Verbindliche Gates:

| Gate | Pruefung |
|---|---|
| SC-G0 | Specification vorhanden |
| SC-G1 | Contract Identity (SC-002) |
| SC-G2 | Code Review |
| SC-G3 | Unit Tests (SC-004) |
| SC-G4 | Fuzz/Invariant Tests (SC-004) |
| SC-G5 | Security Scan (SC-003) |
| SC-G6 | Dependency Check (ATC-STD-204) |
| SC-G7 | Audit (SC-005) |
| SC-G8 | Testnet |
| SC-G9 | Governance Approval |
| SC-G10 | Mainnet Deployment (SC-006) |
| SC-G11 | Contract Verification |
| SC-G12 | Registry Update (SC-019) |
| SC-G13 | Continuous Monitoring |

**Kein Gate — kein Mainnet.**

## 5. Architekturkette (REQ-SC-005)

```text
ATC Enterprise Standards → ATC Blockchain Standards → ATC Smart Contract Standards
→ Contract Specification → Source Code → Tests & Security → Audit → Deployment
→ Registry → Monitoring / AuditTrail
```
Verzahnung: Repository-Standards (201/202), Governance (000, ENT), Security (203), KI-Agenten (AAS-001..025, AI-DEV, SC-020), Dependency/Interface-Tracking (204).



## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-001 | Kategorie-Zuordnung (§1) | MUSS |
| REQ-SC-002 | SSOT-Kette (§2) | MUSS |
| REQ-SC-003 | SemVer (§3) | MUSS |
| REQ-SC-004 | Gate-System SC-G0..G13 (§4) | MUSS |
| REQ-SC-005 | Architektur-Verzahnung (§5) | MUSS |

## Compliance

Pruefung im Rahmen der SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13):
kein Gate — kein Mainnet. Verstoege werden als Finding nach ATC-STD-BUG-001
dokumentiert. Registry-Pflichten nach ATC-STD-SC-019.

## Security Considerations

Das Gate-System ist die Kern-Sicherheitsfunktion: keine Umgehung einzelner Gates, auch nicht durch Agenten (SC-020) oder Owner-Dringlichkeit ohne dokumentierte DEC-Record-Ausnahme (ATC-ENT-002).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) |

## References

- **NORMATIVE:** ATC-STD-000 (Verfassung), ATC-STD-201/202, ATC-STD-203
  (Security), ATC-STD-204 (Dependency & Interface), ATC-STD-SC-001
  (Familien-Hauptstandard)
- **INFORMATIVE:** ATC-AAS-001..025 (Agenten-Standards), AI-DEV-001..012,
  contracts/registry/ (Contract Registry)
