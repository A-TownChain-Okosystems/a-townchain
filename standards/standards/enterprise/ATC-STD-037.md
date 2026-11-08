---
standard:
  id: ATC-STD-037
  title: "Security Configuration Baseline Standard"
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

# ATC-STD-037 — Security Configuration Baseline Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-037 · verbindliche Konfigurationssicherheit · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-035" — §37 ergab 037.

## Abstract

ATC-STD-037 erweitert Assurance vom Code auf Konfigurationen: OS, Container,
Node, Database, Network, Firewall, TLS, API, RPC, Wallet, CI/CD, Cloud,
Secrets, Logging, Monitoring MÜSSEN eine definierte Security Baseline erfüllen
und maschinell prüfbar hinterlegen.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle betriebenen und deployten Konfigurationen; Code-Sicherheit
regeln ATC-STD-018/019.

## §1 Konfigurationsbereiche (REQ-STD-001, MUST)

Je System MUSS die Baseline für die 15 Bereiche (OS, Container, Node,
Database, Network, Firewall, TLS, API, RPC, Wallet, CI/CD, Cloud, Secrets,
Logging, Monitoring) deklariert sein.

## §2 Baseline-Datei (REQ-STD-002, MUST)

Die Baseline MUSS maschinenlesbar hinterlegt sein (je Repo/Systeem), mindestens:

```
security_baseline:
  tls: required
  debug: false
  secrets_in_repository: false
  default_credentials: false
  unnecessary_ports: false
  dependency_lockfile: required
  audit_logging: required
```

Abweichungen MÜSSEN begründet (JUSTIFIED) oder als Finding geführt werden.

## §3 Prüfung (REQ-STD-003, MUST)

Baseline-Einhaltung MUSS in den Assurance-Scanner (ATC-STD-030, Dimension
Security) eingebunden sein; Konfigurationsänderungen an sicherheitsrelevanten
Bereichen MÜSSEN Review-Pflicht haben.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — 15 Bereiche MÜSSEN je System deklariert sein.
- id: REQ-STD-002 — Maschinenlesbare Baseline MUSS existieren; Abweichungen MÜSSEN begründet sein.
- id: REQ-STD-003 — Scanner-Anbindung und Review-Pflicht MÜSSEN bestehen.

## Compliance

Prüfung: Scanner-Zeile Security-Baseline; Baseline-Dateien je Repo.

## Security Considerations

- Baseline-Dateien dürfen KEINE Secrets enthalten (ATC-STD-038).
- Drift zwischen Baseline und Betrieb (Configuration Drift) IST ein Finding.

## Implementierungsstatus

**Status: SPECIFIED** — Baseline-Dateien mit Assurance-Engine-Rollout. SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) — 15
  Bereiche, Baseline-Schema, Scanner-Anbindung. CANDIDATE.

## References

- ATC-STD-030 — Scanner · ATC-STD-038 — Secrets · ATC-STD-028 — Exposition
