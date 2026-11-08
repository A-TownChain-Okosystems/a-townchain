---
standard:
  id: ATC-STD-REPO-MAINT-001
  title: "Repository Maintenance & Lifecycle Standard"
  version: "1.0.0"
  status: approved
  category: repo-maint
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Repositories (26 aktiv, A-TownChain-Okosystems)"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-REPO-MAINT-001 — Repository Maintenance & Lifecycle (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0043). Master-Standard der Familie Repository Maintenance (FAM-46,
> Range ATC-STD-REPO-MAINT-001..999). Bis zur §9-Freigabe gilt dieser Standard
> als CANDIDATE-normativ; Abweichungen sind Findings nach ATC-STD-BUG-001.

---

## Abstract

Dieser Standard definiert den **verbindlichen Repository-Pflegezyklus** für alle
ATC-Repositories. Repository-Pflege ist kein ad-hoc-Aufräumen, sondern ein
kontrollierter Unternehmensprozess: **Alles, was aktualisiert werden kann, wird
geprüft, bewertet und – sofern kompatibel und freigegeben – auf einen aktuellen
unterstützten Stand gebracht.** Kein Update erfolgt blind; jede Änderung wird auf
Kompatibilität, Sicherheit und Auswirkungen geprüft.

## Scope

Gilt für alle 26 aktiven Repositories der Organisation sowie alle künftigen.
Verwandte Standards werden referenziert, nicht dupliziert:
ATC-STD-201..204 (Struktur, Naming, Security/Release, Dependencies),
ATC-STD-REPO-AUDIT-001/002 (Audit-Checks, Health Score),
ATC-STD-VERSION-001, ATC-STD-UPDATE-001, ATC-STD-BUG-001..004, ATC-STD-V2S-000.

---

## §1 Zweck und Zielzustand

Jedes Repository MUSS regelmäßig auf einen aktuellen, konsistenten, sicheren und
wartbaren Zustand gebracht werden (REQ-RM-001). Zielzustand:

> Alles, was aktualisiert werden kann, wird geprüft, bewertet und – sofern
> kompatibel und freigegeben – auf einen aktuellen unterstützten Stand gebracht.

## §2 Pflegebereiche (Pflicht-Checkliste)

Jeder Pflegezyklus MUSS mindestens diese Bereiche prüfen (REQ-RM-002; automatisierbare
Bereiche via ATC-STD-REPO-AUDIT-002 CHECK-Katalog, registry/repo-audit-checks.yaml):

| Bereich | Prüfung | | Bereich | Prüfung |
|---|---|---|---|---|
| Source Code | aktueller Stand | | Configuration | aktuell |
| Dependencies | Updates | | Docker | aktuelle Images |
| Runtime | unterstützte Version | | GitHub Actions | aktuelle Actions |
| Compiler/SDK | Update | | IaC | Terraform/Kubernetes etc. |
| Build-System | Update | | Package Manager | Lockfiles aktuell |
| Tests | funktionsfähig | | Versionierung | konsistent |
| CI/CD | aktuell | | Repository Metadata | aktuell |
| Security | Vulnerability Scan | | Branches | bereinigt |
| Dokumentation | aktuell | | Tags/Releases | konsistent |
| README | aktuell | | Deprecated Code | identifiziert |
| CHANGELOG | vollständig | | TODO/FIXME | bewertet |
| LICENSE | korrekt | | Secrets | geprüft |
| Standards | aktuell | | Artefakte | bereinigt |
| API | Kompatibilität | | | |

## §3 Maintenance Lifecycle (16 Stationen, verbindlich)

```
REPOSITORY DISCOVERY → BASELINE AUDIT → VERSION INVENTORY → UPDATE ANALYSIS →
SECURITY ANALYSIS → COMPATIBILITY ANALYSIS → UPDATE PLAN → UPDATE → BUILD →
TEST → SECURITY SCAN → DOCUMENTATION UPDATE → CHANGELOG → VERSION CHECK →
FINAL AUDIT → MAINTENANCE COMPLETE
```
Jede Station MUSS durchlaufen oder bewusst mit Begründung SKIPPED werden (Skip-Doku
im Maintenance Report, REQ-RM-001).

## §4 P0–P3 Priorisierung

Priorisierung folgt dem Blockierungsmodell von ATC-STD-V2S-000 §10 und der
Severity-Zuordnung nach ATC-STD-BUG-001 (S0–S4 → P0–P3) (REQ-RM-003):

- **P0 — Kritisch (sofort):** bekannte kritische Sicherheitslücke, kompromittierte
  Dependency, exponiertes Secret, vollständig kaputter Build, Datenverlust-Risiko,
  kritische inkompatible Runtime, unsicheres Release.
