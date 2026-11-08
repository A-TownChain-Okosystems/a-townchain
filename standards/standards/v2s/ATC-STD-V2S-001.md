---
standard:
  id: ATC-STD-V2S-001
  title: "Vision Standard"
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

# ATC-STD-V2S-001 — Vision Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-001 definiert den ersten Schritt im Vision-to-Software Lifecycle:
die eindeutige und verbindliche Formulierung einer Software-Vision. Der Standard
stellt sicher, dass jedes Software-Vorhaben mit einer klaren, bewertbaren und
zielgerichteten Ausrichtung startet, bevor weiterführende Ideen, Ziele oder
Anforderungen abgeleitet werden. Er verhindert unklare Zielsetzungen und
unstrukturierte Entwicklungsstarts.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/DARF NICHT — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** alle ATC-Softwareprojekte, Komponenten, Protokolle und KI-Systeme
gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** rein administrative Geschäftsvisionen ohne Software- oder
Technologiebezug sowie Dokumente, die reinen Informationscharakter besitzen.

## §1 Gegenstand & Verortung

Dieser Standard ordnet sich in Station 1 (VISION) des 20-Stationen-Lifecycles von
ATC-STD-V2S-000 ein. Station 1 ist der Ausgangspunkt der gesamten Entwicklungskette.
Sie grenzt sich von Station 2 (IDEA) dadurch ab, dass die Vision den
übergeordneten Rahmen und Zweck vorgibt, während Ideen konkrete Lösungsansätze
oder Teilfunktionen innerhalb dieses Rahmens darstellen. Von Station 3
(BUSINESS/SYSTEM GOAL) unterscheidet sie sich durch ihren strategischen und
qualitativen Charakter, da Ziele erst aus der Vision abgeleitet und quantifiziert
werden.

## §2 Kernregeln

1. **KR-1:** Jedes ATC-Softwareprojekt MUSS eine explizite, schriftlich fixierte und zugängliche Software-Vision besitzen (MUST).
2. **KR-2:** Die Vision MUSS den Zweck des Systems, die Zielgruppe, den zentralen Mehrwert sowie die grundlegenden Randbedingungen eindeutig beschreiben (MUST).
3. **KR-3:** Eine Software-Vision DARF NICHT direkt in Quellcode, konkrete Tasks oder Framework-Implementierungen übersetzt werden (MUST NOT — Bezug Master §2).
4. **KR-4:** Die Vision MUSS technologieneutral formuliert sein und DARF NICHT ohne vorherige Architekturentscheidung spezifische Bibliotheken oder Werkzeuge festlegen.
5. **KR-5:** Wesentliche Revisionsänderungen an einer etablierten Vision MÜSSEN den Change-Request-Prozess gemäß ATC-STD-UPDATE-001 durchlaufen und neu bewertet werden.
6. **KR-6:** Die Erstfassung sowie jede grundlegende Überarbeitung der Vision MUSS durch das zuständige Governance-Gremium oder den Project Lead formell freigegeben werden.

## §3 Artefakte & Nachweispflichten

Für die Station 1 (VISION) entstehen folgende verbindliche Artefakte:

- **Vision Document:** Zentrales Dokument (`docs/VISION.md` oder `.atc/vision.yaml`), das den Zweck, die Zielgruppe und den Hauptnutzen beschreibt.
- **Vision Statement:** Eine prägnante Zusamenfassung (1-3 Sätze) für externe und interne Stakeholder.
- **Scope Boundaries:** Explizite Aufzählung von In-Scope- und Out-of-Scope-Themen.
- **Owner Sign-off Record:** Nachweis der formellen Freigabe durch den Verantwortlichen.

## §4 Quality-Gate-Bezug

Dieser Standard bereitet das erste Quality Gate **V2S-G0** ("Ist die Vision eindeutig?") vor:

- **Prüffrage:** Ist die Vision klar, widerspruchsfrei und von den Stakeholdern akzeptiert?
- **Bedingung:** V2S-G0 MUSS erfolgreich bestanden sein, bevor die Idee-Phase (Station 2) gestartet wird.
- **Gate-Record-Pflicht:** Nachweis durch ein Prüfprotokoll mit Angabe von Prüfer, Datum, Ergebnis (PASS/FAIL) und Referenz auf das Vision Document.
- **Reifegrad:** Der erfolgreiche Abschluss hebt den Projektzustand auf Reifegrad **M1 (DEFINED)** gemäß ATC-STD-V2S-000 §14.

## §5 Schnittstellen

- **Upstream Phase:** Keine (Ausgangspunkt des Lifecycles).
- **Downstream Phase:** Station 2 (IDEA) und Station 3 (BUSINESS/SYSTEM GOAL).
- **Andock-Standards:**
  - ATC-STD-V2S-000 (Lifecycle Master Standard)
  - ATC-STD-000 (Grundlagen und Metadaten)
  - ATC-STD-UPDATE-001 (Change Management bei Revisionsänderungen)
  - ATC-STD-DESC-001 (Beschreibungskonventionen)

## Metriken & Akzeptanzkriterien

- **M1:** 100% aller ATC-Projekte verfügen über ein freigegebenes Vision Document vor dem Entwicklungsstart.
- **M2:** 0 direkte Übersetzungen von der Vision in Code ohne Zwischenschritte (Einhaltung Master §2).
- **M3:** Erfolgsquote beim Passieren des Quality Gates V2S-G0 liegt bei 100% vor Phasenübergang.
- **M4:** Reifegrad M1 ist im Lifecycle-State korrelierend mit dem Bestaensein von V2S-G0 gesetzt.

## Compliance & Verifikation

Die Einhaltung dieses Standards wird durch den automatisierten Validator
`atc_std_validator.py` (Regeln S-01 bis S-25) sowie im Rahmen periodischer
Repository-Audits geprüft. Abweichungen, wie z.B. das Fehlen des Vision Documents
oder unautorisierte Phasenübergänge, führen zu Findings (F-NNN) gemäß ATC-STD-BUG-001.

## Security Considerations

Schon in der Vision MÜSSEN grundlegende Sicherheitsziele (z.B. Datenschutz,
Vertraulichkeit, Verfügbarkeit) verankert werden. Risiken bezüglich der Offenlegung
strategischer Systemziele MÜSSEN durch entsprechende Dokumenten-Klassifizierung
adressiert werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000; **INFORMATIVE:** SCR-0042, ATC-STD-DESC-001, ATC-STD-UPDATE-001
