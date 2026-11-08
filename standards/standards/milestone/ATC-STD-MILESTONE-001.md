---
standard:
  id: ATC-STD-MILESTONE-001
  title: "ATC Milestone Standard — Verbindliche Meilenstein-Governance: Zustandsnachweis, Lebenszyklus, Acceptance Gates, Evidence Packs, maschinenlesbare Registry"
  version: "1.0.0"
  status: approved
  category: milestone
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  effective_date: "2026-09-07"
  review_date: "2027-09-07"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 07.09.2026, 23:38 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-VERSION-001
    - ATC-STD-UPDATE-001
    - ATC-STD-COMPAT-001
    - ATC-STD-AUDIT-001
  related_standards:
    - ATC-STD-AI-DECISION-001
    - ATC-STD-BUG-005
    - ATC-STD-DESC-001
    - ATC-STD-202
    - ATC-STD-MD-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-MILESTONE-001 — ATC Milestone Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 07.09.2026, 23:38 UTC+2);
> normativ in Kraft ab 07.09.2026, §30-eingefroren (ATC-STD-000). Harmonisierung SCR-0018 akzeptiert.
> **Familie:** Milestone Standards (ATC-STD-MILESTONE-001..999) — Kategorie `milestone`, SCR-0018.
> **Rolle in der Governance-Kette:** Roadmap → **MILESTONE-001** → Sprints/Issues/Tasks → Implementation → Tests → Audit → Acceptance.
> **Referenzen:** ATC-STD-000 (Verfassung), VERSION-001 (Releases), UPDATE-001 (Change Control), COMPAT-001 (MAJOR-Gate), AUDIT-001 (Audit-Layer), AI-DECISION-001 (Human Gates), AD-027 (M1-M8-Roadmap).

## Abstract

ATC-STD-MILESTONE-001 definiert verbindliche Regeln für Planung, Definition, Verfolgung,
Nachweis und Abschluss von Meilensteinen im gesamten A-TownChain-Ökosystem. Ein
Meilenstein ist ein **nachweisbar erreichter Systemzustand** — kein Datum, kein Sprint,
keine TODO-Liste, kein Release, keine einzelne Funktion. Abschluss MUSS nur bei
vollständigem, überprüfbarem Nachweis (Acceptance Criteria, Tests, Audit, Evidence)
erfolgen. Der Standard verbindet Roadmap, Sprints, Releases, Audits und KI-Agenten-
Steuerung zu einem durchgängigen Kontrollpunkt zwischen Planung und funktionsfähigem
System. Maschinenlesbarkeit (JSON-Schema + `registry/milestones.yaml` + Validator)
ist Bestandteil des Standards.

Schlüsselwörter zur Interpretation: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE
(empfohlen), DARF/KANN (optional) — gemäß RFC-2119-Deklaration ATC-STD-000 §10.

## Scope

**Gilt:** Alle ATC-Projekte, Repositories, Protokolle, Software, Smart Contracts,
KI-Agenten und Infrastruktur der Organisation A-TownChain-Okosystems. Verbindliche
Meilensteine (Roadmap-, Release-, Programm-Meilensteine) MÜSSEN diesem Standard folgen.

**Gilt nicht:** Informelle Arbeitspakete, Sprints als solche (Zeitzyklen, geregelt
eigenständig), einzelne Issues/Tasks (Run-Level unterhalb des Meilensteins), reine
Dokumentations- oder Vision-Artefakte ohne Systemzustandsbezug (ATC-41+-Einordnung).

## §1 Zweck — Meilenstein als Systemzustand

Ein Meilenstein definiert einen nachweisbar erreichten Entwicklungszustand. Er ist
NICHT: ein Kalenderdatum, ein Sprint, eine TODO-Liste, ein Release oder eine einzelne
Funktion. Ein Meilenstein gilt als erreicht, wenn die definierten Akzeptanz-, Qualitäts-,
Sicherheits-, Dokumentations- und Kompatibilitätskriterien erfüllt UND verifiziert
wurden (REQ-MS-001).

## §2 Meilenstein-Lebenszyklus

Jeder verbindliche Meilenstein DURCHLÄUFT verpflichtend:

```
PLANNED → DEFINED → IN_PROGRESS → FEATURE_COMPLETE → VALIDATION → AUDIT →
ACCEPTED → RELEASED → VERIFIED → CLOSED
```