- **P1 — Hoch (kurzfristig):** wichtige Security-Updates, veraltete
  Kern-Dependencies, nicht unterstützte Runtime, kaputte CI, fehlende Tests,
  API-Inkompatibilität, veraltete GitHub Actions, veraltete Docker Images.
- **P2 — Mittel (regulärer Zyklus):** kleinere Dependency-Updates,
  Dokumentationslücken, technische Schulden, veraltete Konfiguration, fehlende
  Changelog-Einträge, ungenutzter Code.
- **P3 — Niedrig (langfristig):** Refactoring, kosmetische Doku-Verbesserungen,
  kleine Automatisierungen, Developer Experience.

**P0-Regel:** Es DARF kein Maintenance-Abschluss erklärt werden, solange
ungeklärte P0-Findings existieren (→ §17 DoD).

## §5 Update-Regeln (SemVer)

Updates DÜRFEN nicht blind übernommen werden (REQ-RM-004):

- **Patch** (1.2.3→1.2.4): normalerweise automatisch bzw. geringe Prüfung.
- **Minor** (1.2.3→1.3.0): Kompatibilität prüfen.
- **Major** (1.2.3→2.0.0): explizite Kompatibilitätsprüfung + verbindliche Kette:
  `OLD VERSION → BREAKING-CHANGE ANALYSIS → MIGRATION PLAN → UPDATE →
  COMPATIBILITY TEST → REGRESSION TEST → DOCUMENTATION` (REQ-RM-018).

## §6 Dependency Maintenance

Jedes Repository MUSS je Dependency erkennen können (REQ-RM-005):
Current Version, Latest Stable, Supported, Security Status, License,
Breaking Changes, Update Recommendation (Schema-Beispiel im Anhang des
Maintenance Reports). Automatisierung: Dependabot (Ziel 26/26, ATC-STD-204).

## §7 Dependency Update Policy (REQ-RM-006)

- **UPDATE:** wenn Security verbessert, kompatibel, unterstützt, Tests erfolgreich.
- **REVIEW:** wenn Major Release, API verändert, Runtime verändert, Build-System betroffen.
- **HOLD:** wenn inkompatibel, bekannte Regression, ATC-System von alter Version
  abhängig, Migration nicht vorbereitet.
- **REMOVE:** wenn ungenutzt, deprecated, unsicher, besser ersetzt.

## §8 Source-Code-Pflege

Jeder Zyklus identifiziert TODO, FIXME, HACK, XXX, DEPRECATED, UNUSED, DEAD CODE,
DUPLICATED CODE. Funde werden NICHT automatisch gelöscht; jeder erhält einen
Status: OPEN, PLANNED, IN_PROGRESS, RESOLVED, WONT_FIX, ACCEPTED_RISK (REQ-RM-007).

## §9 Dokumentationspflege

Prüfpflicht (soweit für das Repository erforderlich): README.md, CHANGELOG.md,
LICENSE, CONTRIBUTING.md, SECURITY.md, CODE_OF_CONDUCT.md, ARCHITECTURE.md,
ROADMAP.md, STATUS.md. Das README MUSS den tatsächlichen Zustand widerspiegeln
(REQ-RM-008; ATC-STD-README-001, ATC-STD-MD-001).

## §10 Standards-Pflege

Ein Repository DARF keine veralteten ATC-Standards verwenden. Prüfkette:
Repository → Standards Registry (SSOT standards.yaml) → genutzte
Standard-Version → aktuelle Version → Compatibility → Migration nötig?
(REQ-RM-009). Abweichung = Finding nach ATC-STD-BUG-001.

## §11 CI/CD Maintenance

Jede Pipeline MUSS ihrem Zweck entsprechend BUILD, TEST, SECURITY, LINT, PACKAGE,
RELEASE abdecken; Runner-, Action- und Skript-Versionen sind je Zyklus zu prüfen
(REQ-RM-010). Referenz: .github/workflows/ci.yml, security.yml, release.yml
(nach ATC-STD-201).

## §12 Security Maintenance

Jeder Zyklus enthält: Dependency Vulnerability Scan, Secret Scan, License Scan,
Container Scan, Code Security Scan, Configuration Scan (REQ-RM-011). Kritische
Findings werden nach der verbindlichen Kette behandelt:
`Finding → Severity → Affected Component → Risk → Remediation → Verification`
(ATC-STD-BUG-001..004, ATC-STD-203).

## §13 Build & Test Gate

