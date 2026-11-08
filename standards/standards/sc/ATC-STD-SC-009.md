---
standard:
  id: ATC-STD-SC-009
  title: "ATC-STD-SC-009 — Smart Contract Access Control Standard"
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

# ATC-STD-SC-009 — Smart Contract Access Control Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Administrative Funktionen sind niemals unkontrolliert oeffentlich erreichbar: Permission → Role → Governance → Timelock → Execution; standardisierte Rollen (OWNER..AUDITOR).

## Scope

Gilt fuer alle Contracts mit administrativen Funktionen; verstaerkt ATC-STD-203 §Autorisierung.

## 1. Schutz-Pflicht (REQ-SC-026)

Administrative Funktionen MUessen durch Modifier/Checks geschuetzt sein, z. B. ```solidity
modifier onlyGovernance() { require(msg.sender == governance, "NOT_GOVERNANCE"); _; }
```


## 2. Rollen-Enum (REQ-SC-027)

Rollen SOLLEN aus dieser Menge stammen: OWNER, ADMIN, GOVERNANCE, OPERATOR, MINTER, PAUSER, UPGRADER, ORACLE, BRIDGE, TREASURY, AUDITOR.

## 3. Autorisierungskette (REQ-SC-028)

Kritische Funktionen MUssen der Kette folgen: User → Permission Check → Role Check → Governance → Timelock → Execution.




## 4. Explizite Rollen-Definition (REQ-SC-083)

Rollen MUESSEN im Contract als constant bytes32 Bezeichner deklariert werden (z. B. keccak256("MINTER_ROLE")).

## 5. Prinzip der minimalen Rechte (REQ-SC-084)

Administrative Berechtigungen MUESSEN in feingranulare Einzelrollen aufgeteilt werden; monolithische Admin-Rechte sind unzulässig.

## 6. Multi-Signature & Governance Bindung (REQ-SC-085)

Die höchstprivilegierte Rolle (DEFAULT_ADMIN_ROLE) MUSS an ein Multi-Signature-Wallet oder eine DAO-Governance gebunden sein.

## 7. Transparente Rollen-Events (REQ-SC-086)

Jede Vergabe oder Entziehung von Rollen MUSS unmittelbar die Events RoleGranted bzw. RoleRevoked emittieren.

## 8. Metriken & Akzeptanzkriterien

- Rechte-Granularität: MUSS mindestens 2 getrennte Funktionsrollen neben der Admin-Rolle umfassen.
- Multi-Sig Signer Threshold: MUSS mindestens 3-of-5 Schlüssel bei Produktionsrollen vorschreiben.
- Unautorisierte Aufrufe Test: MUSS 100% Revert bei fehlender Rolle in Integrationstests aufweisen.
- Rollen-Auditbarkeit: 100% aller Rollenänderungen MUESSEN in der On-Chain Event-Historie nachvollziehbar sein.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-026 | Schutz-Pflicht (§1) | MUSS |
| REQ-SC-027 | Rollen-Enum (§2) | SOLLTE |
| REQ-SC-028 | Autorisierungskette (§3) | MUSS |
| REQ-SC-083 | Explizite Rollen-Definition | MUSS |
| REQ-SC-084 | Prinzip der minimalen Rechte | MUSS |
| REQ-SC-085 | Multi-Signature & Governance Bindung | MUSS |
| REQ-SC-086 | Transparente Rollen-Events | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Unzureichende Zugriffskontrollen gehören zu den häufigsten Schwachstellen. Strikte Rollentrennung schützt das Protokoll vor Privilegieneskalation und kompromittierten Einzelschlüsseln.

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

- **NORMATIVE:** ATC-STD-000 (Verfassung), ATC-STD-201/202, ATC-STD-203
  (Security), ATC-STD-204 (Dependency & Interface), ATC-STD-SC-001
  (Familien-Hauptstandard)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025 (Agenten-Standards), AI-DEV-001..012,
  contracts/registry/ (Contract Registry), ATC-STD-BUG-001..005 (Bug Standards)
