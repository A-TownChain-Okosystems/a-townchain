# ATC-EVIDENCE-RUN-LEDGER-001 — Run-/Evidence-Lifecycle-Registry

Dokumentklasse: Standards-Draft (atc-standards)
Status: 0.1.0-DRAFT — nicht APPROVED, Review durch Owner erforderlich
Bezug: SCR-0132 (Gate-Methodik), SCR-0086/SCR-0087 (Evidence-Bindung, PR-Promotion),
ATC-EVIDENCE-001 (M4-Evidence-Pakete), SHIVA-TEST-EVIDENCE-001 (workflow_run_id),
ATC-STD-003 (Governance Determinism / SSOT), F-095 (Evidence-Schema, P0)

## 1. Scope

Dieser Draft standardisiert die Run-/Evidence-Lifecycle-Registry (Evidence Run Ledger):
die unveränderliche Registrierung von CI-/Workflow-Ausführungen als Evidence-Identität
und deren Übergänge zu Governance-Verdicts. Er führt KEINE zweite, konkurrierende ID
ein — die GitHub-native `workflow_run.id` ist die immutable Primäridentität (ATC-STD-003:
keine konkurrierende Wahrheitsquellen).

## 2. Identität

Primäridentität je konkreter Ausführungsversuch ist der zusammengesetzte Schlüssel:

```
(repository, workflow, run_id, run_attempt)
```

- `run_id` — GitHub `workflow_run.id`, unveränderlich.
- `run_attempt` — ZÄHLT zur Identität dazu: ein Rerun behält die Run-ID, ändert aber
  den Attempt. Vermischung von Attempts ist verboten.
- `commit_sha` — exakter 40-Hex-SHA des gebundenen Commits.
- `ref`, `event` — Kontext des Ausführungsvorgangs.

Anti-Pattern (normativ verboten): `run_number` als Identität (resettet je Workflow,
kollidiert org-übergreifend).

## 3. Zwei-Ebenen-Modell

Der Ledger trennt verbindlich zwei Ebenen:

1. **FACT-Ebene** — Projektion der GitHub-Fakten (unveränderlich, API-abgeleitet):
   REGISTERED, RUNNING, FAILED; je Attempt zusätzlich die GitHub-Konklusionen
   success, failure, cancelled, timed_out, skipped, action_required.
2. **VERDICT-Ebene** — Governance-Urteile des Ledgers (append-only):
   ANALYZED, VERIFIED, BLOCKED, RESIDUAL.

Invariante LED-1 (SSOT): Widerspricht der Ledger der GitHub-API, ist der Ledger
falsch. Der Ledger annotiert Fakten, überschreibt sie nie. Run-Fakten werden nie
inline kopiert (Logs/Artifacts unterliegen der 90-Tage-Retention); referenziert
werden nur IDs, SHAs und optional Artefakt-Hashes.

## 4. State Machine

```
REGISTERED → RUNNING → ANALYZED → VERIFIED → RESIDUAL
                │           │
                ├──────────►│──► BLOCKED
                ▼
             FAILED
```

| Status   | Ebene  | Bedeutung |
|----------|--------|-----------|
| REGISTERED | FACT | Run mit eindeutiger Run-ID registriert, noch nicht ausgeführt. |
| RUNNING | FACT | Ausführung läuft; Jobs/Steps werden erfasst. |
| FAILED | FACT | Technischer Gate-/Testfehler im Lauf aufgetreten. |
| ANALYZED | VERDICT | Lauf abgeschlossen, Ergebnisse ausgewertet. |
| VERIFIED | VERDICT | Alle verpflichtenden Gates und Evidenzbedingungen auf dem exakten Commit-SHA nachgewiesen. |
| BLOCKED | VERDICT | Verifikation wegen externer/vorgelagerter Blockade nicht abschließbar (GitHub `action_required` mündet hier). |
| RESIDUAL | VERDICT | Grundsätzlich verifiziert, dokumentierter Restbefund verbleibt (terminal). |

### 4.1 Korrektur-/Wiederholungsregel

Ein Fehler heilt NICHT durch den nächsten grünen Run aus. Kette:

