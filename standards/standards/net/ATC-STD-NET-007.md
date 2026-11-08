---
standard:
  id: ATC-STD-NET-007
  title: "ATC-STD-NET-007 — Network Security Standard"
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

# ATC-STD-NET-007 — Network Security Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-NET-001…008 (Netzwerk-Umgebungen Devnet/Testnet/Mainnet) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** Sicherheitsniveau und -pflichten je Netzwerkstufe. Nicht gilt: Applikations-Security ausserhalb des Netzwerkbetriebs
> **Verweise:** ATC-STD-NET-001…008, ATC-STD-BUG-001…004, ATC-STD-203 (GATE-001…010), registry/networks.yaml, AD-004 (Chain-ID 658467), AD-027 (M4)

---


## Abstract

Sicherheit skaliert mit der Stufe: was auf Devnet erlaubt ist, ist auf
Mainnet kategorisch verboten. Die Stufe bestimmt das Trust-Modell.

## 1. Security-Matrix (REQ-NET-061)

| Stufe | Niveau | Debug-Endpunkte | Security-Audit |
|---|---|---|---|
| Devnet | niedrig-mittel | erlaubt | optional je Feature |
| Testnet | mittel-hoch | nur mit Feature-Flag | GATE-012-Pflicht (SECURITY TEST) |
| Mainnet | maximal | VERBOTEN | GATE-013-Pflicht (SECURITY AUDIT PASS) |

## 2. Mainnet-Härtelinie (REQ-NET-062: MUST)

Keine Debug-Endpunkte · keine Test-Token · keine ungeprueften Smart
Contracts (ATVM-Bytecode-Verifier + License Gate als Trust Boundary,
AD-021/022) · Validator-Zugang protocol-defined · Dependencies gemaess
ATC-STD-203 Dependency Policy.

## 3. Testnet-Sicherheitspruefungen (REQ-NET-063)

Security Test, Chaos Test, Fault Injection und Penetration der
NET-002-Testpflicht sind auf Testnet zu absolvieren — NICHT auf Devnet,
NICHT erstmals auf Mainnet.


- **Network-Isolation & Port-Härtung:** Gefährliche RPC/Debug-Ports MUESSEN auf Mainnet extern isoliert und blockiert werden (REQ-NET-064).
- **Automated Threat Response:** Inkonsistente Block-Header oder Angriffsversuche MUESSEN ein automatisches Peer-Isolation-Event auslösen (REQ-NET-065).
- **Cryptographic Key Management:** Validator-Keys MUESSEN gemäß strengen Hardware-Sicherheits-Vorgaben verwahrt werden (REQ-NET-066).
- **Rate-Limiting & DDOS-Protection:** P2P- und RPC-Endpunkte MUESSEN wirksames Rate-Limiting erzwingen (REQ-NET-067).

## Requirements

- id: REQ-NET-061
  title: "Security-Matrix je Stufe verbindlich"
  severity: MANDATORY
- id: REQ-NET-062
  title: "Mainnet-Haertelinie (4 Verbote + Trust Boundary)"
  severity: MANDATORY
- id: REQ-NET-063
  title: "Security-/Chaos-Tests auf Testnet, nie erstmals auf Mainnet"
  severity: MANDATORY
- id: REQ-NET-064
  title: "Network-Isolation & Port-Härtung MUESSEN auf Mainnet durchgesetzt werden"
  severity: MANDATORY
- id: REQ-NET-065
  title: "Automated Threat Response MUSS Peer-Isolation bei Abweichungen auslösen"
  severity: MANDATORY
- id: REQ-NET-066
  title: "Cryptographic Key Management MUSS Hardware-Sicherheitsmodule nutzen"
  severity: MANDATORY
- id: REQ-NET-067
  title: "Rate-Limiting & DDOS-Protection MUESSEN an allen Endpunkten aktiv sein"
  severity: MANDATORY

## Metriken & Akzeptanzkriterien

1. **Audit-Abdeckungsgrad:** MUSS 100% aller netzwerkrelevanten Kernmodule vor Mainnet-Einsatz abdecken.
2. **Debug-Port-Exposition:** MUSS auf Mainnet exakt 0% betragen.
3. **Vulnerability-Behebungszeit:** S0/S1 Sicherheits-Findings MUESSEN innerhalb von 24h behoben sein.
4. **DDOS-Resilienz:** P2P-Netzwerk MUSS 100% der simulierten Angriffs-Vektoren schadlos verkraften.

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
