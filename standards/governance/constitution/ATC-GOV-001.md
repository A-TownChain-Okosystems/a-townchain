---
standard:
  id: ATC-GOV-001
  title: "A-TownChain Governance Constitution"
  version: "1.0.0"
  status: approved
  category: governance
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Governance Authority"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Gesamte Organisation A-TownChain-Okosystems: Org, Repos, Standards, Agents, Releases, Treasury, Protocol"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-002
----

# ATC-GOV-001 — A-TownChain Governance Constitution (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf 11.09.2026 (SCR-0097).
> Level 0 der Governance-Hierarchie; ATC-STD-000 (Standards-Governance)
> bleibt Level-2-Ausführungsnorm und wird von dieser Verfassung getragen —
> nicht ersetzt. Die Verfassung selbst steht NICHT außerhalb des
> Governance-Prozesses: Änderungen laufen ausschließlich über Kap. 22.
> **Governance:** par.9-freigegeben 11.09.2026 (Owner-Direktive, SCR-0102) — wirksam; ATC-STD-000 bleibt Level-2-Ausfuehrungsnorm und wird getragen.

## Abstract

Governance ist kein Dokument, sondern ein durchsetzbares Betriebssystem für
Entscheidungen: Wer darf was entscheiden, nach welchem Prozess, mit welcher
Evidenz, mit welcher Sperrfrist — und wie wird jede Entscheidung später
auditiert. Leitziel (Owner-Formulierung): Keine kritische Entscheidung ohne
definierte Autorität, keine kritische Änderung ohne unabhängige Prüfung,
keine Ausnahme ohne Ablaufdatum, keine Produktion ohne Evidence, keine
Governance-Aktion ohne Audit Trail.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Kap. 1 — Purpose & Scope (Kap. 1–2)

Die Verfassung definiert die unveränderlichen Grundprinzipien der
Organisation A-TownChain-Okosystems. Scope: Organisation, alle Repositories,
Standards, AI-Agenten, Releases, Treasury, Protokoll/Chain.

## Kap. 2 — Governance-Hierarchie (5 Ebenen)

- **L0 Constitution** (dieses Dokument) — Grundgesetz.
- **L1 Organizational Governance** — Entscheidungsrechte, Autoritäten, Quoren
  (authority/authority-matrix.yaml).
- **L2 Standards Governance** — Entstehung/Änderung von Standards
  (ATC-STD-000, ATC-STD-002).
- **L3 Technical/Repository Governance** — Umsetzung in GitHub, Code,
  Releases, Infrastruktur (Governance-Profile, GitHub-Rulesets).
- **L4 Operational Governance** — Incidents, Releases, Audits, Backups,
  Security Events (ATC-STD-020–043, GATE-GOV-001..008).

Jede Ebene MUSS die obere respektieren; Konflikte lösen sich von oben nach unten.

## Kap. 3 — Authority Model: Kein God Mode (Kap. 5–6, REQ-GOV-001)

Es existiert KEINE Rolle mit unbegrenzten Rechten. Stattdessen acht
Authorities mit begrenzten Capability-Sets (maschinenlesbar:
governance/authority/authority-matrix.yaml): Standards, Architecture,
Security, Release, Treasury, Protocol, Infrastructure, AI Governance.
Zwischen Authorities gilt Capability-Trennung: z. B. Security Authority
hat Security Policies/Gates/Emergency Freeze/Security Audit — NICHT
Treasury, Standard-Approval oder beliebige Repository-Löschung.
Owner-Rolle: Inhaber aller Autoritäten-Ämter, ABER Ausübung erfolgt
rollenbasiert über die gleiche Matrix — der Owner entscheidet Ämter-Besetzung
(Kap. 21), nicht Matrix-Ausnahme.

## Kap. 4 — Decision Rights Matrix (REQ-GOV-002)

Jede kritische Entscheidung MUSS die Spalten Propose/Review/Approve/Execute/
Audit besitzen (Beispiel-Matrix s. authority-matrix.yaml): Standard, Architektur,
Security Fix, Mainnet Release, Smart Contract, GitHub Ruleset, Treasury
Transfer, AI-Agent Policy. Der klassische Governance-Fehler ist VERBOTEN:
Eine Person DARF NICHT vorschlagen, genehmigen und selbst ausführen.
Blockade-/Vetorechte sowie Informationspflichten sind je Entscheidung
dokumentiert.

## Kap. 5 — Approval-Quoren (REQ-GOV-003)

