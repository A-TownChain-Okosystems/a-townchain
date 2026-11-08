---
standard:
  id: ATC-STD-SC-011
  title: "ATC-STD-SC-011 — Token Contract Standard"
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

# ATC-STD-SC-011 — Token Contract Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Mindestanforderungen fuer ATC-Token: Total/Max Supply, Minting, Burning, Decimals, Transfer, Allowance, Approval, Emission, Distribution, Treasury, Staking, Governance — Minting/Burning ueber definierte Rollen/Protokollregeln.

## Scope

Gilt fuer alle SC-TOKEN-Contracts; ATC-Token-Standards ATC-001 (Genesis), ATC-8300 (ERC-20-artig), ATC-9900 (Governance/DAO) muessen diese Anforderungen erfuellen.

## 1. Pflichtfunktionen (REQ-SC-031)

Token-Contracts MUESSEN mindestens definieren: Total Supply, Max Supply, Minting, Burning, Decimals, Transfer, Allowance, Approval, Emission, Distribution, Treasury, Staking, Governance.

## 2. Mint/Burn-Kontrolle (REQ-SC-032)

Minting und Burning MUESSEN ueber klar definierte Rollen (SC-009: MINTER, GOVERNANCE) bzw. Protokollregeln kontrolliert werden; Emissionsgrenzen als Invariant (SC-004: totalSupply <= maxSupply).

## 3. ATC-Integration (REQ-SC-033)

ATC-Tokens (ATC-001, ATC-8300, ATC-9900) MUESSEN in der Contract Registry (SC-019) mit Kategorie SC-TOKEN registriert sein.




## 4. Obergrenzen-Garantie (Max Supply) (REQ-SC-091)

Der maxSupply Wert MUSS als immutable Variable fixiert sein; Minting-Operationen über maxSupply MUESSEN zwingend fehlschlagen.

## 5. Sichere Allowance-Verwaltung (REQ-SC-092)

Token-Contracts MUESSEN Mechanismen gegen Front-Running bei Approvals bieten (z. B. increaseAllowance/decreaseAllowance oder Permit).

## 6. Transfer-Validierung & Zero-Address Protection (REQ-SC-093)

Transfers an die Nulladresse (0x0) oder den Token-Contract selbst MUESSEN mit explizitem Revert unterbunden werden.

## 7. Deterministische Emissionsberechnung (REQ-SC-094)

Staking- und Emissionsberechnungen MUESSEN deterministisch und frei von gerundeten Präzisionsverlusten durchgeführt werden.

## 8. Metriken & Akzeptanzkriterien

- Max Supply Invariante: totalSupply <= maxSupply MUSS zu 100% aller Blöcke gelten.
- Nulladressen-Schutz: 100% der Transfers an 0x0 MUESSEN fehlschlagen.
- Permit-Signatur-Validierung: MUSS EIP-712 Konformität erfüllen.
- Emissions-Präzision: MUSS Rundungsfehler auf < 1 wei begrenzen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-031 | Pflichtfunktionen (§1) | MUSS |
| REQ-SC-032 | Mint/Burn-Rollenkontrolle (§2) | MUSS |
| REQ-SC-033 | Registry-Registrierung (§3) | MUSS |
| REQ-SC-091 | Obergrenzen-Garantie (Max Supply) | MUSS |
| REQ-SC-092 | Sichere Allowance-Verwaltung | MUSS |
| REQ-SC-093 | Transfer-Validierung & Zero-Address Protection | MUSS |
| REQ-SC-094 | Deterministische Emissionsberechnung | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Tokens bilden die ökonomische Basis des Ökosystems. Fehlende Limits oder manipulierbare Minting-Funktionen führen zum Totalverlust der Token-Integrität und MUESSEN durch Gate SC-G3 rigoros geprüft werden.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-009 (Access Control), ATC-STD-SC-019 (Registry)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/, ATC-STD-BUG-001..005 (Bug Standards)
