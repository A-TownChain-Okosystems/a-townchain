# SCR-0051 — Bereitliegender Workflow-Patch (Owner-Aktion wegen GH013)

## Status
**BEREIT — wartet auf Owner-Anwendung.** Der OAuth-Push der Workflow-Datei
wurde per Repository-Regel abgelehnt (GH013: kein `workflow`-Scope, kongruent
zu Finding F-010). Zwei Wege zur Aktivierung:

**Weg A (30 Sekunden):** In GitHub UI die Zeilen unten in
`.github/workflows/naming-governance.yml` einfuegen — nach dem Schritt
„Standards-Validierung" und vor „Repository-Audit" sowie fetch-depth beim
checkout ergaenzen.

**Weg B:** Token mit `workflow`-Scope bereitstellen, dann pusht Aurora den
fertigen Patch selbst.

## Patch (exakt)

1. checkout ergaenzen (F-009-Konsistenz mit ci.yml):

```yaml
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0        # <- neu
```

2. Neuer Schritt nach „Standards-Validierung", vor „Repository-Audit":

```yaml
      - name: "Meta-Daten-Audit (SCR-0051, ATC-STD-000 §8)"
        run: python3 tools/atc-std-validator/meta_data_audit.py
```

## Wirkung
- Jeder Push/PR auf atc-standards prueft automatisch alle Standard-Dateien
  auf vollstaendige Metadaten (MD-A..MD-F, SCR-0050): Frontmatter, 10
  Pflichtfelder, Wertvalidierung, ID-/Kategorie-Konsistenz. Funde = CI rot.
- Audit-Tool ist CI-Gate-faehig bereits committed+gepusht (exit 1 bei Funden).
- YAML-Sonderfall geloest: Step-Name mit Doppelpunkt ist gequotet.
