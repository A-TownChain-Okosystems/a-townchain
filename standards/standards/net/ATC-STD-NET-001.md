---
standard:
  id: ATC-STD-NET-001
  title: "ATC-STD-NET-001 — Devnet Standard"
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

# ATC-STD-NET-001 — Devnet Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Netzwerkstufe Devnet (ATC-DEVNET). Nicht gilt: Testnet/Mainnet-Regeln, Mainnet-Wert-Semantik
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Devnet ist die Experimentierstufe: Entwickler duerfen schnell brechen.
Devnet muss nicht stabil sein — aber jeder Build MUSS reproduzierbar und
testbar sein (REQ-NET-005).

| Stufe | Zweck | Datenwert | Sicherheit | Release-Status |
|---|---|---|---|---|
| Devnet | Entwicklung & schnelle Tests | wertlos | niedrig-mittel | instabil erlaubt |
| Testnet | Integration, externe Tests & Security | wertlos | mittel-hoch | kontrollierte Aenderungen |
| Mainnet | Produktivbetrieb | realer Wert | maximal | Aenderungen nur nach Governance |

## 1. Eigenschaften (REQ-NET-001, REQ-NET-002)

| Feld | Wert |
|---|---|
| Network | ATC-DEVNET |
| Chain-ID | eindeutig, Allokation registry/networks.yaml (658469) |
| Token | DEV-ATC |
| Wert | 0 (wertlos, kein realer Wert, KEIN Handel) |
| Reset | erlaubt |
| Genesis | mutable (automatische Neuerstellung erlaubt, NET-005) |
| Validators | flexibel |

## 2. Erlaubt auf Devnet (REQ-NET-003)

Breaking Changes · neue Consensus-Implementierungen · neue
ATCLang-Versionen · Smart-Contract-Tests · Mining-/Validator-Tests ·
Datenbank- und State-Resets · automatische Genesis-Neuerstellung ·
experimentelle APIs · KI-Agenten-Tests.

## 3. Devnet-Gate GATE-011 (REQ-NET-006: MUST)

```
BUILD → UNIT TESTS → INTEGRATION TESTS → DEVNET DEPLOYMENT → SMOKE TEST
```

> Devnet muss nicht stabil sein, aber jeder Build muss reproduzierbar
> und testbar sein.

## 4. Promotionsregel (REQ-NET-007)

Nichts wird durch Devnet allein Mainnet-ready (NET-002 §4). Promotion
Devnet→Testnet nur nach GATE-011-PASS (NET-004).

## Requirements

- id: REQ-NET-001
  title: "Devnet ist Pflicht-Tier 1 der Promotion-Pipeline"
  severity: MANDATORY
- id: REQ-NET-002
  title: "Eigenschaften-Matrix (ATC-DEVNET, DEV-ATC wertlos, Reset erlaubt, Genesis mutable, Validators flexibel)"
  severity: MANDATORY
- id: REQ-NET-003
  title: "Experimentier-Freiraum: Breaking Changes, neue Consensus/ATCLang, Resets, experimentelle APIs erlaubt"
  severity: MANDATORY
- id: REQ-NET-005
  title: "Jeder Build reproduzierbar und testbar (Reproducible Builds)"
  severity: MANDATORY
- id: REQ-NET-006
  title: "GATE-011 vor jeder Devnet-Promotion durchlaufen"
  severity: MANDATORY
- id: REQ-NET-007
  title: "Devnet-PASS allein begruendet keine Mainnet-Readiness"
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
