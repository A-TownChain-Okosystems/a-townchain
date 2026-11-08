---
standard:
  id: ATC-AAS-015
  title: "ATC-AAS-015 — Agent Git Standard"
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

# ATC-AAS-015 — Agent Git Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-015 (Agent Git Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Operationen (normativ)

branch · commit · push · pull request · review · merge · tag · release.
Force-Push auf main/review-Zweigen verboten (AI-DEV-007 §7); Merge nur
über das Merge-Gate (AI-DEV-007 §6); Tags/Releases unveränderlich
(ATC-STD-000 §34, SCR-0003 Option B).

## 2. Commit-Typen

feat · fix · refactor · docs · test · security · build · ci · chore

Format & Trailer unverändert AI-DEV-007 §1: `ATC-TASK-NNNN: <type>: <desc>`
plus Trailer-Block (Agent-ID, Task-ID, Finding-ID, Action-ID, AI-Role,
Validation).

## 3. SCR-0006 — Erweiterung des Typ-Sets (angenommen 07.09.2026)

AI-DEV-007 §1 normierte feat|fix|docs|test|refactor|chore|spec; dieser
Standard ergänzt `security`, `build`, `ci`, `refactor` (teils neu). Da
AI-DEV-007 approved und unveränderlich ist (ATC-STD-000 §30), wird die
Typ-Set-Vereinheitlichung über SCR-0006 formal nachgezogen; bis dahin
sind beide Sets per AI-DEV-007 v1.0.1 vereinheitlicht (SCR-0006 ACCEPTED, 07.09.2026).

## 4. Pflichtbezug

Jeder Commit trägt Task- oder Requirement-Bezug (Trailer); Commits ohne
Bezug sind nur für rein menschliche Änderungen zulässig.

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
## 5. Ergänzende Verbindliche Betriebs- und Governance-Regeln

1. Agenten MÜSSEN bei Git-Operationen ausschließlich die zugelassenen Commit-Typen verwenden.
2. Commit-Nachrichten MÜSSEN die zugehörige Task-ID oder Issue-Nummer enthalten.
3. Direct Push auf geschützte Branches (wie `main`) MUSS für Agenten ausnahmslos gesperrt bleiben.
4. Alle von Agenten erstellten Git-Commits MÜSSEN digital signiert werden.
5. Vor jedem Push MUSS ein Rebase auf den aktuellen Stand des Ziel-Branches durchgeführt werden.
6. Fehlgeschlagene Push-Versuche MÜSSEN analysiert und dürfen nicht in einer Endlosschleife wiederholt werden.

## 6. Metriken & Akzeptanzkriterien

- 100% aller Commits MÜSSEN den definierten Commit-Typen entsprechen.
- 100% der Agenten-Commits MÜSSEN valide digitale Signaturen tragen.
- 0 ungeplante Push-Konflikte MÜSSEN im Repository entstehen.
- Direct-Push-Versuche auf geschützte Branches MÜSSEN zu 100% abgelehnt werden.

## 7. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS über Validator-Läufe (S-01 bis S-25) geprüft werden. Abweichungen werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 protokolliert. Git-Mängel MÜSSEN bereinigt werden.

## 8. Security Considerations

Signierte Commits stellen sicher, dass Code-Änderungen fälschungssicher dem jeweiligen Agenten zugeordnet werden können. Nicht signierte Commits MÜSSEN vom Repository abgewiesen werden.

## 9. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 10. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-007, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET, SCR-0003, SCR-0006
