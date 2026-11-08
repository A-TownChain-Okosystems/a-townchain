---
standard:
  id: ATC-STD-AI-DEV-004
  title: "ATC-STD-AI-DEV-004 — AI Task Management Standard"
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

# ATC-STD-AI-DEV-004 — AI Task Management Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026 (ATC-STD-000 §9); normativ in Kraft · **Reihe:** ATC-STD-AI-DEV-001…012
> **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Kernprinzip:** Jede Agentenarbeit hat eine Task-ID, die Issue → Branch →
> Commits → PR → Tests → Doku → Audit verbindet. Ein Task ist niemals implizit.
> **Referenzen:** ATC-STD-AI-DEV-001 (§7 Task Identity & State Machine), ATC-STD-000 (§37 ID-Allokation), ATC-STD-BUG-001 (Findings)

---

## Abstract

AI-DEV-004 normiert Erzeugung, Struktur, Lifecycle und Übergabe von Agenten-Tasks.
Die Task-ID `ATC-TASK-NNNN` ist der rote Faden, an dem alle Artefakte einer
Agentenarbeit hängen — sie ist Voraussetzung dafür, dass ein anderer Agent
einen unterbrochenen Prozess übernehmen kann, ohne die Historie neu zu
interpretieren.

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Task-Record (Pflichtstruktur)

Jeder Task ist maschinenlesbar in `.github/ai/tasks/ATC-TASK-NNNN.yaml` abgelegt
(Vorlage: `templates/ai/task.template.yaml`):

```yaml
task:
  id: ATC-TASK-00427
  title: "Implement parser rule for ATCLang expression syntax"
  created: "2026-09-07T19:00:00Z"
  agent: {id: ATC-AI-DEV-001, version: "1.1.0"}
  task_reference: {issue: "#427", milestone: null, standard: "ATC-STD-042"}
  status: IN_PROGRESS          # Lifecycle §2
  findings: [F-001]
  actions: [ACT-001, ACT-002]
  assumptions: []              # ASSUMPTION-ANNN, verification.required blockiert COMPLETED
  evidence: []                 # Fundstellen (AI-DEV-001 §4)
  next_action: {id: ACT-003, priority: P1, depends_on: [ACT-002]}
  human_review: {required: false, requested: false}
```

Pflichtfelder: `id`, `title`, `created`, `agent`, `task_reference`, `status`,
`findings`, `actions`, `next_action`, `human_review`.

## 2. Task-Lifecycle

```
CREATED → ASSIGNED → IN_PROGRESS → (BLOCKED ⇄ IN_PROGRESS)*
        → READY_FOR_REVIEW → (CHANGES_REQUESTED → IN_PROGRESS)*
        → COMPLETED | CANCELLED
```

Status-Übergänge erfolgen ausschließlich in dieser Reihenfolge; `BLOCKED` und
`CHANGES_REQUESTED` sind Rückkehrschleifen. Jeder Übergang wird im Task-Record
mit Zeitstempel protokolliert (`history`-Liste).

## 3. ID-Allokation

- `ATC-TASK-NNNN` wird fortlaufend vergeben und NIEMALS wiederverwendet
  (ATC-STD-000 §37).
- Task-IDs sind organisationsweit eindeutig; das ausführende Repository führt
  die Nummernvergabe seiner Tasks.

## 4. Verzweigung und Verweise (Traceability)

Ein Task MUSS alle zugehörigen Artefakte referenzieren:

```
ATC-TASK-NNNN
    ├── Issue #NNN
    ├── branch: ai/ATC-TASK-NNNN
    ├── commits (Trailer Task-ID, AI-DEV-007)
    ├── PR [AI] ATC-TASK-NNNN (AI-DEV-007)
    ├── CI-Run-IDs
    ├── Test-Ergebnisse (TEST-NNN)
    ├── Dokumentation
    └── Audit Record AUD-NNN (AI-DEV-009)
```

Umgekehrt MUSS jedes Artefakt die Task-ID tragen (Commit-Trailer, PR-Body,
Audit-Record). Ein Artefakt ohne Task-Referenz gilt als nicht agentenfähig.

## 5. Übergabe (Handover)

Ein Task ist jederzeit übergebefähig, wenn gilt:
- Task-Record ist vollständig (§1),
- `next_action` ist aktuell und explizit (AI-DEV-001 §8),
- offene Annahmen sind im Assumption-Register vermerkt,
- `history` ist lückenlos.

Der übernehmende Agent startet mit `DISCOVERING` (AI-DEV-001 §7) — Task-Record
ersetzt Discovery NICHT, verkürzt sie aber auf das Wesentliche.

## 6. Verbotene Muster

