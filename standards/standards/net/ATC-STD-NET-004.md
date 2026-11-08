---
standard:
  id: ATC-STD-NET-004
  title: "ATC-STD-NET-004 — Network Promotion Standard"
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

# ATC-STD-NET-004 — Network Promotion Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Promotion Devnet→Testnet→Mainnet und Umgebungskonfiguration. Nicht gilt: Within-Tier-Betrieb (NET-001…003)
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Die ATC Network Promotion Pipeline definiert den strikt getrennten
Stufenbergang als Release-, Security- und Governance-Gate. Stufen werden
nie uebersprungen (REQ-NET-031).

```
DEVELOPMENT → DEVNET (GATE-011) → TESTNET (GATE-012 mit Security/Load/
Recovery-Gates) → RELEASE CANDIDATE → MAINNET (GATE-013) → PRODUCTION
```

## 1. Gate-Registry (REQ-NET-032)

| Kanonische ID | Label | Ort | Inhalt |
|---|---|---|---|
| GATE-011 | GATE-DEVNET | Devnet→Testnet | Build, Unit, Integration, Devnet-Deploy, Smoke |
| GATE-012 | GATE-TESTNET | Testnet→RC | E2E, Load, Security, Chaos, Recovery, Audit |
| GATE-013 | GATE-MAINNET | RC→Mainnet | 11-stufige Kette (NET-003 §3) |

Fortfuehrung der GATE-001…010 (ATC-STD-203); IDs numerisch per §7.

## 2. Promotions-Record (REQ-NET-033: MUST)

Jede Promotion ist ein dokumentierter Vorgang: SCR-NNN + Gate-Records
GATE-011/012/013 + AUD-NNN-Abschluss (Verkettung gemaess ATC-STD-BUG-003).

## 3. Umgebungstrennung: EIN Code, DREI Konfigurationen (REQ-NET-034)

Code wird NICHT dreifach kopiert. Trennung erfolgt per Konfiguration:

```
ATC Core
 ├── Environment Configuration: devnet / testnet / mainnet
 ├── Genesis Configuration   (NET-005)
 ├── Network Configuration   (Chain-ID, Peers, Ports)
 ├── Consensus Configuration
 └── Feature Flags
```

Kanonisch: .atc/network/{devnet,testnet,mainnet}.yaml, validiert gegen
schemas/network-environment.schema.json (CI-Pflicht). Zielstruktur des
Repository-Layouts (infrastructure/, genesis/, docs/network/) ist
Informative Referenz fuer M6/M8 (AD-026/027).

## 4. Architektur-Gleichheit (REQ-NET-035: MUST)

Testnet und Mainnet verwenden dieselbe Protokollarchitektur. Der
Unterschied liegt primär in Genesis, Chain-ID, Keys, Infrastruktur,
Tokenwert und Governance-Status — nicht in einer anderen Implementierung.

## Requirements

- id: REQ-NET-031
  title: "Kein Ueberspringen von Stufen (Devnet→Testnet→Mainnet strikt sequenziell)"
  severity: MANDATORY
- id: REQ-NET-032
  title: "GATE-011/012/013 registriert und numerisch kanonisch"
  severity: MANDATORY
- id: REQ-NET-033
  title: "Promotions-Record: SCR + Gate-Records + AUD-Abschluss"
  severity: MANDATORY
- id: REQ-NET-034
  title: "EIN Code, DREI Konfigurationen (.atc/network/*.yaml, schema-validiert)"
  severity: MANDATORY
- id: REQ-NET-035
  title: "Testnet/Mainnet gleiche Protokollarchitektur"
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