Fehlerpfad: `VALIDATION/AUDIT → FAILED → REMEDIATION → VALIDATION`.

Es ist VERBOTEN, Statussprünge zu überspringen — insbesondere DARF ein Meilenstein
nicht direkt von IN_PROGRESS auf ACCEPTED springen (REQ-MS-002). Jeder Phasenwechsel
MUSS mit Evidence (Commits, Testberichte, Audit-Records) unterlegt sein.

## §3 Meilenstein-ID

Pflichtformat (REQ-MS-003):

- Basis: `ATC-M-NNN` (dreistellig, fortlaufend, z. B. `ATC-M-001`)
- Programme: `ATC-M-<PROGRAMM>-NNN` (z. B. `ATC-M-CORE-001`, `ATC-M-CHAIN-001`,
  `ATC-M-WALLET-001`, `ATC-M-MINER-001`, `ATC-M-AI-001`, `ATC-M-GAME-001`)
- Programme MÜSSEN in `registry/milestones.yaml` deklariert sein; IDs DÜRFEN nicht
  wiederverwendet werden (ENTSCHIEDEN-Regel analog ATC-STD-000 §37).

## §4 Pflichtfelder

Jeder Meilenstein MUSS mindestens folgende Felder führen (REQ-MS-004):

| Feld | Pflicht |
|---|---|
| Milestone ID | Ja |
| Name | Ja |
| Version | Ja |
| Ziel | Ja |
| Scope (Gilt/Nicht-Gilt) | Ja |
| Requirements | Ja |
| Deliverables | Ja |
| Acceptance Criteria | Ja |
| Tests | Ja |
| Security Status | Ja |
| Documentation Status | Ja |
| Compatibility Status | Ja |
| Dependencies | Ja |
| Owner | Ja |
| Responsible Agent | falls KI beteiligt |
| Start | Ja |
| Target Date | Ja |
| Actual Completion | nach Abschluss |
| Evidence | Ja |
| Audit Result | Ja |
| Status | Ja |

## §5 Meilenstein-Kategorien (Maturity)

Jeder Meilenstein MUSS einer Reifeklasse zugeordnet sein (REQ-MS-005):

| Klasse | Bedeutung |
|---|---|
| M0 | Foundation (Repository, Architektur, Standards, CI/CD, Dev-Umgebung, Doku) |
| M1 | Prototype (erster funktionsfähiger Prototyp) |
| M2 | Alpha (Kernfunktionen vorhanden) |
| M3 | Beta (System funktional und testbar) |
| M4 | Feature Complete (definierter Scope vollständig) |
| M5 | Security Ready (Security-/Robustheitsprüfung abgeschlossen) |
| M6 | Release Candidate (RC vollständig validiert) |
| M7 | Production Ready (Produktionsanforderungen erfüllt) |
| M8 | Production/Mainnet (produktiver Betrieb) |

Hinweis: Die Reifeklassen M0–M8 sind Kategorien-Tags und unabhängig von der
Org-Roadmap-Nummerierung (AD-027 M1–M8 sind konkrete Instanzen, s. §18).

## §6 Evidence Pack

Ein Meilenstein DARF nur bei vollständigem Nachweis geschlossen werden (REQ-MS-006).
Evidence Pack (Ablage je Meilenstein, bevorzugt `evidence/ATC-M-NNN/`):

```
ATC-M-NNN/
├── requirements/      ├── implementation/   ├── tests/
├── security/          ├── compatibility/   ├── documentation/
├── audit/             ├── changelog/       └── evidence/
```

Evidence KANN enthalten: Testberichte, CI-Ergebnisse, Build-Artefakte, Auditberichte,
Screenshots, Logs, Benchmark-Ergebnisse, Hashes, Git-Commits, Release-Artefakte,
Dokumentationsnachweise. Mindestens MUSS je Gate ein Artefakt vorliegen.

## §7 Acceptance Gates

Ein Meilenstein DARF nur ACCEPTED werden, wenn alle Gates bestanden sind (REQ-MS-007):

```
Requirement Gate → Implementation Gate → Test Gate → Security Gate →
Compatibility Gate → Documentation Gate → Audit Gate → Acceptance Gate
```