- Implizite Tasks ("die KI hat eben etwas geändert") ohne Task-Record.
- Wiederverwendung abgeschlossener Task-IDs.
- Status-Sprünge ohne protokollierten Übergang (z.B. CREATED → COMPLETED).
- COMPLETED bei `assumptions[*].verification.required: true` ohne Bestätigung.

## 7. Completion

COMPLETED nur nach den Gates aus AI-DEV-001 §12 und Anlage des Audit-Records
(AI-DEV-009). Der Task-Record wird mit `completed:`-Zeitstempel geschlossen
und bleibt dauerhaft abgelegt (keine Löschung).

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
## 8. Verbindliche Erweiterungsregeln

- 8.1 Jedes Task-Record MUSS bei jeder Zustandsänderung atomares Append-Only-Logging auf dem Speicherlaufwerk durchführen.
- 8.2 Die maximale Gleichzeitigkeit von Tasks pro Agent MUSS auf exakt 1 beschränkt bleiben, um Zustandskonflikte zu vermeiden.
- 8.3 Beim Fehlschlagen einer Task-Phase MUSS der Task-Status unverzüglich auf BLOCKED gesetzt und eine Ursachenanalyse angestoßen werden.
- 8.4 Übergaben (Handover) an andere Agenten MÜSSEN ein vollständiges Re-Signing des Task-Records beinhalten.
- 8.5 Task-IDs MÜSSEN nach dem Schema TASK-YYYYMMDD-NNN vergeben und zentral validiert werden.

## 9. Metriken & Akzeptanzkriterien

- **Metrik 1:** Lückenlosigkeit des Task-Lifecycles MUSS 100% betragen.
- **Metrik 2:** Maximal erlaubte Zeitspanne im Zustand IN_PROGRESS ohne Heartbeat MUSS 300 Sekunden betragen.
- **Metrik 3:** Erfolgsquote automatisierter Handover-Validierungen MUSS bei 100% liegen.
- **Metrik 4:** Fehlerquote bei der ID-Allokation MUSS exakt 0% betragen.

## 10. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 11. Security Considerations

- Task-Records MÜSSEN fälschungssicher gespeichert werden.
- Unberechtigte Modifikationen an fremden Tasks MÜSSEN durch Zugriffskontrollen unterbunden werden.
- Verwaiste Tasks MÜSSEN nach Ablauf des Timeouts automatisch in den Status EXPIRED übergehen.

## 12. Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initial release / Sammelfreigabe |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 13. References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- ATC-STD-042
- ATC-STD-AI-DEV-001
- ATC-STD-AI-DEV-004
- ATC-STD-BUG-001

## 14. Verbindliche Erweiterungsregeln

- 8.1 Jedes Task-Record MUSS bei jeder Zustandsänderung atomares Append-Only-Logging auf dem Speicherlaufwerk durchführen.
- 8.2 Die maximale Gleichzeitigkeit von Tasks pro Agent MUSS auf exakt 1 beschränkt bleiben, um Zustandskonflikte zu vermeiden.
- 8.3 Beim Fehlschlagen einer Task-Phase MUSS der Task-Status unverzüglich auf BLOCKED gesetzt und eine Ursachenanalyse angestoßen werden.
- 8.4 Übergaben (Handover) an andere Agenten MÜSSEN ein vollständiges Re-Signing des Task-Records beinhalten.
- 8.5 Task-IDs MÜSSEN nach dem Schema TASK-YYYYMMDD-NNN vergeben und zentral validiert werden.

## 15. Metriken & Akzeptanzkriterien

- **Metrik 1:** Lückenlosigkeit des Task-Lifecycles MUSS 100% betragen.
- **Metrik 2:** Maximal erlaubte Zeitspanne im Zustand IN_PROGRESS ohne Heartbeat MUSS 300 Sekunden betragen.
- **Metrik 3:** Erfolgsquote automatisierter Handover-Validierungen MUSS bei 100% liegen.
- **Metrik 4:** Fehlerquote bei der ID-Allokation MUSS exakt 0% betragen.
- **Metrik 5:** Integritätsrate der Task-State-Records MUSS 100% betragen.

## 16. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 17. Security Considerations

- Task-Records MÜSSEN fälschungssicher gespeichert werden.
- Unberechtigte Modifikationen an fremden Tasks MÜSSEN durch Zugriffskontrollen unterbunden werden.
- Verwaiste Tasks MÜSSEN nach Ablauf des Timeouts automatisch in den Status EXPIRED übergehen.

## 18. Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initial release / Sammelfreigabe |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 19. References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- ATC-STD-042
- ATC-STD-AI-DEV-001
- ATC-STD-AI-DEV-004
- ATC-STD-BUG-001
