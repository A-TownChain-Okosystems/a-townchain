---
document_id: ATC-BACKLOG-README-001
title: "Master-Backlog — Aufbau und Provenienz"
status: active
normative: false
owner: A-TownChain-Okosystems
---

# Master-Backlog — Aufbau und Provenienz

Das Backlog ist eine **Ableitung**, keine zweite Wahrheit. Es fuehrt keine eigenen
Zustaende und keine handgepflegten Zaehler.

## Quellen

| Quelle | Rolle | Ort |
|---|---|---|
| Blocker-Register `ATC-BLOCKER-REGISTER-001` v0.3.7 | Inhalt: Titel, Problem, Closure-Kriterien, Evidence | `a-townchain-ecosystem`, `docs/system-readiness/BLOCKER_REGISTER.yaml`, gepinnt in `sources/register-2026-11-17.yaml` |
| Kuratierung | SSOT-Zuordnung, Phase, Abhaengigkeiten, Zusatzaufgaben | `curation.yaml` |
| Live-Erhebung | Zahlen des Organisationsbestands | `verification.yaml`, erzeugt von `tools/verify_org.py` |

## Erzeugen

```bash
python3 backlog/tools/verify_org.py      # Live-Zahlen (braucht GITHUB_ACCESS_TOKEN)
python3 backlog/tools/build_backlog.py   # YAML + Markdown
python3 backlog/tools/validate_backlog.py
```

## Regeln

- **Ein SSOT je Aufgabe.** Zwei kanonische Eigentuemer fuer dieselbe Funktion sind
  ein Finding, nicht eine Alternative.
- **Kein Status ohne Evidence.** Ein geschlossener Task braucht Repository, Branch,
  vollstaendigen Commit-SHA, Datei, Workflow-Run, Job, Step und Exit-Code.
- **Gruene CI ist kein Funktionsnachweis.** Sie belegt, dass ein Job lief.
- **Widersprueche werden gefuehrt.** Weicht eine Behauptung aus einem Bericht vom
  Live-Bestand ab, steht das im Abschnitt `corrections`, nicht in einer Fussnote.
- **Neue Registerstaende kommen als neuer Snapshot**, nie als stille Aenderung an
  der gepinnten Kopie.