| Risiko | Quorum |
|---|---|
| Normal Change | 2 unabhängige Approvals |
| High Risk | 3-of-5 Governance Approval |
| Critical Infrastructure | 3-of-5 Security/Architecture Approval |
| Protocol/Consensus | 4-of-7 Governance Approval + Timelock |
| Emergency | 2-of-N Emergency Authority + zwingender Post-Incident-Audit |

Blockchain-/Treasury-Entscheidungen MÜSSEN eine Timelock-Schicht haben
(Genehmigung von Ausführung getrennt). **Übergangsbestimmung (real):** Die
Organisation ist aktuell eine Ein-Personen-Org mit geteilter Agent-Identität.
Bis ein zweiter Reviewer-Account existiert, gelten die Quoren mit
kompensierenden Kontrollen (ATC-EXC-001-Muster: dokumentierte Exception,
Ablaufdatum, Test-Evidence, Post-Merge-Audit). ATC-EXC-001 MUSS spätestens
30.09.2026 durch den zweiten Account abgelöst sein — sonst Fail-Closed.

## Kap. 6 — Emergency Governance (Kap. 15, REQ-GOV-004)

Break-Glass: Detection → Emergency Authority → 2-of-N Approval → Immediate
Containment → Notification → Post-Incident Review → Governance Audit. Jede
Emergency Decision erhält ATC-EMG-NNN, danach ZWINGEND ATC-AUD-NNN.
Emergency Authority DARF KEIN dauerhafter Machtkanal werden (Befristung,
automatischer Rückfall in Normal-Governance; Operationalisierung:
ATC-STD-035/036).

## Kap. 7 — Exceptions (Kap.REQ-GOV-005)

„Diesmal machen wir eine Ausnahme" DARF NICHT informell passieren. Jede
Ausnahme IST ATC-EXC-NNN mit Reason, Scope, Risk, Duration, Owner, Approver,
Compensating Controls, Expiration, Review. **NO PERMANENT EXCEPTION** — jede
Ausnahme läuft automatisch ab (Fail-Closed; praktiziert: ATC-EXC-001).

## Kap. 8 — Conflict of Interest (Kap. 17, REQ-GOV-006)

Personen mit Entscheidungsbefugnis MÜSSEN relevante Interessenkonflikte
deklarieren: No Conflict / Declared Conflict / Recusal Required / Independent
Review Required. Wer einen Smart Contract entwickelt, DARF NICHT selbst dessen
Security-Review genehmigen — Governance Violation.

## Kap. 9 — Appeals (Kap. 20, REQ-GOV-007)

Decision → Appeal (ATC-APPEAL-NNN) → Independent Review → Final Decision.
Governance ist damit rechtsähnlich strukturiert, nicht nur autoritär.

## Kap. 10 — AI-Agent Governance (Kap. 16, REQ-GOV-008)

Kein Agent erhält AI→GitHub→Production. Pflichtkette: Agent Identity → Task
Scope → Permission Check → Repository Policy → Change → Automated
Verification → Human/Authority Approval → Merge. Jeder Agent MUSS führen:
Agent ID, Agent Owner, Capabilities, Repositories, Allowed/Forbidden Actions,
Maximum Risk Level, Audit Log, Expiration, Human Escalation. Agieren im
Rahmen von AAS/AI-DEV-Familie, Manifest und Hub-Authorization; Agent-Pushes
unterliegen derselben Gate- und Evidence-Pflicht wie menschliche Änderungen.

## Kap. 11 — Governance Gates (REQ-GOV-009)

Acht verbindliche Gates (maschinenlesbar: governance/decision-rights/
governance-gates.yaml): GOV-GATE-001 Governance Compliance, 002 Architecture,
003 Security, 004 Test/Verification, 005 Repository, 006 Release, 007
Operational Readiness, 008 Post-Release Verification. „Tests erfolgreich"
ohne Evidence-Verweise IST NICHT GÜLTIG — Gates fordern Evidence-Pfade
(test-report/, fuzzing-report/, security-report/ …). Release nur bei
ALL REQUIRED = PASS/APPROVED.

## Kap. 12 — GitHub als Enforcement-Layer (Kap. REQ-GOV-010)

