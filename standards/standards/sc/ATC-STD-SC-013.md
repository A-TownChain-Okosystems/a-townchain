---
standard:
  id: ATC-STD-SC-013
  title: "ATC-STD-SC-013 — DeFi Contract Standard"
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

# ATC-STD-SC-013 — DeFi Contract Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Anforderungen fuer Staking-, Liquidity- und Swap-Contracts: Invarianten fuer Wertfluesse, Manipulationsresistenz (Flash-Loan, Oracle, Price), klare Withdrawal-Regeln und Emergency-Mechanismen.

## Scope

Gilt fuer alle SC-DEFI-Contracts.

## 1. Wertfluss-Invarianten (REQ-SC-036)

DeFi-Contracts MUESSEN Wertfluss-Invarianten als Invariant-Tests codieren (z. B. stakedBalance <= totalDeposited; liquidityConstantin in beiden Assets).

## 2. Manipulationsresistenz (REQ-SC-037)

Staking/Liquidity/Swaps MUESSEN gegen Flash-Loan-, Oracle- und Price-Manipulation gehaertet sein (SC-003 §1).

## 3. Withdrawal & Emergency (REQ-SC-038)

Withdrawal-Regeln MUESSEN dokumentiert sein; Emergency-Verhalten nach dem Emergency-Zustaende-Modell (SC-001/Serie: NORMAL→SUSPICIOUS→PAUSED→INVESTIGATION→RECOVERY→RESUME).




## 4. Pool-Invarianten-Garantie (REQ-SC-099)

DeFi-Liquidity-Pools MUESSEN die mathematischen Invarianten nach jeder Interaktion strikt validieren.

## 5. Verpflichtender Slippage-Schutz (REQ-SC-100)

Swap- und Liquidity-Funktionen MUESSEN benutzerdefinierte Min-Output- bzw. Max-Input-Parameter auswerten und bei Abweichung revertieren.

## 6. Anti-Flash-Loan-Schutz (REQ-SC-101)

Preissensitive Operationen MUESSEN vor Manipulationen durch Same-Block Flash Loans geschützt sein (z. B. via TWAP Oracles oder Block-Locks).

## 7. Notfall-Liquiditäts-Abhebung (REQ-SC-102)

DeFi-Contracts MUESSEN eine emergencyWithdraw-Funktion bieten, die Kapitalbergung ohne Belohnungsberechnung erlaubt.

## 8. Metriken & Akzeptanzkriterien

- Invarianten-Härte: MUSS 100% der Invarianten-Prüfungen in Invarianten-Tests bestehen.
- Slippage-Toleranz Durchsetzung: 100% aller Swaps mit unzureichendem Output MUESSEN revertieren.
- Flash-Loan Resistenz: MUSS 0 manipulative Preisänderungen im selben Block erlauben.
- Notfall-Auszahlungs-Latenz: Emergency Withdraw MUSS auch bei gepausiertem Orakel funktionieren.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-036 | Wertfluss-Invarianten (§1) | MUSS |
| REQ-SC-037 | Manipulationsresistenz (§2) | MUSS |
| REQ-SC-038 | Withdrawal/Emergency (§3) | MUSS |
| REQ-SC-099 | Pool-Invarianten-Garantie | MUSS |
| REQ-SC-100 | Verpflichtender Slippage-Schutz | MUSS |
| REQ-SC-101 | Anti-Flash-Loan-Schutz | MUSS |
| REQ-SC-102 | Notfall-Liquiditäts-Abhebung | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

DeFi-Protokolle tragen hohe finanzielle Risiken. Preismanipulationen und Flash-Loan-Angriffe gehören zu den Hauptbedrohungen und MUESSEN durch Mehrfach-Sicherungen isoliert werden.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-003 (Security), ATC-STD-SC-004 (Testing)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/, ATC-STD-BUG-001..005 (Bug Standards)
