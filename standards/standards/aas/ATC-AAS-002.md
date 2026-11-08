---
standard:
  id: ATC-AAS-002
  title: "ATC-AAS-002 — Agent Capability Standard"
  version: "1.1.0"
  status: approved
  category: aas
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-AAS-002 — Agent Capability Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-002 (Agent Capability Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Deklarationskategorien (kanonisch)

`capabilities.yaml` deklariert je Kategorie TRUE/FALSE + ggf. Detail-Einschränkung:
Repository lesen · Code schreiben · Tests ausführen · Issues erstellen ·
Pull Requests erstellen · Dokumentation ändern · Releases vorbereiten ·
CI/CD ausführen · Infrastruktur verändern · Secrets verwenden ·
Blockchain-Deployments · Smart Contracts · Datenbankzugriff.

## 2. Kernregel

**Nicht deklarierte Fähigkeiten gelten als nicht vorhanden.** (AI-DEV-002 §1,
hier als konkrete Kategorie-Taxonomie verbindlich.)

## 3. Erweiterungen

Neue Kategorien nur über Ergänzung dieses Standards (SCR, ATC-STD-000 §30)
— eine einzelne Agent-Konfiguration darf keine privaten neuen Kategorien
erfinden.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Agenten-spezifisch: Agent-Identitaet via AGENT_MANIFEST verifizierbar; Permissions nach Least-Privilege; Delegationen dokumentiert und widerrufbar; Zugangsdaten ausschliesslich als $ENV-Platzhalter (ATC-STD-203); Agent-Kommunikation authentifiziert, nie anonym.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203, ATC-AAS-001 · INFORMATIVE: AGENT_MANIFEST.md v3.1.7, Registry-Kategorie aas
## 4. Ergänzende Verbindliche Betriebs- und Governance-Regeln

1. Nicht explizit in `capabilities.yaml` als `TRUE` deklarierte Fähigkeiten MÜSSEN vom System unverzüglich blockiert werden (Default-Deny).
2. Dynamische Capability-Erweiterungen im laufenden Betrieb MÜSSEN strikt untersagt werden; Änderungen erfordern einen genehmigten PR.
3. Vor der Ausführung jeder schreibenden oder ausführenden Operation MUSS der Agent seine Capability-Deklaration verifizieren.
4. Capability-Deklarationen MÜSSEN granular auf Ziel-Repositories und Dateipfade eingeschränkt werden.
5. Der Versuch, eine nicht-deklarierte Capability zu nutzen, MUSS als Sicherheitsereignis protokolliert und gemeldet werden.
6. Capability-Prüfregeln MÜSSEN vor dem Aufruf externer Schnittstellen oder Systemwerkzeuge ausgeführt werden.

## 5. Metriken & Akzeptanzkriterien

- 100% aller versuchten Operationen MÜSSEN erfolgreich gegen die Capability-Matrix validiert werden.
- 0 unautorisierte Capability-Ausführungen MÜSSEN im gesamten Systembetrieb toleriert werden.
- Capability-Prüfungslatenz MUSS unter 10 Millisekunden liegen.
- Verletzungen der Capability-Regeln MÜSSEN zu 100% zur Blockade führen.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) verifiziert werden. Abweichungen werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 erfasst. Jede Nichtkonformität MUSS dokumentiert werden.

## 7. Security Considerations

Das Capability-Modell dient als zentrale Begrenzung von Agenten-Aktionen zur Schadensminimierung. Capability-Konfigurationen MÜSSEN schreibgeschützt gegen Manipulationen durch den Agenten selbst abgesichert sein. Unerlaubte Capability-Aufrufe MÜSSEN zum Systemstopp führen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-002, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
