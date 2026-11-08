---
standard:
  id: ATC-STD-LICENSE-005
  title: "Third-Party License Management Standard"
  version: "1.1.0"
  status: approved
  category: license
  authority: A-TownChain Ecosystems
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

# ATC-STD-LICENSE-005 — Third-Party License Management Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0, §30-eingefroren) — Standard der Familie ATC License
> System (FAM-44); §9-FREIGEGEBEN 08.09.2026, 03:55 UTC+2 via Owner-Direktive
> „ATC-Lizenzsystem als eigene Standardfamilie etablieren" (SCR-0037). Klar getrennt
> von SPDX-Standardlizenzen (Repos behalten Apache-2.0 als Basisschicht, SCR-0036).

## Abstract

ATC-STD-LICENSE-005 (Third-Party License Management Standard) ist der fünfte. Standard der Familie **ATC License System** (FAM-44)
im ATC Enterprise Standards Framework und Teil des ATC-LICENSE-Systems v1.0.0
(License Core, License Types, License Registry, SPDX/Metadaten, Compliance Engine,
Audit System). Dritt-/Copyleft-Lizenzverwaltung im ATC-LICENSE-Kontext: Erfassung, SBOM-Kopplung, Kompatibilitätsprüfung gegen den je Projekt gültigen ATC-Lizenztyp und dokumentierte Pflichten aus Drittlicenses.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Third-Party License Management Standard im Zuständigkeitsfeld der Lizenz- und Nutzungsrechte-Governance
des ATC-Ökosystems. **Gilt nicht:** Ersetzung von SPDX-Standardlizenzen auf
Repository-Ebene; rechtliche Beratung (Lizenztexte ersetzen keine Rechtsberatung).

## §1 Gegenstand & Ökosystem-Verortung

Verortet in der Governance-Schicht des ATC-Ökosystems (FAM-44, Range ATC-STD-LICENSE-001..009). Das ATC-LICENSE-System ist die zentrale Lizenz- und Nutzungsrechte-Governance für Code, Protokolle, Smart Contracts, Assets, KI-Modelle, Marken und Daten — klar getrennt von SPDX-Standardlizenzen: Repos behalten Apache-2.0 (SCR-0036) als maschinenlesbare Basisschicht; ATC-LICENSE regelt die Ökosystem-Ebene via licenses/-Registry und ATC-LICENSE.yaml-Manifeste.

## §2 Kernregeln

**KR-1:** Alle Dritt-/Transitiv-Abhängigkeiten MÜSSEN mit Lizenz erfasst sein (SBOM-/Lockfile-Kopplung).
**KR-2:** Dritt-Lizenzen MÜSSEN auf Kompatibilität mit dem Projekt-Lizenztyp geprüft sein; Konflikte sind als Findings (F-NNN) zu führen.
**KR-3:** Pflichten aus Dritt-Lizenzen (Attribution, Copyleft, Notice) MÜSSEN dokumentiert und eingehalten sein.
**KR-4:** 0 inkompatible Dritt-Lizenz-Einleitungen in OPEN-Typ-Projekte.
**KR-5:** Eigenmächtige Lizenzwechsel von Abhängigkeiten (z. B. Lizenz-Änderung Upstream) MÜSSEN Alerts erzeugen.
**KR-6:** Die Copyleft-Risikobewertung MUSS je Release aktuell sein.

**KR-7:** CI-Pipelines MUESSEN bei jedem Build automatische Scans aller transitiven Abhängigkeiten durchführen (REQ-LIC-039).
**KR-8:** Bei Dual-Licensing-Abhängigkeiten MUSS die gewählte Lizenzoption explizit in der SBOM dokumentiert werden (REQ-LIC-040).
**KR-9:** Inkompatible Copyleft-Bibliotheken MUESSEN isoliert oder durch kompatible Alternativen ersetzt werden (REQ-LIC-041).

