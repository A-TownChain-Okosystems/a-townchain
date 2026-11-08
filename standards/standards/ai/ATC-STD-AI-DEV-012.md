---
standard:
  id: ATC-STD-AI-DEV-012
  title: "ATC-STD-AI-DEV-012 — Multi-Agent Coordination Standard"
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

# ATC-STD-AI-DEV-012 — Multi-Agent Coordination Standard (v1.1.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001, AGENT_PROTOCOL (1-Agent-per-Repo)

## Abstract

ATC-STD-AI-DEV-012 (Multi-Agent Coordination Standard) — 1-Agent-per-Repo-Exklusivitaet, Artefakt-basierte Kommunikation, Handover-Regeln, Konfliktbehandlung als Governance-Verstoss); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Repo-Exklusivität

Pro Repository arbeitet zu jedem Zeitpunkt genau EIN Agent (1-Agent-per-Repo,
AgentAssignment-Registry). Repo-Wechsel nur wenn: alle Dateien vollständig,
Tests grün, gepusht, dokumentiert (AGENT_PROTOCOL-Regel).

## 2. Kommunikation über Artefakte

Agenten kommunizieren ausschließlich über maschinenlesbare Artefakte:
Task-Records (AI-DEV-004), Audit-Records (AI-DEV-009), Findings,
Commit-Trailer (AI-DEV-007). Keine Annahmen über internen Zustand anderer
Agenten; der Stand ist im Artefakt oder er existiert nicht.

## 3. Übergabe (Handover)

Übergabe nur bei übergabefähigem Task (AI-DEV-004 §5): vollständiger
Task-Record, aktuelle next_action, offene Annahmen registriert, history
lückenlos. Übernehmender Agent startet mit DISCOVERING (AI-DEV-003) und
vermerkt `handover_from: ATC-AI-…-NNN`.

## 4. Konfliktbehandlung

Bei parallelen Änderungen am gleichen Ziel (Verstoß gegen §1): Owner-
Eskalation (AI-DEV-011 §2), Findings S1, Auflösung durch den zuständigen
Agenten mit Sync-Record (BUG-004). Merge-Konflikte durch Regelverstoß
sind keine technischen Zufälle, sondern Governance-Verstöße.

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

- 5.1 Jeder Agent MUSS die Repo-Exklusivität beachten und darf ausschließlich auf dem ihm zugewiesenen Repository operieren.
- 5.2 Die Kommunikation zwischen verschiedenen Agenten MUSS ausschließlich über definierte Artefakte (z.B. Task-Records, PRs) erfolgen.
- 5.3 Bei Ressourcenkonflikten MUSS eine automatisierte Sperrfunktion (Lock-Datei) aktiviert werden.
- 5.4 Deadlocks in der Multi-Agenten-Koordination MÜSSEN durch Timeouts erkannt und aufgelöst werden.
- 5.5 Handover-Prozesse zwischen Agenten MÜSSEN eine vollständige Konsistency-Prüfung der Übergabedateien durchlaufen.

## 6. Metriken & Akzeptanzkriterien

- **Metrik 1:** Einhaltung der Repo-Exklusivität MUSS 100% betragen.
- **Metrik 2:** Deadlock-Quote bei Multi-Agenten-Operationen MUSS 0% sein.
- **Metrik 3:** Erfolgsquote bei gegenseitigen Handover-Prozessen MUSS über 99% liegen.
- **Metrik 4:** Zeitdauer zur Konfliktauflösung MUSS unter 5 Sekunden betragen.

## 7. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 8. Security Considerations

- Agenten dürfen gegenseitig keine internen Zustände direkt manipulieren.
- Jeglicher Datenaustausch MUSS durch Schema-Validierung abgesichert werden.
- Unbefugte Nachrichtenübermittlung zwischen Agenten MUSS unterbunden werden.

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
- ATC-STD-AI-DEV-012

## 11. Verbindliche Erweiterungsregeln

- 5.1 Jeder Agent MUSS die Repo-Exklusivität beachten und darf ausschließlich auf dem ihm zugewiesenen Repository operieren.
- 5.2 Die Kommunikation zwischen verschiedenen Agenten MUSS ausschließlich über definierte Artefakte (z.B. Task-Records, PRs) erfolgen.
- 5.3 Bei Ressourcenkonflikten MUSS eine automatisierte Sperrfunktion (Lock-Datei) aktiviert werden.
- 5.4 Deadlocks in der Multi-Agenten-Koordination MÜSSEN durch Timeouts erkannt und aufgelöst werden.
- 5.5 Handover-Prozesse zwischen Agenten MÜSSEN eine vollständige Konsistency-Prüfung der Übergabedateien durchlaufen.

## 12. Metriken & Akzeptanzkriterien

- **Metrik 1:** Einhaltung der Repo-Exklusivität MUSS 100% betragen.
- **Metrik 2:** Deadlock-Quote bei Multi-Agenten-Operationen MUSS 0% sein.
- **Metrik 3:** Erfolgsquote bei gegenseitigen Handover-Prozessen MUSS über 99% liegen.
- **Metrik 4:** Zeitdauer zur Konfliktauflösung MUSS unter 5 Sekunden betragen.
- **Metrik 5:** Konsistenz-Prüfrate übergebener Multi-Agenten-Artefakte MUSS 100% erreichen.

## 13. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 14. Security Considerations

- Agenten dürfen gegenseitig keine internen Zustände direkt manipulieren.
- Jeglicher Datenaustausch MUSS durch Schema-Validierung abgesichert werden.
- Unbefugte Nachrichtenübermittlung zwischen Agenten MUSS unterbunden werden.

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
- ATC-STD-AI-DEV-012
