---
standard:
  id: ATC-STD-SC-010
  title: "ATC-STD-SC-010 — Smart Contract Treasury Standard"
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

# ATC-STD-SC-010 — Smart Contract Treasury Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Alle Finanzbewegungen nachvollziehbar: Events je Transaktion, Multi-Signature, Spending Limits, Timelocks, Emergency Stop, Governance Approval, vollstaendige Audit Logs.

## Scope

Gilt fuer Treasury- und werttransferierende Contracts (SC-TOKEN, SC-DEFI, SC-MINING, SC-MARKET).

## 1. Event-Pflicht (REQ-SC-029)

Jede relevante Finanztransaktion MUSS ein Event erzeugen, z. B. `FundsTransferred(address indexed from, address indexed to, uint256 amount, bytes32 indexed reason)`.

## 2. Kontrollen (REQ-SC-030)

Treasury-Contracts SOLLEN unterstuetzen: Multi-Signature, Spending Limits, Timelocks, Emergency Stop, Governance Approval, vollstaendige Audit Logs.




## 3. Multi-Signature Auszahlungs-Pflicht (REQ-SC-087)

Jede Auszahlung aus einer Treasury MUSS durch ein Multi-Signature-Schema mit mindestens 3 von 5 Bestätigungen freigegeben werden.

## 4. On-Chain Spending Limits (REQ-SC-088)

Treasury-Contracts MUESSEN tägliche und transaktionsbezogene Auszahlungsobergrenzen (Spending Limits) programmatisch durchsetzen.

## 5. Notfall-Vault-Routing (REQ-SC-089)

Ein Notfall-Sweep von Treasury-Guthaben MUSS ausschließlich an vorab immutable registrierte Notfall-Vault-Adressen zulässig sein.

## 6. Bilanzielle Invarianz (REQ-SC-090)

Treasury-Verträge MUESSEN nach jeder Transaktion die Invariante TotalAssets >= CommittedLiabilities verifizieren.

## 7. Metriken & Akzeptanzkriterien

- Multi-Sig Genehmigungsquote: MUSS >= 60% der Schlüssel-Inhaber für Auszahlungen verlangen.
- Spending Limit Abweichung: MUSS 0% Überschreitung zugelassener Limits garantieren.
- Audit-Log Abdeckung: 100% aller Geldbewegungen MUESSEN lückenlos geloggt sein.
- Notfall-Routing-Prüfung: MUSS 100% Fehlversuche bei unberechtigten Zieladressen erzeugen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-029 | Event-Pflicht (§1) | MUSS |
| REQ-SC-030 | Kontrollmechanismen (§2) | SOLLTE |
| REQ-SC-087 | Multi-Signature Auszahlungs-Pflicht | MUSS |
| REQ-SC-088 | On-Chain Spending Limits | MUSS |
| REQ-SC-089 | Notfall-Vault-Routing | MUSS |
| REQ-SC-090 | Bilanzielle Invarianz | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Treasury-Contracts verwalten direkte Vermögenswerte des Ökosystems und sind primäre Angriffsziele. Schutz gegen Reentrancy, Signature Manipulation und Flash-Loan-Insolvenz MUSS lückenlos gewährleistet sein.

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
