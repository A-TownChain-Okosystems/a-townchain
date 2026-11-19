---
title: Lint-Gates
summary: Welche Linter wann laufen, woher die Regeln kommen und was ein gruenes Lint-Gate belegt.
---

# Lint-Gates

Lint ist die frueheste Qualitaetsbarriere im Repo: Syntax, Formatierung, Importe, offensichtlich toter Code. Es ist ausdruecklich keine Sicherheitsfreigabe.

## Gate-Matrix

| Sprache / Artefakt | Werkzeug | Regelquelle | Gepruefter Umfang |
|---|---|---|---|
| Python | Ruff 0.16.7 (lint) | Org-Profil, gepinnt auf Hub-SHA `7994fca` (ATC-STD-081) | eigener Quellcode |
| Python | Ruff 0.16.7 (format) | gleiches Profil (ATC-STD-082) | eigener Quellcode |
| Markdown | markdownlint-cli2 | `.markdownlint-cli2.jsonc` im Repo, spiegelt die Org-Policy (MD025, MD003 dokumentiert abweichend) | eigener Dokumentationsbestand |
| Rust | Clippy + rustfmt | Workspace-Profile | `components/`, `kernel/` |
| YAML | Workflow- und Registry-Validatoren | `standards/tools/` | `.github/`, `standards/` |

Jedes Gate laeuft in der CI an den exakten Commit-SHA gebunden. Ein Ergebnis ohne SHA ist kein Nachweis.

## Was ein gruenes Lint-Gate belegt

| Verfahren | Nachweis |
|---|---|
| Lint | Code entspricht formalen Qualitaetsregeln |
| Build | Software laesst sich kompilieren |
| Unit Tests | einzelne Funktionen arbeiten korrekt |
| Integration Tests | Komponenten funktionieren zusammen |
| Security Tests | Sicherheitsanforderungen werden geprueft |
| E2E Tests | Gesamtsystem funktioniert ueber Prozessgrenzen |

Ein gruener Lint-Lauf sagt nichts ueber Laufzeitverhalten, Sicherheit oder Korrektheit der Logik aus. Er sagt nur: die vereinbarten Regeln sind eingehalten.

## Regeln fuer Ausnahmen

1. Keine lokalen Sonderregeln ohne dokumentierte Begruendung.
2. Ausnahmen sind Scope-Abgrenzungen, keine Regelabschaltungen: importierte Fremdbestaende werden nicht geprueft, eigener Code vollstaendig.
3. Jede Ausnahme nennt Grund, Umfang und Bedingung, unter der sie entfaellt.
4. Befunde werden behoben, nicht stummgeschaltet.

### Dokumentierte Ausnahmen

**`docs/os-docs/**`** — importierter Dokumentations-Snapshot des aufgeloesten Docs-Repos (SCR-0127). Wird nicht gebaut, nicht getestet, nicht ausgeliefert; die kanonische Implementierung liegt in `modules/`, `apps/` und `components/`. Gilt fuer Ruff (ueber `.gitignore`), markdownlint (ueber `ignores`) und CodeQL (ueber `paths-ignore`). Entfaellt, sobald der Snapshot aus dem Monorepo entfernt wird.

**`ai/**`, `apps/**`, `components/**`, `kernel/**`, `modules/**`** — importierte Modul- und App-Bestaende. Sie werden gebaut und getestet, ihre Dokumentation ist aber nicht der Dokumentationsbestand dieses Repositories; markdownlint prueft sie deshalb nicht.

## Bekannte Schwachstelle im Ruff-Workflow

Der Workflow uebergibt zwei `--exclude`-Flags. Ruff wertet die Option als einzelnen Wert aus, das zweite Flag faellt weg, und der Doku-Snapshot wird mitgeprueft — in der CI 3.206 Befunde, lokal gruen. Der Patch auf eine Komma-Liste liegt in `.ci/pending-workflows`; der Push braucht einen Token mit `workflow`-Scope. Bis dahin haelt der `.gitignore`-Eintrag den Snapshot aus dem Scan.
