---
standard:
  id: ATC-STD-NET-006
  title: "ATC-STD-NET-006 — Network Upgrade Standard"
  version: "1.1.0"
  status: approved
  category: net
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: null
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-NET-006 — Network Upgrade Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Formalisierter Protokoll-Upgrade-Prozess, insbesondere Mainnet. Nicht gilt: Devnet-Experimente (NET-001 §2)
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Auf Mainnet ist der Consensus frozen — jede Protokollaenderung laeuft
ueber einen formalisierten Upgrade-Prozess mit Governance-Approval
(REQ-NET-051).

## 1. Upgrade-Prozess (REQ-NET-052: MUST — Reihenfolge verbindlich)

```
PROPOSAL (SCR-NNN) → TECHNICAL REVIEW → TESTNET-NACHWEIS (GATE-012-Re-Run)
→ GOVERNANCE APPROVAL → AKTIVIERUNGSHOEHE/-EPOCH FESTLEGEN →
KOORDINIERTES DEPLOYMENT → BEOBACHTUNG → ABGESCHLOSSEN
```

## 2. Rollback-Politik (REQ-NET-053)

Jedes Upgrade definiert VOR der Aktivierung einen dokumentierten
Rollback-/Notfallpfad (§32 Emergency, ATC-STD-000). Auf Mainnet gibt es
keinen Chain-State-Rollback — nur Protokoll-Downgrades vor Aktivierung
oder Folge-Upgrades.

## 3. Stufenspezifik (REQ-NET-054)

Devnet: Upgrades frei, Feature-Flags experimentell. Testnet:
Upgrade-Mechanismus selbst ist Testpflicht-Gegenstand (NET-002 §2).
Mainnet: nur NET-006-Prozess.


- **Aktivierungs-Signal-Schwellenwert:** Die Upgrade-Signalisierung MUSS mindestens 80% der aktiven Staking-Power aufweisen (REQ-NET-055).
- **State-Migration-Determinismus:** State-Migrations-Skripte MUESSEN im Testnet auf identische Hashes geprüft werden (REQ-NET-056).
- **Emergency-Pause-Mechanismus:** Ein automatischer Stopp MUSS bei unvorhergesehenen State-Abweichungen während des Upgrades greifen (REQ-NET-057).
- **Backward-Compatibility-Proof:** Die Kompatibilität mit bestehenden Node-Clients MUSS vor der Upgrade-Freigabe belegt sein (REQ-NET-058).

## Requirements

- id: REQ-NET-051
  title: "Mainnet-Consensus frozen; Aenderungen nur via Upgrade-Prozess"
  severity: MANDATORY
- id: REQ-NET-052
  title: "8-stufiger Upgrade-Prozess mit Governance-Approval"
  severity: MANDATORY
- id: REQ-NET-053
  title: "Rollback-/Notfallpfad vor Aktivierung dokumentiert"
  severity: MANDATORY
- id: REQ-NET-054
  title: "Stufenspezifik Devnet/Testnet/Mainnet fuer Upgrades"
  severity: MANDATORY
- id: REQ-NET-055
  title: "Aktivierungs-Signal-Schwellenwert MUSS >= 80% Staking-Power betragen"
  severity: MANDATORY
- id: REQ-NET-056
  title: "State-Migration-Determinismus MUSS im Testnet bewiesen werden"
  severity: MANDATORY
- id: REQ-NET-057
  title: "Emergency-Pause-Mechanismus MUSS bei Abweichungen greifen"
  severity: MANDATORY
- id: REQ-NET-058
  title: "Backward-Compatibility-Proof MUSS vor Upgrade erbracht werden"
  severity: MANDATORY

## Metriken & Akzeptanzkriterien

1. **Upgrade-Signalisierungsquote:** MUSS >= 80% der aktiven Staking-Power vor Aktivierung erreichen.
2. **Testnet-Stabilität vor Mainnet:** MUSS mindestens 14 Tage fehlerfreien Testnet-Betrieb belegen.
3. **Migration-Dauer Obergrenze:** Die State-Migration MUSS innerhalb von maximal 100 Blöcken abgeschlossen sein.
4. **Rollback-Bereitschaftsquote:** MUSS einen 100% verifizierten Notfallpfad vor der Freigabe belegen.

## Compliance & Verifikation

Geprueft per Gate-Aufzeichnung (GATE-011…013-Records) und Review gegen deklarierten REQ-NET-Anforderungen (AUD-NNN-Abschluss gemaess ATC-STD-BUG-004). Die Einhaltung aller Vorgaben MUSS kontinuierlich in den Validator-Läufen S-01 bis S-25 sowie durch Repository-Audits nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 überprüft werden. Unvollständige Nachweise erzeugen Findings F-001 bis F-999. Maschinenlesbar: schemas/network-environment.schema.json validiert Umgebungskonfigurationen (.atc/network/{devnet,testnet,mainnet}.yaml); Registry: registry/networks.yaml. Verstoss gegen MANDATORY = GATE BLOCKED.

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
Zusätzliche Sicherheitsregel: Sämtliche Sicherheitsvorgaben der jeweiligen Stufe MUESSEN zwingend eingehalten werden; ein fehlerhafter Zustand auf Mainnet MUSS das Release-Gate unverzüglich stoppen.
