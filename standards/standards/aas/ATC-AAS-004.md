---
standard:
  id: ATC-AAS-004
  title: "ATC-AAS-004 — Agent Scope Standard"
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

# ATC-AAS-004 — Agent Scope Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-004 (Agent Scope Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Arbeitsbereich-Definition

```yaml
scope:
  repositories: [atclang, atc-vm]
  directories: [src/, tests/]
  excluded:
    - .github/secrets/
    - production/
    - infrastructure/mainnet/
```

## 2. Regeln

- Zugriffe (read wie write) außerhalb `scope` sind verboten; `excluded`
  blockiert selbst bei sonst erlaubtem Repository/Pfad.
- Scope ist Teil des Identity-Manifests (ATC-AAS-001 §1) und je Task
  als `allowed_files`/`forbidden_files` weiter verfeinerbar (ATC-AAS-007).
- Scope-Überschreitung = Governance-Verstoß → sofortiges Anhalten, Finding
  (AI-DEV-005), BLOCKED (ATC-AAS-020).

## 3. Zweck

Blast-Radius-Begrenzung: Ein Entwicklungsagent kann andere Systeme nicht
versehentlich verändern.

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

1. Zugriffe durch Agenten MÜSSEN strikt auf den in `repository_scope` definierten Pfad- und Repositorienbereich beschränkt sein.
2. Versuche, den Scope über Path Traversal oder relative Symlinks zu verlassen, MÜSSEN zur sofortigen Blockade der Sitzung führen.
3. Pfaddeklarationen im Scope MÜSSEN vor der Ausführung kanonisch aufgelöst und abgeglichen werden.
4. Bei projektübergreifenden Aufgaben MUSS für jedes Ziel-Repository ein separater Scope-Eintrag vorliegen.
5. Änderungen an der Scope-Definition während der Laufzeit MÜSSEN durch ein Human Approval abgesichert werden.
6. Scope-Verletzungen MÜSSEN unverzüglich an den Security-Monitor gemeldet und im Audit-Log festgehalten werden.

## 5. Metriken & Akzeptanzkriterien

- 100% aller Lese- und Schreibzugriffe MÜSSEN innerhalb des deklarierten Scopes liegen.
- 0 unerlaubte Zugriffe außerhalb des Repository-Scopes MÜSSEN erlaubt sein.
- Verifikationsdauer von Pfadgrenzen MUSS unter 5ms pro Operation liegen.
- Scope-Prüfungen MÜSSEN ohne Ausnahmegenehmigung bei allen Pfadoperationen greifen.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch Validator-Läufe (S-01 bis S-25) überprüft werden. Verstöße werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 behandelt. Scope-Fehler MÜSSEN behoben werden.

## 7. Security Considerations

Scope-Begrenzungen verhindern ungewollte Querzugriffe auf sensible Repository-Bereiche. Sämtliche Pfadprüfungen MÜSSEN vor der Dateisysteminteraktion stattfinden. Abweichungen MÜSSEN den isolierten Modus auslösen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-005, ATC-AAS-001, ATC-AAS-007, ATC-AAS-020, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
