---
standard:
  id: ATC-STD-560
  title: "Requirements Standard"
  version: "1.2.0"
  status: approved
  category: development
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

# ATC-STD-560 — Requirements (v1.2.0, APPROVED)

> **Status:** APPROVED (v1.2.0, §30-eingefroren) — Standard aus Katalog-Slot der Familie
> Requirements Engineering (FAM-32); §9-FREIGEGEBEN 08.09.2026, 02:15 UTC+2 — APPROVED, normativ,
> §30-eingefroren. Struktur-Elaboration v1.1.0 via SCR-0031; Slot-Fertigbau v1.2.0 via
> SCR-0034 (08.09.2026, 03:15 UTC+2): slot-spezifische Pruefkriterien (§6) mit eigener REQ-Menge
> je Gegenstand. Engineering-Bindung (Code/Tests) entsteht bei Slot-Aktivierung
> via SCR/MINOR (ehrlich dokumentiert).

## Abstract

ATC-STD-560 (Requirements) ist der Standard für den gleichnamigen Katalog-Slot der Familie
**Requirements Engineering** (FAM-32, Range ATC-STD-560..566) im ATC Enterprise Standards Framework. Er
definiert den Gegenstand, seine Verortung im Ökosystem, die familienweiten
Kernregeln, die slot-spezifischen Prüfkriterien (§6) mit je-Kriterium-Nachweis,
Compliance- und Verifikationspflichten sowie Security-Betrachtungen. Der Standard
ist APPROVED, normativ in Kraft und §30-eingefroren (Owner-§9-Freigabe 08.09.2026,
02:15 UTC+2, SCR-0030-Batch); Elaboration SCR-0031 (v1.1.0) und Slot-Fertigbau
SCR-0034 (v1.2.0) sind additive MINOR-Updates (ATC-STD-UPDATE-001 UPD-G03).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Der durch den Slot-Titel (Requirements) bezeichnete Gegenstandsbereich im
Zuständigkeitsfeld der Familie Requirements Engineering. **Gilt nicht:** Bereiche, die durch
fachlich konkretere Standards derselben Familie verbindlich geregelt sind
(Subsidiarität: der konkrete Standard geht vor).

## §1 Gegenstand & Ökosystem-Verortung

Dieser Standard adressiert **Requirements** im Katalog-Slot ATC-STD-560 der Familie Requirements Engineering
(FAM-32); die Zuordnung folgt registry/framework.yaml (S-21) und darf nur via SCR
geändert werden.

Requirements-Engineering: REQ-Registry (registry/requirements.yaml, SSOT), Traceability REQ→STD→AUDIT, Familien-/Kategorie-Anträge (ATC-FAM-REQ, ATC-CAT-REQ).

**Katalog-Referenz:** keine zusätzliche Katalog-Notiz; Verortung ausschließlich über Familie und Slot.

## §2 Kernregeln (familienweit, elaboriert)

1. **KR-1:** Anforderungen MÜSSEN eindeutige REQ-IDs tragen und im REQ-Register geführt werden.
2. **KR-2:** Traceability MUSS lückenlos sein (REQ→Standard→Implementierung→Nachweis).
3. **KR-3:** Anforderungsänderungen MÜSSEN über die Change-Control-Kette laufen.
4. **KR-4:** Anforderungen MÜSSEN testbar/verifizierbar formuliert sein (keine Wunsch-Formulierungen).
5. **KR-5:** Kategorien-/Familienbildung folgt der Taxonomie (TAXONOMY-001).
6. **KR-6:** Offene Anforderungen MÜSSEN sichtbar sein (keine stillen Backlog-Items).

## §3 Schnittstellen & Kopplungen

- **Registry-Kopplung:** standards.yaml/versions.yaml (Version/Status S-14/S-19),
  Katalog-Slot in registry/framework.yaml.
- **Governance-Kette:** SCR → VERSION → UPDATE (UPD-G03 MINOR) → COMPAT (bei MAJOR)
  → AUDIT; Findings via registry/findings.yaml (F-NNN).
- **Nachbarfamilien:** Familie Requirements Engineering — Subsidiarität: konkretere Standards gehen vor.
- **Agenten-Bindung:** Vollmandat via .github/ai/agent.yaml; Umsetzungspflicht nach
  AGENT_MANIFEST.

## §4 Metriken & Akzeptanzkriterien

- **M1:** Traceability-Abdeckung 100 %
- **M2:** 0 untestbare REQs
- **M3:** Backlog-Sichtbarkeit

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

## §6 Slot-Spezifikation Requirements — verbindliche Prüfkriterien

Jedes Kriterium ist normativ (MUSS). Nachweis je Kriterium: Konzept-/Design-Dokument
plus AUD-Record, oder Validator-/Testlauf — je nach Art des Kriteriums; bei
Slot-Aktivierung wird der Nachweis je Kriterium einzeln erbracht.

- **P1** (MUSS): Eindeutige REQ-IDs im REQ-Register — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P2** (MUSS): Testbarkeits-/Verifizierbarkeitsformulierung — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung
- **P3** (MUSS): Traceability (REQ->STD->AUDIT) — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P4** (MUSS): Priorisierung und Sichtbarkeit — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung
- **P5** (MUSS): Aenderung nur via Change-Control — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P6** (MUSS): Interaktionsmuster konsistent — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung
- **P7** (MUSS): Zustandsfeedback ( Laden/Fehler/Leer) — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P8** (MUSS): Accessibility-Basis (Kontrast, Tastatur) — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung

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
- **REQ-STD-011** (§6/P1): Eindeutige REQ-IDs im REQ-Register
- **REQ-STD-012** (§6/P2): Testbarkeits-/Verifizierbarkeitsformulierung
- **REQ-STD-013** (§6/P3): Traceability (REQ->STD->AUDIT)
- **REQ-STD-014** (§6/P4): Priorisierung und Sichtbarkeit
- **REQ-STD-015** (§6/P5): Aenderung nur via Change-Control
- **REQ-STD-016** (§6/P6): Interaktionsmuster konsistent
- **REQ-STD-017** (§6/P7): Zustandsfeedback ( Laden/Fehler/Leer)
- **REQ-STD-018** (§6/P8): Accessibility-Basis (Kontrast, Tastatur)

## Security Considerations

Phantom-Anforderungen ohne Verifikationspfad; Traceability-Brüche verhindern Impact-Analysen.

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
  Katalog-Slot ATC-STD-560 (Familie Requirements Engineering, FAM-32) via SCR-0030; §9-FREIGEGEBEN
  08.09.2026, 02:15 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- ATC-STD-000 (Standards Governance & Specification), ATC-STD-FRAMEWORK-001
- ATC-STD-UPDATE-001/CHANGE-001 (Change-Control), ATC-STD-BUG-005 (RCA)
- registry/framework.yaml (FAM-32), registry/standards.yaml + versions.yaml

*ATC-STD-560 v1.2.0 · Slot-Fertigbau via SCR-0034 · Aurora (Superagent) · 08.09.2026*
