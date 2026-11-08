---
standard:
  id: ATC-STD-AI-DEV-007
  title: "ATC-STD-AI-DEV-007 — AI Git Commit & Pull Request Standard"
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

# ATC-STD-AI-DEV-007 — AI Git Commit & Pull Request Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe (SCR-0006) 07.09.2026 (ATC-STD-000 §9); normativ in Kraft · **Reihe:** ATC-STD-AI-DEV-001…012
> **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Kernprinzip:** Commit und PR sind die öffentlichen, maschinenlesbaren
> Nachweise der Agentenarbeit. GitHub-Automationen werten die Historie
> ausschließlich aus Trailern und PR-Struktur aus.
> **Referenzen:** ATC-STD-AI-DEV-001 (§11 GitHub-Erkennbarkeit), ATC-STD-AI-DEV-004 (Task-IDs), AGENT_PROTOCOL.md (Übergang §6)

---

## Abstract

AI-DEV-007 normiert Commit-Format, Commit-Trailer, Branch-Namen, Pull-Request-
Struktur und Labels für KI-Agenten. Aussagen sind verbindlich (MUSS/SOLLTE/DARF
im Sinne von RFC 2119). Ziel: Jeder Commit und jedes PR ist ohne
Kontextwissen einer externen Person (oder eines anderen Agenten) vollständig
zuordbar: welcher Agent, welcher Task, welches Finding, welche Aktion, welches
Validierungsergebnis.

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Commit-Format (normativ)

```
ATC-TASK-NNNN: <type>: <kurzbeschreibung>

<optionaler Body: OBSERVATION vs. DECISION mit reason (AI-DEV-001 §9)>

Agent-ID: ATC-AI-DEV-001
Task-ID: ATC-TASK-00427
Finding-ID: F-001
Action-ID: ACT-004
AI-Role: software-development
Validation: PASS|FAIL|PENDING
```

- `<type>`: `feat|fix|docs|test|refactor|security|build|ci|chore|spec` (Conventional Commits,
  V-16-kompatibel).
- Trailer sind verpflichtend; fehlende Trailer = Commit gilt als
  menschlicher Commit und wird als Agentenarbeit zurückgewiesen.
- Pro Commit genau EIN Agent; gemischte Commits sind verboten.

## 2. Commit-Trailer-Kompatibilität (Übergang)

Der bisherige `[agent: …]`-Inline-Tag aus AGENT_PROTOCOL.md bleibt bis Tag 30
nach APPROVED von AI-DEV-001 gültig (AI-DEV-001 §14). Ab dann ist ausschließlich
der Trailer-Block aus §1 normativ. Übergangscommits dürfen BEIDE tragen.

## 3. Branch-Namen

```
ai/ATC-TASK-NNNN[-<slug>]
```

Ein Branch gehört zu genau einem Task (AI-DEV-004 §4). Direkte Commits auf
`main` durch Agenten sind verboten (`no_direct_main_merge`).

## 4. Pull-Request-Format (normativ)

**Titel:** `[AI] ATC-TASK-NNNN <kurzbeschreibung>`

**Body (Pflichtabschnitte):**

```markdown
## Agent
- ID: ATC-AI-DEV-001 · Role: Software Development Agent · Version: 1.0.0

## Task
ATC-TASK-00427 (Issue #427)

## Findings
- F-001 — Parser-Regel fehlt

## Changes
- Modified src/parser.rs
- Added tests/parser.rs

## Evidence
- src/parser.rs:120-184
- ATC-STD-042 §4.2

## Validation
- cargo test --workspace: PASS (CI-Run #1234)

## Next Action
ACT-005 — Integration Testing

## Human Decision Required
No
```

Fehlende Abschnitte = PR unvollständig; Merge-Gate verweigert (§6).

## 5. Labels

Verbindlich auf Task-Issues, Branches/PRs, wo anwendbar:
`ai`, `ai-agent`, `ai-generated` (vollautomatisch), `ai-assisted` (mit Human),
`agent:<name>`, `status:<state>` (discovery|implementation|testing|review|blocked),
`authority:<read|write|review|merge>`.

## 6. Merge-Gate

Ein AI-PR ist mergefähig nur wenn:
1. alle §4-Pflichtabschnitte vorhanden,
2. Validation einen grünen CI-Run referenziert,
3. der zugehörige Task-Record `READY_FOR_REVIEW` ist,
4. `human_review.required: true` → Human-Approval dokumentiert ist
   (AI-DEV-011, geplant),
