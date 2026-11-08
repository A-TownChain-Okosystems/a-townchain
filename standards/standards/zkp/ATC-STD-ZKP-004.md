---
standard:
  id: ATC-STD-ZKP-004
  title: "ATC-STD-ZKP-004 — On-Chain Verification Standard"
  version: "1.1.0"
  status: approved
  category: zkp
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

# ATC-STD-ZKP-004 — On-Chain Verification Standard (v1.1.0, APPROVED)

> **Version:** 1.1.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

On-Chain-Verifikation: Der ZKP Verifier verifiziert Proofs in der Chain (Verify Proof, Check Nullifier, Update State) und die Proof Registry speichert/referenziert verifizierte Proofs.

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-401 | Verifizierte Proofs MUESSEN in der Proof Registry referenziert werden (Hash, Circuit-Version, Beweissystem). | §S.2 |
| id: REQ-ZKP-402 | Die On-Chain-Verifikation MUSS der Reihenfolge Verify Proof → Check Nullifier → Update State folgen. | §S.2 |
| id: REQ-ZKP-403 | Die Verifikations-Gas-/Kosten MUESSEN deterministisch und begrenzt sein (DoS-Schutz). | §S.2 |
| id: REQ-ZKP-404 | Verifikationsfehler MUessen als strukturierte Findings (ATC-STD-BUG-001) geloggt werden. | §S.3 |

| id: REQ-ZKP-405 | On-Chain-Verifier-Verträge MÜSSEN den Gasverbrauch pro Verifikation unter dem L1-Block-Gas-Limit halten. | §S.2 |
| id: REQ-ZKP-406 | Öffentliche Eingaben (Public Inputs) MÜSSEN vor der Verifikation einer Wertebereichsprüfung unterzogen werden. | §S.2 |
| id: REQ-ZKP-407 | Vertrags-Upgrades von Verifiern MÜSSEN eine 48-Stunden-Sperrfrist (Time-Lock) einhalten. | §S.3 |
| id: REQ-ZKP-408 | Batch-Verifikationen MÜSSEN atomar fehlschlagen, wenn auch nur ein einzelner Beweis ungültig ist. | §S.3 |

## Metriken & Akzeptanzkriterien

- **Metrik 1:** Maximaler Gasverbrauch pro On-Chain-Verifikation MUSS unter 300.000 Gas liegen.
- **Metrik 2:** Erfolgsquote bei gültigen On-Chain-Beweisen MUSS exakt 100% betragen.
- **Metrik 3:** Abweisungsrate ungültiger Public Inputs MUSS 100% erreichen.
- **Metrik 4:** Sicherheitsabdeckung im Verifier-Smart-Contract MUSS 100% betragen.


## Metriken & Akzeptanzkriterien

- **Metrik 1:** Maximaler Gasverbrauch pro On-Chain-Verifikation MUSS unter 300.000 Gas liegen.
- **Metrik 2:** Erfolgsquote bei gültigen On-Chain-Beweisen MUSS exakt 100% betragen.
- **Metrik 3:** Abweisungsrate ungültiger Public Inputs MUSS 100% erreichen.
- **Metrik 4:** Sicherheitsabdeckung im Verifier-Smart-Contract MUSS 100% betragen.
- **Metrik 5:** Latenz der On-Chain-Beweisregistrierung MUSS innerhalb eines Block-Intervalls liegen.

## Compliance & Verifikation

Geprüft durch: (1) `atc-std-validator` (Registry-Sync S-14, Kopf-Sync S-19, Testläufe S-01 bis S-25), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen MÜSSEN als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 geführt und im Ticket-System protokolliert werden.

## Security Considerations

- Verifier-Smart-Contracts MÜSSEN gegen Reentrancy-Angriffe geschützt sein.
- Verifikationsschlüssel MÜSSEN in unveränderlichem Speicher (Immutable Storage) abgelegt werden.
- Notfall-Pause-Funktionen MÜSSEN durch Multi-Sig-Governance abgesichert sein.
- Front-Running-Angriffe auf On-Chain-Verifikationen MÜSSEN durch Commit-Reveal-Schemata verhindert werden.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung aus dem Owner-Entwurf ZKP-Layer (AD-045). Status: APPROVED |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung, v1.1.0 APPROVED)
- ATC-STD-100
- ATC-STD-201
- ATC-STD-BUG-001
- ATC-STD-ZKP-001
- ATC-STD-ZKP-004
- ATC-STD-ZKP-010

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie)
