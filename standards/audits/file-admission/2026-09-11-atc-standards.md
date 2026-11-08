---
title: File Admission Pilot-Audit atc-standards 2026-09-11
summary: SCR-0099 CHECK-FILE-001 Erstlauf — 1360 Dateien geprueft, 269
  QUARANTINED als Review-Backlog, 0 REJECTED
---

# File Admission Pilot-Audit — atc-standards (11.09.2026, SCR-0099)

Erstlauf des Admission-Checkers (ATC-STD-220 §6) gegen das Governance-Root-Repo.

## Ergebnis

Dateien: 1360 | ADMITTED 595 | CONDITIONAL 496 | QUARANTINED 269 | REJECTED 0
RESULT: WARN (Quarantine-Review erforderlich)

- **0 REJECTED** — keine Secrets, Keystores, Caches, .venv- oder Archiv-Dateien
  im Repo (Vorfall a-townchain `.venv-t` haette erkannt: Regel greift).
- **496 CONDITIONAL** — ueberwiegend deklarierte generierte Sichten
  (registry/standards/ Per-Standard-Records, views, registry.lock) mit
  Generator-Deklaration nach ATC-STD-220 §4 / ATC-STD-002 §6.
- **269 QUARANTINED** — Pfadfamilien ohne ableitbares Entitlement, vor allem
  `approval/` (Freigabe-Evidence), `atc/` (ATC-01..99-Protokoll-Kladden),
  weitere Legacy-Verzeichnisse. Keine stillegewordene Ausnahme: Backlog ist
  der Review-Worklist fuer ATC-STD-225-Rollout (Entitlement-Matrix je Pfad
  ergaenzen oder Datei adjudizieren).

## Wert

Der Erstlauf beweist das Fail-Closed-Prinzip am eigenen Repo: Presence is
not permission — 269 Dateien sind technisch vorhanden, ihre Berechtigung
ist bis auf Weiteres ungeklaert und MUSS durchlaufen werden.

## Reproduktion

`python3 tools/file_admission/file_admission.py` (Exit 2 = WARN im
Quarantine-Modus). Strict-Modus fuer Gates: `--strict`.
