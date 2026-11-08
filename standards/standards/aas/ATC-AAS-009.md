---
standard:
  id: ATC-AAS-009
  title: "ATC-AAS-009 — Agent Change Standard"
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

# ATC-AAS-009 — Agent Change Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-009 (Agent Change Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Änderungsdokumentation (Pflicht je Änderung, im PR-Body)

```
WHAT     — was wurde geändert
WHY      — warum war es notwendig (reason mit REQ-/Standard-Bezug)
WHERE    — welche Dateien
IMPACT   — welche Systeme betroffen (Dependency-Check ATC-STD-204)
TEST     — welche Tests/Validierungen (mit CI-Run-Referenz, AAS-010)
RISK     — welche Risiken bestehen (Restrisiken, bekannte Grenzen)
ROLLBACK — wie wird zurückgenommen (git revert …, Migrationshinweise)
```

## 2. Regeln

- Jedes der 7 Felder ist auszufüllen; „n/a" ist eine gültige, aber
  dokumentierte Aussage.
- RISK bei Blockchain-Kernkomponenten (Consensus/Crypto/State/Tokens)
  verpflichtet zur Human-Approval-Kennzeichnung (AAS-017).
- ROLLBACK ist für MERGE-fähige PRs zwingend konkret (kein „siehe oben").

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
## 3. Ergänzende Verbindliche Betriebs- und Governance-Regeln

1. Jede Code- oder Dokumentenänderung MUSS im Body des Pull Requests detailliert und nachvollziehbar begründet werden.
2. Breaking Changes MÜSSEN im Change-Log und der PR-Beschreibung explizit hervorgehoben werden.
3. Änderungen an normativen Vorgaben MÜSSEN die Dokumentenversion im Frontmatter gemäß SemVer anpassen.
4. Agenten MÜSSEN zusammengehörige Modifikationen in atomaren Commits zusammenfassen.
5. Vor dem Einreichen eines PRs MUSS ein lokales Git-Diff ausgeführt werden, um unbeabsichtigte Änderungen auszuschließen.
6. Änderungsnachweise MÜSSEN mit den zugrundeliegenden Requirement-IDs verknüpft und validiert werden.

## 4. Metriken & Akzeptanzkriterien

- 100% aller PRs von Agenten MÜSSEN einen vollständigen Änderungsdokumentationsblock enthalten.
- 0 unidentifizierte Breaking Changes MÜSSEN in freigegebenen PRs auftreten.
- Diff-Prüfung MUSS vor jeder Commit-Erstellung durchgeführt werden.
- Dokumentationspflichten MÜSSEN bei allen Commits eingehalten werden.

## 5. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS über Validator-Läufe (S-01 bis S-25) sichergestellt werden. Mängel werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 festgehalten. Unvollständige PRs MÜSSEN abgewiesen werden.

## 6. Security Considerations

Transparent dokumentierte Änderungen verhindern das unbemerkte Einbringen von Sicherheitslücken oder unerwünschten Funktionen. Unbefugte Modifikationen MÜSSEN sofort zurückgerollt werden.

## 7. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 8. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, ATC-ENT-002, ATC-STD-000, ATC-STD-204, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