## §3 Schnittstellen & Kopplungen

- **Registry-Kopplung:** standards.yaml/versions.yaml (S-14/S-19), Katalog-Slot in
  registry/framework.yaml (FAM-44), License-Registry licenses/LICENSE-REGISTRY.yaml.
- **Governance-Kette:** SCR → VERSION → UPDATE → COMPAT (MAJOR) → AUDIT; Findings
  via registry/findings.yaml (F-NNN).
- **Nachbarstandards:** ATC-STD-LICENSE-001..009 (FAM-44), REPO-AUDIT-002
  (Health Score), ATC-STD-CHANGE-001/COMPAT-001 (Change-Control), F-046/Lizenz-SCR-0036.
- **Agenten-Bindung:** Vollmandat via .github/ai/agent.yaml.

## §4 Metriken & Akzeptanzkriterien

- **M-1:** 100 % aller aktiven Lizenztypen MUESSEN vollständig mit Metadaten deklariert sein.
- **M-2:** 0 falsche Open-Source-Bezeichnungen; jede Einschränkung MUSS ehrlich klassifiziert werden.
- **M-3:** Die Durchsetzungsquote MUSS in 100 % aller CI-Builds durch das Compliance-Gate nachgewiesen werden.
- **M-4:** Lizenzänderungs- und Widerrufseinträge MUESSEN innerhalb von 24 Stunden global synchronisiert sein.

## §5 Compliance & Verifikation

Compliance wird über die Gesamt-Validierung (CI, Validator-Läufe S-01 bis S-25) je Registry-Eintrag geprüft; die License-Compliance Engine (ATC-STD-LICENSE-006) erweitert dies je Repository. Abweichungen MUESSEN als Findings (F-001 bis F-999) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 erfasst und bearbeitet werden.

## Requirements (normativ)

- **REQ-STD-001** (§1): Gegenstand eindeutig definiert und im Katalog verortet.
- **REQ-STD-002** (§2): Fachliche Regeln als deklarierte, verifizierbare REQ-IDs.
- **REQ-STD-003** (§2): Compliance nachweisbar über Validator-Gates oder Prüfung.
- **REQ-STD-004** (§2): Änderungen ausschließlich über die Change-Control-Kette.
- **REQ-STD-005** (§2): Sicherheitsaspekte dokumentiert (Security Considerations).
- **REQ-LIC-014** (§2): Dritt-/Transitiv-Lizenzen vollständig erfasst.
- **REQ-LIC-015** (§2): Kompatibilitätsprüfung + Konflikt-Findings.
- **REQ-LIC-016** (§2): Pflichten aus Dritt-Lizenzen dokumentiert.
- **REQ-LIC-039** (§2): Transitiv-Lizenz-Scan MUSS automatisiert in CI laufen.
- **REQ-LIC-040** (§2): Dual-Licensing-Behandlung MUSS in der SBOM eindeutig gewählt sein.
- **REQ-LIC-041** (§2): Copyleft-Isolation MUSS inkompatible Lizenzen vom Kern trennen.

## Security Considerations

Stille Copyleft-Kontamination; Lizenzwechsel Upstream unbemerkt → Rechtsrisiko.

Ehrlichkeitsregel: Lizenz-/Review-Zustände MÜSSEN ehrlich benannt sein (ACTIVE/
PLANNED, OSD-Selbstprüfung vs. ausstehender externer Review); erfundene rechtliche
Zusagen sind verboten. ATC-Lizenztexte sind keine Rechtsberatung; externe
rechtliche Prüfung wird empfohlen und ist als ausstehend dokumentiert.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |
Erweiterter Schutz: Jegliche Unklarheiten oder Verstöße gegen Lizenzbestimmungen MUESSEN unverzüglich als Severity S1 eingestuft werden. Unberechtigte Lizenzänderungen oder gefälschte Manifeste MUESSEN das Release-Gate blockieren.
