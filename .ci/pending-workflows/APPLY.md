# Ausstehende Workflow-Umstellung (SCR-0146)

Diese drei Dateien sind fertig und geprueft, konnten aber nicht ueber den
Agenten-Token gepusht werden: GitHub verlangt fuer Aenderungen an
`.github/workflows/**` den OAuth-Scope `workflow`, der dem Token fehlt
(`read:org, read:user, repo`).

Sie liegen deshalb hier als Quelle bereit und sind inhaltlich identisch mit dem
Stand, der in PR #116 verifiziert wurde.

## Anwenden

```bash
./tools/ci/apply_pending_workflows.sh          # kopiert nach .github/workflows/
git diff .github/workflows/                    # Sichtpruefung
git commit -am "ci(SCR-0146): Linter-Scope und CodeQL-Konfiguration"
git push
```

Das Skript prueft vorher, ob die Zieldateien unveraendert sind, und bricht bei
Abweichungen ab, damit keine fremde Aenderung ueberschrieben wird.

## Inhalt der Aenderung

| Datei | Aenderung | Zweck |
|---|---|---|
| `ruff.yml` | `--exclude docs/os-docs` in Lint und Format-Check | importierter Doku-Snapshot raus aus dem Lauf |
| `markdownlint.yml` | Glob-Scope auf eigenen Bestand, `config` gesetzt | 0 Befunde statt Bestandsrauschen |
| `codeql.yml` | `config-file: ./.github/codeql/codeql-config.yml` | `paths-ignore: docs/os-docs` greift |

Regelsatz, Ruff-Version (0.16.7) und Policy-SHA bleiben unveraendert.
