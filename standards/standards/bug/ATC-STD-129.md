---
standard:
  id: ATC-STD-129
  title: "Incident Correlation Standard"
  version: "1.2.0"
  status: approved
  category: bug
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards: []
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-129 — Incident Correlation (v1.2.0, APPROVED)

> **Status:** APPROVED (v1.2.0, §30-eingefroren) — Standard aus Katalog-Slot der Familie
> Bug & Fehler-Management (FAM-07); §9-FREIGEGEBEN 08.09.2026, 02:15 UTC+2 — APPROVED, normativ,
> §30-eingefroren. Struktur-Elaboration v1.1.0 via SCR-0031; Slot-Fertigbau v1.2.0 via
> SCR-0034 (08.09.2026, 03:15 UTC+2): slot-spezifische Pruefkriterien (§6) mit eigener REQ-Menge
> je Gegenstand. Engineering-Bindung (Code/Tests) entsteht bei Slot-Aktivierung
> via SCR/MINOR (ehrlich dokumentiert).

## Abstract

ATC-STD-129 (Incident Correlation) ist der Standard für den gleichnamigen Katalog-Slot der Familie
**Bug & Fehler-Management** (FAM-07, Range ATC-STD-120..131) im ATC Enterprise Standards Framework. Er
definiert den Gegenstand, seine Verortung im Ökosystem, die familienweiten
Kernregeln, die slot-spezifischen Prüfkriterien (§6) mit je-Kriterium-Nachweis,
Compliance- und Verifikationspflichten sowie Security-Betrachtungen. Der Standard
ist APPROVED, normativ in Kraft und §30-eingefroren (Owner-§9-Freigabe 08.09.2026,
02:15 UTC+2, SCR-0030-Batch); Elaboration SCR-0031 (v1.1.0) und Slot-Fertigbau
SCR-0034 (v1.2.0) sind additive MINOR-Updates (ATC-STD-UPDATE-001 UPD-G03).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Der durch den Slot-Titel (Incident Correlation) bezeichnete Gegenstandsbereich im
Zuständigkeitsfeld der Familie Bug & Fehler-Management. **Gilt nicht:** Bereiche, die durch
fachlich konkretere Standards derselben Familie verbindlich geregelt sind
(Subsidiarität: der konkrete Standard geht vor).

## §1 Gegenstand & Ökosystem-Verortung

Dieser Standard adressiert **Incident Correlation** im Katalog-Slot ATC-STD-129 der Familie Bug & Fehler-Management
(FAM-07); die Zuordnung folgt registry/framework.yaml (S-21) und darf nur via SCR
geändert werden.

Fehler-Management: Findings F-NNN in registry/findings.yaml (SSOT), RCA-Kette ATC-STD-BUG-005, Priorisierung P0–P4.

**Katalog-Referenz:** keine zusätzliche Katalog-Notiz; Verortung ausschließlich über Familie und Slot.

## §2 Kernregeln (familienweit, elaboriert)

1. **KR-1:** Jeder Fehler MUSS eindeutig identifizierbar sein (F-NNN, Kategorie, Schwere, Entdecker, Datum).
2. **KR-2:** Fehler DÜRFEN nicht doppelt geführt werden (Duplikat-Check bei Anlage).
3. **KR-3:** P0-Fehler blockieren Releases (Health E).
4. **KR-4:** Jeder behobene Fehler MUSS die Ursache dokumentieren (RCA) — Symptom-Fixes allein genügen nicht.
5. **KR-5:** Fehlerzustände in Datenbanken (KaiOsTodo) MÜSSEN mit der Registry konsistent sein; Phantom-Einträge werden bereinigt.
6. **KR-6:** Track-Präfixe (K/WIN/DESKTOP/…) MÜSSEN Namenskollisionen vermeiden.

## §3 Schnittstellen & Kopplungen

- **Registry-Kopplung:** standards.yaml/versions.yaml (Version/Status S-14/S-19),
  Katalog-Slot in registry/framework.yaml.
- **Governance-Kette:** SCR → VERSION → UPDATE (UPD-G03 MINOR) → COMPAT (bei MAJOR)
  → AUDIT; Findings via registry/findings.yaml (F-NNN).
- **Nachbarfamilien:** Familie Bug & Fehler-Management — Subsidiarität: konkretere Standards gehen vor.
- **Agenten-Bindung:** Vollmandat via .github/ai/agent.yaml; Umsetzungspflicht nach
  AGENT_MANIFEST.

## §4 Metriken & Akzeptanzkriterien

- **M1:** 0 doppelte Findings
- **M2:** RCA-Quote 100 % bei P0/P1
- **M3:** P0-Aufräumzeit im Ziel

- **M4:** Slot-Fertigbau: 8 verbindliche Prüfkriterien (§6) mit Nachweisangabe
  deklariert; Abdeckung nachzuweisen via AUD-Record bei Slot-Aktivierung.

Akzeptanz gilt als nachgewiesen, wenn die genannten Kriterien in einem AUD-Record
oder Validator-Lauf dokumentiert sind; fehlende Nachweise werden als Findings
geführt und nach ATC-STD-BUG-005 (RCA) bearbeitet.

## §5 Compliance & Verifikation

