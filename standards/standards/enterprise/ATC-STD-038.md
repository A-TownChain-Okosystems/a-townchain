---
standard:
  id: ATC-STD-038
  title: "Secrets & Credential Security Standard"
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

# ATC-STD-038 — Secrets & Credential Security Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-038 · Verbot, Detection und Reaktion für Secrets · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-036" — §37 ergab 038.

## Abstract

ATC-STD-038 verankert: Secrets dürfen NIEMALS Bestandteil des
Source-Control-Zustands sein. Es regelt Detection → Revocation → Rotation →
Verification: Das Löschen einer Datei IST KEINE Reaktion auf einen Leak.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für API Keys, Private Keys, Wallet Seeds, Tokens, Passwörter,
Zertifikate, Signing Keys, CI Secrets, Cloud Credentials — in allen
Repositories, CI-Umgebungen und Deployment-Artefakten.

## §1 Source-Control-Verbot (REQ-STD-001, MUST)

Secrets MÜSSEN NIEMALS in Source-Control committet werden (push protection
enabled; .gitignore-Abdeckung). Secret-Scanning MUSS in CI laufen
(ATC-STD-018 §5); ein Leak IST automatisch P0-SEC.

## §2 Leak-Reaktion (REQ-STD-002, MUST)

Bei geleaktem Secret MUSS die Kette greifen: Detection → Revocation →
Rotation → Verification. Das bloße Löschen der Datei MUSS NICHT als Reaktion
gelten; History-Sperrung/-Bereinigung MUSS bewertet werden (Git-Historie
behalten den Secret-Zustand).

## §3 Verwaltung (REQ-STD-003, MUST)

Secrets MÜSSEN über Secrets-Management (Env-/Vault-Pattern, $ENV-Referenzen —
AGENTS-Regel: niemals Klartext-Werte in Logs/Reports/Doku) verwaltet werden;
Ausnahmen MÜSSEN dokumentiert und befristet sein.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Source-Control-Verbot MUSS gelten; Leaks sind P0.
- id: REQ-STD-002 — 4-Schritt-Leak-Reaktion MUSS vollständig durchlaufen werden; Löschen genügt nicht.
- id: REQ-STD-003 — Secrets-Management MUSS verwendet werden; Klartext-Verbot überall.

## Compliance

Prüfung: Secret-Scanning je Repo (CI), Leak-Historie, Rotation-Nachweise.

## Security Considerations

- Commit-Historie IST Exposition: Bewertung MUSS History-Scan einschließen.
- Agenten-/Tool-Logs MÜSSEN maskierte Token-Werte ausgeben (Regel: [REDACTED]).

## Implementierungsstatus

**Status: PARTIALLY IMPLEMENTED** — Push-Protection + Secret-Scanning auf
atc-standards aktiv (AUD-001 F-002/F-003 RESOLVED); Org-weiter Rollout über
Assurance-Engine ausstehend. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) — Verbot,
  Leak-Kette, Verwaltungsregeln. CANDIDATE.

## References

- ATC-STD-018 §5 — Secret Scanning · ATC-STD-035 — Zero-Day (Rotation)
- ATC-STD-030 — Scanner (P0-SEC) · Governance-Security-Regel (AGENTS)
