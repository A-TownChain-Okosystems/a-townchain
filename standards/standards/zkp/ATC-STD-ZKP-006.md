---
standard:
  id: ATC-STD-ZKP-006
  title: "ATC-STD-ZKP-006 — ZK Identity Standard"
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

# ATC-STD-ZKP-006 — ZK Identity Standard (v1.1.0, APPROVED)

> **Version:** 1.1.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

Privacy-preserving Identity (ATC-ZK-Identity Protocol): Credentials, die Aussagen beweisen (Alter ausreichend, Credential gueltig, nicht widerrufen), ohne die Identitaet offenzulegen.

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-601 | Identity-Beweise DUERFEN niemals die zugrunde liegende Identitaet offenlegen (Zero-Knowledge-Pflicht). | §S.2 |
| id: REQ-ZKP-602 | Credential-Widerrufe MUESSEN pruefbar sein (Revocation-Check im Proof). | §S.2 |
| id: REQ-ZKP-603 | Identity-Circuits MUESSEN der Circuit Registry (ATC-STD-ZKP-003) unterliegen. | §S.3 |

| id: REQ-ZKP-604 | Credential-Aussteller MÜSSEN gültige kryptografische Signaturen auf den Credentials anbringen. | §S.2 |
| id: REQ-ZKP-605 | Identity-Proofs MÜSSEN ein Frische-Zeitfenster (Nonce/Timestamp) enthalten, um Replay-Attacken zu verhindern. | §S.2 |
| id: REQ-ZKP-606 | Die Anzahl selektiv offengelegter Attribute MUSS minimiert werden (Data Minimization). | §S.3 |
| id: REQ-ZKP-607 | Widerrufene Credentials MÜSSEN unverzüglich im On-Chain-Revocation-Tree aktualisiert werden. | §S.3 |

## Metriken & Akzeptanzkriterien

- **Metrik 1:** Anonymitäts-Garantie bei Identity-Proofs MUSS 100% betragen (kein Linkage-Fehler).
- **Metrik 2:** Überprüfungsdauer eines ZK-Identity-Claims MUSS unter 100 ms liegen.
- **Metrik 3:** Aktualisierungszeit von Widerrufslisten MUSS unter 5 Sekunden betragen.
- **Metrik 4:** Fehlerquote bei der Attributs-Offenlegung MUSS 0% betragen.


## Metriken & Akzeptanzkriterien

- **Metrik 1:** Anonymitäts-Garantie bei Identity-Proofs MUSS 100% betragen (kein Linkage-Fehler).
- **Metrik 2:** Überprüfungsdauer eines ZK-Identity-Claims MUSS unter 100 ms liegen.
- **Metrik 3:** Aktualisierungszeit von Widerrufslisten MUSS unter 5 Sekunden betragen.
- **Metrik 4:** Fehlerquote bei der Attributs-Offenlegung MUSS 0% betragen.
- **Metrik 5:** Gültigkeitsprüfung von Aussteller-Signaturen MUSS zu 100% verifiziert werden.

## Compliance & Verifikation

Geprüft durch: (1) `atc-std-validator` (Registry-Sync S-14, Kopf-Sync S-19, Testläufe S-01 bis S-25), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen MÜSSEN als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 geführt und im Ticket-System protokolliert werden.

## Security Considerations

- Identitätsnachweise dürfen NIEMALS personenbeziehbare Daten (PII) im Plaintext übertragen.
- Signaturen der Credential-Issuer MÜSSEN gegen Quanten-Attacken und Fälschungen abgesichert werden.
- Replay-Schutz für Identity-Proofs MUSS auf Protokollebene erzwungen werden.
- Identitäts-Tokens MÜSSEN mit Verfallsdaten versehen sein.

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
- ATC-STD-ZKP-003
- ATC-STD-ZKP-006
- ATC-STD-ZKP-010

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie)
