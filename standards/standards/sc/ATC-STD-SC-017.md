---
standard:
  id: ATC-STD-SC-017
  title: "ATC-STD-SC-017 — GameFi Contract Standard"
  version: "1.1.0"
  status: approved
  category: sc
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle Smart Contracts des A-TownChain-Oekosystems"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-SC-017 — GameFi Contract Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Anforderungen fuer Game-Contracts (Shivamon, Items, Rewards): deterministisches Gameplay-Asset-Verhalten, Item-Erstellung/Rarity ueber rollenkontrollierte Minting-Regeln, Reward-Emissionen mit Obergrenzen und Anti-Cheat-Events.

## Scope

Gilt fuer alle SC-GAME-Contracts (Genesis Chronicles / Shivamon).

## 1. Asset-Verhalten (REQ-SC-047)

GameFi-Assets (Shivamon, Items) MUESSEN deterministische Erstellungs- und Transfer-Regeln haben; Minting ueber rollenkontrollierte Game-Logik (SC-009).

## 2. Reward-Emissionen (REQ-SC-048)

Reward-Emissionen MUESSEN Obergrenzen (Invariant: rewardsIssued <= rewardPool) und dokumentierte Verteilungsregeln haben.

## 3. Anti-Cheat-Nachvollziehbarkeit (REQ-SC-049)

Wichtige Game-Ereignisse sind Event-pflichtig (SC-008) fuer Anti-Cheat-Monitoring (DefenderGPT).




## 4. Verifizierbare Zufallsgenerierung (VRF) (REQ-SC-115)

Zufallsbasierte Spiel- und Drop-Mechanismen MUESSEN verifizierbaren Zufall (VRF) oder Commit-Reveal-Schemas nutzen; Blockhash-Zufall ist unzulässig.

## 5. Emissions-Caps für Game-Rewards (REQ-SC-116)

In-Game Reward-Ausschüttungen MUESSEN durch tägliche und globale Obergrenzen im Contract fest verankert sein.

## 6. Anti-Bot & Rate-Limiting Modifikatoren (REQ-SC-117)

Interaktive Spielaktionen MUESSEN Cooldown-Zeiten per Block oder Zeitstempel erzwingen, um Bot-Spam einzudämmen.

## 7. Verbindliche Item-Integrität (REQ-SC-118)

Eigenschaften und Seltenheitsgrade von In-Game-Assets MUESSEN nach dem Minting unveränderlich On-Chain gespeichert werden.

## 8. Metriken & Akzeptanzkriterien

- VRF Entropie-Prüfung: MUSS 100% fälschungssichere Zufallswerte nachweisen.
- Reward-Emissions-Cap: MUSS 0% Überschreitung der Tagesbudgets garantieren.
- Rate-Limiting Effektivität: MUSS Transaktionen innerhalb der Cooldown-Phase zu 100% blockieren.
- Asset-Integritäts-Rate: MUSS 0 unbefugte Parameteränderungen bei Spielgegenständen aufweisen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-047 | Asset-Verhalten (§1) | MUSS |
| REQ-SC-048 | Reward-Grenzen (§2) | MUSS |
| REQ-SC-049 | Anti-Cheat-Events (§3) | MUSS |
| REQ-SC-115 | Verifizierbare Zufallsgenerierung (VRF) | MUSS |
| REQ-SC-116 | Emissions-Caps für Game-Rewards | MUSS |
| REQ-SC-117 | Anti-Bot & Rate-Limiting Modifikatoren | MUSS |
| REQ-SC-118 | Verbindliche Item-Integrität | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

GameFi-Verträge kombinieren Gaming-Logik mit echten Vermögenswerten. Schutz vor Zufallsmanipulation (MEV/Reentrancy) und Bot-Ausbeutung MUSS durch strikte Modifikatoren gesichert werden.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## References

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-011 (Token)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/, ATC-STD-BUG-001..005 (Bug Standards)
