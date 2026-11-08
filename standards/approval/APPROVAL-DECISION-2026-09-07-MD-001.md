# Approval Decision — ATC-STD-MD-001 v1.0.0

**Datum:** 07.09.2026, 21:05 UTC+2 · **Entscheider:** Owner (Michael Wroblewski) · **Status: APPROVED**

## Owner-Approval

**Entscheidung:** **APPROVED / FREIGEGEBEN** am 07.09.2026, 21:05 UTC+2.
Owner: Michael Wroblewski — Freigabe per Owner-Direktmandat „Freigabe" im
Builder-Chat (Todo #117).

**Gegenstand:** ATC-STD-MD-001 v1.0.0 (CANDIDATE → APPROVED) — ATC Markdown
& Documentation Standard (Owner-Entwurf Michael, Formalfassung
ATC-AI-ARCH-001): Markdown als verbindliches Documentation-Standard-Modell
je Repository.

**Umfang:** 16 Anforderungen (REQ-MD-001..016): Pflicht-/bedingte Dokumente
(§1), Dokumentationshierarchie (§2), UPPER_SNAKE_CASE-Namen (§3),
Überschriften (§4), Metadaten-Header (§5), Dokument-Status-Enum (§6),
SemVer (§7), Tabellen (§8), Codeblöcke (§9), relative Links (§10),
Cross-References (§11), CHANGELOG-Format (§12), maschinenlesbare STATUS.md
(§13), AI-Agenten-Instruktionen (§14), Documentation-Consistency-Gate (§15),
Dokumenttyp-Klassifikation (§16). Normenhierarchie: README-001 lex specialis
für README; Verfassung §9 für Standard-Dokumente. Gates MD-01..10 in
tools/atc-md-validator/check_md.py.

**Evidenz bei Freigabe:** validate_all 103/103 COMPLIANT · MD-Gate CONFORM
(atc-standards) · README-Gate 13/13 · Contract-Registry CONFORM · R3 100/100
GATE PASS · Mutation 12/12 · Commits e503d1c/548ac16/0d45fc0/5aaf107.

## Konsequenzen

- ATC-STD-MD-001 ist **normativ in Kraft**; Immutabilität per ATC-STD-000
  §30 — Änderungen ab sofort nur via SCR.
- Registry: **103 Standards, 103 APPROVED, 0 offen.**
- Übergangsfrist bis 07.10.2026: MD-Konformitäts-Rollout auf alle 26 Repos
  (check_md.py je Repo; Agenten-lokale Prüfung bis zur CI-Integration,
  workflow-Scope-Blocker F-009/F-010).
- Task #117: Freigabe erledigt; Rollout-Komponente läuft.
