---
standard:
  id: ATC-STD-REPO-AUDIT-002
  title: "ATC Repository Audit Checklisten- & Health-Score-Standard — Konkrete automatisierbare Checks (CHECK-001, CHECK-002, …) und standardisierter Repository Health Score für jeden KI-/Automatisierungsagenten"
  version: "1.0.0"
  status: approved
  category: repo-audit
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 08.09.2026, 00:05 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-REPO-AUDIT-001
    - ATC-STD-AUDIT-001
    - ATC-STD-BUG-005
    - ATC-STD-COMPAT-001
    - ATC-STD-FRAMEWORK-001
  related_standards:
    - ATC-STD-000
    - ATC-STD-MILESTONE-001
    - ATC-STD-VERSION-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-REPO-AUDIT-002 — Audit-Checklisten- & Health-Score-Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 08.09.2026, 00:05 UTC+2);
> normativ in Kraft ab 08.09.2026, §30-eingefroren (ATC-STD-000). SCR-0021 akzeptiert.
> **Familie:** Repository Audit (FAM-41, Slot 002). **Kopplungen:** REPO-AUDIT-001
> (Prozess/A-E-Status), AUDIT-001 (AUD-Records), BUG-005 (Findings/RCA), FRAMEWORK-001.

## Abstract

ATC-STD-REPO-AUDIT-002 macht den Repository-Audit aus REPO-AUDIT-001 maschinell
ausführbar: Ein Katalog konkreter, automatisierbarer Checks (**CHECK-001 … CHECK-064**,
SSOT `registry/repo-audit-checks.yaml`) deckt alle 16 Prüfbereiche ab; jeder Check
liefert PASS/WARN/FAIL/SKIP mit Evidence; ein gewichteter, standardisierter
**Repository Health Score** (0–100 → A–E) macht Ergebnisse über Repos, Agenten und
Zeitpunkte vergleichbar. Ein KI-Entwicklungsauditor MUSS für jedes ATC-Repository
denselben Prozess ausführen und am Ende denselben Health Score erzeugen.

Schlüsselwörter: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE (empfohlen),
DARF/KANN (optional) — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Alle Repository-Audits nach Inkrafttreten; Automatisierung durch KI-Agenten
(governance-ci, Auditor-Agent) und manuelle Audits gleichermaßen.

**Gilt nicht:** Die qualitativen Regeln von REPO-AUDIT-001 (bleiben dort normativ);
Org-weite Audits (AUDIT-001-Ebene, z. B. AUD-2026-0002) — diese KANNEN die Checks
nutzen, folgen aber AUDIT-001.

## §1 Zweck