```
Run N → FAILED → Fix-Commit → Run N+1 (neu registriert) → ANALYZED → VERIFIED
```

Die Fix-Kante ist一等-class im Schema: der Eintrag von Run N+1 (bzw. des
Fix-Commits) trägt `resolves_runs: [<run_id>#<attempt>, …]`. Damit ist die
komplette Remediation-Geschichte jedes Findings rekonstruierbar und die
heutige implizite „letzter grüner Lauf gewinnt"-Semantik des SCR-0086-Regex-
Replacements (Überschreiben früherer Attempts) wird abgelöst.

### 4.2 SKIP-Semantik

`[skip ci]`-Pushes erzeugen keinen Run und damit keinen Ledger-Eintrag — das
gilt ausdrücklich auch für die Evidence-Promotion-Commits selbst. Scanner und
Checker MÜSSEN diese Lücke als zulässig behandeln (keine „missing run"-
False-Positive).

## 5. GREEN ≠ VERIFIED

Governance-Regel: GREEN CI ist kein VERIFIED. Die Run-Registry registriert
ausschließlich den unveränderlichen Ausführungsvorgang. VERIFIED entsteht nur
durch die Evidence-/Verification-Logik.

Abschlussbedingung (mechanisch, konsistent mit SCR-0132):

> VERIFIED erst dann, wenn alle verpflichtenden Gates auf dem exakt verifizierten
> Commit SUCCESS nachgewiesen sind, der Verdict-Verweis auf einen ANALYZED-Datensatz
> desselben Attempts zeigt, und keine offenen blockierenden Befunde bestehen.

VERIFIED ist damit re-derivierbar statt nur behauptet.

## 6. Schema (Entwurf)

```yaml
workflow_run:
  run_id: 123456789
  run_attempt: 1
  repository: A-TownChain-Okosystems/<repo>
  workflow: <workflow-name>
  commit_sha: <exact-40-hex>
  ref: refs/heads/main
  event: push
  jobs:
    - job_id: <id>
      steps:
        - step_id: <id>
          exit_code: 0
  artifacts:
    - <artifact-id>   # nur ID + optional SHA-256, kein Inline-Inhalt
  evidence:
    status: ANALYZED        # VERDICT-Ebene, nie aus run conclusion abgeleitet
    resolves_runs: []       # Fix-Kante, falls Remediation
    verification_result: <ref>
```

Invariante LED-2: `evidence.status` DARF NICHT aus der GitHub-Konklusion
abgeleitet werden — sonst reimplementiert der Ledger implizit GREEN=VERIFIED.

## 7. RESIDUAL-Pflichtfelder

RESIDUAL nur als terminaler Verdict MIT:
- `residual_findings: [F-…]` — alle P ≥ 2 (nicht-blockierend),
- Clearing-Referenz: welcher künftige Befund/Lauf sie auflöst.

Ohne Clearing-Regel wird RESIDUAL zum vergessenen Endzustand.

## 8. Migrationspfad

1. Dieser Draft → Review → APPROVED (atc-standards, PR-Pfad).
2. Checker-Regel: Ledger-Einträge stichprobenweise gegen die GitHub-API
   verifizieren (kanonisches Repo-Audit-Muster).
3. Workflow-Migration der 13/13 SCR-0086-Workflows: Regex-Replace →
   Append je `(run_id, run_attempt)`; Felder `run_attempt`, `event`, `ref`
   ergänzen (2–3 Zeilen je Workflow, Bauform der laufenden atc-oracle/
   atc-node-PR-Promotion-Fixes — beide sind die Pilotkandidaten).
4. Zentrale Querschnitts-Projektion im gen_views-Muster (generiert, nie
   Handpflege).

## 9. Offene Punkte

- OEP-A: Ablageort des Ledgers (je-Repo-Evidence-Block vs. zentrale
  Registry-Datei in atc-standards) — Owner-Entscheidung.
- OEP-B: Retention/GC-Regel für Ledger-Einträge altgebauter Workflows.
- OEP-C: Ob BLOCKED automatisch aus `action_required` folgt oder Analysten-
  Verdict bleibt.
