---
standard:
  id: ATC-STD-LICENSE-001
  title: "License Governance Standard"
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

# ATC-STD-LICENSE-001 — License Governance Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0, §30-eingefroren) — Standard der Familie ATC License
> System (FAM-44); §9-FREIGEGEBEN 08.09.2026, 03:55 UTC+2 via Owner-Direktive
> „ATC-Lizenzsystem als eigene Standardfamilie etablieren" (SCR-0037). Klar getrennt
> von SPDX-Standardlizenzen (Repos behalten Apache-2.0 als Basisschicht, SCR-0036).

## Abstract

ATC-STD-LICENSE-001 (License Governance Standard) ist der erste. Standard der Familie **ATC License System** (FAM-44)
im ATC Enterprise Standards Framework und Teil des ATC-LICENSE-Systems v1.0.0
(License Core, License Types, License Registry, SPDX/Metadaten, Compliance Engine,
Audit System). Dachnorm des ATC-LICENSE-Systems: definiert Rechte, Pflichten, Einschränkungen und Durchsetzung über alle Lizenztypen; drei Ebenen strikt getrennt (OPEN / RESTRICTED / PROPRIETARY); keine Pseudo-Open-Source (ATC-OSS-* MUSS OSD-kompatibel sein, sonst darf sie nicht als Open Source bezeichnet werden).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** License Governance Standard im Zuständigkeitsfeld der Lizenz- und Nutzungsrechte-Governance
des ATC-Ökosystems. **Gilt nicht:** Ersetzung von SPDX-Standardlizenzen auf
Repository-Ebene; rechtliche Beratung (Lizenztexte ersetzen keine Rechtsberatung).

## §1 Gegenstand & Ökosystem-Verortung

Verortet in der Governance-Schicht des ATC-Ökosystems (FAM-44, Range ATC-STD-LICENSE-001..009). Das ATC-LICENSE-System ist die zentrale Lizenz- und Nutzungsrechte-Governance für Code, Protokolle, Smart Contracts, Assets, KI-Modelle, Marken und Daten — klar getrennt von SPDX-Standardlizenzen: Repos behalten Apache-2.0 (SCR-0036) als maschinenlesbare Basisschicht; ATC-LICENSE regelt die Ökosystem-Ebene via licenses/-Registry und ATC-LICENSE.yaml-Manifeste.

## §2 Kernregeln

**KR-1:** Das ATC-LICENSE-System MUSS die drei Ebenen OPEN/RESTRICTED/PROPRIETARY strikt trennen.
**KR-2:** Lizenztypen der Ebene OPEN MÜSSEN mit der Open Source Definition (OSD) vereinbar sein; Verstöße DÜRFEN nicht als 'Open Source' bezeichnet werden.
**KR-3:** Rechte, Pflichten und Einschränkungen MÜSSEN je Lizenztyp maschinenlesbar deklariert sein (Metadaten-Schema §5).
**KR-4:** Durchsetzung erfolgt über die Compliance Engine (LICENSE-006) und das License-Audit (LICENSE-007); BLOCK-Zustände MÜSSEN Releases stoppen.
**KR-5:** Das System ist klar getrennt von SPDX-Standardlizenzen: Repos behalten ihre SPDX-Basislizenz (Apache-2.0, SCR-0036); ATC-LICENSE regelt die Ökosystem-Ebene.
**KR-6:** Kein Pseudo-Open-Source: RESTRICTED/PROPRIETARY-Typen DÜRFEN nicht mit Open-Source-Begriffen beworben werden.

**KR-7:** Jede Erteilung oder Einräumung von Nutzungsrechten MUSS über ein maschinenlesbares Manifest in `licenses/` dokumentiert werden (REQ-LIC-029).
**KR-8:** Bei Verstößen gegen die Lizenzbedingungen MUSS das Nutzungsrecht automatisch als REVOKED in der Registry geführt und widerrufen werden (REQ-LIC-030).
**KR-9:** Für kostenpflichtige oder kommerzielle Nutzungsklassen MUSS eine monatliche Abrechnungs- und Nutzungsevidenz vorgelegt werden (REQ-LIC-031).
**KR-10:** Eine rückwirkende Einschränkung bereits erteilter Lizenzen MUSS ausgeschlossen sein (Immutabilitätsgebot, REQ-LIC-032).

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
- **REQ-LIC-001** (§2): Drei-Ebenen-Modell (OPEN/RESTRICTED/PROPRIETARY) definiert und je Lizenztyp zugeordnet.
- **REQ-LIC-002** (§2): OSD-Kompatibilitätsprüfung als Pflicht-Gate für alle OPEN-Typen.
- **REQ-LIC-003** (§2): Metadaten je Lizenztyp nach Schema (LICENSE-002) vorhanden.
- **REQ-LIC-004** (§2): Durchsetzung über Compliance Engine + Audit nachweisbar.
- **REQ-LIC-029** (§2): Erteilungs- und Lizenzvergabepflicht in `licenses/` MUSS dokumentiert werden.
- **REQ-LIC-030** (§2): Widerrufsrecht und Entzug MUSS bei Regelverstößen als REVOKED geführt werden.
- **REQ-LIC-031** (§2): Abrechnungs- und Auditpflicht MUSS monatliche Nutzungsevidenzen erbringen.
- **REQ-LIC-032** (§2): Immutabilität der Lizenzvergabe MUSS rückwirkende Einschränkungen ausschließen.

## Security Considerations

Ehrlichkeitswidrige Open-Source-Bezeichnung für eingeschränkte Typen; rechtliche Unsicherheit durch nicht geprüfte Eigenlizenztexte (Rechtsberatung ausstehend).

Ehrlichkeitsregel: Lizenz-/Review-Zustände MÜSSEN ehrlich benannt sein (ACTIVE/
PLANNED, OSD-Selbstprüfung vs. ausstehender externer Review); erfundene rechtliche
Zusagen sind verboten. ATC-Lizenztexte sind keine Rechtsberatung; externe
rechtliche Prüfung wird empfohlen und ist als ausstehend dokumentiert.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |
