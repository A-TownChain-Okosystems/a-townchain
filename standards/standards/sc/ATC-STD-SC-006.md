---
standard:
  id: ATC-STD-SC-006
  title: "ATC-STD-SC-006 — Smart Contract Deployment Standard"
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

# ATC-STD-SC-006 — Smart Contract Deployment Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Jedes Deployment erzeugt einen maschinenlesbaren Deployment Record (deployment_id, contract_id, network, address, block, transaction, commit, compiler, deployer, timestamp, verification, audit) — jederzeit feststellbar: welcher Code laeuft auf welcher Chain unter welcher Adresse.

## Scope

Gilt fuer alle Deployments (Testnet + Mainnet); Records in contracts/registry/deployments.yaml (SC-019).

## 1. Deployment Record (REQ-SC-017)

Jeder Deployment MUSS einen Deployment Record erzeugen: deployment_id (ATC-DEP-NNNN), contract_id, network, chain_id, address, block, transaction, commit, compiler, compiler_version, deployer, timestamp, verification, audit.

## 2. Verification (REQ-SC-018)

Nach Deployment MUSS die Contract-Verification (SC-G11) erfolgen und im Deployment Record vermerkt sein.

## 3. Registry-Update (REQ-SC-019)

Jedes Deployment MUSS zeitnah in die Contract Registry eingetragen werden (SC-G12, SC-019).




## 4. Deterministische Adressgenerierung (REQ-SC-071)

Deployments MUESSEN präzise und deterministisch über CREATE2 mit standardisiertem Salt durchgeführt werden, um identische Adressen über Netzwerke hinweg zu gewährleisten.

## 5. Automatische Explorer-Verifikation (REQ-SC-072)

Der Quellcode aller deployten Contracts MUSS innerhalb von 15 Minuten nach dem Deployment vollständig auf den primären Block-Explorern verifiziert werden.

## 6. Multi-Chain-Synchronisation (REQ-SC-073)

Bei Multi-Chain-Deployments MUSS der Bytecode-Hash auf allen Zielnetzwerken identisch sein und im Registry-Record dokumentiert werden.

## 7. Initialisierungs-Sanity-Check (REQ-SC-074)

Unmittelbar nach dem Deployment MUSS eine automatisierte Sanity-Check-Transaktion die korrekte Initialisierung aller State-Variablen verifizieren.

## 8. Metriken & Akzeptanzkriterien

- Verifikations-Latenz: MUSS unter 15 Minuten nach Block-Inklusion liegen.
- CREATE2 Adress-Gleichheit: MUSS 100% Übereinstimmung über alle konfigurierten Chains aufweisen.
- Deployment-Record Vollständigkeit: MUSS alle Pflichtfelder nach REQ-SC-017 enthalten.
- Sanity-Check Pass-Rate: MUSS 100% vor der ersten interaktiven Nutzung betragen.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-017 | Deployment Record (§1) | MUSS |
| REQ-SC-018 | Verification (§2) | MUSS |
| REQ-SC-019 | Registry-Update (§3) | MUSS |
| REQ-SC-071 | Deterministische Adressgenerierung | MUSS |
| REQ-SC-072 | Automatische Explorer-Verifikation | MUSS |
| REQ-SC-073 | Multi-Chain-Synchronisation | MUSS |
| REQ-SC-074 | Initialisierungs-Sanity-Check | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Sichere Deployments verhindern Front-Running bei der Initialisierung und gewährleisten, dass ausschließlich geprüfter Quellcode in Produktion gelangt. Deployment-Keys MUESSEN nach SC-G10 sicher aufbewahrt werden.

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