Compliance wird über die Gesamt-Validierung (CI, S-01..S-25) je Registry-Eintrag
geprüft: Metadaten-Vollständigkeit, Naming, Status-/Version-Konsistenz und
Registry-Konsistenz. Abweichungen werden als Findings (F-NNN) geführt und nach
ATC-STD-BUG-005 (RCA) bearbeitet.

## §6 Slot-Spezifikation Incident Correlation — verbindliche Prüfkriterien

Jedes Kriterium ist normativ (MUSS). Nachweis je Kriterium: Konzept-/Design-Dokument
plus AUD-Record, oder Validator-/Testlauf — je nach Art des Kriteriums; bei
Slot-Aktivierung wird der Nachweis je Kriterium einzeln erbracht.

- **P1** (MUSS): Erfassungsschema (Zeit, Schwere, Betroffenheit) — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P2** (MUSS): Eindaemmung vor Ursachenanalyse (Runbook) — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung
- **P3** (MUSS): Kommunikations- und Eskalationspflichten — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P4** (MUSS): RCA- und Postmortem-Pflicht — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung
- **P5** (MUSS): Praventionsableitung und Regelrevision — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P6** (MUSS): Pipeline-Stufen und Gate-Definitionen — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung
- **P7** (MUSS): Blockaderegeln (rot blockiert) — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P8** (MUSS): Zugriffsschutz fuer Gate-Aenderungen (GH013) — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung

## Requirements (normativ)

- **REQ-STD-001** (§1): Gegenstand eindeutig definiert und im Katalog verortet.
- **REQ-STD-002** (§2): Fachliche Regeln als deklarierte, verifizierbare REQ-IDs.
- **REQ-STD-003** (§2): Compliance nachweisbar über Validator-Gates oder Prüfung.
- **REQ-STD-004** (§2): Änderungen ausschließlich über die Change-Control-Kette.
- **REQ-STD-005** (§2): Sicherheitsaspekte dokumentiert (Security Considerations).
- **REQ-STD-006**: Ökosystem-Verortung nachgewiesen (§1).
- **REQ-STD-007**: Familien-Kernregeln KR-1..KR-6 eingehalten und verifizierbar (§2).
- **REQ-STD-008**: Schnittstellen zu Registry/Governance-Kette/Agenten gebunden (§3).
- **REQ-STD-009**: Metriken M-1..M-4 definiert, Nachweis via AUD-Record (§4).
- **REQ-STD-010**: Familienspezifische Security-Bedrohungen katalogisiert (§5).
- **REQ-STD-011** (§6/P1): Erfassungsschema (Zeit, Schwere, Betroffenheit)
- **REQ-STD-012** (§6/P2): Eindaemmung vor Ursachenanalyse (Runbook)
- **REQ-STD-013** (§6/P3): Kommunikations- und Eskalationspflichten
- **REQ-STD-014** (§6/P4): RCA- und Postmortem-Pflicht
- **REQ-STD-015** (§6/P5): Praventionsableitung und Regelrevision
- **REQ-STD-016** (§6/P6): Pipeline-Stufen und Gate-Definitionen
- **REQ-STD-017** (§6/P7): Blockaderegeln (rot blockiert)
- **REQ-STD-018** (§6/P8): Zugriffsschutz fuer Gate-Aenderungen (GH013)

## Security Considerations

Phantom-Einträge verfälschen Fortschrittsdaten; Symptom-Fixes ohne RCA führen zu Wiederholungsfehlern.

Ehrlichkeitsregel: Sicherheitszustände MÜSSEN ehrlich benannt sein
(ACTIVE/PARTIAL/PLANNED); erfundene Sicherheitszusagen sind verboten (vgl.
ATC-STD-PROTOCOL-003).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.2.0** (2026-09-08): MINOR via SCR-0034 — Slot-Fertigbau: slot-spezifische
  Prüfkriterien (§6, 8 Kriterien mit je-Kriterium-Nachweis), REQ-STD-006..018
  slot-spezifisch, M4-Abdeckungsmetrik ergänzt. Additiv, abwärtskompatibel
  (ATC-STD-UPDATE-001 UPD-G03).
- **1.1.0** (2026-09-08): MINOR via SCR-0031 — Struktur-Elaboration: familien-spezifische
  Kernregeln (§2), Ökosystem-Verortung (§1), Schnittstellen (§3), Metriken (§4)
  und Security-Bedrohungen; REQ-STD-006..010 additiv.
- **1.0.0** (2026-09-08): Initial Release — Grundgerüst-Standard aus
  Katalog-Slot ATC-STD-129 (Familie Bug & Fehler-Management, FAM-07) via SCR-0030; §9-FREIGEGEBEN
  08.09.2026, 02:15 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- ATC-STD-000 (Standards Governance & Specification), ATC-STD-FRAMEWORK-001
- ATC-STD-UPDATE-001/CHANGE-001 (Change-Control), ATC-STD-BUG-005 (RCA)
- registry/framework.yaml (FAM-07), registry/standards.yaml + versions.yaml

*ATC-STD-129 v1.2.0 · Slot-Fertigbau via SCR-0034 · Aurora (Superagent) · 08.09.2026*
