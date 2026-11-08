---
standard:
  id: ATC-AAS-025
  title: "ATC-AAS-025 — Agent Repository Manifest Standard"
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

# ATC-AAS-025 — Agent Repository Manifest Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-025 (Agent Repository Manifest Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Repo-Manifest (Pflicht je R2+-Repository)

Ablage: `.github/ai/agent.yaml` (Ablageort gemäß AI-DEV-001 §6; der
Owner-Entwurf nannte `.agent/agent.yaml` — inhaltlich identisch übernommen).

```yaml
standard: ATC-AAS-025
repository: atclang
agents:
  allowed: [ATC-AI-DEV-001, ATC-AI-TEST-001, ATC-AI-AUDIT-001]
required_standards: [ATC-STD-000, ATC-AAS-001, ATC-AAS-003, ATC-AAS-007,
                     ATC-AAS-008, ATC-AAS-011]
workflow:
  require_tests: true
  require_audit: true
  require_pr: true
  require_human_merge: true
```

## 2. Verbindlichkeit

- Ein Agent darf ein Repo nur betreten, wenn er in `agents.allowed`
  steht UND sein Identity-Manifest (AAS-001) die Repo-Standards aus
  `required_standards` referenziert.
- Die `workflow`-Gates sind mit dem Merge-Gate (AI-DEV-007 §6) verzahnt:
  require_human_merge=true verhindert Agent-Merges endgültig.

## 3. Entry-Chain (automatisiert feststellbar)

IDENTITY → SCOPE → RULES → TASK → ACTION → TEST → EVIDENCE → REVIEW

Beim Repo-Eintritt prüft der Agent Manifest → Scope (AAS-004) →
Standards (AAS-006 Priorität) → offene Tasks (AAS-007) und dokumentiert
das Ergebnis im Task-Record (Discovery-Record, AI-DEV-003 §2).

## 4. Rollout

Nach APPROVED: Ablage in allen R2+-Repos binnen der 30-Tage-Frist
(07.10.2026, identisch zur Agent-Manifest-Auflage aus AI-DEV-001 §16;
Task #111 deckt beide ab).

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

1. Jedes R2+-Repository MUSS ein vollständiges Repository-Manifest unter `.github/ai/repository.yaml` enthalten.
2. Das Repo-Manifest MUSS die zugelassenen Agenten, Rollen und Repository-Richtlinien deklarieren.
3. Änderungen am Repository-Manifest MÜSSEN durch den Repository-Owner genehmigt werden.
4. Agenten MÜSSEN das Repo-Manifest beim ersten Zugriff validieren und Richtlinien einhalten.
5. Fehlt das Repo-Manifest, MUSS das Repository für automatische Agenten-Aktionen gesperrt werden.
6. Das Repo-Manifest MUSS im Versionierungs-Tool gegen unberechtigte Änderungen geschützt sein.

## 6. Metriken & Akzeptanzkriterien

- 100% der R2+-Repositories MÜSSEN ein valides Repo-Manifest besitzen.
- 0 unautorisierte Aktionen MÜSSEN in Repositories ohne Manifest durchgeführt werden.
- Validierungsdauer des Repo-Manifests MUSS unter 50ms liegen.
- Schema-Prüfungen des Manifests MÜSSEN zu 100% erfolgreich sein.

## 7. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) geprüft werden. Abweichungen werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 gemeldet. Manifest-Mängel MÜSSEN behoben werden.

## 8. Security Considerations

Das Repository-Manifest dient als lokale Sicherheits-Policy für den Agentenbetrieb im jeweiligen Repository. Nicht konforme Manifeste MÜSSEN die Sperrung des Repositories nach sich ziehen.

## 9. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 10. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-001, AI-DEV-003, AI-DEV-007, ATC-AAS-001, ATC-AAS-003, ATC-AAS-007, ATC-AAS-008, ATC-AAS-011, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
