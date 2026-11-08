---
standard:
  id: ATC-STD-LICENSE-008
  title: "Trademark Separation Standard"
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

# ATC-STD-LICENSE-008 — Trademark Separation Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0, §30-eingefroren) — Standard der Familie ATC License
> System (FAM-44); §9-FREIGEGEBEN 08.09.2026, 03:55 UTC+2 via Owner-Direktive
> „ATC-Lizenzsystem als eigene Standardfamilie etablieren" (SCR-0037). Klar getrennt
> von SPDX-Standardlizenzen (Repos behalten Apache-2.0 als Basisschicht, SCR-0036).

## Abstract

ATC-STD-LICENSE-008 (Trademark Separation Standard) ist der achte. Standard der Familie **ATC License System** (FAM-44)
im ATC Enterprise Standards Framework und Teil des ATC-LICENSE-Systems v1.0.0
(License Core, License Types, License Registry, SPDX/Metadaten, Compliance Engine,
Audit System). Strikte Trennung Code ≠ Marke ≠ Asset: ATC-Lizenzen gewähren KEINE impliziten Markenrechte; geschützte Bezeichnungen (A-TownChain, ATC, ShivaCore, Globus OS, Aurora, Shivamon, Logos/Brand-Assets) unterliegen einer separaten Trademark Policy; trademark_use ist default restricted.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Trademark Separation Standard im Zuständigkeitsfeld der Lizenz- und Nutzungsrechte-Governance
des ATC-Ökosystems. **Gilt nicht:** Ersetzung von SPDX-Standardlizenzen auf
Repository-Ebene; rechtliche Beratung (Lizenztexte ersetzen keine Rechtsberatung).

## §1 Gegenstand & Ökosystem-Verortung

Verortet in der Governance-Schicht des ATC-Ökosystems (FAM-44, Range ATC-STD-LICENSE-001..009). Das ATC-LICENSE-System ist die zentrale Lizenz- und Nutzungsrechte-Governance für Code, Protokolle, Smart Contracts, Assets, KI-Modelle, Marken und Daten — klar getrennt von SPDX-Standardlizenzen: Repos behalten Apache-2.0 (SCR-0036) als maschinenlesbare Basisschicht; ATC-LICENSE regelt die Ökosystem-Ebene via licenses/-Registry und ATC-LICENSE.yaml-Manifeste.

## §2 Kernregeln

**KR-1:** ATC-Lizenztexte MÜSSEN ausdrücklich KEINE Markenrechte gewähren (trademark_use: restricted).
**KR-2:** Die Markenliste (A-TownChain, ATC, ShivaCore, Globus OS, Aurora, Shivamon, Logos/Brand-Assets) MUSS zentral geführt werden.
**KR-3:** Markennutzung DURCH Dritte erfordert separate, schriftliche Erlaubnis (Trademark Policy, Owner-Gate).
**KR-4:** Brand-/Logo-Assets sind NICHT über Code-Lizenzen mitlizenziert (ATC-ASSET separat).
**KR-5:** Manifeste MÜSSEN trademarks.included: false + separate_policy: true deklarieren (sofern keine Ausnahme erteilt).
**KR-6:** Namens-/Logo-Verwechslungsgefahr in Forks/Abwandlungen MUSS verboten sein (no-endorsement).

**KR-7:** Alle Marken, Logos und rechtlich geschützten Namen MUESSEN zentral in `TRADEMARKS.md` gelistet sein (REQ-LIC-045).
**KR-8:** Bei Nutzung von ATC-Marken MUSS ein deutlicher Hinweistext auf den Markeninhaber eingefügt werden (REQ-LIC-046).
**KR-9:** Eine Kombination von ATC-Marken mit Drittmarken MUSS ohne schriftliche Genehmigung verboten sein (REQ-LIC-047).

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
- **REQ-LIC-023** (§2): Keine impliziten Markenrechte in ATC-Lizenztexten.
- **REQ-LIC-024** (§2): Markenliste zentral geführt, Nutzung nur via Trademark Policy.
- **REQ-LIC-025** (§2): No-Endorsement-Klausel je Lizenztyp.
- **REQ-LIC-045** (§2): Marken-Registerführung MUSS zentral in TRADEMARKS.md erfolgen.
- **REQ-LIC-046** (§2): Attributions-Pflicht MUSS sichtbaren Urheber-Hinweis erzwingen.
- **REQ-LIC-047** (§2): Co-Branding-Verbot MUSS ohne Freigabe strikt durchgesetzt werden.

## Security Considerations

Implizite Markenrechte erzeugen Markenrechtsverstöße und Reputations-/Verwechslungsrisiken (Pseudo-Affiliation).

Ehrlichkeitsregel: Lizenz-/Review-Zustände MÜSSEN ehrlich benannt sein (ACTIVE/
PLANNED, OSD-Selbstprüfung vs. ausstehender externer Review); erfundene rechtliche
Zusagen sind verboten. ATC-Lizenztexte sind keine Rechtsberatung; externe
rechtliche Prüfung wird empfohlen und ist als ausstehend dokumentiert.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |
