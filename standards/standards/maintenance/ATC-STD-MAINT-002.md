---
standard:
  id: ATC-STD-MAINT-002
  title: "ATC-STD-MAINT-002 — Maintenance Lifecycle Standard"
  version: "1.0.0"
  status: approved
  category: maint
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-14"
  updated: "2026-09-14"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-14"
  review_date: null
  applies_to: "Alle Maintenance-Aufgaben des Oekosystems (praeventiv, korrektiv, adaptiv, sicherheitskritisch)"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-MAINT-002 — Maintenance Lifecycle Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — §9-Freigabe erteilt per Sammelfreigabe SCR-0124 (Owner-Direktive 14.09.2026). Dach: ATC-STD-MAINT-000.

## Abstract

Einheitlicher Maintenance-Lifecycle fuer das gesamte Oekosystem: 13 Phasen von DETECT bis CLOSE,
kompatibel mit der bestehenden GSEPF-Engineering-Governance (DISCOVER..MERGE).

## §1 Die 13 Phasen (normativ)

```
DETECT → ASSESS → CLASSIFY → PLAN → APPROVE → IMPLEMENT → TEST
   → VALIDATE → DEPLOY → MONITOR → EVIDENCE → CLOSE
```

| Phase | Inhalt | Exit-Kriterium |
|---|---|---|
| DETECT | Finding/Bedarf erkannt (Scanner, Test, Audit, Report) | Maintenance-Item angelegt |
| ASSESS | Auswirkung, Komponente, Dringlichkeit bewertet | Bewertung dokumentiert |
| CLASSIFY | Klasse M0-M3 zugeordnet (MAINT-001) | Klasse im Item vermerkt |
| PLAN | Massnahme, Scope, Rollen (SoD bei M2/M3) geplant | Plan dokumentiert |
| APPROVE | Review-Gate je Klasse (M0 Standard..M3 Emergency) | Freigabe erteilt |
| IMPLEMENT | Aenderung umgesetzt | Commit vorhanden |
| TEST | Unit/Integration getestet | Tests PASS |
| VALIDATE | Unabhaengige Validierung (Validator ≠ Implementer bei M2/M3) | Validations-Report |
| DEPLOY | Ausbringung (Release/Pipeline) | Deployment-Evidence |
| MONITOR | Post-Deploy-Beobachtung (Health, Regression) | Health PASS |
| EVIDENCE | Evidence-Record vervollstaendigt (MAINT-019) | Record vollständig |
| CLOSE | Abschluss mit Klassifikations- und KPI-Vermerk | Status CLOSED |

## §2 Kompatibilitaet mit GSEPF (kein paralleles Chaos-System)

| GSEPF | Maintenance-Lifecycle |
|---|---|
| DISCOVER / UNDERSTAND | DETECT / ASSESS |
| PLAN | CLASSIFY / PLAN |
| HUMAN APPROVAL | APPROVE |
| IMPLEMENT / TEST | IMPLEMENT / TEST |
| AUDIT | VALIDATE |
| REVIEW / COMMIT / PR / MERGE / DEPLOY | DEPLOY / MONITOR |
| DOCUMENT | EVIDENCE |
| — | CLOSE |

## §3 Regeln (normativ)

1. Kein CLOSE ohne vollstaendige EVIDENCE-Phase („No Evidence, No Trust").
2. M3-Aufgaben durchlaufen die Phasen beschleunigt, aber vollstaendig (Emergency-Pfad §32).
3. Phasen duerfen fuer M0 zusammengefasst dokumentiert werden, solange Evidence vollstaendig ist.

## REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-MAINT-021 | Der 13-Phasen-Lifecycle ist verbindlich | MUST |
| REQ-MAINT-022 | Kein CLOSE ohne vollstaendigen Evidence-Record | MUST |
| REQ-MAINT-023 | M2/M3: VALIDATE durch von IMPLEMENTER unabhaengige Rolle | MUST |
| REQ-MAINT-024 | Lifecycle-Dokumentation ist maschinenlesbar (MAINT-019-Schema) | SHOULD |

## Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Bestehende Ablaeufe (BUG/UPDATE/CI-Gates) decken Teile des Lifecycles ab;
eine formale 13-Phasen-Durchfuehrung ist NICHT implementiert. CLAIMED != PASS.

## Compliance

Dach: ATC-STD-MAINT-000; Klassifikation: MAINT-001; Evidence: MAINT-019.