Governance MUSS technisch erzwingen, nicht nur beschreiben: organisations-
weite Rulesets (verbindliche Reviews, Status Checks, Code-Scanning, kein
Force-Push) gemäß ATC-GITHUB-001..010-Ableitung (governance/
decision-rights/github-rulesets.md, phasiger Rollout). Aktive Rulesets sind
für Leserechte einsehbar → Auditability. Policy→Standard→Control→Evidence
ist die verbindliche Kette: POLICY („Main branches must be protected") →
STANDARD → CONTROL (GitHub Ruleset) → EVIDENCE (Ruleset-Config, CI-Results,
Approvals, Audit-Log).

## Kap. 13 — Repository-Governance-Profile (REQ-GOV-011)

Jedes Repository MUSS einem Governance-Profil zugeordnet sein
(maschinenlesbar: governance/repository-governance/profiles.yaml):
G0 Experimental, G1 Standard, G2 Production, G3 Critical, G4 Sovereign/Core
(Felder: governance_profile, security_level, availability, data_classification,
release_class, review_level). Kernprojekte (a-townchain, atclang,
atc-shivacore, globus-os) tragen strengere Governance als experimentelle
Projekte; Authority-Chains je Kernprojekt domänenspezifisch
(Protocol/Language/Kernel/OS-Architecture Authority + Security).

## Kap. 14 — Governance Score (REQ-GOV-012)

GS = 20 % Decision Integrity, 15 % Security Governance, 15 % Standards
Compliance, 10 % Repository Compliance, 10 % Auditability, 10 % Transparency,
10 % Change Management, 5 % Emergency Governance, 5 % Succession/Recovery.
Harte Deckel: Critical Violation → max 5/10 · Fehlender Audit Trail → max
7/10 · Single-Person Critical Authority → max 6/10 · Unkontrollierter
Produktionszugriff → max 4/10 · Kein Emergency-Verfahren → max 8/10. Schöne
Dokumentation KANN KEINE 10/10 erzeugen — nur Evidence.

## Kap. 15 — Transparenz & Audit-Kadenz (Kap. 18/19, REQ-GOV-013)

Transparenz-Dashboard (aktive Proposals, Reviews, Freigaben, Exceptions,
Audits, Standards-Zähler) wird aus der Registry generiert — Registry+Repo
schlagen Wiki. Audit-Kadenz: monatlich Operational Review, vierteljährlich
Governance Audit, halbjährlich Authority Review, jährlich Constitution
Review, je Major Release Governance Compliance Audit, je Security Incident
sofortige Governance Review (ATC-AUD-NNN).

## Kap. 16 — Succession & Recovery (Kap. 21, REQ-GOV-014)

Recovery-/Nachfolgeregelung: Amtsübergabe dokumentiert, Credentials/Keys
geordnet übergeben (ATC-STD-038-Kette), Notfall-Zugriff geregelt, Succession
 jährlich geprüft.

## Kap. 17 — ID-System (REQ-GOV-015)

Governance ist maschinenlesbar: ATC-GOV-NNN (Governance-Entscheidungen/
Standards dieser Familie — neue Präfix-Familie per SCR-0097), ATC-AUD-NNN
(Audits), ATC-INC-NNN (Incidents), ATC-EMG-NNN (Emergency Decisions),
ATC-APPEAL-NNN, ATC-VOTE-NNN, ATC-REL-X.Y.Z (Releases), ATC-EXC-NNN
(Exceptions, praktiziert). Bestehende Systeme bleiben: SCR-NNNN = org-Change-
Request (deckungsgleich mit ATC-CR-Konzept), REQ-<std>-NNN (ATC-STD-002 §4),
Org-Gates ATC-GATE-<DOM>-NNN, AD-NNN grandfathered. Evidence Trail je
Entscheidung: Proposal → Review → Evidence → Decision → Approval →
Implementation → Verification → Audit.

## Kap. 18 — Amendment Process (Kap. 22, REQ-GOV-016)

Änderungen dieser Verfassung NUR über: Proposal (ATC-GOV-NNN CR) → Review
(Technical/Security) → Governance-Approval nach Kap. 5-Quorum → Versionierung
(SemVer MAJOR für Prinzipien-Änderungen) → Changelog + Audit-Eintrag.
Es existiert KEIN Weg, ATC-GOV-001 außerhalb dieses Prozesses zu ändern.
Dissolution/Sunset (Kap. 23): nur durch MAJOR-Amendment mit
(hoechstes Quorum (Einstimmigkeit aller Authorities)).

## REQ-Matrix (normative Anforderungen)

- id: REQ-GOV-001 — Authorities MUSS begrenzte Capability-Sets haben; God Mode ist VERBOTEN.
- id: REQ-GOV-002 — Decision Rights Matrix MUSS je kritischer Entscheidung Propose/Review/Approve/Execute/Audit besitzen.
- id: REQ-GOV-003 — Quoren MÜSSEN gelten; Timelock MUSS Blockchain/Treasury; Übergangsregeln MÜSSEN befristet sein (Fail-Closed).
- id: REQ-GOV-004 — Emergency MUSS ATC-EMG + zwingenden Post-Audit haben; kein dauerhafter Machtkanal.
- id: REQ-GOV-005 — Exceptions MÜSSEN formalisiert sein; NO PERMANENT EXCEPTION.
- id: REQ-GOV-006 — COI MUSS deklariert werden; Selbst-Review ist Governance Violation.
- id: REQ-GOV-007 — Appeals MÜSSEN unabhängig reviewbar sein.
- id: REQ-GOV-008 — AI-Agenten MÜSSEN die Pflichtkette mit Permission Check, Approval und Audit Log durchlaufen.
- id: REQ-GOV-009 — 8 Governance Gates MÜSSEN Evidence erzwingen; Release nur bei ALL REQUIRED PASS.
- id: REQ-GOV-010 — GitHub MUSS als Enforcement-Layer genutzt werden (Rulesets, Policy→Standard→Control→Evidence).
- id: REQ-GOV-011 — Repositories MÜSSEN Governance-Profilen (G0–G4) zugeordnet sein.
- id: REQ-GOV-012 — Governance Score MUSS harte Deckel haben.
- id: REQ-GOV-013 — Transparenz MUSS aus der Registry generiert; Audit-Kadenz MUSS eingehalten werden.
- id: REQ-GOV-014 — Succession MUSS geregelt sein.
- id: REQ-GOV-015 — ID-System MUSS maschinenlesbar sein; Bestand bleibt (SCR, REQ, AD grandfathered).
- id: REQ-GOV-016 — Amendment MUSS ausschließlich über Kap. 18 laufen.

## Compliance

Prüfung: Governance-Audit nach Kap. 15; GS-Berechnung (Kap. 14); Gates via
CI/Assurance-Engine; Evidence unter evidence/ + audits/.

## Security Considerations

- Constitution-Datei ist Schutzziel höchster Stufe: Branch-Protection,
  Versionierung, Audit-Pflicht.
- Übergangs-Quoren (Ein-Personen-Org) sind das größte aktuelle Risiko —
  ATC-EXC-001-Ablauf 30.09. MUSS durch zweiten Account abgelöst werden.
- Ruleset-Rollout nicht gegen technische Realität erzwingen (GPG-Signierung
  erst nach Release-Key — phasiger Plan).

## Implementierungsstatus

**Status: SPECIFIED** — Verfassung + maschinenlesbare Komponenten
(authority-matrix.yaml, governance-gates.yaml, profiles.yaml) angelegt;
Ableitung ATC-GOV-002..015 als Roadmap (Kap. „Familien-Roadmap" unten);
GitHub-Ruleset-Rollout phasig nach Owner-Freigabe. SSOT:
registry/standard-implementation.yaml.

## Familien-Roadmap (ATC-GOV-002..015, SCR-0097 Slots)

GOV-002 Authority & Decision Rights · GOV-003 Roles & Responsibilities ·
GOV-004 Proposal Standard · GOV-005 Voting & Approval · GOV-006 Emergency
Governance · GOV-007 Exception Management · GOV-008 Conflict of Interest ·
GOV-009 Governance Audit · GOV-010 Transparency · GOV-011 Succession &
Recovery · GOV-012 AI-Agent Governance · GOV-013 Repository Governance ·
GOV-014 Release Governance · GOV-015 Governance Metrics & Scoring.
(Keine Schatten-IDs: Slots werden erst mit Registry-Zeile bei Bau belegt.)

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Governance Operating System (SCR-0097) —
  23-Kapitel-Verfassung komprimiert auf 18 normative Kapitel, 5-Ebenen-
  Hierarchie, 8 Authorities, Decision Rights Matrix, Quoren mit Timelock,
  Emergency/Exception/COI/Appeals/Succession, AI-Agent-Kette, 8 Gates,
  GitHub-Enforcement, G0-G4-Profile, GS mit harten Deckeln, Amendment-Prozess.
  CANDIDATE.

## References

- ATC-STD-000 — Standards-Governance (L2) · ATC-STD-002 — Familien-/ID-Architektur
- ATC-STD-020–043 — Assurance-Familie (L4) · ATC-EXC-001 — praktizierte Exception
- .github-Hub — Authorization/Control Plane (Agent-Governance)