**ATC-MILESTONE-ACCEPTANCE-GATE (Hartregel):** Ein Meilenstein ist nicht „fertig",
weil der Code funktioniert. Er ist fertig, wenn der definierte Gesamtnachweis
vollständig ist.

## §8 Abhängigkeiten

Meilensteine MÜSSEN Abhängigkeiten explizit deklarieren (REQ-MS-008). Ein Meilenstein
DARF NICHT als ACCEPTED markiert werden, solange eine deklarierte kritische Dependency
nicht mindestens ACCEPTED ist. Zyklen sind unzulässig (DAG-Regel analog
registry/dependencies.yaml).

## §9 Meilenstein ↔ Sprint

Governance-Kette (REQ-MS-009):

```
Roadmap → Milestone → Requirements → Issues → Tasks → Sprints →
Implementation → Tests → Audit → Milestone Acceptance
```

**Sprint ≠ Meilenstein.** Ein Sprint ist ein Zeit-/Arbeitszyklus; ein Meilenstein ist
ein erreichter Systemzustand. Sprints KÖNNEN zu einem Meilenstein beitragen, dürfen
ihn aber nicht ersetzen.

## §10 Meilenstein ↔ Release

Ein Release KANN einen oder mehrere Meilensteine enthalten (REQ-MS-010). Ein Release
DARF nur veröffentlicht werden, wenn seine kritischen Meilensteine ACCEPTED sind
(Kopplung an ATC-STD-VERSION-001 Release-Pflichten).

```
v0.8.0
 ├── ATC-M-WALLET-004
 ├── ATC-M-NODE-003
 └── ATC-M-API-002
```

## §11 Major-Version-Integration

Bei einem MAJOR Release gilt die Kette (REQ-MS-011; Kopplung an UPDATE-001 UPD-G04
und COMPAT-001):

```
Major Release → Dependency Audit → Compatibility Audit → Standards Audit →
Documentation Audit → Migration → Regression Tests → Milestone Revalidation
```

Bereits abgeschlossene (ACCEPTED/VERIFIED/CLOSED) Meilensteine MÜSSEN bei Breaking
Changes erneut validiert werden (Revalidation). Ergebnis MUSS in `registry/milestones.yaml`
 dokumentiert werden (Revalidations-Vermerk im Statusfeld `revalidated`).

## §12 Meilenstein-Risiko

Jeder Meilenstein MUSS ein Risikomodell führen (REQ-MS-012):

```
risk:
  overall: LOW | MEDIUM | HIGH | CRITICAL
  technical: / security: / dependency: / compatibility: / operational: / business:
```

CRITICAL-overall-Meilensteine erfordern eine erweiterte Audit-Tiefe (AUDIT-001).

## §13 KI-Agenten und Meilensteine

KI-Agenten DÜRFEN einen Meilenstein bearbeiten, MÜSSEN ihn aber ohne explizite
Governance-Autorisierung (Owner-§9-Freigabe bzw. Human Gate gemäß AI-DECISION-001)
als endgültig ACCEPTED/CLOSED markieren NICHT (REQ-MS-013). Der Agent MUSS je
Meilenstein-Phase dokumentieren: Agent ID, Agent Version, Task, Repository, Branch,
Commit, Changes, Tests, Findings, Known Issues, Dependencies, Evidence, Next Action.
Damit ist nachvollziehbar: Wer hat was verändert, warum, wo und mit welchem Nachweis.

## §14 Definition of Done

Ein Meilenstein gilt als DONE, wenn ALLE Punkte erfüllt sind (REQ-MS-014):

- [ ] Requirements erfüllt
- [ ] Scope vollständig
- [ ] Code implementiert
- [ ] Unit Tests bestanden
- [ ] Integration Tests bestanden
- [ ] Regression Tests bestanden
- [ ] Security geprüft
- [ ] Dependencies geprüft
- [ ] Kompatibilität geprüft
- [ ] Dokumentation aktualisiert
- [ ] Wiki aktualisiert
- [ ] README aktualisiert, falls erforderlich
- [ ] CHANGELOG aktualisiert
- [ ] offene Critical/High Findings behandelt
- [ ] Audit abgeschlossen
- [ ] Evidence gespeichert
- [ ] Release-Zuordnung dokumentiert
- [ ] Acceptance erteilt

## §15 Statusdefinitionen

