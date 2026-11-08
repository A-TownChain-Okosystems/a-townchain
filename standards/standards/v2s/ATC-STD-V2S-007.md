---
standard:
  id: ATC-STD-V2S-007
  title: "Architecture Standard"
  version: "1.0.0"
  status: candidate
  category: v2s
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Projekte gemäß ATC-STD-V2S-000 Scope"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-V2S-007 — Architecture Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-007 regelt die architektonische Gestaltung und Dokumentation von Software-
Systemen im ATC-Ökosystem. Er schreibt die Erstellung von Architecture Decision Records
(ADRs) mit den Pflichtbereichen Decision, Reason, Alternatives und Status vor und etabliert
das Zwei-Ebenen-Modell des DECISIONS_REGISTERs gemäß ATC-STD-201. Er stellt sicher, dass
keine Implementierung ohne freigegebene Architekturentscheidung gestartet wird.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/DARF NICHT — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** alle Architekturentscheidungen, Systemstrukturen, Technologienentscheidungen
und Systemgrenzen von ATC-Softwareprojekten.
**Nicht-Gilt:** mikro-lokale Code-Refactorings innerhalb einer einzelnen Klasse oder
Funktion ohne Auswirkung auf Schnittstellen oder Systemverhalten.

## §1 Gegenstand & Verortung

Dieser Standard definiert Station 6 (ARCHITECTURE) des Lifecycles gemäß ATC-STD-V2S-000 §1.
Station 6 baut auf den Spezifikationen aus Station 5 auf. Sie legt das architektonische
Fundament und die Schnittstellen-Muster fest, auf denen das Software-Design (Station 7)
und die Implementierung (Station 8) aufbauen.

## §2 Kernregeln

1. **KR-1:** Vor Beginn jeglicher Implementierungsarbeiten MUSS jede wesentliche Architekturentscheidung als ADR dokumentiert sein (MUST — Bezug Master §4).
2. **KR-2:** Jeder ADR MUSS zwingend die vier Abschnitte Decision, Reason, Alternatives und Status enthalten (MUST).
3. **KR-3:** Der Status eines ADRs MUSS den Wert ACCEPTED aufweisen, bevor darauf aufbauende Entwicklungsaufgaben begonnen werden dürfen (MUST — Bezug Master §4).
4. **KR-4:** Architekturentscheidungen MÜSSEN im Zwei-Ebenen-Modell verwaltet werden: zentrale ADRs im DECISIONS_REGISTER (Hub), repo-lokale ADRs mit Verweis dorthin (gemäß ATC-STD-201).
5. **KR-5:** Verworfene Alternativen MÜSSEN mit einer nachvollziehbaren technischen oder fachlichen Begründung dokumentiert werden.
6. **KR-6:** Architektonische Abweichungen ohne genehmigten ADR DÜRFEN NICHT in den Hauptzweig übernommen werden (MUST NOT).

## §3 Artefakte & Nachweispflichten

In Station 6 (ARCHITECTURE) entstehen folgende verpflichtende Artefakte:

- **Architecture Decision Record (ADR):** Einzeldokumentation (`docs/adr/ADR-NNN.md` oder `.atc/adr/ADR-NNN.yaml`).
- **DECISIONS_REGISTER Sync Log:** Nachweis der Synchronisation zwischen lokalem Repository und zentralem Hub-Register.
- **Architecture Overview Diagram:** Systemübersicht zur Veranschaulichung der Komponenten und Datenflüsse.
- **ADR Review Record:** Nachweis der Freigabe durch das Architecture Board oder den Lead Architect.

## §4 Quality-Gate-Bezug

Dieser Standard bildet den Kern des Quality Gates **V2S-G2** ("Ist die Architektur freigegeben?"):

- **Prüffrage:** Sind alle Architekturentscheidungen durch akzeptierte ADRs abgedeckt und im Register erfasst?
- **Bedingung:** Gate V2S-G2 MUSS bestanden sein, bevor Implementierungsarbeiten (Station 8) aufgenommen werden.
- **Gate-Record-Pflicht:** Dokumentation des ADR-Status und der Freigabe im Gate-Record.
- **Reifegrad:** Der erfolgreiche Abschluss hebt den Reifegrad auf **M3 (ARCHITECTED)** gemäß ATC-STD-V2S-000 §14.

## §5 Schnittstellen

- **Upstream Phase:** Station 5 (SPECIFICATION, ATC-STD-V2S-006).
- **Downstream Phase:** Station 7 (DESIGN, ATC-STD-V2S-008) und Station 8 (IMPLEMENTATION, ATC-STD-V2S-009).
- **Andock-Standards:**
  - ATC-STD-V2S-000 (Lifecycle Master)
  - ATC-STD-201 (Architecture Decision Records & Registry)
  - ATC-STD-NET-001 (Netzwerk-Architektur)
  - ATC-STD-BUG-001 (Finding-Klassifizierung)

## Metriken & Akzeptanzkriterien

- **M1:** 100% aller wesentlichen Architekturentscheidungen besitzen ein gültiges ADR.
- **M2:** 0 Implementierungsaufgaben gestartet ohne ADR im Status ACCEPTED.
- **M3:** 100% Konformität mit dem Zwei-Ebenen-Modell des DECISIONS_REGISTER.
- **M4:** 100% Erfassung aller erwogenen Alternativen in den bestehenden ADRs.

## Compliance & Verifikation

Die Verifikation erfolgt automatisiert durch `atc_std_validator.py` sowie durch
Architektur-Audits. Verstöße wie nicht-dokumentierte Architekturänderungen oder
ADRs im Status PROPOSED bei Entwicklungsstart führen zu Findings (F-NNN) gemäß
ATC-STD-BUG-001.

## Security Considerations

Sicherheitsrelevante Architekturmuster (z.B. Defense-in-Depth, Zero-Trust-Isolation,
Kryptographie-Konzepte) MÜSSEN als eigenständige ADRs strukturiert analysiert und
freigegeben werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000; **INFORMATIVE:** SCR-0042, ATC-STD-201, ATC-STD-V2S-006, ATC-STD-V2S-008
