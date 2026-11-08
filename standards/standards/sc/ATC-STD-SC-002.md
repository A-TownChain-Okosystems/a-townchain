---
standard:
  id: ATC-STD-SC-002
  title: "ATC-STD-SC-002 — Smart Contract Identity Standard"
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

# ATC-STD-SC-002 — Smart Contract Identity Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Jeder Contract ist eindeutig identifizierbar und rueckverfolgbar: contract_id, Version, Netzwerk, Sprache, Runtime, Upgradeability sowie vollstaendige Quell- und Deployment-Verknuepfung (Repository → Commit → Deployment-Adresse).

## Scope

Gilt fuer alle Contracts; maschinenlesbare Identitaet in Contract-Metadaten und Contract Registry (SC-019).

## 1. Pflichtidentitaet (REQ-SC-006)

Jeder Contract MUSS definieren: contract_id (ATC-SC-<KATEGORIE>-NNN), name, version, network, chain_id, contract_type, status, language, runtime, owner, upgradeable. Beispiel: `contract_id: ATC-SC-TOKEN-001, version: 1.0.0, network: mainnet, chain_id: ATC-1, language: Solidity, runtime: EVM, upgradeable: false`.

## 2. Quell-Verknuepfung (REQ-SC-007)

Zusaetzlich MUSS jeder Contract referenzieren: source_repository, contract_path, commit, compiler, compiler_version, optimization, audit_status, deployment_date, deployment_address — damit jeder Contract eindeutig mit Code → Repository → Commit → Deployment verbunden ist.




## 3. Deterministische Identifikator-Generierung (REQ-SC-059)

Die contract_id MUSS nach dem Schema ATC-SC-<KATEGORIE>-NNN gebildet werden und darf sich während des gesamten Lebenszyklus des Contracts nicht ändern. Bei Vererbung oder Modulstruktur MUSS jedes separat deployte Bytecode-Artefakt eine eigene eindeutige Identität besitzen.

## 4. Unveränderbarkeit von Identitäts-Metadaten (REQ-SC-060)

Identitätsrelevante Zustandsvariablen (z. B. contract_id, created_at_block, initial_owner) MUESSEN als immutable oder constant im Contract-Code deklariert werden, um nachträgliche Manipulationen auszuschließen.

## 5. Laufzeit-Metadaten-Schnittstelle (REQ-SC-061)

Jeder Contract MUSS eine öffentliche Schnittstelle (z. B. getContractIdentity()) bereitstellen, welche die Metadaten strukturierbar zur On-Chain- und Off-Chain-Verifikation zurückgibt.

## 6. Identitäts-Event-Protokollierung (REQ-SC-062)

Änderungen von Administrations- oder Eigentümerrechten MUESSEN ein standardisiertes Event auslösen, welches die ursprüngliche und neue Identität des Verwalters lückenlos protokolliert.

## 7. Metriken & Akzeptanzkriterien

- Metadaten-Vollständigkeit: 100% aller geforderten Felder nach REQ-SC-006 MUSS im Contract oder On-Chain-Record vorhanden sein.
- Quellcode-Verifizierbarkeit: Der Quellcode-Hash MUSS zu 100% mit dem im Deployment Record hinterlegten Git-Commit übereinstimmen.
- Abfrage-Effizienz: Die Laufzeit-Abfrage der Identitätsfunktionen MUSS in unter 50.000 Gas ausführbar sein.
- Validator-Konformität: Validator-Testläufe S-01 bis S-25 MUESSEN ohne identitätsbezogene Findings bestanden werden.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-006 | Identitaetsfelder (§1) | MUSS |
| REQ-SC-007 | Code-Repo-Commit-Deployment-Link (§2) | MUSS |
| REQ-SC-059 | Deterministische Identifikator-Generierung | MUSS |
| REQ-SC-060 | Unveränderbarkeit von Identitäts-Metadaten | MUSS |
| REQ-SC-061 | Laufzeit-Metadaten-Schnittstelle | MUSS |
| REQ-SC-062 | Identitäts-Event-Protokollierung | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

Identitätsdaten enthalten keine sensitiven Secrets; deployment_address wird erst nach bestandenem Gate SC-G10 freigegeben. Unveränderliche Identitätsmetadaten verhindern Spoofing- und Sybil-Angriffe auf Systemebene und gewährleisten die Integrität im gesamten A-TownChain-Ökosystem.

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
