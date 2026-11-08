---
standard:
  id: ATC-STD-BUG-002
  title: "ATC-STD-BUG-002 — Bug Documentation Standard"
  version: "1.1.0"
  status: approved
  category: bug
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: null
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-BUG-002 — Bug Documentation Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-BUG-001…004 (Bug- & Konsistenz-Lebenszyklus) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Kernprinzip:** Trennung Finding → Dokumentation → Fix → Synchronitätsprüfung
> **Scope:** Alle Findings des Oekosystems ab Registrierung. Nicht gilt: Bestands-Findings F-001…F-005 (grandfathered)\n>\n> **Verweise:** ATC-STD-000 (§7 IDs, §24 Registry), ATC-STD-201/202/203, registry/findings.yaml, change-requests/

---

## Abstract

Jedes Finding wird unter einer eindeutigen ID F-NNN so dokumentiert, dass
die gesamte Fehlerhistorie DURCH Dritte nachvollziehbar ist. Kanonischer
Ort: registry/findings.yaml (kompakter Eintrag) — die Pflichtstruktur
garantiert Vergleichbarkeit.

## 1. Pflichtstruktur (REQ-STD-111: MUST)

Jeder Registry-Eintrag MUSS der Struktur aus templates/finding.template.md
entsprechen:

```yaml
finding_id: F-017
title: "Example failure"
status: OPEN            # OPEN | ANALYZED | IN_PROGRESS | FIXED | VERIFIED | CLOSED | WONT_FIX
severity: S1
repository: {org}/{repo}@{branch}  # commit SHA Pflicht
component: {module, file}
description: "Technische Beschreibung des beobachteten Fehlers"
expected_behavior: "Erwartetes Systemverhalten"
actual_behavior: "Beobachtetes Systemverhalten"
reproduction: {steps: [...], reproducible: true}
root_cause: {status: identified|unknown, description: ...}
impact: {security: ..., functionality: ..., compatibility: ...}
evidence: {logs: ..., tests: ..., screenshots: ...}
related: {requirements: [REQ-STD-001], standards: [ATC-STD-BUG-001]}
scr: SCR-021            # wenn Change Request existiert (Pflicht ab Fix)
tests: [TEST-044]       # Reproduktionstests
sync: [SYNC-012]        # Synchronitaetspruefungen (BUG-004)
audit: AUD-031          # Abschlusspruefung
```

## 2. Nachvollziehbarkeits-Regeln (REQ-STD-112: MUST)

- **Historie vollstaendig:** Jeder Statuswechsel wird mit Datum
  dokumentiert; keine Rueckdatierung.
- **Beleg vor Behauptung:** Evidence (Log-Auszug, Test-Output, Commit-SHA)
  MUSS referenziert sein, wo verfuegbar (Reality-Check-Regel).
- **Erwartet vs. tatsaechlich:** Beide Verhalten MUeSSEN getrennt
  dokumentiert sein — "geht nicht" ist keine Fehlerbeschreibung.
- **Verkettung:** Jedes Finding zeigt auf seinen SCR (falls Fix), seine
  TEST-NNN, SYNC-NNN und AUD-NNN — die Kette ist in beide Richtungen
  pruefbar.

## 3. Bestands-Findings (REQ-STD-113)

Die vor diesem Standard angelegten Findings F-001…F-005 (Naming-Review)
sind Bestand: Severity-Notationen MINOR/MEDIUM/LOW bleiben unberuehrt
(≈ S3/S2/S3); neue Findings nutzen ausschliesslich S0-S4.

## 4. Verweise

ATC-STD-BUG-001 (Finding-Regeln), ATC-STD-BUG-003 (Fix-Lifecycle),
registry/findings.yaml, templates/finding.template.md.\n\n## Requirements\n\n- id: REQ-STD-111\n  title: "Pflichtstruktur je Finding-Eintrag (templates/finding.template.md)"\n  severity: MANDATORY\n- id: REQ-STD-112\n  title: "Nachvollziehbarkeit: Historie, Beleg vor Behauptung, erwartet/tatsaechlich getrennt, Verkettung"\n  severity: MANDATORY\n- id: REQ-STD-113\n  title: "Bestands-Findings F-001…F-005 grandfathered; neue nur S0-S4"\n  severity: CONDITIONAL\n

## Metriken & Akzeptanzkriterien

1. **Evidenz-Vollständigkeitsrate:** MUSS 100% aller OPEN Findings aufweisen; fehlerhafte Einträge blockieren den Merge.
2. **Root-Cause-Erfassungsquote:** MUSS vor Statusübergang zu IN_PROGRESS 100% betragen.
3. **Linkage-Abdeckungsgrad:** MUSS für 100% der FIXED Findings eine gültige SCR-ID in `registry/findings.yaml` nachweisen.
4. **Triagierungszeit-Obergrenze:** Erstbewertung MUSS innerhalb von 24 Stunden nach Registrierung erfolgen.

## Compliance & Verifikation

Geprueft wird per Review der Finding-/SCR-/Merge-Records gegen die oben deklarierten REQ-STD-Anforderungen
(Manual: Review-Chain gemaess ATC-STD-000 §26; automatisiert: Bestandteile
in atc-std-validator/atc-repo-audit, Ausbau dokumentiert in
registry/findings.yaml). Verstoss gegen MANDATORY = NON-COMPLIANT = GATE
BLOCKED (ATC-STD-BUG-004). (Erweiterung: Die Einhaltung aller Dokumentationsstandards MUSS kontinuierlich in den Validator-Läufen S-01 bis S-25 sowie durch Repository-Audits nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 überprüft werden. Unvollständige Finding-Einträge erzeugen automatisierte Findings F-001 bis F-999 und blockieren das Merge-Gate).

## Security Considerations

S0-Kriterien (Konsensbruch, Schluesselkompromittierung,
Protokollintegritaet) haben Vorrang vor allem Funktions-Backlog; der
Fix-Lifecycle erzwingt SECURITY CHECK vor REVIEW (BUG-003). Security-
Relevanz jedes Findings ist im Feld impact.security zu dokumentieren
(BUG-002).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |
Erweiterte Sicherheitsvorgabe: Sicherheitsrelevante Findings (S0/S1) MUESSEN mit höchster Priorität behandelt werden. Vertrauliche Zugangsdaten, Schlüssel oder Secrets DÜRFEN MUSS-gemäß NIEMALS im Evidenz-Feld von Finding-Einträgen veröffentlicht werden.
