---
standard:
  id: ATC-AAS-011
  title: "ATC-AAS-011 — Agent Verification Standard"
  version: "1.1.0"
  status: approved
  category: aas
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-AAS-011 — Agent Verification Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-011 (Agent Verification Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Verifikationsebenen

L1 Syntax (Lint/Compile) · L2 Unit Tests · L3 Integration Tests ·
L4 System Validation (End-to-End, Devnet-basiert, ATC-STD-NET-001).

Je Änderung gilt die höchste anwendbare Ebene als Mindest-Validierung;
L-Angabe ist Teil des Evidenzblocks (AAS-010).

## 2. Zusatz-Dimensionen für Blockchain-Kernkomponenten

Security · Consensus · Cryptography · State Transition · Economic Rules ·
Compatibility.

Betroffene Komponenten (atc-blockchain, atc-algorithm, atc-vm, a-townchain,
atc-contracts, atc-zkp) DÜRFEN nur mit Prüfung aller zutreffenden
Zusatz-Dimensionen als COMPLETED abgeschlossen werden (Findings bei
Abweichung, Human-Approval-Pflicht AAS-017).

## 3. Verifikationsmatrix

`verification: {level: L1..L4, dimensions: [...], result, evidence}` je Task
(AI-DEV-008 §1).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Agenten-spezifisch: Agent-Identitaet via AGENT_MANIFEST verifizierbar; Permissions nach Least-Privilege; Delegationen dokumentiert und widerrufbar; Zugangsdaten ausschliesslich als $ENV-Platzhalter (ATC-STD-203); Agent-Kommunikation authentifiziert, nie anonym.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203, ATC-AAS-001 · INFORMATIVE: AGENT_MANIFEST.md v3.1.7, Registry-Kategorie aas
## 4. Ergänzende Verbindliche Betriebs- und Governance-Regeln

1. Verifizierungen MÜSSEN mehrstufig auf Syntax-, Semantik- und Integrationsebene durchgeführt werden.
2. Für Blockchain-Kernkomponenten MÜSSEN zwingend zusätzliche Verifikationsdimensionen (Determinismus, Gas-Verbrauch) geprüft werden.
3. Die Verifikationsmatrix MUSS vor jeder Freigabe vollständig mit positiven Ergebnissen durchlaufen werden.
4. Verifikationsprüfungen der Stufen V1 und V2 MÜSSEN automatisch in der CI/CD-Pipeline ausgeführt werden.
5. Fehlgeschlagene Verifikationsschritte MÜSSEN mit einer detaillierten Ursachenanalyse dokumentiert werden.
6. Verifikationsberichte MÜSSEN im Ziel-Repository abgelegt und gegen nachträgliche Modifikation geschützt werden.

## 5. Metriken & Akzeptanzkriterien

- 100% aller Blockchain-Smart-Contracts MÜSSEN die Verifizierungsstufe V3 erfolgreich bestehen.
- Testabdeckung bei Verifizierungsprüfungen MUSS mindestens 95% betragen.
- 0 Verifikationsumgehungen MÜSSEN bei Produktions-Releases vorkommen.
- Verifikationsläufe MÜSSEN vor dem Merge abgeschlossen sein.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch Validator-Läufe (S-01 bis S-25) verifiziert werden. Abweichungen führen zu Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005. Mängel MÜSSEN behoben werden.

## 7. Security Considerations

Strenge Verifizierungsstandards verhindern die Ausführung von fehlerhaftem oder unsicherem Code in kritischen Systemumgebungen. Fehlgeschlagene Prüfungen MÜSSEN das Deployment sichern.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-008, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET, ATC-STD-NET-001
