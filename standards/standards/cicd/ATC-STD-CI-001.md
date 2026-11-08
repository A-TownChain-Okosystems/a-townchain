---
standard:
  id: ATC-STD-CI-001
  title: "Reproducible CI Dependencies"
  version: "1.0.0"
  status: approved
  category: cicd
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle CI/CD-Workflows und Validatoren im ATC-Oekosystem (26 Repositories): Dependency-Deklaration, Installation, Reproduzierbarkeit, Fresh-Runner-Faehigkeit, Fehlerklassifikation, Regression, Propagation"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-CI-001 — Reproducible CI Dependencies (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0054, Issue #1 P1-Change). Familie FAM-50 (CI/CD-Standards, Range ATC-STD-CI-001..999).
> **Ausloeser:** AUD-2026-0003 F-035/F-037 (PyYAML auf Runner fehlte, CI lief im
> ungeprueften Fallback gruen). **Bezug:** ATC-STD-ERR-000/004/005/008/009.

---

## Zweck

CI-Erfolg darf nie vom zufaelligen Zustand eines Runners abhaengen. Jede
externe Dependency eines Workflows ist deklarativ erfasst, wird vor Nutzung
installiert, versionseingegrenzt und auf einem frischen Runner geprueft.
Dependency-Fehler werden eindeutig klassifiziert und systematisch
weiterverfolgt — ein gefundener Fehler wird nie nur an der Fundstelle behoben
(ATC-STD-ERR-000).

## Kernprinzip (P0)

> CI darf nicht erfolgreich sein, weil Runner-Zustaende zufaellig guenstig sind.
> Ein gepruefter Fallback (Degradation mit WARN) ist fuer Dependency-Fehler
> unzulaessig — fehlende Dependency = FAIL mit Klassifikation.

## Anforderungsmatrix

| REQ-ID | Anforderung | Prioritaet |
|---|---|---|
| REQ-CI-001 | Jede externe Dependency muss deklarativ erfasst sein (requirements.txt) | P1 |
| REQ-CI-002 | CI darf keine vorinstallierten Pakete voraussetzen | P1 |
| REQ-CI-003 | Dependencies muessen vor ihrer Nutzung installiert werden (pip install -r requirements.txt) | P1 |
| REQ-CI-004 | Dependency-Versionen muessen reproduzierbar sein (Schranken/Pins) | P1 |
| REQ-CI-005 | requirements.txt-Aenderungen muessen relevante CI ausloesen | P1 |
| REQ-CI-006 | Jeder Validator muss auf einem frischen Runner funktionieren | P1 |
| REQ-CI-007 | Dependency-Fehler muessen eindeutig klassifiziert werden | P1 |
| REQ-CI-008 | Nach einem Dependency-Fehler ist eine Repository-weite Suche erforderlich | P1 |
| REQ-CI-009 | Ein behobener Fehler erhaelt einen Regressionstest (vor Fix rot, nach Fix gruen) | P1 |
| REQ-CI-010 | CI darf nicht erfolgreich sein, weil Runner-Zustaende zufaellig guenstig sind | P0 |

## Dependency-Fehlerklassifikation (REQ-CI-007)

| Klasse | Bedeutung | Behandlung |
|---|---|---|
| DEPENDENCY_MISSING | Package fehlt im Runner und ist nicht deklariert/installiert | FAIL — REQ-CI-001..003 |
| VERSION_CONFLICT | Installierte Version ausserhalb der Schranken | FAIL — REQ-CI-004 |
| INSTALL_FAILURE | Installation schlaegt fehl (Netz/Registry) | FAIL — Workflow-Retry |
| RUNTIME_IMPORT_ERROR | Import-Fehler zur Laufzeit | FAIL — Root-Cause + REQ-CI-008-Suche |

## Fehlerweiterverfolgung (REQ-CI-008, Lifecycle)

Fehler entdeckt -> klassifizieren -> Root Cause bestimmen -> alle aehnlichen
Stellen suchen (ERR-004 Propagation Scan) -> alle betroffenen Workflows pruefen
(ERR-005 Cross-Repository Scan) -> Fix implementieren -> Regressionstest
hinzufuegen (CI-009) -> Fresh-Runner-Test -> Standards aktualisieren ->
Audit erneut ausfuehren -> Issue schliessen (verifiziert + verhindert, nicht
nur gefixt).

