---
standard:
  id: ATC-STD-ZKP-007
  title: "ATC-STD-ZKP-007 — Private Transaction Standard"
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

# ATC-STD-ZKP-007 — Private Transaction Standard (v1.1.0, APPROVED)

> **Version:** 1.1.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

Private Transaktionen (Alice → ZKP Prover → Proof → Chain) und GameFi-Beweise (Item-Besitz, Level, Quest-Abschluss, Score, PvP-Voraussetzungen) ohne On-Chain-Publikation der Spieldaten.

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-701 | Private Transaktionen MUESSEN vollstaendig als Proof verifizierbar sein; die privaten Eingabedaten DUERFEN nicht offengelegt werden. | §S.2 |
| id: REQ-ZKP-702 | GameFi-Beweise MUESSEN die Spielregeln im Circuit abbilden (Regel-Einhaltung ist Teil des Proofs). | §S.2 |
| id: REQ-ZKP-703 | Private Transaktionen MUESSEN den Commitment-/Nullifier-Standard (ATC-STD-ZKP-005) einhalten. | §S.3 |

| id: REQ-ZKP-704 | Wert-Erhaltungssummen MÜSSEN mittels homomorpher Additivität im Circuit ohne Werteoffenlegung verifiziert werden. | §S.2 |
| id: REQ-ZKP-705 | Range-Proofs MÜSSEN sicherstellen, dass keine negativen Beträge oder Überläufe erzeugt werden. | §S.2 |
| id: REQ-ZKP-706 | Transaktionsgebühren MÜSSEN getrennt und ohne Verletzung der Privatsphäre abgeführt werden. | §S.3 |
| id: REQ-ZKP-707 | Spend-Proofs MÜSSEN eine gültige Verknüpfung zu bestehenden Commitments nachweisen. | §S.3 |

## Metriken & Akzeptanzkriterien

- **Metrik 1:** Erfolgsquote bei der Verifikation privater Wert-Transfers MUSS 100% betragen.
- **Metrik 2:** Fehlerrate bei Range-Proof-Validierungen MUSS 0% sein.
- **Metrik 3:** Latenz der privaten Transaktionsverarbeitung MUSS unter 250 ms liegen.
- **Metrik 4:** Präzision der Gebührenberechnung im Circuit MUSS exakt 100% erreichen.


## Metriken & Akzeptanzkriterien

- **Metrik 1:** Erfolgsquote bei der Verifikation privater Wert-Transfers MUSS 100% betragen.
- **Metrik 2:** Fehlerrate bei Range-Proof-Validierungen MUSS 0% sein.
- **Metrik 3:** Latenz der privaten Transaktionsverarbeitung MUSS unter 250 ms liegen.
- **Metrik 4:** Präzision der Gebührenberechnung im Circuit MUSS exakt 100% erreichen.
- **Metrik 5:** Durchsatz von privaten Wertübertragungen MUSS mindestens 100 TPS betragen.

## Compliance & Verifikation

Geprüft durch: (1) `atc-std-validator` (Registry-Sync S-14, Kopf-Sync S-19, Testläufe S-01 bis S-25), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen MÜSSEN als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 geführt und im Ticket-System protokolliert werden.

## Security Considerations

- Transaktionsbeträge und Absender-/Empfängeradressen MÜSSEN vollständig verborgen bleiben.
- Überlauf-Schutz in Range-Proofs MUSS mathematisch bewiesen und auditierter Bestandteil sein.
- Double-Spend-Versuche MÜSSEN zum dauerhaften Ausschluss der Transaktion führen.
- Confidential-Asset-Typen MÜSSEN durch kryptografische Blinding-Faktoren isoliert werden.

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
- ATC-STD-ZKP-007
- ATC-STD-ZKP-010

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie)
