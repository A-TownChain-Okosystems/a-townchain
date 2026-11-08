---
standard:
  id: ATC-STD-ZKP-005
  title: "ATC-STD-ZKP-005 — Commitment & Nullifier Standard"
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

# ATC-STD-ZKP-005 — Commitment & Nullifier Standard (v1.1.0, APPROVED)

> **Version:** 1.1.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

Kryptografische Commitments (private Balances: public Commitment C, private Balance/Owner-Secret) und Nullifier (Double-Spend-Prävention bei privaten Assets).

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-501 | Commitments MUESSEN mit den Primitiven des ATC Cryptographic Core (SHA-256, AD-001) gebildet oder explizit per AD freigegeben sein. | §S.2 |
| id: REQ-ZKP-502 | Jede private Ausgabe MUSS einen Nullifier konsumieren; Doppelte Nullifier MUESSEN abgelehnt werden. | §S.2 |
| id: REQ-ZKP-503 | Balance-Beweise (z.B. 'Besitzer darf mindestens X ausgeben') MUessen den Kontostand nicht offenlegen. | §S.2 |
| id: REQ-ZKP-504 | Nullifier- und Commitment-Manager MUESSEN unabhaengig testbar sein (Unit-Test-Pflicht). | §S.3 |

| id: REQ-ZKP-505 | Die Tiefe des Commitment-Merkle-Baums MUSS fest vorgegeben und gegen Re-Hashing-Attacken geschützt sein. | §S.2 |
| id: REQ-ZKP-506 | Nullifier-Sets MÜSSEN performant durchsuchbar sein, um Replay-Angriffe ohne Verzögerung abzuwehren. | §S.2 |
| id: REQ-ZKP-507 | Kollisionsfreie Hashing-Algorithmen MÜSSEN für alle Commitment-Berechnungen erzwungen werden. | §S.3 |
| id: REQ-ZKP-508 | Die Generierung von Zero-Value-Nullifiern MUSS vom System strikt unterbunden werden. | §S.3 |

## Metriken & Akzeptanzkriterien

- **Metrik 1:** Suchzeit im On-Chain Nullifier-Set MUSS O(1) oder O(log N) betragen.
- **Metrik 2:** Kollisionsrate der Commitment-Berechnungen MUSS theoretisch und praktisch 0% betragen.
- **Metrik 3:** Erfolgsquote der Double-Spend-Prävention MUSS exakt 100% sein.
- **Metrik 4:** Maximale Einfügezeit in den Commitment-Baum MUSS unter 50 ms liegen.


## Metriken & Akzeptanzkriterien

- **Metrik 1:** Suchzeit im On-Chain Nullifier-Set MUSS O(1) oder O(log N) betragen.
- **Metrik 2:** Kollisionsrate der Commitment-Berechnungen MUSS theoretisch und praktisch 0% betragen.
- **Metrik 3:** Erfolgsquote der Double-Spend-Prävention MUSS exakt 100% sein.
- **Metrik 4:** Maximale Einfügezeit in den Commitment-Baum MUSS unter 50 ms liegen.
- **Metrik 5:** Speichereffizienz der Nullifier-Set-Indexierung MUSS optimiert sein.

## Compliance & Verifikation

Geprüft durch: (1) `atc-std-validator` (Registry-Sync S-14, Kopf-Sync S-19, Testläufe S-01 bis S-25), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen MÜSSEN als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 geführt und im Ticket-System protokolliert werden.

## Security Considerations

- Commitments und Nullifier MÜSSEN kryptografisch voneinander unkorreliert sein.
- Das Nullifier-Set MUSS persistent auf der Chain gespeichert und gegen Löschung geschützt werden.
- Zugriffe auf den Commitment-Baum MÜSSEN nebenläufigkeitssicher synchronisiert werden.
- Verbrauchte Nullifier MÜSSEN dauerhaft gesperrt bleiben und dürfen niemals reaktiviert werden.

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
- ATC-STD-ZKP-005
- ATC-STD-ZKP-010

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie)
