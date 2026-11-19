---
title: Quality Levels
summary: Drei Pruefebenen fuer A-TownChain — Code Lint, Repository Lint, Ecosystem Compliance — mit Zustaendigkeit und Nachweispflicht.
---

# Quality Levels

Drei Ebenen, drei Zustaendigkeiten. Jede Ebene beantwortet eine andere Frage, und keine ersetzt die andere.

| Ebene | Frage | Werkzeug | Nachweis |
|---|---|---|---|
| 1 Code Lint | Ist der Code formal sauber? | Ruff, markdownlint, Clippy | Gate am Commit-SHA |
| 2 Repository Lint | Erfuellt das Repository seine Struktur- und Governance-Pflichten? | `standards/tools/atc-repo-audit/atc_repo_audit.py` (ATC-STD-201/202/203) | GATE PASS/NO-GO plus Health Score |
| 3 Ecosystem Compliance | Stimmen die Zustaendigkeiten zwischen den Komponenten? | `tools/quality/ecosystem_compliance.py` | Pruefbericht mit BLOCKED/WARN je Regel |

## Wer definiert, wer prueft

- **`standards/`** (ATC Governance Root) definiert, was eingehalten werden muss: ATC-STD-081/082 fuer Code-Qualitaet, ATC-STD-201/202/203 fuer Repository-Governance, ATC-STD-300 fuer die Nachweiskette.
- **Repository-CI** prueft die lokale Einhaltung: Ebene 1 und 2 laufen bei jedem Push und Pull Request.
- **Ecosystem Control Plane** prueft die organisationsweite Einhaltung: Ebene 3, gefahren aus `ecosystem/control-plane` und dem Governance-Hub.

## Ebene 3 im Detail

`tools/quality/ecosystem_compliance.py` prueft neun Regeln in sechs Gruppen:

| Regel | Prueft |
|---|---|
| E-01 | Registry lesbar, `count` stimmt, keine doppelten Repo-Namen, jede Zeile hat eine ID |
| E-02 | Jede Komponente in `components/` ist registriert, eingefrorene Repos haben einen kanonischen Ort |
| E-03 | Dieselbe Faehigkeit wird nicht zweimal gebaut (Ueberschneidung `components/` gegen `apps/`) |
| E-04 | Keine Referenz auf ein aufgeloestes Repository im ausfuehrbaren Code |
| E-05 | Jede Faehigkeit nennt genau eine kanonische Quelle, `.atc/ownership.yaml` vorhanden |
| E-06 | Workspace-Mitglieder existieren als Verzeichnis, `.atc/repository.yaml` vorhanden |

Statuslogik: `BLOCKED` verhindert den Merge, `WARN` verlangt eine dokumentierte Begruendung, `PASS` ist der Normalfall. Befunde werden behoben, nicht stummgeschaltet.

## Ein Aufruf fuer alle drei Ebenen

```bash
python3 tools/quality/atc_quality.py                 # Ebenen 1, 2, 3
python3 tools/quality/atc_quality.py --levels 3      # nur Ecosystem
python3 tools/quality/atc_quality.py --out artifacts/quality/report.json
```

Der Bericht enthaelt `commit_sha`, `generated_at` und je Ebene `status` und `detail`. Ein Ergebnis ohne SHA ist kein Nachweis.

## Stand 18.11.2026 (Commit `2fd3cdd`)

| Ebene | Status | Bemerkung |
|---|---|---|
| 1 Code Lint | PASS | Ruff lint und format sauber, Markdown Lint sauber |
| 2 Repository Lint | PASS | Auditor meldet GATE: PASS |
| 3 Ecosystem | PASS | 9 Pruefungen, 2 Hinweise |

Die zwei Hinweise sind dokumentiert und keine Defekte:

1. **E-03** — acht Faehigkeiten liegen sowohl in `components/` als auch in `apps/` (contracts, explorer, indexer, interop, mining, node, oracle, storage). Die `apps/`-Fassungen sind stillgelegt und nicht im Workspace; kanonisch ist `components/`.
2. **E-04** — `AGENT_MANIFEST.md` verlinkt historische Repos, die nach der Konsolidierung archiviert sind. Die Tabelle beschreibt den Stand vom 07.09.; die kanonischen Orte sind im Manifest nachgetragen.

## Was diese Ebenen nicht belegen

Ein gruenes Gate auf allen drei Ebenen sagt: Regeln, Struktur und Zustaendigkeiten stimmen. Es sagt nichts ueber Sicherheit, Laufzeitverhalten oder Korrektheit der Logik. Dafuer gelten Security-Tests, E2E-Nachweise und das Security-Audit.