| Status | Bedeutung |
|---|---|
| PLANNED | Vorgesehen |
| DEFINED | Vollständig spezifiziert |
| IN_PROGRESS | Entwicklung läuft |
| BLOCKED | Abhängigkeit verhindert Fortschritt |
| FEATURE_COMPLETE | Implementierung abgeschlossen |
| VALIDATION | Technische Validierung |
| AUDIT | Audit läuft |
| FAILED | Kriterien nicht erfüllt |
| ACCEPTED | Fachlich/technisch akzeptiert (Gates §7 bestanden) |
| RELEASED | Veröffentlicht |
| VERIFIED | Nach Veröffentlichung verifiziert |
| CLOSED | Endgültig abgeschlossen |
| SUPERSEDED | Durch neuen Meilenstein ersetzt |

## §16 Zentrale Governance-Regel

**ATC-MILESTONE-GOVERNANCE-RULE (Hartregel, REQ-MS-016):**

> Kein ATC-Meilenstein darf allein aufgrund einer behaupteten Fertigstellung
> geschlossen werden. Jeder Abschluss benötigt überprüfbare Acceptance Criteria,
> Testnachweise, Auditnachweise und eine eindeutige Zuordnung zu Code, Dokumentation
> und Version.

Damit wird der Meilenstein zum zentralen Kontrollpunkt zwischen Roadmap und
tatsächlich funktionsfähigem System.

## §17 Maschinenlesbarkeit (Registry + Schema + Validator)

Der Standard MUSS maschinenlesbar durchsetzbar sein (REQ-MS-017, REQ-MS-018):

1. **JSON-Schema:** `schemas/milestone.schema.json` definiert Struktur, Pflichtfelder,
   ID-Patterns, Status- und Risiko-Enums maschinell.
2. **Registry:** `registry/milestones.yaml` ist SSOT aller verbindlichen Meilensteine;
   jedes Entry folgt dem JSON-Schema. Neue verbindliche Meilensteine MÜSSEN dort
   registriert werden (Registry-Pflicht).
3. **Validator:** Der Standards-Validator prüft die Registry strukturell (S-20:
   Pflichtfelder, ID-Pattern, Status-Enum, Evidence-Pflicht bei ACCEPTED+,
   Dependency-Auflösung, eindeutige IDs).

## §18 Migrationsklausel — Bestands-Roadmap AD-027

Die bestehende verbindliche Lauffähigkeits-Roadmap M1–M8 (AD-027) wird als erste
konkrete Meilenstein-Instanzen übernommen (REQ-MS-019): `ATC-M-001`..`ATC-M-008`
mit ihren Run-Kriterien (Reality-Check-Regel: Test-/Boot-/Run-Nachweis). Deren
aktueller Stand ist in `registry/milestones.yaml` registriert; künftige
Zustandswechsel MÜSSEN über den Lebenszyklus §2 erfolgen. Ergänzende Programm-
Meilensteine (z. B. ATC-M-CHAIN-001) DÜRFEN ergänzt werden, ohne die Basis-Nummern
zu verletzen.

## Requirements (normativ)

- **REQ-MS-001** (§1): Ein Meilenstein MUSS als erreichter Systemzustand mit
  Akzeptanz-, Qualitäts-, Sicherheits-, Dokumentations- und Kompatibilitätskriterien
  definiert sein.
- **REQ-MS-002** (§2): Der Lebenszyklus MUSS vollständig durchlaufen werden;
  Statussprünge (insb. IN_PROGRESS → ACCEPTED) sind VERBOTEN.
- **REQ-MS-003** (§3): Meilenstein-IDs MÜSSEN dem Format ATC-M-NNN bzw.
  ATC-M-<PROGRAMM>-NNN entsprechen und DÜRFEN nicht wiederverwendet werden.
- **REQ-MS-004** (§4): Alle Pflichtfelder MÜSSEN geführt werden.
- **REQ-MS-005** (§5): Jeder Meilenstein MUSS einer Reifeklasse M0–M8 zugeordnet sein.
- **REQ-MS-006** (§6): Ein Abschluss MUSS einen Evidence Pack mit je Gate
  mindestens einem Artefakt enthalten.
- **REQ-MS-007** (§7): ACCEPTED setzt das Bestehen aller 8 Gates voraus
  (ATC-MILESTONE-ACCEPTANCE-GATE).
