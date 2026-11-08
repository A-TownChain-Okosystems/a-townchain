---
standard:
  id: ATC-STD-ZKP-001
  title: "ATC-STD-ZKP-001 — ZKP Architecture Standard"
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

# ATC-STD-ZKP-001 — ZKP Architecture Standard (v1.1.0, APPROVED)

> **Version:** 1.1.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

Definiert die ZKP-Layer als eigenstaendige Protokollschicht zwischen A-TownChain L1 und den Anwendungen/Rollups: eine kryptografische Verifikationsschicht, KEIN eigenes Netzwerk. Verortet die sechs Kernmodule (Proof Generator, Circuit Registry, Verification Engine, Commitment Manager, Nullifier Manager, Proof Cache) und das Repository atc-zkp (7-Crate-Layout, Rust).

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-101 | Die ZKP-Layer MUSS als Verifikationsschicht innerhalb der A-TownChain implementiert sein — kein eigener Konsens-, Netzwerk- oder State-Layer. | §S.1/Designentscheidung |
| id: REQ-ZKP-102 | Die ZKP-Layer MUSS die sechs Kernmodule in dieser Struktur vorhalten (Proof Generator, Circuit Registry, Verification Engine, Commitment Manager, Nullifier Manager, Proof Cache). | §S.2 |
| id: REQ-ZKP-103 | Die kanonische Implementierung MUSS in Rust erfolgen (ATC-STD-100 L1, Kryptografie-Bindung). | §S.2 |
| id: REQ-ZKP-104 | Aenderungen an der Layer-Architektur MUESSEN als AD mit Owner-Freigabe gefuehrt werden. | §S.5 |
| id: REQ-ZKP-105 | Das Repo atc-zkp MUSS das ATC-STD-201-Layout erfuellen (.atc-Metadaten, Manifest, Skeleton-Stufen). | §S.5 |

| id: REQ-ZKP-106 | Die Verification Engine MUSS sämtliche eingehenden Beweise gegen die Circuit Registry validieren, bevor der Verifikationsprozess gestartet wird. | §S.2 |
| id: REQ-ZKP-107 | Der Proof Cache MUSS verifizierte Beweise zeitlich begrenzt zwischenspeichern; abgelaufene Cache-Einträge MÜSSEN atomar gelöscht werden. | §S.2 |
| id: REQ-ZKP-108 | Der Commitment Manager MUSS sicherstellen, dass keine ungültigen Commitments in den Zustand übernommen werden. | §S.3 |
| id: REQ-ZKP-109 | Architekturänderungen an den sechs Kernmodulen MÜSSEN ein formales Audit nach G18-Security-Richtlinien durchlaufen. | §S.5 |

## Metriken & Akzeptanzkriterien

- **Metrik 1:** Latenz der Verifikationsentscheidung MUSS im Mittel unter 150 ms liegen.
- **Metrik 2:** Trefferquote des Proof Caches MUSS für wiederholte Verifikationen mindestens 90% betragen.
- **Metrik 3:** Fehlerrate der Modul-Schnittstellen MUSS exakt 0% sein.
- **Metrik 4:** Abdeckung der Architekturkomponenten durch Unit-Tests MUSS mindestens 95% betragen.


## Metriken & Akzeptanzkriterien

- **Metrik 1:** Latenz der Verifikationsentscheidung MUSS im Mittel unter 150 ms liegen.
- **Metrik 2:** Trefferquote des Proof Caches MUSS für wiederholte Verifikationen mindestens 90% betragen.
- **Metrik 3:** Fehlerrate der Modul-Schnittstellen MUSS exakt 0% sein.
- **Metrik 4:** Abdeckung der Architekturkomponenten durch Unit-Tests MUSS mindestens 95% betragen.
- **Metrik 5:** Durchsatz der Verification Engine MUSS mindestens 200 Verifikationen pro Sekunde erreichen.

## Compliance & Verifikation

Geprüft durch: (1) `atc-std-validator` (Registry-Sync S-14, Kopf-Sync S-19, Testläufe S-01 bis S-25), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen MÜSSEN als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 geführt und im Ticket-System protokolliert werden.

## Security Considerations

- ZKP-Komponenten sind S4-kritisch (Kryptografie-Bindung, ATC-STD-100): Implementierung MUSS in Rust erfolgen.
- Trusted-Setup-Kriterien und Fuzzing-Pflichten gelten nach ATC-STD-ZKP-010.
- Der G18 Security-Audit (AD-023) ist Voraussetzung für jeden Freeze.
- Speicherzugriffe innerhalb des Proof Caches MÜSSEN gegen Race Conditions geschützt werden.

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
- ATC-STD-ZKP-010

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie)
