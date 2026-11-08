---
standard:
  id: ATC-STD-028
  title: "Attack Surface Management Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle kritischen ATC-Systeme mit Exposition"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
    - ATC-STD-029
----

# ATC-STD-028 — Attack Surface Management (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Continuous Assurance 11.09.2026
> (SCR-0094; Owners Referenz "ATC-STD-026" — Slot 026 ist durch Security
> Regression Prevention belegt, §37 ergab 028). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-028 · Angriffsflächen-Inventar + Vektor-Zustände ·
> **Governance:** ATC-STD-000

## Abstract

ATC-STD-028 verlangt, dass jedes kritische System seine Angriffsfläche vollständig
kennt und bewertet: Unknown Attack Surface IST ein Security Finding.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle Systeme mit Exposition gegen außen (kritische Repos C1/S4 nach
ATC-STD-202/018). Nicht-Gegenstand: Threat-Analyse einzelner Assets
(ATC-STD-029).

## §1 Angriffsflächen-Inventar (REQ-STD-001, MUST)

Jedes kritische System MUSS seine Expositionsvektoren inventarisieren —
mindestens: API, RPC, P2P, Wallet-Schnittstellen, Explorer, Bridge, Oracle,
Smart Contracts, Admin Interfaces. Unbekannte/undokumentierte Angriffsfläche
IST ein Security Finding.

## §2 Vektor-Zustände (REQ-STD-002, MUST)

Für jeden inventarisierten Vektor MUSS der Zustand geführt werden:

| Zustand | Bedeutung |
|---|---|
| EXPOSED | bewusst exponiert |
| PROTECTED | Schutzkontrolle aktiv |
| MONITORED | Überwachung aktiv (ATC-STD-021) |
| TESTED | Tests gegen den Vektor vorhanden |
| JUSTIFIED | EXPOSED ohne Schutz MUSS gerechtfertigt sein |

EXPOSED ohne PROTECTED/MONITORED MUSS JUSTIFIED sein — andernfalls Finding.

## §3 Review-Kopplung (REQ-STD-003, MUST)

Das Angriffsflächen-Inventar MUSS bei jedem Technology Review (ATC-STD-018 §7)
und jedem Release-Gate (ATC-GATE-SEC-001) mitgeführt werden; neue Schnittstellen
MUSS das Inventar vor Go-Live erweitern.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Inventarpflicht: Expositionsvektoren MÜSSEN vollständig erfasst sein; Unbekanntes IST Finding.
- id: REQ-STD-002 — Vektor-Zustände: 5 Zustände MÜSSEN je Vektor geführt werden; EXPOSED ohne Schutz MUSS gerechtfertigt sein.
- id: REQ-STD-003 — Review-Kopplung: Inventar MUSS in Reviews und Gates mitgeführt und vor Go-Live erweitert werden.

## Compliance

Prüfung: Repository-Audit gegen Inventar-Vollständigkeit; Gate-Abgleich
(ATC-GATE-SEC-001 Zeile Attack Surface).

## Security Considerations

- Inventar-Dokumente sind Zielobjekte: Detaillierte Expositionsbeschreibungen
  MÜSSEN zugriffsbeschränkt gespeichert werden (nur Metadaten öffentlich).
- Schatten-Schnittstellen (ungetestete Endpunkte) sind die häufigste
  Unknown Attack Surface: CI-Verdrahtung SOLLTE exponierte Endpunkte
  automatisch gegen das Inventar prüfen.

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Evidence ab erstem
Angriffsflächen-Inventar je kritischem Repo. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Continuous Assurance (SCR-0094) —
  Inventarpflicht, 5 Vektor-Zustände, Review-/Gate-Kopplung. CANDIDATE.

## References

- ATC-STD-000 — Governance Root
- ATC-STD-018 — Security Baseline & Reviews
- ATC-STD-021 — Continuous Security Monitoring (MONITORED)
- ATC-STD-029 — Threat Modeling (Asset-Analyse)
- ATC-GATE-SEC-001 — Continuous Security & Technology Assurance Gate
