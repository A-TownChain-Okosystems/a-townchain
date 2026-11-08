---
standard:
  id: ATC-STD-AI-DEV-008
  title: "ATC-STD-AI-DEV-008 — AI Testing & Validation Standard"
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

# ATC-STD-AI-DEV-008 — AI Testing & Validation Standard (v1.1.0, APPROVED)

> **Status:** APPROVED — Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9)
> **Reihe:** ATC-STD-AI-DEV-001…012 · **Basiert auf:** AI-DEV-001 §7 (TESTING/VALIDATING), §12 (Test Gate)

## Abstract

ATC-STD-AI-DEV-008 (AI Testing & Validation Standard) — Test-Pflicht je Aenderung, Validierung nur mit CI-Run-Referenz (TEST-NNN), Fail-Handling (DEBUGGING-Pflicht, kein Bypass), Uebergabe an Audit-Record); sofort APPROVED per Owner-Sammelfreigabe 07.09.2026 (ATC-STD-000 §9.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: AI-Entwicklungstätigkeiten von Agenten in allen ATC-Repositories (Manifeste, Commits, PRs, Tests, Dokumentation, Audit).
Nicht-Gilt: menschliche Entwicklungsprozesse ohne Agentenbezug; Enterprise-Entscheidungen (ATC-ENT).

## 1. Test-Pflicht

Keine IMPLEMENTING-Änderung ohne zugehörige Tests (neu oder erweitert).
Ausnahmen nur bei reinen Dokumentations-/Konfigurations-Änderungen — im
Task-Record als solche deklariert.

## 2. Validierungsergebnis

`validation` MUSS je Dimension (tests, lint, build, security) einen Status
(PASS|FAIL|PENDING) UND eine CI-Run-Referenz tragen (AI-DEV-009 §1).
"PASS ohne Run" ist ungültig. Regressionstests werden als TEST-NNN
referenziert (BUG-002).

## 3. Fail-Handling

FAIL → Status TESTING→DEBUGGING (AI-DEV-001 §7), Finding anlegen (S-Skala),
Fix als neue ACT-NNN. Kein COMPLETED mit FAIL. Kein Bypassen roter Tests
(im Build-Skript auskommentierte Tests = Governance-Verstoß, S1).

## 4. Übergabe an Audit

Das Validierungsergebnis ist vollständig in den Audit-Record (AI-DEV-009 §1)
zu übernehmen; konsistenzprüfung code↔tests (§12 Konsistenzmatrix) ist Teil
der Validation.

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

- 5.1 Für jeden generierten oder geänderten Code MUSS eine automatisierte Testsuite mit ausreichender Abdeckung ausgeführt werden.
- 5.2 Tests MÜSSEN in einer isolierten Sandbox-Umgebung ohne Netzwerkzugriff ausgeführt werden.
- 5.3 Testergebnisse MÜSSEN in einem strukturierten XML/JSON-Format zur maschinellen Auswertung abgelegt werden.
- 5.4 Im Falle eines Testfehlers MUSS der Agent die Fehlerausgabe analysieren und als kategorisiertes Finding dokumentieren.
- 5.5 Flaky Tests MÜSSEN isoliert und mit einem entsprechenden Warning-Flag versehen werden.

## 6. Metriken & Akzeptanzkriterien

- **Metrik 1:** Testabdeckung für neu erstellten Code MUSS mindestens 85% betragen.
- **Metrik 2:** Erfolgsquote der Testausführungen vor dem PR-Release MUSS 100% sein.
- **Metrik 3:** Maximale Laufzeit der Testsuite MUSS unter 300 Sekunden liegen.
- **Metrik 4:** Reproduzierbarkeit von Testergebnissen MUSS bei 100% liegen.

## 7. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 8. Security Considerations

- Testskripte MÜSSEN frei von schädlichen Seiteneffekten auf das Wirtssystem sein.
- Die Injektion von Schadcode über Test-Inputs MUSS durch Input-Sanitizing verhindert werden.
- Testdaten MÜSSEN frei von echten Zugangsdaten oder personenbezogenen Daten sein.

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
- ATC-STD-AI-DEV-008

## 11. Verbindliche Erweiterungsregeln

- 5.1 Für jeden generierten oder geänderten Code MUSS eine automatisierte Testsuite mit ausreichender Abdeckung ausgeführt werden.
- 5.2 Tests MÜSSEN in einer isolierten Sandbox-Umgebung ohne Netzwerkzugriff ausgeführt werden.
- 5.3 Testergebnisse MÜSSEN in einem strukturierten XML/JSON-Format zur maschinellen Auswertung abgelegt werden.
- 5.4 Im Falle eines Testfehlers MUSS der Agent die Fehlerausgabe analysieren und als kategorisiertes Finding dokumentieren.
- 5.5 Flaky Tests MÜSSEN isoliert und mit einem entsprechenden Warning-Flag versehen werden.

## 12. Metriken & Akzeptanzkriterien

- **Metrik 1:** Testabdeckung für neu erstellten Code MUSS mindestens 85% betragen.
- **Metrik 2:** Erfolgsquote der Testausführungen vor dem PR-Release MUSS 100% sein.
- **Metrik 3:** Maximale Laufzeit der Testsuite MUSS unter 300 Sekunden liegen.
- **Metrik 4:** Reproduzierbarkeit von Testergebnissen MUSS bei 100% liegen.
- **Metrik 5:** Fehlerquote bei der Generierung von Testberichten MUSS exakt 0% betragen.

## 13. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 14. Security Considerations

- Testskripte MÜSSEN frei von schädlichen Seiteneffekten auf das Wirtssystem sein.
- Die Injektion von Schadcode über Test-Inputs MUSS durch Input-Sanitizing verhindert werden.
- Testdaten MÜSSEN frei von echten Zugangsdaten oder personenbezogenen Daten sein.

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
- ATC-STD-AI-DEV-008
