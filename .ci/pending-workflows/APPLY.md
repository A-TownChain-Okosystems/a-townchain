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

## Befund 18.11.2026 (verifiziert)

- **ruff.yml**: Zwei `--exclude`-Flags. Ruff behandelt die Option als einzelnen
  Wert; nur einer greift, `docs/os-docs` wird gescannt (3.206 Befunde in der CI,
  lokal gruen). Patch nutzt jetzt die Komma-Liste.
- **codeql.yml**: `init` ruft `codeql-config.yml` nicht auf, deshalb greift
  `paths-ignore: docs/os-docs` nicht und CodeQL meldet 59 Befunde aus dem
  importierten Snapshot. Patch ergaenzt `config-file`.

## Neu: quality-gates.yml (SCR-0147)

Fuehrt Ebene 1 bis 4 ueber `tools/quality/atc_quality.py` aus und legt den
SHA-gebundenen Bericht als Artefakt ab. Ebene 4 prueft das Master-Backlog
(IDs, Abhaengigkeiten, SSOT, Evidence). Braucht wie die uebrigen Patches
`workflow`-Scope.

## Neu: CodeQL-Scope wirklich anwenden (SCR-0147)

Die Datei `.github/codeql/codeql-config.yml` liegt am richtigen Ort, wird vom
`init`-Schritt aber nicht geladen: im Log erscheint sie nur als *extrahierte*
Datei, nicht als Konfiguration. Deshalb meldet CodeQL weiter die 58 Befunde aus
`docs/os-docs/**`, obwohl der eigene Code bei 0 steht.

Eine Zeile in `.github/workflows/codeql.yml` genuegt:

```yaml
      - uses: github/codeql-action/init@v3
        with:
          languages: javascript,python
          config-file: ./.github/codeql/codeql-config.yml
```

Danach greift `paths-ignore: docs/os-docs/**` und der aggregierte CodeQL-Check
wird gruen, sobald der eigene Code sauber ist (Stand: 0 Befunde).
