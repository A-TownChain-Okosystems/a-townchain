---
standard:
  id: ATC-STD-021
  title: "Continuous Security Monitoring Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories, Nodes, Smart Contracts, APIs, Infrastruktur"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
    - ATC-STD-020
----

# ATC-STD-021 — Continuous Security Monitoring (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Continuous Assurance 11.09.2026 (SCR-0094):
> Sicherheit wird kontinuierlich überwacht, nicht nur beim Release geprüft. Bis zur §9-Freigabe
> nicht wirksam. **Scope:** ATC-STD-021 · Monitoring-Geltungsbereich + Zustandsmodell ·
> **Governance:** ATC-STD-000

## Abstract

ATC-STD-021 definiert verbindlichen kontinuierlichen Security-Monitoring-Geltungsbereich
und ein Zustandsmodell je überwachter Einheit. Ergänzt ATC-STD-018 ( punktuelle
Assurance) zu Dauerüberwachung: Repository, Dependencies, Build System, CI/CD,
Infrastructure, Nodes, Smart Contracts, APIs, Network, Containers, Endpoints.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle überwachten Einheiten der Organisation. Nicht Gegenstand: Penetration
Testing (ATC-STD-018 §5), Incident Response (ATC-STD-020).

## §1 Monitoring-Geltungsbereich (REQ-STD-001, MUSS)

Kontinuierlich überwacht MÜSSEN sein: Repository (Commits, Branch-Protection),
Dependencies, Build System, CI/CD-Pipelines, Infrastructure, Nodes (Testnet/Mainnet),
Smart Contracts, APIs, Network (P2P, RPC), Container, Endpoints.

## §2 Zustandsmodell (REQ-STD-002, MUSS)

Jede überwachte Einheit MUSS einen Zustand führen:

| Zustand | Bedeutung |
|---|---|
| MONITORED | laufend beobachtet, keine Auffälligkeit |
| WARNING | Auffälligkeit erkannt, Beobachtung/Analyse läuft |
| VULNERABLE | bekannte ungepatchte relevante Schwachstelle |
| COMPROMISED | Hinweise auf tatsächliche Kompromittierung |
| CONTAINED | Kompromittierung eingedämmt (ATC-STD-020 §3) |
| RECOVERED | Wiederherstellung abgeschlossen |
| VERIFIED | Wiederherstellung verifiziert (Evidence) |

Zustandsübergänge MÜSSEN mit Evidence (ATC-STD-018 §9) und Finding-/Incident-Verweis
dokumentiert sein. COMPROMISED MUSS ATC-STD-020 (SEV-Kaskade) auslösen.

## §3 Kontinuität (REQ-STD-003, MUST)

Sicherheitsprüfung DARF NICHT auf Release-Zeitpunkte beschränkt sein; Monitoring
MUSS kontinuierlich (automatisiert) laufen. Monitoring-Ergebnisse MÜSSEN in den
§11-Statusfeldern von ATC-STD-018 einfließen.

## §4 Alerting & Eskalation (REQ-STD-004, MUST)

Zustandswechsel auf WARNING/VULNERABLE/COMPROMISSED MÜSSEN ein Finding/Incident
(registry/findings.yaml bzw. ATC-VULN-Records nach ATC-STD-020 v1.1.0) erzeugen
und gemäß ATC-STD-022-SLA eskalieren. Schweigen des Monitorings IST kein
Sicherheitsnachweis (No Evidence = No Security Claim).

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Monitoring-Geltungsbereich: die 11 Einheitsklassen MÜSSEN erfasst und überwacht sein.
- id: REQ-STD-002 — Zustandsmodell: 7 Zustände MÜSSEN geführt und mit Evidence belegt werden; COMPROMISED MUSS Incident-Response auslösen.
- id: REQ-STD-003 — Kontinuität: Prüfung MUSS kontinuierlich, nicht nur release-bezogen erfolgen.
- id: REQ-STD-004 — Alerting: Zustandsverschlechterungen MÜSSEN Findings/Incidents und SLA-Eskalation erzeugen.

## Compliance

Prüfung: Repository-/Org-Audit (ATC-STD-REPO-AUDIT-003) gegen §1-Bereich und
§2-Zustandsführung; Monitoring-Verdrahtung im CI (ATC-STD-018 §6).

## Security Considerations

- Monitoring selbst ist Angriffsfläche: Monitor-Daten MÜSSEN zugriffsbeschränkt sein.
- False-Negative-Risiko: kein Monitoring fehlender Einheiten = Unknown Attack
  Surface = Finding (ATC-STD-028 Attack Surface Management).

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Implementierungs-Evidence entsteht mit
Aufbau des Org-Monitorings (SCR-0094-Nachfolge). SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Continuous Assurance (SCR-0094) —
  Geltungsbereich, 7-Zustands-Modell, Kontinuitäts- und Eskalationspflicht. CANDIDATE.

## References

- ATC-STD-000 — Governance Root
- ATC-STD-018 — Technology Currency & Security Assurance (Statusfelder §11)
- ATC-STD-020 — Incident & Vulnerability Response (v1.1.0: ATC-VULN-Records)
- ATC-STD-022 — Security Patch Management (SLA)
- ATC-STD-028 — Attack Surface Management
