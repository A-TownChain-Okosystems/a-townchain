---
standard:
  id: ATC-STD-NET-003
  title: "ATC-STD-NET-003 — Mainnet Standard"
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

# ATC-STD-NET-003 — Mainnet Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Netzwerkstufe Mainnet (ATC-MAINNET, Chain-ID 658467 permanent). Nicht gilt: Experimentier- und Reset-Freiheit
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Mainnet ist das Produktionsnetzwerk mit reellem wirtschaftlichem Wert.
Nach dem Genesis-Moment ist der Zustand unumkehrbar: Reset VERBOTEN,
Genesis immutable, Consensus frozen (REQ-NET-021).

| Stufe | Zweck | Datenwert | Sicherheit | Release-Status |
|---|---|---|---|---|
| Devnet | Entwicklung & schnelle Tests | wertlos | niedrig-mittel | instabil erlaubt |
| Testnet | Integration, externe Tests & Security | wertlos | mittel-hoch | kontrollierte Aenderungen |
| Mainnet | Produktivbetrieb | realer Wert | maximal | Aenderungen nur nach Governance |

## 1. Eigenschaften (REQ-NET-022)

| Feld | Wert |
|---|---|
| Network | ATC-MAINNET |
| Chain-ID | 658467 (permanent, AD-004, immutable) |
| Token | ATC |
| Wert | real |
| Reset | VERBOTEN |
| Genesis | immutable (NET-005) |
| Consensus | frozen |
| Protocol | governed (NET-006) |
| Validators | permissionless / defined by protocol |

## 2. Verbote auf Mainnet (REQ-NET-023: MUST NOT)

Manuelle State-Resets · Test-Token · Debug-Endpunkte · experimentelle
Consensus-Regeln · unkontrollierte Datenbankmigrationen · direkte
Aenderungen am Chain State · ungepruefte Smart Contracts.

## 3. Mainnet-Gate GATE-013 (REQ-NET-024: MUST — Reihenfolge verbindlich)

```
TESTNET PASS → SECURITY AUDIT PASS → CONSENSUS VALIDATION →
PERFORMANCE BASELINE → RECOVERY VALIDATION → GENESIS VALIDATION →
REPRODUCIBLE BUILD → RELEASE CANDIDATE → GOVERNANCE APPROVAL →
MAINNET GENESIS → MAINNET
```

## 4. Protokollaenderungen (REQ-NET-025)

Jede Protokollaenderung benoetigt den formalisierten Upgrade-Prozess
nach NET-006 (Governance-Approval Pflicht). Direkte Eingriffe in den
Chain State sind ausgeschlossen.

## Requirements

- id: REQ-NET-021
  title: "Nach Mainnet-Genesis: Reset VERBOTEN, Genesis immutable, Consensus frozen"
  severity: MANDATORY
- id: REQ-NET-022
  title: "Eigenschaften-Matrix (ATC-MAINNET, Chain-ID 658467 permanent, ATC realer Wert, Protocol governed)"
  severity: MANDATORY
- id: REQ-NET-023
  title: "Verbotsliste (7 Verbote) eingehalten"
  severity: MANDATORY
- id: REQ-NET-024
  title: "GATE-013 in vollstaendiger Reihenfolge vor Mainnet-Promotion"
  severity: MANDATORY
- id: REQ-NET-025
  title: "Protokollaenderungen nur via NET-006-Upgrade-Prozess"
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