- **REQ-MS-008** (§8): Dependencies MÜSSEN deklariert sein; ACCEPTED bei offener
  kritischer Dependency ist VERBOTEN; Zyklen unzulässig.
- **REQ-MS-009** (§9): Sprint und Meilenstein MÜSSEN getrennt werden; Sprints
  KÖNNEN beitragen, ersetzen den Zustandsnachweis aber NICHT.
- **REQ-MS-010** (§10): Ein Release DARF nur mit akzeptierten kritischen
  Meilensteinen veröffentlicht werden.
- **REQ-MS-011** (§11): Bei MAJOR/Breaking Changes MÜSSEN abgeschlossene
  Meilensteine revalidiert werden (Kopplung UPD-G04/COMPAT-001).
- **REQ-MS-012** (§12): Jeder Meilenstein MUSS ein Risikomodell mit overall-Level
  und sechs Risikoarten führen.
- **REQ-MS-013** (§13): KI-Agenten MÜSSEN Vollständigkeit der 12 Dokumentationsfelder
  sicherstellen und DÜRFEN ohne Governance-Autorisierung kein ACCEPTED/CLOSED setzen.
- **REQ-MS-014** (§14): DONE erfordert alle 18 DoD-Punkte.
- **REQ-MS-015** (§15): Nur die definierten 13 Statuswerte sind zulässig.
- **REQ-MS-016** (§16): Die ATC-MILESTONE-GOVERNANCE-RULE ist uneingeschränkt
  verbindlich („kein Abschluss ohne Nachweis").
- **REQ-MS-017** (§17): Maschinenlesbarkeit (Schema, Registry, Validator) MUSS
  vorgehalten und in der CI geprüft werden.
- **REQ-MS-018** (§17): Jeder verbindliche Meilenstein MUSS in registry/milestones.yaml
  registriert sein (Registry-Pflicht analog ATC-STD-000 §7).
- **REQ-MS-019** (§18): Die AD-027-Roadmap M1–M8 WIRD als ATC-M-001..008 registriert
  geführt; deren Kriterien bleiben verbindlich.

## Security Considerations

Meilenstein-Acceptance ist eine Sicherheitsbarriere: Ohne Security Gate und
Audit Gate kein ACCEPTED (§7). Agenten-seitige Abschlussmarkierungen sind ohne
Human Gate ungültig (§13, AI-DECISION-001). Evidence Packs sind manipulations-
geschützt zu führen (Hashes/Git-Referenzen); nachträgliche Änderungen am Evidence
eines CLOSED-Meilensteins sind nur über AUDIT-001-Records dokumentierbar.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.0.0** (2026-09-07): Initial Release — Owner-Entwurf Michael Wroblewski
  (Builder-Chat 23:31), harmonisiert mit UPDATE-001 (Gates/Releases), COMPAT-001
  (MAJOR-Revalidation), AUDIT-001 (Audit-Gates), VERSION-001 (Release-Bindung),
  AI-DECISION-001 (Human Gates), DESC-001 (Beschreibungsstruktur), AD-027
  (M1-M8-Migration). 19 REQ-MS; maschinenlesbar via schemas/milestone.schema.json,
  registry/milestones.yaml, Validator S-20. SCR-0018; §9-Freigabe Michael Wroblewski 07.09.2026, 23:38 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- ATC-STD-000 (Verfassung, §7 Registry, §30 Immutabilität)
- ATC-STD-VERSION-001 (Release/Versioning-Pflichten)
- ATC-STD-UPDATE-001 (UPD-G04 MAJOR-Gate, Change Control)
- ATC-STD-COMPAT-001 (MAJOR-Kompatibilität, Revalidation)
- ATC-STD-AUDIT-001 (Audit-Layer, AUD-G-Gates)
- ATC-STD-AI-DECISION-001 (Human Gates, Agenten-Autorisierung)
- AD-027 (Lauffähigkeits-Roadmap M1–M8), AD-026 (Bauhierarchie)
- registry/milestones.yaml (SSOT), schemas/milestone.schema.json (Maschinenformat)

*ATC-STD-MILESTONE-001 v1.0.0 · Owner-Entwurf Michael Wroblewski · Aurora (Superagent) · 07.09.2026 · SCR-0018*
