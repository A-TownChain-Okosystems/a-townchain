---
standard:
  id: ATC-STD-035
  title: "Zero-Day Response Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories und die Organisation gesamt"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
----

# ATC-STD-035 — Zero-Day Response Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-035 · Reaktion auf Schwachstellen ohne verfügbaren Patch · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-033" — §37 ergab 035.

## Abstract

ATC-STD-035 regelt die Zero-Day-Reaktion: gerade weil möglicherweise kein Patch
existiert, MUSS die Kette Confirm→Identify Exposure→Isolate→Mitigate→Monitor→
Patch→Verify→Recover greifen — mit definierten Containment-Maßnahmen.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für Zero-Days (aktive/beitende Ausnutzung ohne Patch). Sonstige
Incidents: ATC-STD-020; Notfall-Änderungen: ATC-STD-036.

## §1 Zero-Day-Kette (REQ-STD-001, MUST)

ZERO-DAY → CONFIRM → IDENTIFY EXPOSURE → ISOLATE → MITIGATE → MONITOR →
PATCH (sobald verfügbar) → VERIFY → RECOVER. Kein Schritt DARF übersprungen
werden; ISOLATE MUSS vor MITIGATE stehen.

## §2 Containment-Maßnahmen (REQ-STD-002, MUST)

Mindestens folgende Optionen MÜSSEN je Betroffenheit geprüft und dokumentiert
werden: Disable Feature, Restrict Network Access, Disable Endpoint, Increase
Monitoring, Rotate Credentials, Revoke Affected Artifacts, Pause Deployment,
Emergency Release.

## §3 Monitoring-Pflicht (REQ-STD-003, MUST)

Bis zum Patch MUSS erhöhtes Monitoring (ATC-STD-021) laufen; Exploit-Versuche
MÜSSEN als Incident-Records geführt werden.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — 9-Schritt-Kette MUSS vollständig durchlaufen werden.
- id: REQ-STD-002 — 8 Containment-Optionen MÜSSEN geprüft/dokumentiert sein.
- id: REQ-STD-003 — Erhöhtes Monitoring MUSS bis Patch-Verfügbarkeit laufen.

## Compliance

Prüfung: Zero-Day-Records (evidence/vulnerabilities/), Ketten-Vollständigkeit,
Monitoring-Nachweis.

## Security Considerations

- Rotation von Credentials MUSS auch alle abhängigen Systeme erfassen.
- Over-Blocking (Productivität vs. Sicherheit): Isolation MUSS dokumentiert
  rückgängig werden können (Recovery-Nachweis).

## Implementierungsstatus

**Status: SPECIFIED** — erste Anwendung im Ernstfall; Übungslauf (Tabletop)
SOLLTE erfolgen. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) —
  Zero-Day-Kette, 8 Containment-Maßnahmen, Monitoring-Pflicht. CANDIDATE.

## References

- ATC-STD-020 — Incident Response · ATC-STD-021 — Monitoring
- ATC-STD-036 — Emergency Security Change · ATC-STD-034 — Exploitability
