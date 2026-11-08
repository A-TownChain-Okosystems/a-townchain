---
standard:
  id: ATC-STD-LICENSE-003
  title: "License Registry Standard"
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

# ATC-STD-LICENSE-003 — License Registry Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0, §30-eingefroren) — Standard der Familie ATC License
> System (FAM-44); §9-FREIGEGEBEN 08.09.2026, 03:55 UTC+2 via Owner-Direktive
> „ATC-Lizenzsystem als eigene Standardfamilie etablieren" (SCR-0037). Klar getrennt
> von SPDX-Standardlizenzen (Repos behalten Apache-2.0 als Basisschicht, SCR-0036).

## Abstract

ATC-STD-LICENSE-003 (License Registry Standard) ist der dritte. Standard der Familie **ATC License System** (FAM-44)
im ATC Enterprise Standards Framework und Teil des ATC-LICENSE-Systems v1.0.0
(License Core, License Types, License Registry, SPDX/Metadaten, Compliance Engine,
Audit System). SSOT-Register licenses/LICENSE-REGISTRY.yaml: alle 9 Lizenztypen (ATC-LIC-OSS-001 … EXPERIMENTAL-009) plus Dach (CORE) mit IDs, Versionen, Ebenen, Lifecycle-Status und Projekt-Bindungen; einheitliche ID-Vergabe, keine beliebigen IDs.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** License Registry Standard im Zuständigkeitsfeld der Lizenz- und Nutzungsrechte-Governance
des ATC-Ökosystems. **Gilt nicht:** Ersetzung von SPDX-Standardlizenzen auf
Repository-Ebene; rechtliche Beratung (Lizenztexte ersetzen keine Rechtsberatung).

## §1 Gegenstand & Ökosystem-Verortung

Verortet in der Governance-Schicht des ATC-Ökosystems (FAM-44, Range ATC-STD-LICENSE-001..009). Das ATC-LICENSE-System ist die zentrale Lizenz- und Nutzungsrechte-Governance für Code, Protokolle, Smart Contracts, Assets, KI-Modelle, Marken und Daten — klar getrennt von SPDX-Standardlizenzen: Repos behalten Apache-2.0 (SCR-0036) als maschinenlesbare Basisschicht; ATC-LICENSE regelt die Ökosystem-Ebene via licenses/-Registry und ATC-LICENSE.yaml-Manifeste.

## §2 Kernregeln

**KR-1:** IDs MÜSSEN dem Schema ATC-LIC-<TYP>-<NNN> folgen: OSS-001, SOURCE-002, PROTOCOL-003, COMMERCIAL-004, PROPRIETARY-005, ASSET-006, AI-007, DATA-008, EXPERIMENTAL-009; Dach: ATC-LICENSE-CORE.
**KR-2:** Lifecycle je Typ: PLANNED → SPECIFIED (ACTIVE) → DEPRECATED → RETIRED; nur registrierte Typen sind gültig.
**KR-3:** Statusangaben MÜSSEN ehrlich sein: nur 5 Typen sind voll spezifiziert (CORE/OSS/PROTOCOL/ASSET/AI); die übrigen sind PLANNED.
**KR-4:** Projekt-Bindungen erfolgen ausschließlich via ATC-LICENSE.yaml-Manifest (LICENSE-004), nicht über Textprosa.
**KR-5:** Änderungen an der Registry nur via SCR.
**KR-6:** Registry und Katalog (FAM-44) MÜSSEN konsistent sein.

**KR-7:** Jedes Projekt MUSS genau eine primäre Lizenz-ID aus der Registry in seiner manifest.yaml deklarieren (REQ-LIC-036).
**KR-8:** Statusänderungen in `licenses/LICENSE-REGISTRY.yaml` MUESSEN unverzüglich im gesamten Ökosystem propagiert werden (REQ-LIC-037).
**KR-9:** Eine doppelte Vergabe von License-IDs MUSS durch den Validator automatisiert blockiert werden (REQ-LIC-038).

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
- **REQ-LIC-008** (§2): ID-Schema verbindlich; keine abweichenden IDs.
- **REQ-LIC-009** (§2): Lifecycle-Zustand je Typ maschinenlesbar.
- **REQ-LIC-010** (§2): Registry-Konsistenz mit Manifesten prüfbar (Validator S-26, ausstehend).
- **REQ-LIC-036** (§2): Manifest-Registrierung MUSS primäre License-ID binden.
- **REQ-LIC-037** (§2): Status-Synchronität MUSS im Ökosystem unverzüglich wirksam werden.
- **REQ-LIC-038** (§2): Konfliktfreie ID-Vergabe MUSS durch Validator-Gates erzwungen werden.

## Security Considerations

Wildwuchs durch unregistrierte Ad-hoc-Lizenzen; Inkonsistenz Registry↔Projekt-Manifeste.

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
