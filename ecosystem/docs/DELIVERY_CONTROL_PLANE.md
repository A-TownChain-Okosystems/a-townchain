# Delivery Control Plane — Roadmap, Wiki Index und Sprintsteuerung

**Status:** initialer maschinenlesbarer Entwurf; nicht als vollständige Verifikation aller Repositories zu verstehen.  
**SSOT:** [control-plane/DELIVERY_REGISTRY.json](../control-plane/DELIVERY_REGISTRY.json)  
**Architektur:** [ARCHITECTURE.md](../ARCHITECTURE.md) · [Control Matrix](../control-plane/CONTROL_MATRIX.md) · [Control Plane](../control-plane/README.md)

## Wiki-/Dokumentationsindex

Das GitHub-Wiki ist für dieses Repository derzeit deaktiviert. Bis es explizit aktiviert wird, ist dieser versionierte Dokumentationsindex die navigierbare Wiki-Ersatzstruktur. Inhalte werden nicht in einem zweiten, widersprüchlichen Wiki-SSOT dupliziert.

| Bereich | Kanonische Quelle |
|---|---|
| Architektur, Schichten, Systemgrenzen | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| Zuständigkeiten / Konfliktauflösung | [CONTROL_MATRIX.md](../control-plane/CONTROL_MATRIX.md) |
| Statusmodell und Evidence-Regeln | [STATUS_SCHEMA.yaml](../control-plane/STATUS_SCHEMA.yaml) |
| Roadmap, Meilensteine, Checkpoints und Lieferstatus | [DELIVERY_REGISTRY.json](../control-plane/DELIVERY_REGISTRY.json) |
| Komponenten-/Funktionsliste und Schnittstellen | [ATC-COMPONENT-FUNCTION-CATALOG-001.json](architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json) |
| Produktionskriterien und maschinenlesbare Validierung | [ATC-COMPONENT-FUNCTION-CATALOG-001.md](architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.md) |
| Kernel-Implementierung und Boot-Evidence | [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) |
| Blockchain-/VM-/Algorithmusimplementierung | [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) |
| ATC-Standards und Konformität | [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) |
| ATCLang | [atclang](https://github.com/A-TownChain-Okosystems/atclang) |
| Aurora AI | [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) |
| Genesis Engine | [genesis-engine](https://github.com/A-TownChain-Okosystems/genesis-engine) |

## Roadmap und Meilensteine

Die Registry ist die maschinenlesbare Quelle für Roadmap-Einträge, Meilensteine, Checkpoints, Komponenten, Funktionen, Sprints und TODOs. Ein Status wird nur aus nachprüfbaren Repository-/CI-Daten aktualisiert. Nicht belegte Annahmen bleiben `PLANNED`, `BLOCKED` oder `RESIDUAL`.

1. **MS-01 — Evidence-driven Governance:** stabile IDs, exakter SHA, Run → Job → Step → Logs, Root Cause, Fix und Retest.
2. **MS-02 — ShivaCore Kernel Safety:** VMM-Mutation/Rollback, Mapping-Lifetime, Rust- und Boot-/UEFI-/ACPI-Evidence.
3. **MS-03 — A-TownChain Protocol:** kanonische Economics, TX-V2-Vektoren und explizite Konsensentscheidung.
4. **MS-04 — ATCLang/ATC-VM:** deterministisches Bytecodeformat, Limits, Fehlerbehandlung und Konformitätstests.
5. **MS-05 — Aurora/Genesis Integration:** Service-Grenzen, Berechtigungen und keine Verlagerung von KI-Autorität in den Kernel-TCB oder Konsens.

## Checkpoint-Regeln

- **IMPLEMENTED** bedeutet, dass eine Änderung vorhanden ist; es bedeutet nicht, dass sie verifiziert ist.
- **VERIFIED** verlangt die exakte getestete Commit-SHA, erfolgreiche erforderliche Gates und verlinkte Evidence.
- Ein laufender, übersprungener oder nicht erreichbarer Required Check ist kein grüner Check.
- Artifact-Upload-Erfolg allein belegt nicht, dass die Governance-Semantik des Artefakts erfüllt ist.
- Keine Gates schwächen, keine Evidence fingieren und keinen PR automatisch mergen.

## Komponenten- und Funktionskatalog

Der [maschinenlesbare Katalog](architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json) ist die SSOT für komponentenbezogene Verantwortlichkeiten, Funktions-IDs, Ein-/Ausgaben, Fehlerfälle, Abnahmekriterien, Testklassen und bekannte Schnittstellen. Die zugehörige [Dokumentation](architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.md) beschreibt Produktionsgates und Protokollrisiken.

Der Katalog ist derzeit `BASELINE_RESIDUAL`; die Release-Entscheidung ist `BLOCKED`. Das bedeutet, dass der Katalog strukturiert und validierbar gemacht wird, nicht dass alle Komponenten produktionsreif sind. Der Katalog darf nur nach geprüftem Quell-/Schnittstellennachweis aktualisiert werden. Unbekannte Fakten bleiben offen.

## Sprint- und TODO-Steuerung

Der aktuelle Sprint ist bewusst nicht kalendarisch terminiert: zuerst rote Gates und Evidence-Integrität, danach SSOT-/Komponentenabgleich, dann Kernel- und Protokoll-Checkpoints. Termine werden erst gesetzt, wenn echte Issue-/Milestone-Daten geprüft sind. Jede Änderung an der Registry muss aus GitHub-/CI-Evidence abgeleitet werden; fehlende Evidenz bleibt explizit offen.

## Aktualisierungsvertrag für den Gate-Fix-Loop

Bei jedem Durchlauf:
1. Repositories und offene PRs aus der aktiven Registry neu inventarisieren.
2. Fehlgeschlagene und neu fehlgeschlagene Actions-Läufe erfassen; Run-ID, Job-ID, Step, Exit-Code und Log sichern.
3. Root Cause isolieren und minimal korrigieren, ohne Gates abzuschwächen.
4. Neue Gates auf dem exakten Korrektur-SHA erneut prüfen.
5. Registry-Einträge, Roadmap, Checkpoints, Sprint-Ziele und TODO-Status synchronisieren.
6. Komponenten-/Funktionslisten mit kanonischer SSOT-Zuständigkeit abgleichen.
7. Fehlende Daten als `BLOCKED`/`RESIDUAL` dokumentieren. Kein automatischer Merge.

## Full repository coverage (33 visible repositories)

The machine-readable registry tracks all 33 visible repositories in `repository_inventory` and `repository_delivery_plans`. Each delivery plan covers documentation/wiki, roadmap, milestones, checkpoints, components, functions/capabilities, sprints and TODOs.

**Important:** these records establish coverage tracking, not completion. Each plan must be inspected against the real repository before purpose, functions, status or success are recorded. Archived repositories remain inventoried but are not active implementation targets by default.

### Per-repository audit contract

For each repository, record:
1. Default branch and exact current HEAD SHA.
2. README, docs/wiki availability, specifications and tests; link canonical sources or record a gap.
3. Component/function/interface inventory and canonical ownership; report SSOT conflicts.
4. Open PRs and their exact head SHAs.
5. Required workflow runs, jobs, steps and logs; distinguish pending, failed, skipped and successful checks.
6. Repository-specific roadmap, milestones, checkpoints, sprint/TODO mappings and measurable acceptance criteria.
7. Evidence references and audit timestamp. Mark `VERIFIED` only after acceptance criteria are supported by exact-SHA evidence.

The daily loop reconciles the repository census first, then processes P0 failures and stale records. It keeps archived repositories visible, never invents sprint dates, never weakens gates and never auto-merges.
