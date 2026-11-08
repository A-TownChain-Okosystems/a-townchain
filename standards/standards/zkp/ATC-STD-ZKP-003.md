---
standard:
  id: ATC-STD-ZKP-003
  title: "ATC-STD-ZKP-003 — Circuit Standard"
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

# ATC-STD-ZKP-003 — Circuit Standard (v1.1.0, APPROVED)

> **Version:** 1.1.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

Registrierung und Versionierung zugelassener Circuits: Circuit Registry, Circuit Compiler (uebersetzt definierte Berechnungen in ZKP-Circuits) und ZKP-Governance (Freigabe, Deprecation, Upgrade von Circuits).

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-301 | Circuits MUESSEN in der Circuit Registry registriert und versioniert sein; unregistrierte Circuits DUERFEN nicht verifiziert werden. | §S.2 |
| id: REQ-ZKP-302 | Circuit-Aenderungen MUESSEN ueber ZKP-Governance (Freigabe/Deprecation/Upgrade) laufen. | §S.2 |
| id: REQ-ZKP-303 | Der Circuit Compiler MUSS deterministisch uebersetzen (gleicher Input, gleicher Circuit). | §S.2 |
| id: REQ-ZKP-304 | Depreacierte Circuits MUESSEN gesperrt und dokumentiert werden (kein stillschweigendes Entfernen). | §S.2 |

| id: REQ-ZKP-305 | Sämtliche Circuits MÜSSEN automatisierte Tests auf Under-Constrained-Bedingungen bestehen. | §S.2 |
| id: REQ-ZKP-306 | Die Anzahl der R1CS-Constraints pro Circuit MUSS durch Obergrenzen beschränkt sein, um DoS zu verhindern. | §S.2 |
| id: REQ-ZKP-307 | Kompilierte Circuit-Artefakte MÜSSEN mit ihrer SHA-256-Prüfsumme in der Registry verankert werden. | §S.3 |
| id: REQ-ZKP-308 | Circuit-Signale MÜSSEN explizit als public oder private klassifiziert sein. | §S.3 |

## Metriken & Akzeptanzkriterien

- **Metrik 1:** Under-Constrained-Erkennungsrate in Schaltungstests MUSS 100% betragen.
- **Metrik 2:** Maximale Constraint-Anzahl pro Basis-Circuit MUSS unter 1.000.000 liegen.
- **Metrik 3:** Deterministische Reproduzierbarkeit der Kompilierung MUSS bei 100% liegen.
- **Metrik 4:** Testabdeckung für ZK-Circuits MUSS mindestens 95% erreichen.


## Metriken & Akzeptanzkriterien

- **Metrik 1:** Under-Constrained-Erkennungsrate in Schaltungstests MUSS 100% betragen.
- **Metrik 2:** Maximale Constraint-Anzahl pro Basis-Circuit MUSS unter 1.000.000 liegen.
- **Metrik 3:** Deterministische Reproduzierbarkeit der Kompilierung MUSS bei 100% liegen.
- **Metrik 4:** Testabdeckung für ZK-Circuits MUSS mindestens 95% erreichen.
- **Metrik 5:** Kompilierungszeit komplexer Circuits MUSS unter 60 Sekunden betragen.

## Compliance & Verifikation

Geprüft durch: (1) `atc-std-validator` (Registry-Sync S-14, Kopf-Sync S-19, Testläufe S-01 bis S-25), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen MÜSSEN als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 geführt und im Ticket-System protokolliert werden.

## Security Considerations

- Circuits MÜSSEN frei von unbeschränkten Signalen sein, die unechte Beweise ermöglichen könnten.
- Die Integrität der Circuit-Bytecodes MUSS vor der Ausführung kryptografisch verifiziert werden.
- Formale Verifikation von Schaltungsinvarianten MUSS für kritische Kern-Circuits durchgeführt werden.
- Sämtliche Zwischensignale MÜSSEN nach der Zeugenerzeugung (Witness Generation) im Speicher gelöscht werden.

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
- ATC-STD-ZKP-010

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie)
