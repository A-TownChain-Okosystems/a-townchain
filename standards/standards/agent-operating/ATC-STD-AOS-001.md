---
standard:
  id: ATC-STD-AOS-001
  title: "ATC Agent Operating Standard — Verbindliches Session-Mandat für alle KI-Agenten: die 14 Session-Fragen, Session-Lifecycle, maschinenlesbarer Session-Record, Audit-Nachweis"
  version: "1.0.0"
  status: approved
  category: agent-operating
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 08.09.2026, 00:18 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-FRAMEWORK-001
    - ATC-STD-AI-DECISION-001
    - ATC-STD-AUDIT-001
  related_standards:
    - ATC-STD-BUG-005
    - ATC-STD-MILESTONE-001
    - ATC-STD-REPO-AUDIT-001
    - ATC-STD-UPDATE-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-AOS-001 — ATC Agent Operating Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 08.09.2026, 00:18 UTC+2);
> normativ in Kraft ab 08.09.2026, §30-eingefroren (ATC-STD-000). SCR-0022 akzeptiert.
> **Familie:** Agent Operating (ATC-STD-AOS-001..999, FAM-20). **Kopplungen:**
> AI-DECISION-001 (Human Gates übergeordnet), AAS-001..025, FRAMEWORK-001 §8,
> AUDIT-001, BUG-005.

## Abstract

ATC-STD-AOS-001 macht das 14-Fragen-Mandat des ATC Enterprise Standards Framework
(FRAMEWORK-001 §8) zum vollwertigen, verbindlichen Operating Standard: Jeder KI-Agent
MUSS vor jeder Aktionsreihe seine Session-Identität, Rolle, Rechte, sein Ziel-Repository,
die aktuelle Version, die geltenden Standards und Anforderungen, den bisherigen Stand,
den nächsten zulässigen Schritt, seine Verifikations-, Test-, Dokumentations- und
Audit-Pflichten beantworten — nachvollziehbar dokumentiert in einem maschinenlesbaren
Session-Record. Damit wird aus Agenten-Arbeiten eine auditierbare, traceierbare
Betriebsführung.

Schlüsselwörter: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE (empfohlen),
DARF/KANN (optional) — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Alle KI-Agenten des Ökosystems (Entwicklungs-, Audit-, Governance- und
Betriebsagenten), insbesondere den entwickelnden Agenten, in jeder Session und vor
jeder Aktionsreihe (Commit, PR, Release, Audit, Registry-Änderung).

**Gilt nicht:** In-Product-KI innerhalb von Anwendungen (dafür AAS-Familie); manuelle
menschliche Arbeit (kann das Mandat freiwillig übernehmen).

## §1 Zweck

