---
standard:
  id: ATC-STD-ERR-000
  title: "No Local Fix Without System Verification (Error Master)"
  version: "1.0.0"
  status: approved
  category: err
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Gesamtes A-TownChain-Oekosystem: Repositories, Software, Smart Contracts, Doku, Standards, APIs, KI-Agenten, Infrastruktur, Prozesse"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-ERR-000 — No Local Fix Without System Verification (Error Master) (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

---

## Zweck

Enterprise-Grundregel des A-TownChain-Oekosystems: **Jeder relevante Fehler wird als
potenziell systemweit auftretender Fehler behandelt, bis durch einen dokumentierten Scan
nachgewiesen wurde, dass er lokal begrenzt ist.** Ein gefundener Fehler wird niemals nur
an der Fundstelle behoben. Aus einem einzelnen Bug wird ein systematischer
Verbesserungsprozess.

## 1. Grundprinzip — 8-Punkte-Pruefliste (je relevantem Fehler MUSS geprueft werden)

1. Wo wurde der Fehler gefunden? 2. Warum ist er entstanden? 3. Welche anderen Stellen
koennen denselben Fehler enthalten? 4. Welche Systeme sind davon abhaengig? 5. Ist die
Dokumentation ebenfalls betroffen? 6. Existieren aehnliche Fehler in anderen Repositories?
7. Kann derselbe Fehler durch einen automatisierten Test erkannt werden? 8. Welche
organisatorische oder technische Massnahme verhindert eine Wiederholung?

## 2. Fehlerlebenszyklus (14 Stationen, verbindlich)

```
DETECTED → CLASSIFIED → REPRODUCED → ROOT_CAUSE_IDENTIFIED → PROPAGATION_SCAN →
IMPACT_ANALYSIS → FIX_PLANNED → FIX_IMPLEMENTED → REGRESSION_TESTED →
PREVENTION_IMPLEMENTED → DOCUMENTATION_UPDATED → CROSS_REPOSITORY_RESCAN →
VERIFIED → CLOSED
```

Ein Fehler DARF NICHT direkt von DETECTED nach CLOSED springen (REQ-ER-003).
Jede Station wird von einem Kernstandard der Familie getragen (s. References).

## 3. P0/P1/P2/P3-Pflichtmatrix

- **P0 — systemkritisch (Pflicht):** Root-Cause-Analyse (ERR-003/BUG-005),
  Cross-Repository-Scan (ERR-005), Dependency-Impact (ERR-007), Regressionstest
  (ERR-008), Preventive Control (ERR-009), Documentation-Audit (ERR-006),
  Post-Fix-Scan (ERR-012), Abschluss-Audit.
- **P1 — hoch (Pflicht):** Root Cause, Propagation-Scan (ERR-004), Regressionstest,
  Dokumentationspruefung, Post-Fix-Verifikation.
- **P2 — mittel (Pflicht):** Fehlerklassifikation (ERR-002), Ursache, Fix,
  geeigneter Test.
- **P3 — niedrig (Pflicht):** dokumentieren, bewerten, bei Bedarf beheben.

## 4. KI-Agenten-Regel (hart)

> Ein Agent DARF einen Fehler nicht als lokal betrachten, bevor ein Propagation-Scan
> (ERR-004) durchgefuehrt wurde.

Nach jedem Fix MUSS der Agent die 10 Fragen beantworten: (1) Was ist der Fehler?
(2) Was ist das Fehlermuster? (3) Warum ist er passiert? (4) Wo koennte dasselbe
Muster erneut auftreten? (5) Welche Repositories sind betroffen? (6) Welche Doku ist
betroffen? (7) Welche Abhaengigkeiten sind betroffen? (8) Welcher Regressionstest
beweist die Korrektur? (9) Welche Praevention verhindert die Wiederholung?
(10) Wurde nach dem Fix erneut global gesucht?

## 5. Fehlerklasse statt Einzelfehler

Langfrist fuehrt ATC eine **Error Pattern Library** (ERR-010): jeder neue Fehler wird
automatisch mit bekannten Fehlerklassen (ATC-ERR-PATTERN-001..NNN) verglichen.
Jeder wichtige Fehler erhaelt einen dauerhaften **Error Knowledge Record**
(ERR-013, error_id ATC-ERR-NNNN).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-001 | No-Local-Fix: relevanter Fehler gilt als systemweit bis dokumentierter Scan das Gegenteil belegt |
| REQ-ER-002 | 8-Punkte-Pruefliste je relevantem Fehler vollstaendig durchlaufen |
| REQ-ER-003 | 14-Stationen-Lifecycle; kein Sprung DETECTED→CLOSED; Skip nur mit Begruendung |
| REQ-ER-004 | P0/P1-Pflichtsets sind zwingend; P2/P3 Minimum je Matrix |
| REQ-ER-005 | Agenten-Regel: kein Fix-Abschluss vor Propagation-Scan (10-Fragen-Nachweis) |
| REQ-ER-006 | Knowledge Record je wichtigem Fehler (ATC-ERR-NNNN, ERR-013-Format) |
| REQ-ER-007 | Fehlermuster extrahieren und gegen Pattern-Library matchen |
| REQ-ER-008 | UNKNOWN-Status ist kein OK; Follow-up-Pflicht (ERR-007) |
| REQ-ER-009 | Fuer kritische Fehlerklassen: Prevention Gate (ERR-015) errichten |

## Implementierungsstatus (Pivot-Doktrin ATC-ORG-AUDIT-002)

| Zustand | Wert |
|---|---|
| Standard-Status | **SPECIFIED** (v1.0.0 CANDIDATE; Umsetzung nach §9) |
| Implementierung | geplant: Error-Knowledge-Records je Fix; Propagation-Scan-Doku in Bug-Records (BUG-002) |
| CI-Gate | geplant: Validator-Erweiterung — Fix ohne Scan-Nachweis = Finding |
| Evidence | geplant: docs/error-records/ im Docs-Hub; Kopplung an BUG-005-Closure-Gate |
| Coverage-KPIs | Implementation/Enforcement/Evidence Coverage ab erstem Fehlerfall |

## Compliance

Geprueft per atc-std-validator (S-01..S-25) und BUG-005-Closure-Gate; Verstoss gegen
REQ-ER-001..009 = Finding (F-NNN) nach ATC-STD-BUG-001..005.
Governance-Kernregel: Registry + Repository schlagen README, Wiki und Chat.

## Security Considerations

Error-Records duerfen keine Klartext-Zugangsdaten enthalten (nur $ENV-Platzhalter).
Security-Fehler (Fehlerklasse Security Configuration, BUG-005-Klasse 18) erfordern
Security-Scan im Post-Fix-Audit (ERR-012) — Verstoss = P0.

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — Owner-Entwurf kanonisiert, FAM-47 errichtet (SCR-0044) |

## References

**NORMATIV:** ATC-STD-000 (§33/§37), ATC-STD-BUG-001..005, ATC-STD-REPO-AUDIT-002,
ATC-STD-REPO-MAINT-001 (§12/§13), ATC-STD-ERR-001..015 · **INFORMATIVE:** SCR-0044,
SCR-0011/0012 (Historie: erster ERR-001-Entwurf als BUG-005 integriert),
registry/framework.yaml (FAM-47)