5. keine unbestätigten pflichtblockierenden Annahmen vorliegen.

## 7. Verbotene Muster

- Commit ohne Task-ID (außer rein menschliche Commits).
- Force-Push auf `main` oder Review-Zweige durch Agenten.
- PR-Body als Freitext ohne Pflichtstruktur.
- Secrets/Token in Commits, Branches oder PR-Bodies (`no_secret_access`).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

| 1.0.1 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-203, ATC-STD-AI-DEV-001 · INFORMATIVE: Roadmap MK8 (Security), Model-Registry
## 8. Verbindliche Erweiterungsregeln

- 8.1 Alle durch den Agenten erzeugten Commits MÜSSEN mittels GPG oder SSH kryptografisch signiert sein.
- 8.2 Commit-Nachrichten MÜSSEN dem konventionellen Standard folgen und zwingend eine gültige Task-ID enthalten.
- 8.3 Ein Pull Request MUSS automatisch gecancelt werden, wenn die verknüpften CI-Validation-Checks fehlschlagen.
- 8.4 PRs dürfen einen Umfang von maximal 500 geänderten Zeilen nicht überschreiten, um Überprüfungsqualität zu sichern.
- 8.5 Sämtliche geforderten Trailer-Felder MÜSSEN im Footer der Commit-Nachricht lückenlos vorhanden sein.

## 9. Metriken & Akzeptanzkriterien

- **Metrik 1:** Signaturquote aller Agenten-Commits MUSS exakt 100% betragen.
- **Metrik 2:** Konformitätsrate der Commit-Trailer MUSS 100% erreichen.
- **Metrik 3:** Erfolgsquote beim automatisierten Mergen fehlerfreier PRs MUSS über 98% liegen.
- **Metrik 4:** Bearbeitungszeit fehlerhafter Commits MUSS unter 300 Sekunden liegen.

## 10. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 11. Security Considerations

- Ein direkter Merge in den Main-Branch MUSS ohne Owner-Freigabe technisch unterbunden werden.
- Commit-Signaturen MÜSSEN vor jedem Merge serverseitig verifiziert werden.
- Schlüsselmaterial zur Signierung MUSS in HSM/KMS gesichert werden.

## 12. Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.1 | 2026-09-07 | Initial release / Sammelfreigabe |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 13. References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- ATC-STD-042
- ATC-STD-AI-DEV-001
- ATC-STD-AI-DEV-004
- ATC-STD-AI-DEV-007

## 14. Verbindliche Erweiterungsregeln

- 8.1 Alle durch den Agenten erzeugten Commits MÜSSEN mittels GPG oder SSH kryptografisch signiert sein.
- 8.2 Commit-Nachrichten MÜSSEN dem konventionellen Standard folgen und zwingend eine gültige Task-ID enthalten.
- 8.3 Ein Pull Request MUSS automatisch gecancelt werden, wenn die verknüpften CI-Validation-Checks fehlschlagen.
- 8.4 PRs dürfen einen Umfang von maximal 500 geänderten Zeilen nicht überschreiten, um Überprüfungsqualität zu sichern.
- 8.5 Sämtliche geforderten Trailer-Felder MÜSSEN im Footer der Commit-Nachricht lückenlos vorhanden sein.

## 15. Metriken & Akzeptanzkriterien

- **Metrik 1:** Signaturquote aller Agenten-Commits MUSS exakt 100% betragen.
- **Metrik 2:** Konformitätsrate der Commit-Trailer MUSS 100% erreichen.
- **Metrik 3:** Erfolgsquote beim automatisierten Mergen fehlerfreier PRs MUSS über 98% liegen.
- **Metrik 4:** Bearbeitungszeit fehlerhafter Commits MUSS unter 300 Sekunden liegen.
- **Metrik 5:** Validierungsrate der PR-Metadaten MUSS 100% betragen.

## 16. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 17. Security Considerations

- Ein direkter Merge in den Main-Branch MUSS ohne Owner-Freigabe technisch unterbunden werden.
- Commit-Signaturen MÜSSEN vor jedem Merge serverseitig verifiziert werden.
- Schlüsselmaterial zur Signierung MUSS in HSM/KMS gesichert werden.

## 18. Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.1 | 2026-09-07 | Initial release / Sammelfreigabe |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 19. References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- ATC-STD-042
- ATC-STD-AI-DEV-001
- ATC-STD-AI-DEV-004
- ATC-STD-AI-DEV-007
