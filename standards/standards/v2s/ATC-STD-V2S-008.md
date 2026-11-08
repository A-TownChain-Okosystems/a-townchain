---
standard:
  id: ATC-STD-V2S-008
  title: "Software Design Standard"
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

# ATC-STD-V2S-008 — Software Design Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-008 regelt den detaillierten Entwurf von Softwarekomponenten, Modulen,
Datenmodellen und Schnittstellenverträgen im ATC-Ökosystem. Der Standard verpflichtet
zur Anwendung des C4-Modells (Context, Container, Component, Code) sowie zur formalen
Spezifikation von Interface-Verträgen. Er stellt sicher, dass Komponenten hochgradig
kohärent, schwach gekoppelt und frei von Widersprüchen zu den Architekturentscheidungen sind.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/DARF NICHT — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** das detaillierte Design aller Softwarekomponenten, Schnittstellen,
Protokolle, Smart Contracts und Modulstrukturen in ATC-Projekten.
**Nicht-Gilt:** makroskopische Systemarchitekturen (siehe ATC-STD-V2S-007) und rein
organisatorische Abläufe.

## §1 Gegenstand & Verortung

Dieser Standard definiert Station 7 (DESIGN) des Lifecycles gemäß ATC-STD-V2S-000 §1.
Station 7 nimmt die Vorgaben aus der Architektur (Station 6) auf und verfeinert sie
in Ausführungs- und Moduldesigns. Sie bildet die unmittelbare Schnittstelle zur
Implementierung (Station 8) und stellt sicher, dass Quellcode auf vorbereiteten
Interface-Verträgen aufbaut.

## §2 Kernregeln

1. **KR-1:** Das Software-Design MUSS alle Komponenten, Schnittstellen, Datenflüsse und Zustandsübergänge explizit festlegen (MUST).
2. **KR-2:** Softwarestrukturen SOLLTEN nach dem C4-Modell (Context, Container, Component, Code) strukturiert und dokumentiert werden.
3. **KR-3:** Alle öffentlichen Schnittstellen MÜSSEN maschinenlesbare Interface-Verträge (z.B. OpenAPI, Protobuf, AsyncAPI oder JSON Schema) besitzen (MUST).
4. **KR-4:** Modulgrenzen MÜSSEN nach den Prinzipien hoher Kohäsion und geringer Kopplung entworfen werden.
5. **KR-5:** Das Software-Design DARF NICHT im Widerspruch zu akzeptierten ADRs (ATC-STD-V2S-007) stehen (MUST NOT).
6. **KR-6:** Datenmodelle und Schnittstellenänderungen MÜSSEN abwärtskompatibel gestaltet oder mit expliziten Migrationspfaden spezifiziert werden.

## §3 Artefakte & Nachweispflichten

In Station 7 (DESIGN) entstehen folgende obligatorische Artefakte:

- **Software Design Document (SDD):** Detaillierte Dokumentation (`docs/DESIGN.md` oder `.atc/design.yaml`).
- **C4 Diagrams:** Diagramme auf Context-, Container- und Component-Ebene.
- **Interface Contract Specifications:** Formale Schemaspezifikationen in `schemas/` oder `api/`.
- **Data Flow Specifications:** Darstellung von Datenflüssen und Datenverarbeitungsfolgen.

## §4 Quality-Gate-Bezug

Dieser Standard bereitet den Phasenübergang von der Architektur zur Implementierung
im Quality Gate **V2S-G2** und **V2S-G3** vor:

- **Prüffrage:** Sind alle Komponenten und Schnittstellen vollständig entworfen und durch Interface-Verträge abgesichert?
- **Gate-Record-Pflicht:** Nachweis der Design-Review-Ergebnisse im Gate-Record.
- **Reifegrad:** Der erfolgreiche Abschluss des Designs unterstützt das Verharren auf Reifegrad **M3 (ARCHITECTED)** und die Freigabe zu M4 (IMPLEMENTED).

## §5 Schnittstellen

- **Upstream Phase:** Station 6 (ARCHITECTURE, ATC-STD-V2S-007).
- **Downstream Phase:** Station 8 (IMPLEMENTATION, ATC-STD-V2S-009).
- **Andock-Standards:**
  - ATC-STD-V2S-000 (Lifecycle Master)
  - ATC-STD-201 (Architecture Standards)
  - ATC-STD-001 (Coding Standards)
  - ATC-STD-COMPAT-001 (Kompatibilitätsstandards)

## Metriken & Akzeptanzkriterien

- **M1:** 100% aller öffentlichen Schnittstellen verfügen über maschinenlesbare Verträge.
- **M2:** 100% C4-Diagrammabdeckung für Kernkomponenten.
- **M3:** 0 zirkuläre Abhängigkeiten zwischen Modulkomponenten.
- **M4:** 100% Konformität mit übergeordneten ADRs.

## Compliance & Verifikation

Die Einhaltung wird durch automatisierte Schema-Validierung (z.B. Linting von OpenAPI/Protobuf)
sowie den `atc_std_validator.py` verifiziert. Designmängel oder fehlende Interface-Verträge
werden als Finding (F-NNN) gemäß ATC-STD-BUG-001 erfasst.

## Security Considerations

Im Software-Design MÜSSEN Eingabevorgaben (Input Validation), Datenbankschemata und
Verschlüsselungsprotokolle auf Komponentenebene gehärtet werden, um Injektions- und
Datenleck-Risiken auszuschließen.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000; **INFORMATIVE:** SCR-0042, ATC-STD-V2S-007, ATC-STD-V2S-009, ATC-STD-001
