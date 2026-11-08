---
standard:
  id: ATC-STD-SC-012
  title: "ATC-STD-SC-012 — NFT Contract Standard"
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

# ATC-STD-SC-012 — NFT Contract Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026, 21:00 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-SC-FRAMEWORK.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** ATC Smart Contract Standards Framework (ATC-STD-SC-001..020).

## Abstract

Anforderungen fuer NFT-, Asset- und Collection-Contracts: eindeutige Token-IDs, Ownership-Transfer, Approvals, Metadata-Referenzen (off-chain), Royalty-Unterstützung und Event-Pflicht.

## Scope

Gilt fuer alle SC-NFT- und SC-MARKET-Contracts (Marketplace siehe SC-013-Verzahnung).

## 1. Kernpflichten (REQ-SC-034)

NFT-Contracts MUESSEN eindeutige Token-IDs, Ownership-Transfer, Approvals und Metadata-Referenzen implementieren; Transfers und Approvals sind Event-pflichtig (SC-008).

## 2. Collections & Royalties (REQ-SC-035)

Collections SOLLEN Royalty-Mechanismen und Creator-Attribute unterstuetzen; Marketplace-Integration (SC-MARKET) MUSS ueber definierte Interfaces laufen (ATC-STD-204).




## 3. EIP-2981 Royalty Schnittstelle (REQ-SC-095)

NFT-Contracts MUESSEN den EIP-2981 Standard zur On-Chain-Abfrage von Lizenzgebühren (Royalties) unterstützen.

## 4. Dezentrale Metadaten-Unveränderlichkeit (REQ-SC-096)

Token-URIs MUESSEN auf dezentralen, korrumpierungssicheren Speicherstrukturen (IPFS, Arweave) verankert sein.

## 5. Deterministische Token-ID Generierung (REQ-SC-097)

Token-IDs MUESSEN lückenlos aufsteigend oder über kryptografisch eindeutige Hashes generiert werden.

## 6. Batch-Operationen Schutz (REQ-SC-098)

Batch-Minting und Batch-Transfers MUESSEN gegen Array-Overflows und Unchecked Gas Exhaustion geschützt sein.

## 7. Metriken & Akzeptanzkriterien

- Royalty Standard Konformität: MUSS EIP-2981 Schnittstellentests zu 100% bestehen.
- Metadaten-Verfügbarkeit: MUSS 100% gültige IPFS/Arweave URIs aufweisen.
- Token-ID Eindeutigkeit: MUSS 0 Kollisionen in Fuzzing-Tests erzeugen.
- Batch Gas Limit: MUSS das Ausführen von Batch-Aktionen innerhalb eines max. Gas-Limits garantieren.

## Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-SC-034 | Kernpflichten (§1) | MUSS |
| REQ-SC-035 | Collections/Royalties (§2) | SOLLTE |
| REQ-SC-095 | EIP-2981 Royalty Schnittstelle | MUSS |
| REQ-SC-096 | Dezentrale Metadaten-Unveränderlichkeit | MUSS |
| REQ-SC-097 | Deterministische Token-ID Generierung | MUSS |
| REQ-SC-098 | Batch-Operationen Schutz | MUSS |

## Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben wird automatisiert über die SC-Compliance-Gates (ATC-STD-SC-001 §Gates, SC-G0..G13) sowie regelmäßige Validator-Läufe (S-01 bis S-25) des atc-validator geprüft. Etwaige Verstöße oder Abweichungen MUESSEN als Findings F-001..F-999 gemäß ATC-STD-BUG-001..005 dokumentiert werden. Die Registrierungs- und Validierungspflicht richtet sich nach ATC-STD-SC-019.

## Security Considerations

NFT-Contracts schützen digitale Assets und Sammlerstücke. Sichere Metadaten und reentrancy-freie Transfer-Hooks MUESSEN zwingend umgesetzt werden.

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

- **NORMATIVE:** ATC-STD-000, ATC-STD-203, ATC-STD-204, ATC-STD-SC-001 (Familien-Hauptstandard), ATC-STD-SC-008 (Events)
- **INFORMATIVE:** SCR-0041 (Change Request), ATC-AAS-001..025, AI-DEV-001..012, contracts/registry/, ATC-STD-BUG-001..005 (Bug Standards)
