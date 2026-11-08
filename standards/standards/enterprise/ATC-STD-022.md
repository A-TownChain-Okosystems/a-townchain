---
standard:
  id: ATC-STD-022
  title: "Security Patch Management Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories und Betriebsumgebungen"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
    - ATC-STD-020
----

# ATC-STD-022 — Security Patch Management (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Continuous Assurance 11.09.2026 (SCR-0094):
> Verbindliche Reaktionszeiten auf Schwachstellen. Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-022 · Patch-SLAs + Emergency Response · **Governance:** ATC-STD-000

## Abstract

ATC-STD-022 legt verbindliche Ziel-Reaktionszeiten für Security-Patches fest und
definiert den Notfallpfad für aktiv ausgenutzte Schwachstellen. Damit wird aus
"wir sollten Updates installieren" (ATC-STD-018) eine befristete Pflicht.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle Schwachstellen-Befunde (ATC-STD-018 §3, ATC-STD-020). Nicht
Gegenstand: allgemeine Technologie-Updates (ATC-STD-018 §7 Reviews).

## §1 Patch-SLAs (REQ-STD-001, MUST)

| Severity | Zielreaktion (ab Detektion/Triage) |
|---|---|
| Critical | ≤ 24 h |
| High | ≤ 72 h |
| Medium | ≤ 14 Tage |
| Low | ≤ 30 Tage |

Die SLA-Uhr MUSS mit Detektion/Triage (ATC-STD-020 §1) starten; SLA-Verletzung
MUSS ein Finding erzeugen und eskalieren.

## §2 Active-Exploit-Notfallpfad (REQ-STD-002, MUST)

Bei aktiv ausgenutzter Schwachstelle MUSS die strengere Kette greifen:

ACTIVE EXPLOIT → Emergency Security Response → Immediate Mitigation →
Emergency Patch → Verification → Post-Incident Review.

Immediate Mitigation MUSS vor dem Emergency Patch stehen (Containment-Optionen
nach ATC-STD-020 §3); der Notfallpfad MUSS NICHT die Human-Gates von ATC-STD-020
(SEV-1/2) aussetzen, außer die Mitigation erfordert unmittelbares Handeln —
nachträglich MUSS vollständig dokumentiert werden.

## §3 Patch-Verifikation (REQ-STD-003, MUST)

Jeder Security-Patch MUSS verifiziert werden (Tests + Security-Scan + Evidence-
Record) und einen Regressionstest erzeugen (ATC-STD-026). Unverifizierte Patches
GELTEN NICHT als Behbung.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Patch-SLAs: 4-stufige Zielreaktionszeiten MÜSSEN eingehalten und Verstöße als Finding geführt werden.
- id: REQ-STD-002 — Active-Exploit-Pfad: Notfallkette MUSS greifen; Mitigation MUSS vor Patch stehen; Dokumentationspflicht nachgeholt werden.
- id: REQ-STD-003 — Patch-Verifikation: Jeder Patch MUSS verifiziert sein und einen Regressionstest erzeugen.

## Compliance

Prüfung: SLA-Tracking im Vulnerability-Record (ATC-STD-020 v1.1.0: detected_at,
patched_at), Audit gegen SLA-Tabelle; Emergency-Pfad via Incident-Records.

## Security Considerations

- Patch-Deployment selbst angreifbar: Emergency-Patches MÜSSEN über denselben
  Supply-Chain-Schutz (ATC-STD-019) laufen — kein Bypass.
- SLA-Drift: SLA-Tabelle ist revisionspflichtig (review_date), Verschärfungen
  via SCR.

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Evidence mit erstem
Vulnerability-Management-Betrieb. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Continuous Assurance (SCR-0094) — SLA-Tabelle,
  Active-Exploit-Notfallpfad, Patch-Verifikationspflicht. CANDIDATE.

## References

- ATC-STD-000 — Governance Root
- ATC-STD-018 — Vulnerability Management (Quelle der Befunde)
- ATC-STD-020 — Incident & Vulnerability Response (v1.1.0: Records, SEV-Modell)
- ATC-STD-026 — Security Regression Prevention
