---
standard:
  id: ATC-STD-V2S-025
  title: "Deprecation Standard"
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

# ATC-STD-V2S-025 — Deprecation Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-025 regelt die geordnete Abkündigung (Deprecation) veralteter Softwareversionen,
Schnittstellen (APIs), Protokolle, Smart Contracts und Komponenten im ATC Vision-to-Software
Lifecycle. Der Standard definiert Abkündigungsfristen, Warnmechanismen, Migrationspflichten
und Garantien während der Übergangsphase.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** Alle abzukündigenden Module, API-Endpunkte, Datenbankschemata, Protokoll-Features,
Smart Contracts und KI-Agenten-Schnittstellen im ATC-Ökosystem.
**Nicht-Gilt:** Unveröffentlichte interne Prototypen und nicht-normative Dokumente.

## §1 Gegenstand & Verortung

Deprecation ist die geordnete Vorstufe der endgültigen Abschaltung (V2S-026 / End-of-Life Standard).
Sie gibt Entwicklern, Operatoren und Nutzern ausreichend Zeit, auf neuere Versionen (V2S-024)
zu migrieren, ohne dass es zu plötzlichen Systemausfällen oder Inkompatibilitäten kommt.

Der Phasenübergang verläuft gemäß dem Schema:
`ACTIVE → DEPRECATED → END-OF-LIFE`

- **ACTIVE:** Regulär unterstützte Komponente im Produktionsbetrieb.
- **DEPRECATED:** Abgekündigte Komponente mit aktiven Warnungen und fester EOL-Frist.
- **END-OF-LIFE:** Dauerhaft abgeschaltete und archivierte Komponente (V2S-026).

## §2 Kernregeln

1. **KR-1 (Formaler Beschluss & Fristen):** Die Abkündigung MUSS formal angekündigt werden. Es MÜSSEN Mindest-Notice-Periods eingehalten werden: mindestens 90 Tage für öffentliche APIs/Anwendungen und mindestens 180 Tage für dezentrale Kernprotokolle.
2. **KR-2 (Deprecation Warnings):** Abgekündigte Schnittstellen MÜSSEN im Quellcode (`@deprecated`), in API-Response-Headern (`Deprecation: true`, `Link: <...>; rel="successor"`) und in Protokoll-Logs eindeutig gekennzeichnet werden.
3. **KR-3 (Migrationsleitfaden-Pflicht):** Für jede abgekündigte Komponente MUSS zwingend ein Migrationsleitfaden veröffentlicht werden, der konkrete Nachfolge-Lösungen und Transformationspfade beschreibt.
4. **KR-4 (Sicherheits-Garantie während Deprecation):** Während der gesamten Deprecation-Phase MÜSSEN kritische Sicherheits-Patches (P0/P1) für die abgekündigte Komponente weiter bereitgestellt werden.
5. **KR-5 (Lifecycle-State Kennzeichnung):** Der maschinenlesbare Lifecycle-State MUSS auf `status: deprecated` gesetzt werden. Zudem MUSS das geplante EOL-Datum (`planned_eol_date`) hinterlegt werden.
6. **KR-6 (Verbot neuer Abhängigkeiten):** Neue Softwarekomponenten oder Releases DÜRFEN NICHT von als deprecated markierten Modulen oder Schnittstellen abhängig gemacht werden (MUST NOT).

## §3 Artefakte & Nachweispflichten

- **Deprecation Notice:** Formale Abkündigungsmitteilung mit Begründung, Zeitplan und EOL-Datum.
- **Migration Guide:** Dokumentierter Leitfaden mit Code-Beispielen für den Umstieg auf Nachfolgesysteme.
- **Runtime Warning Implementation:** Technische Nachweise über aktive HTTP-Deprecation-Header, Compiler-Warnings oder Log-Meldungen.
- **Deprecation Register:** Eintrag im zentralen Komponenten-Register mit Status `deprecated`.

## §4 Lifecycle-State-Bezug

Der Standard pflegt und aktualisiert den maschinenlesbaren Lifecycle-State
(`.atc/lifecycle-state.yaml`) gemäß ATC-STD-V2S-000 §16:

- Sets `lifecycle.status`: `deprecated`
- Sets `lifecycle.deprecation_notice_date`: ISO-8601 Datum
- Sets `lifecycle.planned_eol_date`: ISO-8601 Datum des bevorstehenden End-of-Life

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-024 (Version Upgrade Standard).
- **Downstream:** ATC-STD-V2S-026 (End-of-Life Standard).
- **Andock-Standards:** ATC-STD-VERSION-001 (Versionierung), ATC-STD-COMPAT-001 (Kompatibilität).

## Metriken & Akzeptanzkriterien

- **M1 (Notice Period Compliance):** 100 % aller Abkündigungen halten die vorgeschriebenen Mindestfristen (90/180 Tage) ein.
- **M2 (Migration Guide Coverage):** 100 % der abgekündigten Komponenten verfügen über einen freigegebenen Migrationsleitfaden.
- **M3 (Deprecated Dependency Rate):** 0 % neue Abhängigkeiten auf abgekündigte Komponenten in aktuellen Builds.
- **M4 (Runtime Warning Rate):** 100 % der abgekündigten API-Endpoints senden aktive Deprecation-Header.

## Compliance & Verifikation

Die Einhaltung wird durch atc-repo-audit, statische Code-Analysen (AST-Linter auf `@deprecated`),
API-Header-Scans und Governance-Audits verifiziert (S-01..S-25). Vorzeitige Abschaltungen
oder fehlende Migrationshinweise erzeugen Findings (F-001..F-999) nach ATC-STD-BUG-001.

## Security Considerations

Abgekündigte Komponenten neigen zum Verwahrlosen und werden häufig zu Sicherheitsrisiken.
Security-Analysen MÜSSEN auch während der Deprecation-Phase aufrechtverhalten werden. Ist
ein sicherer Weiterbetrieb während der Deprecation-Phase wegen gravierender Schwachstellen
nicht möglich, greift die Emergency-Decommissioning-Regel nach ATC-STD-UPDATE-001.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-VERSION-001, ATC-STD-COMPAT-001
**INFORMATIVE:** SCR-0042
