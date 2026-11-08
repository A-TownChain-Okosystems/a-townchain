---
standard:
  id: ATC-STD-NET-002
  title: "ATC-STD-NET-002 — Testnet Standard"
  version: "1.0.0"
  status: approved
  category: net
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: null
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-NET-002 — Testnet Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Netzwerkstufe Testnet (ATC-TESTNET). Nicht gilt: Devnet-Experimentierfreiheit, Mainnet-Wert-Semantik
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Testnet ist die realistische Simulation des spaeteren Mainnets. Hier darf
die Architektur nicht mehr beliebig geaendert werden — der Unterschied zum
Mainnet liegt in Genesis, Chain-ID, Keys, Infrastruktur, Tokenwert und
Governance-Status, NICHT in einer anderen Implementierung (REQ-NET-011,
siehe auch NET-004 §3).

| Stufe | Zweck | Datenwert | Sicherheit | Release-Status |
|---|---|---|---|---|
| Devnet | Entwicklung & schnelle Tests | wertlos | niedrig-mittel | instabil erlaubt |
| Testnet | Integration, externe Tests & Security | wertlos | mittel-hoch | kontrollierte Aenderungen |
| Mainnet | Produktivbetrieb | realer Wert | maximal | Aenderungen nur nach Governance |

## 1. Eigenschaften (REQ-NET-012)

| Feld | Wert |
|---|---|
| Network | ATC-TESTNET |
| Chain-ID | eindeutig, Allokation registry/networks.yaml (658468) |
| Token | TEST-ATC |
| Wert | 0 (wertlos) |
| Reset | nur kontrolliert (dokumentiert + SCR) |
| Genesis | versioniert (NET-005) |
| Validators | registriert |
| Consensus | Mainnet-kompatibel (REQ-NET-013: MUST) |

## 2. Testpflicht (REQ-NET-014: MUST — alle Punkte)

Consensus · Validatoren · PoW/PoS/PoH-Komponenten · P2P · Wallet · RPC ·
Explorer · Smart Contracts · ATCLang · VM · Bridges · Governance ·
Staking · Tokenomics · Fee Market · Slashing · Recovery ·
State Synchronization · Upgrade Mechanismus · Security · Performance ·
Chaos/Fault Tests

## 3. Testnet-Gate GATE-012 (REQ-NET-015: MUST)

```
BUILD → UNIT TESTS → INTEGRATION → DEVNET → TESTNET → E2E TEST → LOAD TEST
→ SECURITY TEST → CHAOS TEST → RECOVERY TEST → AUDIT
```

## 4. Kardinalregel (REQ-NET-016)

> Ein Feature ist nicht deshalb Mainnet-ready, weil es auf Devnet
> funktioniert. Testnet-Pass ist notwendige, nicht hinreichende
> Bedingung fuer die Mainnet-Promotion.

## Requirements

- id: REQ-NET-011
  title: "Testnet nutzt dieselbe Protokollarchitektur wie Mainnet (Unterschiede nur Genesis/Chain-ID/Keys/Infra/Tokenwert/Governance)"
  severity: MANDATORY
- id: REQ-NET-012
  title: "Eigenschaften-Matrix (ATC-TESTNET, TEST-ATC wertlos, Reset kontrolliert, Genesis versioniert, Validators registriert)"
  severity: MANDATORY
- id: REQ-NET-013
  title: "Consensus Mainnet-kompatibel"
  severity: MANDATORY
- id: REQ-NET-014
  title: "Vollstaendige Testpflicht-Liste (24 Gebiete) auf Testnet erfuellt"
  severity: MANDATORY
- id: REQ-NET-015
  title: "GATE-012 vor jeder Testnet-Promotion durchlaufen"
  severity: MANDATORY
- id: REQ-NET-016
  title: "Devnet-Funktionieren begruendet keine Mainnet-Readiness"
  severity: MANDATORY


## Compliance

Geprueft per Gate-Aufzeichnung (GATE-011…013-Records) und Review gegen
deklarierten REQ-NET-Anforderungen (AUD-NNN-Abschluss gemaess ATC-STD-BUG-004). Maschinenlesbar:
schemas/network-environment.schema.json validiert Umgebungskonfigurationen
(.atc/network/{devnet,testnet,mainnet}.yaml); Registry: registry/networks.yaml.
Verstoss gegen MANDATORY = GATE BLOCKED.

## Security Considerations

Sicherheitsniveau der Stufe gemaess Matrix (NET-007): Devnet
niedrig-mittel (Debug erlaubt), Testnet mittel-hoch (Security-/Chaos-Tests
Pflicht), Mainnet maximal (keine Debug-Endpunkte, keine ungeprueften
Vertraege, ATVM-Verifier + License Gate als Trust Boundary).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Mandat AD-041) |

## References

**NORMATIVE:** ATC-STD-NET-001…008 (Reihe), ATC-STD-000 (§7 IDs, §33),
ATC-STD-203 (Release-Gates, Conventional Commits), ATC-STD-BUG-003/004
(Fix-/Merge-Gate), registry/networks.yaml · schemas/network-environment.schema.json
**INFORMATIVE:** AD-004 (Chain-ID 658467), AD-021/022 (ATCLang-Baseline),
AD-027 (Lauffaehigkeits-Roadmap M4), docs/roadmap/