## Erzwingung

- **E-5-Stage** in validate_all.py (SCR-0053/0054): Regressionstest
  test_ci_dependency_governance.py laeuft bei jedem Push/PR — prueft
  Deklaration (T1), Workflow-Installation (T2), Klassifikation (T3),
  Versionsschranken (T4). Laeuft selbst stdlib-only (CI-006).
- **S-25** (Frontmatter-YAML): fehlende PyYAML = FAIL (DEPENDENCY_MISSING),
  kein Fallback mehr (vorher: WARN — CI-010-Verstoss, Issue #1).
- **Fresh-Runner-Test**: GitHub-Hosted-Runner sind prinzipbedingt frisch —
  jeder CI-Lauf IST der Fresh-Runner-Test, sobald REQ-CI-003 erfuellt ist.

## Issue #1 — Acceptance Criteria

- [ ] naming-governance.yml installiert requirements.txt vor Python-Validatoren.
- [ ] Workflow funktioniert auf einem frischen GitHub Runner.
- [ ] Alle anderen .github/workflows/*.yml wurden auf fehlende Dependency-Installation geprueft.
- [ ] requirements.txt ist versioniert bzw. reproduzierbar spezifiziert.
- [ ] Nach erfolgreicher CI und Regressionstest wird Issue #1 geschlossen.

## Abgrenzung

ATC-STD-BUG-003/005 decken den Fehler-PROZESS ab; ATC-STD-ERR-000..015 die
systemische PROPAGATION; dieser Standard regelt die CI-DEPENDENCY-Klasse
konkret. Update-Kanal fuer Dependencies: ATC-STD-UPDATE-Familie (Dependabot,
AUD-2026-0002 F-025). Beide Workflows (naming-governance.yml, ci.yml) sind
pflicht, `pip install -r requirements.txt` zu verwenden — ad-hoc-Installation
einzelner Pakete (z. B. pip install pyyaml) ist unzulaessig (REQ-CI-001/003).

## Implementierungsstatus

SPECIFIED — Mechanismen teils live: E-5-Regressionstest + T1-T4 (erzwungen je
Push/PR), S-25-Striktheit, requirements.txt mit Schranken. Offen (Owner-Aktion
GH013/F-010, Issue #1): Dependency-Installation in naming-governance.yml und
Umstellung ci.yml auf requirements.txt. ENFORCED-Sprung erst nach CI-gruen auf
frischem Runner mit aktiven Gates (DoD: alle 5 Acceptance Criteria).

## Security Considerations

Dependency-Installation aus unspezifizierten Quellen ist unzulaessig — nur
PyPI/requirements.txt (REQ-CI-001). Versionsschranken reduzieren
Supply-Chain-Risiken (REF: ATC-STD-204 Supply-Chain, ERR-PATTERN-008). Keine
Geheimnisse in Workflows; Dependency-Fehler-Meldungen enthalten niemals
Zugangsdaten (ATC-AAS-001).

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung (Owner-Entwurf, Issue #1 P1-Change, SCR-0054): REQ-CI-001..010, Klassifikation, Lifecycle, Erzwingung via E-5/S-25 |

## References

- Issue #1 (atc-standards): naming-governance.yml-Haertung — P1-Change mit dauerhafter Fehlerpraevention
- AUD-2026-0003 F-035/F-037 — undeklarierte CI-Dependency, ungepruefter Fallback
- ATC-STD-ERR-004/005/008/009 — Propagation, Cross-Repo-Scan, Regression, Praevention
- ATC-STD-BUG-003/005 — Fix-Lifecycle, Root-Cause-Analyse
- ATC-STD-REPO-MAINT-001 (FAM-46) — Pflegezyklus CI/CD, REQ-RM-0xx
- ATC-STD-UPDATE-Familie — Dependency-Update-Kanal (Dependabot)
- SCR-0026, SCR-0053, SCR-0054; Commit 8406d69 (S-20-Fallback-Fix, requirements.txt)