Ein Update gilt NICHT als abgeschlossen, bis validiert wurde (REQ-RM-012).
Minimal: Install, Build, Unit Tests, Integration Tests, Lint, Type Check,
Security Check. Je Repository zusätzlich: E2E, Contract Tests, Performance,
Smart Contract Tests, Network Tests, Migration Tests.

## §14 Git Hygiene

Prüfen: Branches, Tags, Releases, Merge-Konflikte, uncommittete Dateien,
generierte Dateien, Large/Binary Files, versehentliche Secrets, stale Branches
(REQ-RM-013). Ein Repository DARF niemals eingecheckt enthalten: .env,
Credentials, Private Keys, API-Keys, Wallet-Keys, Passwort-Dateien,
Build-Artefakte. Verstöße = P0 (Sicherheitsregel: Secrets nur als Env-Variablen).

## §15 Version Consistency

Versionen MÜSSEN konsistent sein (REQ-RM-014, ATC-STD-VERSION-001):
package.json = Cargo.toml = pyproject.toml = VERSION = README = CHANGELOG =
Git Tag = Release = Docker Image. Zielzustand: `Tag = Manifest = CHANGELOG =
Release` (AUD-2026-0002/F-026, Issue #96).

## §16 Maintenance Report

Nach jedem vollständigen Durchlauf MUSS ein Report erzeugt werden (REQ-RM-015)
mit: status (PASS/FAIL), updates (dependencies/security/actions/docs),
findings (p0..p3), tests (build/unit/integration/security), documentation,
compatibility — als maschinenlesbarer YAML-/Markdown-Record (Kopplung an
AUD-Records nach ATC-STD-REPO-AUDIT-002).

## §17 Definition of Done

Repository Maintenance ist erst abgeschlossen, wenn (REQ-RM-016):
Repository analysiert, Dependencies geprüft, Security geprüft, Runtime geprüft,
Build geprüft, Tests durchgeführt, CI/CD geprüft, Dokumentation geprüft,
Standards geprüft, Versionen synchronisiert, CHANGELOG aktualisiert,
TODO/FIXME bewertet, Git-Hygiene geprüft, Final Audit durchgeführt,
**keine ungeklärten P0 vorhanden**, Maintenance Report erstellt.

## §18 Maintenance Status (Labels je Repository)

CURRENT (vollständig aktuell), MAINTENANCE_REQUIRED, OUTDATED,
SECURITY_REQUIRED, MIGRATION_REQUIRED, BLOCKED, AUDIT_REQUIRED,
COMPLIANT (ATC-Standards erfüllt) (REQ-RM-017). Labels pflegt der
Maintenance Report bzw. künftig der Maintenance Agent.

## §19 Automatisierung (Maintenance Agent)

Langfrist-Ziel: ATC Repository Maintenance Agent (Analog REPO-AUDIT-003
Auditor-Agent, geplant): Repository Scanner + Standards Registry →
Dependency/Version Scanner → Risk Analyzer → Update Planner →
Update Executor → Test/Security → Compliance Audit → Maintenance Report.
Der Agent DARF automatisch analysieren; kritische Änderungen MÜSSEN über
definierte Approval Gates (Human-Gate nach ATC-STD-REPO-AUDIT-002) laufen
(REQ-RM-019).

## §20 Lebenszyklus-Grundsatz & Familienabgrenzung

**Kernprinzip: Ein ATC-Repository gilt niemals als „fertig".** Es befindet sich
permanent im Lifecycle:
`CREATE → DEVELOP → AUDIT → UPDATE → TEST → RELEASE → MONITOR → MAINTAIN → AUDIT ↺`
(REQ-RM-020; Integration mit dem 20-Stationen-Lifecycle ATC-STD-V2S-000:
V2S regiert Entwicklung bis EOL, REPO-MAINT-001 regiert den operativen
Erhaltungszustand).

**Familie:** REPO-MAINT-001 ist der Master der Familie FAM-46. Die im
Owner-Entwurf skizzierte 20-Standard-Familie (REPO-001..020) wird NICHT
dupliziert — Struktur, Naming, Security, Dependencies, Versionierung, Releases,
Audits sind durch ATC-STD-201..204, VERSION-001, UPDATE-001, REPO-AUDIT-001/002
normativ abgedeckt und werden hier verbindlich referenziert. Folge-Slots
(REPO-MAINT-002 ff., z.B. Maintenance Agent) nach Bedarf über SCR.

---

## REQ-Matrix (normativ)

| REQ | Abschnitt | Verpflichtung (MUST) |
|---|---|---|
| REQ-RM-001 | §1, §3 | Verbindlicher 16-Stationen-Pflegezyklus je Repository; Skip nur mit Begründung |
| REQ-RM-002 | §2 | 29 Pflegebereiche je Zyklus prüfen (Checkliste, REPO-AUDIT-002-Kopplung) |
| REQ-RM-003 | §4 | P0–P3-Priorisierung mit BUG-001-/V2S-000-Mapping; keine P0 am Zyklus-Ende |
| REQ-RM-004 | §5 | SemVer-Update-Regeln; kein blindes Übernehmen |
| REQ-RM-005 | §6 | Dependency-Inventar (current/latest/supported/security/license/breaking/empfehlung) |
| REQ-RM-006 | §7 | Update-Policy UPDATE/REVIEW/HOLD/REMOVE |
| REQ-RM-007 | §8 | TODO/FIXME-Statusmodell; kein Auto-Löschen |
| REQ-RM-008 | §9 | Dokumentations-Pflichtdateien; README spiegelt Ist-Zustand |
| REQ-RM-009 | §10 | Keine veralteten Standard-Versionen; Registry-Abgleich |
| REQ-RM-010 | §11 | Pipeline-Abdeckung BUILD/TEST/SECURITY/LINT/PACKAGE/RELEASE |
| REQ-RM-011 | §12 | 6 Security-Scans je Zyklus + Finding-Behandlungskette |
| REQ-RM-012 | §13 | Build & Test Gate als Abgeschlossenheits-Bedingung |
| REQ-RM-013 | §14 | Git Hygiene; Secrets-Verbot (P0) |
| REQ-RM-014 | §15 | Version-Konsistenz Tag=Manifest=CHANGELOG=Release |
| REQ-RM-015 | §16 | Maintenance Report je Durchlauf |
| REQ-RM-016 | §17 | DoD 16 Punkte, inkl. P0-frei |
| REQ-RM-017 | §18 | Standardisiertes Status-Label je Repository |
| REQ-RM-018 | §5 | Major-Update-Kette mit Breaking-Analysis + Migrationsplan |
| REQ-RM-019 | §19 | Automatisierung nur mit Approval Gates für kritische Änderungen |
| REQ-RM-020 | §20 | Permanenter Repository-Lifecycle; Integration mit V2S-000 |

## Implementierungsstatus (Pivot-Doktrin ATC-ORG-AUDIT-002)

| Zustand | Wert |
|---|---|
| Standard-Status | **SPECIFIED** (v1.0.0 CANDIDATE; Implementierung beginnt nach §9) |
| Implementierung | geplant: Maintenance-Report-Schema + REPO-AUDIT-Check-Kopplung |
| CI-Gate | geplant: Validator-Erweiterung (Maintenance-Report-Pflicht je Zyklus) |
| Evidence | geplant: Wartungsberichte je Repository (docs/ des Docs-Hubs) |
| Coverage-KPIs | Implementation/Enforcement/Evidence Coverage (Re-Audit §15) — Messung ab erstem Zyklus |

## Compliance

Geprüft per atc-std-validator (S-01..S-25), Taxonomie- und Registry-Sync;
Verstöße gegen MUST-Kriterien = Finding (F-NNN) nach ATC-STD-BUG-001..004.
Governance-Kernregel: Registry + Repository schlagen README, Wiki und Chat
(DE/EN: Registry and repository outrank any README, wiki or chat.)

## Security Considerations

Dieser Standard verpflichtet Secret-Scans und Git-Hygiene je Zyklus (§12, §14).
Secrets, Keys und Credentials DÜRFEN niemals im Klartext eingecheckt oder in
Reports/Logs ausgegeben werden — nur `$ENV_VAR`-Platzhalter (Sicherheitsregel
AD-Registry). Maintenance-Updates an sicherheitskritischen Repos (S4-Tier:
atc-node, atc-vm, atc-algorithm, atc-zkp, atc-contracts, atc-shivacore) erfordern
Security-Scan vor Abschluss (G18-Analogon, ATC-STD-203).

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — Owner-Entwurf kanonisiert, Familie FAM-46 errichtet (SCR-0043) |

## References

**NORMATIVE:** ATC-STD-000 (§33/§37), ATC-STD-201..204, ATC-STD-BUG-001..004,
ATC-STD-REPO-AUDIT-001/002, ATC-STD-VERSION-001, ATC-STD-UPDATE-001,
ATC-STD-V2S-000, ATC-STD-README-001, ATC-STD-MD-001 · **INFORMATIVE:**
SCR-0043, Re-Audit ATC-ORG-AUDIT-002 (08.09.), registry/framework.yaml (FAM-46)