Aus jedem Prüfpunkt von REPO-AUDIT-001 werden konkrete, ausführbare Checks; am Ende
steht ein standardisierter Repository Health Score (REQ-RB-001). Gleichheit des
Prozesses für alle Agenten und Repos ist das Ziel („same audit, same score").

## §2 Check-Katalog (CHECK-001 … CHECK-064)

**SSOT:** `registry/repo-audit-checks.yaml` (generiert von
`tools/repo-audit/gen_checks.py`; Änderungen nur via SCR, REQ-RB-011). Struktur:
16 Bereiche mit Gewicht (Summe 100), je Bereich 4 Checks, je Check:
`id` (CHECK-NNN), `area` (einer der 16 Bereiche), `title`, `method`
(AUTO | MANUAL | HYBRID), `weight` (1–3, Signifikanz im Bereich),
`description` (Prüfanweisung + Evidence-Erwartung).

Katalog-Umfang: 64 Checks — je 4 je Bereich (Fehler, Vollständigkeit, Lücken,
Sicherheit, Codequalität, Architektur, Dokumentation, Tests, Build, Abhängigkeiten,
Struktur, Kompatibilität, Versionierung, CI/CD, Governance, Verbesserung).
Bereichs-Gewichte (Summe 100): Sicherheit 12, Tests 10, Build 9, Vollständigkeit/
Dokumentation je 7, Fehler/Lücken/Codequalität/Architektur/Abhängigkeiten/
Kompatibilität je 5–6, Struktur/Versionierung/CI-CD/Governance je 4–5,
Verbesserung 3 (REQ-RB-002).

## §3 Check-Ausführung

Jeder Check MUSS ausgeführt werden mit Ergebnis (REQ-RB-003, REQ-RB-004):
`PASS` (erfüllt, 1.0) · `WARN` (teilweise, 0.5) · `FAIL` (verletzt, 0.0) ·
`SKIP` (nicht anwendbar — Begründung PFLICHT; unbegründetes SKIP IST ein Finding P1).
Je Check MUSS Evidence erzeugt werden (Kommandoausgabe, Artefakt-Link, AUD-Record-
Verweis). **AUTO**-Checks MÜSSEN ohne menschliches Zutun ausführbar sein (CI);
**MANUAL**-Checks durch autorisierte Auditor-Rolle; **HYBRID** = AUTO-Vorarbeit +
menschliche Bestätigung. MANUAL-Bestätigungen von KI-Agenten MÜSSEN durch ein Human
Gate bestätigt werden (AI-DECISION-001, MILESTONE-001 §13).

## §4 Scoring-Formel

```
Bereichsscore  = 100 × Σ(Checkergebnis × Checkgewicht) / Σ(Checkgewichte)   [je Bereich, SKIPs ausgenommen]
Gesamtscore    = Σ(Bereichsscore × Bereichsgewicht) / 100                   [0 … 100, eine Nachkommastelle]
```

SKIPs senken den Zähler NICHT künstlich; ein Bereich mit nur SKIPs wird mit
Begründung aus der Gewichtung genommen (REQ-RB-005). Der Gesamtscore MUSS im
Audit-Report stehen (REQ-RB-006).

## §5 Health-Score → Status-Mapping (A–E)

Verbindliche Abbildung auf REPO-AUDIT-001 §25 (REQ-RB-006):

| Bedingung | Health-Status |
|---|---|
| P0-Finding offen | **E** (CRITICAL/BLOCKED) — unabhängig vom Score |
| Score ≥ 90 UND P0 = 0 UND P1 = 0 | **A** (READY) |
| Score ≥ 75 UND P0 = 0 | **B** (READY WITH FINDINGS) |
| Score ≥ 60 UND P0 = 0 | **C** (REQUIRES IMPROVEMENT) |
| sonst | **D** (NOT RELEASE READY) |

Release-Ableitung: A/B release-fähig (B mit Finding-Tracking); C/D vor Release
verbessern; E blockiert (P0-Regel REPO-AUDIT-001 §22).

## §6 Audit-Report-Format

Jeder Audit erzeugt (REQ-RB-007): (a) einen **AUD-Record** (AUD-YYYY-NNNN) gemäß
AUDIT-001, (b) einen maschinenlesbaren **Health Report** (YAML/JSON): Repo, Audit-ID,
Datum, Auditor (Agent-ID/Rolle), je Check {Ergebnis, Evidence-Ref}, je Bereich
{Score}, Gesamtscore, Health-Status, P0–P4-Zähler, Findings (F-NNN gemäß
registry/findings.yaml, Fortschreibung F-032+), Gaps (GAP-<KAT>-NNN),
Improvements (IMP-NNN), SKIP-Begründungen.

## §7 CI-Integration

AUTO-Checks SOLLEN in governance-ci je Repository laufen (REQ-RB-008);
Ergebnisse fließen in den Health Report. Fehlende CI-Integration ist selbst ein
Audit-Finding (GAP-CI, REPO-AUDIT-001 §17). Der Validator prüft die
Katalog-Integrität mit **S-22** bei jedem Lauf (REQ-RB-002).

## §8 Rollen & Human Gates

Auditor KANN ein KI-Agent sein; Findings, Scores und Reports von Agenten sind
VORLÄUFIG und MÜSSEN vor APPROVAL/RELEASE READY durch den Owner (Human Gate)
bestätigt werden (REQ-RB-009; AI-DECISION-001). Der Owner KANN AUD-Records
nach eigener Prüfung freigeben.

## §9 Katalog-Governance

Neue Checks, geänderte Gewichte oder neue Bereiche NUR via SCR (REQ-RB-011);
Katalog hat eigene Version (an REPO-AUDIT-002-Version gebunden, VERSION-001).
Katalog-Integrität: ≥ 48 Checks, ≥ 3 je Bereich, Gewichte je Check 1–3,
Bereichsgewichte Summe 100 — durch S-22 erzwungen.

## §10 Ausbaustufe REPO-AUDIT-003

Der vollautomatische Auditor-Agent (Pipeline REPO-AUDIT-001 §28) wird als
ATC-STD-REPO-AUDIT-003 (Agent-Spezifikation, AAS-Kopplung) GEPLANT (REQ-RB-012);
bis dahin führen KI-Agenten die Checks manuell/halbautomatisch mit demselben
Katalog aus.

## Requirements (normativ)

- **REQ-RB-001** (§1): Jeder Repository-Audit MUSS nach diesem Standard einen
  standardisierten Health Score erzeugen.
- **REQ-RB-002** (§2): `registry/repo-audit-checks.yaml` ist der verbindliche
  Check-Katalog (SSOT); S-22 MUSS seine Integrität prüfen.
- **REQ-RB-003** (§3): Jeder Check MUSS mit Method-Typ (AUTO/MANUAL/HYBRID) und
  Evidence ausgeführt werden.
- **REQ-RB-004** (§3): Ergebnisse sind PASS/WARN/FAIL/SKIP; SKIP ohne Begründung
  ist ein P1-Finding.
- **REQ-RB-005** (§4): Scoring folgt verbindlich der gewichteten Formel.
- **REQ-RB-006** (§5): Das A–E-Mapping (inkl. P0→E-Regel) ist verbindlich und
  MUSS mit REPO-AUDIT-001 §25 konsistent bleiben.
- **REQ-RB-007** (§6): AUD-Record + maschinenlesbarer Health Report sind
  Pflichtprodukte jedes Audits.
- **REQ-RB-008** (§7): AUTO-Checks SOLLEN in CI laufen; fehlende CI-Integration
  ist ein Finding.
- **REQ-RB-009** (§8): Agenten-Audits sind vorläufig; APPROVAL/RELEASE READY nur
  mit Owner-Human-Gate.
- **REQ-RB-010** (§4): SKIP-Handling MUSS dokumentiert sein (Bereich raus aus
  Gewichtung nur mit Begründung).
- **REQ-RB-011** (§9): Katalog-Änderungen nur via SCR; Versionsbindung an
  VERSION-001.
- **REQ-RB-012** (§10): REPO-AUDIT-003 (Auditor-Agent) ist GEPLANT; bis dahin
  gilt der Katalog für halbautomatische Ausführung verbindlich.

## Security Considerations

Health Reports DÜRFEN keine Secrets im Klartext enthalten (REPO-AUDIT-001 §28).
Evidence-Referenzen auf private Artefakte MÜSSEN signiert/berufsintern bleiben.
Scores sind Steuerungsinformation: Ein manipulierter Score könnte falsche
Release-Entscheidungen auslösen — daher AUD-Record-Pflicht, Human Gate und
S-22-Integritätsprüfung.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.0.0** (2026-09-07): Initial Release — Ausarbeitung der „nächsten Ebene"
  aus REPO-AUDIT-001 §28 (Owner-Richtung 23:54): 64 Checks (CHECK-001..064, je 4
  je Prüfbereich), Gewichte (Bereichssumme 100, Checkgewichte 1–3), Scoring-Formel,
  A–E-Mapping mit P0→E-Regel, Health-Report-Format (AUD-Record-Kopplung),
  Human-Gate-Regel, Katalog-Governance via SCR, S-22-Integritätsprüfung;
  REPO-AUDIT-003 GEPLANT. SCR-0021; §9-Freigabe Michael Wroblewski 08.09.2026, 00:05 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- ATC-STD-REPO-AUDIT-001 (Prozess, §25 A-E, §22 P0-P4, §28)
- registry/repo-audit-checks.yaml (CHECK-Katalog, SSOT) + tools/repo-audit/gen_checks.py
- ATC-STD-AUDIT-001 (AUD-Records), registry/findings.yaml (F-NNN)
- ATC-STD-BUG-005 (RCA), ATC-STD-COMPAT-001 (Kompatibilitäts-Status)
- ATC-STD-AI-DECISION-001 + MILESTONE-001 §13 (Human Gates)
- ATC-STD-FRAMEWORK-001 (FAM-41), ATC-STD-VERSION-001

*ATC-STD-REPO-AUDIT-002 v1.0.0 · Owner-Richtung Michael Wroblewski · Aurora (Superagent) · 07.09.2026 · SCR-0021*
