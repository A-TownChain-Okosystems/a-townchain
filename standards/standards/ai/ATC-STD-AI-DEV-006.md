---
standard:
  id: ATC-STD-AI-DEV-006
  title: "ATC-STD-AI-DEV-006 — AI Decision & Action Standard"
  version: "1.1.0"
  status: approved
  category: ai-dev
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-AI-DEV-006 — AI Decision & Action Standard (v1.1.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §9 (OBSERVATION vs. DECISION), §8 (Actions)

## Abstract

ATC-STD-AI-DEV-006 (AI Decision & Action Standard) — OBSERVATION/DECISION/reason-Trennung, ACT-NNN-Struktur, Next-Action-Pflicht, Entscheidungsverbund ohne autonome Aenderung); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Trennungsgrundsatz

Jede Entscheidung besteht aus drei getrennten Feldern:
- `observation`: Verifizierter Sachverhalt (mit Fundstelle, AI-DEV-005 §2)
- `decision`: Die daraus abgeleitete Handlung
- `reason`: Warum — mit REQ-/Standard-/Task-Bezug

Eine Modellannahme wird nie als Fakt behandelt; Fakt und Annahme sind
unterschieden (`based_on: evidence` vs. `based_on: ASSUMPTION-ANNN`).

## 2. Action-Struktur (ACT-NNN)

`{id, based_on: [F-NNN], action_type, target, objective, reason,
depends_on: [ACT-NNN], required_before, completion_condition}` —
IDs fortlaufend je Task, nie wiederverwendet.

## 3. Next-Action-Pflicht

Nach jeder abgeschlossenen Aktion existiert eine explizite `next_action`
(AI-DEV-001 §8) oder der Task ist COMPLETED mit Audit-Record (AI-DEV-009).
Implizite "weiter sehen wir dann" ist unzulässig.

## 4. Entscheidungsverbund

Autonome Code-Änderungen ohne dokumentierte Entscheidung+reason sind
Governance-Verstoß. Bei Unsicherheit: ASSUMPTION-Register + ggf. Eskalation
an Owner (AI-DEV-011 §2).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-203, ATC-STD-AI-DEV-001 · INFORMATIVE: Roadmap MK8 (Security), Model-Registry
## 5. Verbindliche Erweiterungsregeln

- 5.1 Beobachtungen (OBSERVATION) und Entscheidungen (DECISION) MÜSSEN strikt getrennt in separaten Datenfeldern protokolliert werden.
- 5.2 Jede Action (ACT-NNN) MUSS vor ihrer Ausführung eine Validierung ihrer Vorbedingungen erfolgreich bestehen.
- 5.3 Für jede auszuführende Action MUSS eine definierte Rollback-Strategie deklariert sein.
- 5.4 Die Folgerung der Next-Action MUSS direkt aus den Ergebnissen der vorherigen Action abgeleitet werden.
- 5.5 Aktionen mit Nebenwirkungen auf externe Systeme MÜSSEN vor der Durchführung vom Agenten als risikobehaftet markiert werden.

## 6. Metriken & Akzeptanzkriterien

- **Metrik 1:** Trennungsquote zwischen Beobachtungen und Entscheidungen MUSS 100% betragen.
- **Metrik 2:** Erfolgsrate von Rollback-Operationen im Fehlerfall MUSS mindestens 99% betragen.
- **Metrik 3:** Vollständigkeit der Next-Action-Deklarationen MUSS 100% sein.
- **Metrik 4:** Zeitdauer zur Entscheidungsfindung MUSS unter 1000 ms liegen.

## 7. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 8. Security Considerations

- Entscheidungslogik MUSS deterministisch und nachvollziehbar aufgebaut sein.
- Nicht-autorisierte Befehlsausführungen MÜSSEN durch strikte Befehls-Allowlists verhindert werden.
- Aktionen mit hohem Schadenspotenzial MÜSSEN vor Ausführung im Trockenlauf (Dry-Run) getestet werden.

## 9. Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initial release / Sammelfreigabe |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 10. References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- ATC-STD-AI-DEV-001
- ATC-STD-AI-DEV-006

## 11. Verbindliche Erweiterungsregeln

- 5.1 Beobachtungen (OBSERVATION) und Entscheidungen (DECISION) MÜSSEN strikt getrennt in separaten Datenfeldern protokolliert werden.
- 5.2 Jede Action (ACT-NNN) MUSS vor ihrer Ausführung eine Validierung ihrer Vorbedingungen erfolgreich bestehen.
- 5.3 Für jede auszuführende Action MUSS eine definierte Rollback-Strategie deklariert sein.
- 5.4 Die Folgerung der Next-Action MUSS direkt aus den Ergebnissen der vorherigen Action abgeleitet werden.
- 5.5 Aktionen mit Nebenwirkungen auf externe Systeme MÜSSEN vor der Durchführung vom Agenten als risikobehaftet markiert werden.

## 12. Metriken & Akzeptanzkriterien

- **Metrik 1:** Trennungsquote zwischen Beobachtungen und Entscheidungen MUSS 100% betragen.
- **Metrik 2:** Erfolgsrate von Rollback-Operationen im Fehlerfall MUSS mindestens 99% betragen.
- **Metrik 3:** Vollständigkeit der Next-Action-Deklarationen MUSS 100% sein.
- **Metrik 4:** Zeitdauer zur Entscheidungsfindung MUSS unter 1000 ms liegen.
- **Metrik 5:** Konsistenzquote der ACT-NNN-Ketten MUSS 100% erreichen.

## 13. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 14. Security Considerations

- Entscheidungslogik MUSS deterministisch und nachvollziehbar aufgebaut sein.
- Nicht-autorisierte Befehlsausführungen MÜSSEN durch strikte Befehls-Allowlists verhindert werden.
- Aktionen mit hohem Schadenspotenzial MÜSSEN vor Ausführung im Trockenlauf (Dry-Run) getestet werden.

## 15. Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initial release / Sammelfreigabe |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 16. References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- ATC-STD-AI-DEV-001
- ATC-STD-AI-DEV-006