Das Mandat verhindert die drei Hauptversagen von Agentenarbeit: Kontextlosigkeit
(„welcher Stand?"), Autoritätsüberschreitung („darf ich das?") und
Nachweislosigkeit („was habe ich warum geändert?"). Jede der 14 Fragen MUSS vor der
ersten wirksamen Aktion der Session beantwortbar sein (REQ-AOS-001..014).

## §2 Session-Lifecycle

```
SESSION_START
  ↓ 14 Fragen beantworten (aus autorisierten Quellen, §3)
  ↓ Session-Record erzeugen (§4) — vor der ersten wirksamen Aktion
  ↓ Aktionsreihe (innerhalb der beantworteten Grenzen)
  ↓ Verifikation (Tests/Validator/Gates)
  ↓ Dokumentations-Sync + Audit-Nachweis
  ↓ SESSION_END (Record abgeschlossen: Ergebnisse, Findings, Evidence-Refs)
```

Bei Session-Abbruch MUSS der Record den erreichten Zustand festhalten. Neue
Aktionsreihen außerhalb des beantworteten Kontexts erfordern einen neuen Record
(REQ-AOS-015).

## §3 Die 14 Session-Fragen mit Pflicht-Quellen

| # | Frage | Autorisierte Quelle |
|---|---|---|
| 1 | Wer bin ich? | Agent-Manifest (.github/ai/agent.yaml, AGENT_MANIFEST.md), App-ID |
| 2 | Welche Rolle habe ich? | Rollendefinition (AAS-Familie, Manifest) |
| 3 | Welche Rechte habe ich? | AI-DECISION-001 Human Gates, Tool-Permissions, 14-Regeln |
| 4 | Welches Repository bearbeite ich? | git remote/branch, Registry repositories.yaml |
| 5 | Welche Version ist aktuell? | VERSION-001, Registry versions.yaml, git log |
| 6 | Welche Standards gelten? | registry/standards.yaml (bindend), Agent-Manifest-Bindung |
| 7 | Welche Anforderungen gelten? | SCR, Issues, AD-Register, Milestones (REQ-Quellen) |
| 8 | Was wurde bereits erledigt? | git log, REALITY_STATUS.md, Sprint-/Sektion-Doku |
| 9 | Was ist der nächste zulässige Schritt? | Governance: SCR-Prozess, UPD-G01..G09, Milestone-Lifecycle |
| 10 | Wie verifiziere ich meine Änderung? | Tests, atc-std-validator, Gates (S-01..S-22) |
| 11 | Welche Dateien wurden geändert? | git status/diff (live, je Aktion) |
| 12 | Welche Tests müssen ausgeführt werden? | Repo-Testsuite, ATC-STD-202/203-Konventionen |
| 13 | Welche Dokumentation muss synchronisiert werden? | README/STATUS/CHANGELOG, REQ-RA-016-Regel |
| 14 | Welcher Audit-Nachweis entsteht? | Commit-Trailer, AUD-Record, Evidence-Pack |

Fragen 1–10 MÜSSEN vor der ersten Aktion beantwortet sein; 11–14 begleiten die
Aktionsreihe und MÜSSEN bei SESSION_END vollständig dokumentiert sein (REQ-AOS-016).

## §4 Session-Record (maschinenlesbar)

Governance-relevante Sessions (Commits, PRs, Releases, Audits, Registry-Änderungen)
MÜSSEN einen Session-Record erzeugen (REQ-AOS-015): `AOS-SESS-YYYYMMDD-NNN`
(NNN je Agent fortlaufend je Tag) als YAML/JSON mit: agent_id, agent_name, repo,
branch, base_commit, die 14 Antworten (je Wert + Quell-Ref), Aktionen (je Aktion:
Art, Dateien, Befehl/Tool, Ergebnis), Verifikations-Output (Tests/Validator),
Findings (F-NNN, falls entstanden), Head-Commit, Zeitstempel. Aufbewahrung gemäß
Audit-Retention (AUDIT-001); Storage im Ziel-Repo (docs/ oder audit-Verzeichnis)
ODER im AUD-Record-Verbund.

## §5 Governance-Kopplung

- **Human Gates bleiben übergeordnet** (AI-DECISION-001): Der Mandat sagt, was der
  Agent über sich wissen MUSS — nicht, was er ohne Freigabe DARF.
- Verstöße gegen das Mandat (Aktion ohne beantwortete Fragen/Record) sind
  **governance-widrig** und MÜSSEN als Finding (BUG-005, F-NNN) erfasst werden
  (REQ-AOS-017).
- Der Session-Record ist Evidence im Sinne von MILESTONE-001 §6 und AUDIT-001.
- Agenten-Audits nach REPO-AUDIT-001/002 setzen ein beendetes Mandat voraus.

## Requirements (normativ)

- **REQ-AOS-001** (§3): Identität MUSS vor Aktionsbeginn aus dem autorisierten
  Manifest bezogen sein.
- **REQ-AOS-002** (§3): Die Rolle MUSS deklariert sein; Rollen ohne Definition sind
  unzulässig.
- **REQ-AOS-003** (§3): Rechte-/Verbotsrahmen MUSS bekannt sein; Human-Gate-
  Gegenstände DÜRFEN nicht übergangen werden.
- **REQ-AOS-004** (§3): Ziel-Repository und -Branch MÜSSEN feststehen.
- **REQ-AOS-005** (§3): Die aktuelle Version MUSS aus der Registry/Version-Quelle
  stammen — nicht aus Annahmen.
- **REQ-AOS-006** (§3): Die bindenden Standards MÜSSEN geladen sein (Manifest-
  Bindung genügt als Referenz).
- **REQ-AOS-007** (§3): Aktive Anforderungen MÜSSEN identifiziert sein.
- **REQ-AOS-008** (§3): Der bisherige Stand MUSS aus kanonischen Quellen ermittelt
  sein.
- **REQ-AOS-009** (§3): Der nächste Schritt MUSS governance-zulässig sein
  (SCR/UPD/Milestone-Lifecycle).
- **REQ-AOS-010** (§3): Das Verifikationsverfahren MUSS vorab feststehen.
- **REQ-AOS-011** (§3): Geänderte Dateien MÜSSEN je Aktion erfasst werden.
- **REQ-AOS-012** (§3): Die Testpflicht MÜSSEN benannt und erfüllt werden.
- **REQ-AOS-013** (§3): Die Doku-Synchronisation MUSS geplant und durchgeführt
  werden (Kopplung REQ-RA-016).
- **REQ-AOS-014** (§3): Der entstehende Audit-Nachweis MUSS benannt werden.
- **REQ-AOS-015** (§2/§4): Governance-relevante Sessions MÜSSEN einen
  maschinenlesbaren Session-Record erzeugen; neue Kontexte erfordern einen neuen
  Record.
- **REQ-AOS-016** (§2): Fragen 1–10 vor der ersten Aktion, 11–14 bis SESSION_END
  vollständig.
- **REQ-AOS-017** (§5): Mandatsverstöße SIND Findings (BUG-005); Human Gates
  bleiben übergeordnet.

## Security Considerations

Session-Records DÜRFEN keine Secrets/Token enthalten (nur Quell-Referenzen). Agent-
Identitäten MÜSßen aus autorisierten Manifesten stammen — Selbstdeklaration ohne
Manifest-Bindung ist unzulässig. Records sind manipulationsrelevant (Audit-Evidence)
und unterliegen der Audit-Retention; nachträgliche Änderungen sind nur als
gekennzeichnete Korrekturen zulässig.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.0.0** (2026-09-08): Initial Release — Ausarbeitung des 14-Fragen-Mandats
  aus FRAMEWORK-001 §8 (Owner-Entwurf 07.09. 23:41) zum eigenständigen Operating
  Standard: Session-Lifecycle, 14 Fragen mit autorisierten Quellen, maschinenlesbarer
  Session-Record (AOS-SESS-YYYYMMDD-NNN), Governance-Kopplungen (AI-DECISION-001,
  AUDIT-001, BUG-005, MILESTONE-001, REPO-AUDIT-001), 17 REQ-AOS. SCR-0022;
  §9-Freigabe ausstehend.

## References

- ATC-STD-FRAMEWORK-001 (§8 Mandat, REQ-FW-009, FAM-20)
- ATC-STD-AI-DECISION-001 (Human Gates), ATC-AAS-001..025 (Agenten-Rollen)
- ATC-STD-AUDIT-001 (AUD-Records, Retention), ATC-STD-BUG-005 (Findings/RCA)
- ATC-STD-MILESTONE-001 (§6 Evidence, §13 Human Gate), ATC-STD-REPO-AUDIT-001
- ATC-STD-000 (§14.1 Rollenmodell), ATC-STD-VERSION-001

*ATC-STD-AOS-001 v1.0.0 · Owner-Entwurf Michael Wroblewski · Aurora (Superagent) · 08.09.2026 · SCR-0022*
