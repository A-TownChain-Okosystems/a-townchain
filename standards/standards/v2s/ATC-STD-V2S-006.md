---
standard:
  id: ATC-STD-V2S-006
  title: "Specification Standard"
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

# ATC-STD-V2S-006 — Specification Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-006 legt die verbindlichen Anforderungen an die Erstellung technischer
Spezifikationen (SPEC-Dokumente) fest. Der Standard erzwingt die vollständige
Beantwortung des kanonischen 9-Fragen-Katalogs aus ATC-STD-V2S-000 §3 und regelt die
strikte, sprungfreie Zustandskette IDEA → PROPOSED → SPECIFIED → READY. Er stellt sicher,
dass keine technische Implementierung ohne präzise, geprüfte Spezifikation beginnt.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/DARF NICHT — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** alle technischen Spezifikationen für Softwaremodule, Schnittstellen,
Smart Contracts, Protokolle und KI-Subsysteme im ATC-Ökosystem.
**Nicht-Gilt:** reine Architekturentscheidungsdokumente (ADR), die im Architektur-Standard
(ATC-STD-V2S-007) behandelt werden.

## §1 Gegenstand & Verortung

Dieser Standard definiert Station 5 (SPECIFICATION) des 20-Stationen-Lifecycles nach
ATC-STD-V2S-000 §1. Station 5 verfeinert die in Station 4 (REQUIREMENTS) erfassten
Anforderungen in eine detaillierte technische Beschreibung. Sie bereitet die
Architektur- und Design-Phasen (Stationen 6 und 7) vor.

## §2 Kernregeln

1. **KR-1:** Jede technische Spezifikation MUSS den 9-Fragen-Katalog nach Master §3 vollständig beantworten: 1. Was? 2. Warum? 3. Schnittstellen? 4. Daten? 5. Abhängigkeiten? 6. Grenzen? 7. Fehlerfälle? 8. Sicherheitsanforderungen? 9. Erfolgsmessung? (MUST).
2. **KR-2:** Die Statusübergänge einer Spezifikation MÜSSEN der Kette IDEA → PROPOSED → SPECIFIED → READY strikt ohne Sprünge folgen (MUST — Bezug Master §3).
3. **KR-3:** Eine Spezifikation DARF NICHT in den Status READY versetzt werden, solange offene technische Fragen oder ungelöste Abhängigkeiten bestehen (MUST NOT).
4. **KR-4:** Alle in der Spezifikation definierten Datenstrukturen und Schnittstellen MÜSSEN durch formale Schemata (z.B. JSON Schema, Protobuf, OpenAPI) präzisiert werden.
5. **KR-5:** Fehlerbehandlung, Ausfallszenarien und Grenzwerte MÜSSEN explizit und prüfbar spezifiziert werden.
6. **KR-6:** Jede Spezifikation MUSS auf mindestens eine gültige REQ-ID (ATC-STD-V2S-004) verweisen.

## §3 Artefakte & Nachweispflichten

In Station 5 (SPECIFICATION) entstehen folgende obligatorische Artefakte:

- **Specification Document:** Technisches Spezifikationsdokument (`docs/SPEC-NNN.md` oder `.atc/specs/SPEC-NNN.yaml`).
- **9-Fragen-Checkliste:** Ausgefüllter Prüfbogen zum Nachweis der Vollständigkeit.
- **Interface & Data Schemata:** Formale Definitionen aller Schnittstellen und Datenstrukturen.
- **SPEC Review Protocol:** Dokumentiertes Peer-Review-Ergebnis vor dem Statusübergang zu READY.

## §4 Quality-Gate-Bezug

Dieser Standard bildet die Grundlage für das Quality Gate **V2S-G1** und bereitet **V2S-G2** vor:

- **Prüffrage:** Ist die Spezifikation vollständig, widerspruchsfrei, geprüft und im Status READY?
- **Gate-Record-Pflicht:** Nachweis des SPEC-Reviews im Gate-Record.
- **Reifegrad:** Der erfolgreiche Abschluss der Spezifikation sichert den Reifegrad **M2 (SPECIFIED)** ab und bereitet M3 vor.

## §5 Schnittstellen

- **Upstream Phase:** Station 4 (REQUIREMENTS, ATC-STD-V2S-004).
- **Downstream Phase:** Station 6 (ARCHITECTURE, ATC-STD-V2S-007) und Station 7 (DESIGN, ATC-STD-V2S-008).
- **Andock-Standards:**
  - ATC-STD-V2S-000 (Lifecycle Master)
  - ATC-STD-202 (Requirements Engineering)
  - ATC-STD-MD-001 (Markdown-Konventionen)
  - ATC-STD-001 (Coding Standards)

## Metriken & Akzeptanzkriterien

- **M1:** 100% Beantwortung aller 9 Fragen im 9-Fragen-Katalog für jede freigegebene SPEC.
- **M2:** 100% Einhaltung der Zustandskette ohne unautorisierte Status-Sprünge.
- **M3:** 100% Traceability-Verknüpfung zu den zugrundeliegenden Requirements (REQ-IDs).
- **M4:** 0 offene Punkte oder ungeklärte Abhängigkeiten im Status READY.

## Compliance & Verifikation

Die Einhaltung wird durch automatisierte Strukturtests des `atc_std_validator.py` sowie
Peer-Reviews verifiziert. Mängel wie unvollständige Beantwortung des 9-Fragen-Katalogs
oder fehlerhafte Statusübergänge führen zu Findings (F-NNN) gemäß ATC-STD-BUG-001.

## Security Considerations

Frage 8 des Katalogs fordert eine explizite Sicherheitsbewertung. Potenzielle
Bedrohungen, Angriffsvektoren und Schutzmaßnahmen (z.B. Verschlüsselung, Authentifizierung)
MÜSSEN in der Spezifikation nachgewiesen werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000; **INFORMATIVE:** SCR-0042, ATC-STD-V2S-004, ATC-STD-202, ATC-STD-MD-001
