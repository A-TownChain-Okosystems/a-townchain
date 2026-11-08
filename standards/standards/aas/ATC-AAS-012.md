---
standard:
  id: ATC-AAS-012
  title: "ATC-AAS-012 — Agent Hallucination / Assumption Standard"
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

# ATC-AAS-012 — Agent Hallucination / Assumption Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-012 (Agent Hallucination / Assumption Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Aussagen-Klassifikation (Pflicht)

```
FACT       — mit Fundstelle verifiziert
EVIDENCE   — aus Evidenz abgeleitet (AAS-010 §3)
INFERENCE  — logische Folgerung aus verifizierten Fakten
ASSUMPTION — nicht verifizierte Annahme (ASSUMPTION-ANNN, AI-DEV-001 §10)
UNKNOWN    — keine Evidenz vorhanden
```

Beispiel:
```yaml
statement: "ATCLang VM uses Rust."
classification: FACT
evidence: {repository: atclang, file: Cargo.toml}
```

## 2. Kernregeln

- Ohne Evidenz: classification `UNKNOWN` — niemals „probably …".
- ASSUMPTION mit verification.required blockiert COMPLETED (AI-DEV-001 §10);
  Bestätigung hebt die Klassifikation auf FACT/verifiziert.
- Aussagen in Findings, Audit-Records und PR-Bodies tragen ihre
  Klassifikation implizit über die Fundstellen-Pflicht (AI-DEV-005 §2).
- Verwechslung von INFERENCE mit FACT ist ein Governance-Verstoß (S2).

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

1. Jede vom Agenten generierte Aussage MUSS explizit als Fakt, Schlussfolgerung oder Annahme deklariert werden.
2. Unbestätigte Annahmen MÜSSEN vor der Verwendung in der Ausführungsphase verifiziert oder verworfen werden.
3. Spekulative Aussagen (Klasse 'Guess') MÜSSEN in normativen Dokumenten und Code-Kommentaren strikt vermieden werden.
4. Erkennt der Agent eine Halluzination in früheren Ausgaben, MUSS er diese sofort korrigieren und markieren.
5. Kritische Systementscheidungen MÜSSEN ausschließlich auf verifizierten Fakten (Klasse 'Fact') basieren.
6. Klassifikations-Labels MÜSSEN maschinenlesbar sein und automatisierte Prüfungen ermöglichen.

## 4. Metriken & Akzeptanzkriterien

- 100% aller Aussagen in PRs MÜSSEN eine gültige Klassifizierung tragen.
- 0 unüberprüfte Annahmen MÜSSEN in produktivem Code oder Konfigurationen verbleiben.
- Erfassungsquote von Korrekturen MUSS 100% betragen.
- Halluzinations-Scans MÜSSEN vor dem Einreichen ausgeführt werden.

## 5. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS über Validator-Läufe (S-01 bis S-25) verifiziert werden. Verstöße werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 erfasst. Unklassifizierte Texte MÜSSEN korrigiert werden.

## 6. Security Considerations

Halluzinationen können Sicherheitslücken erzeugen. Das Filtern ungeprüfter Annahmen schützt das Gesamtsystem vor unvorhersehbarem Agentenverhalten. Unklare Behauptungen MÜSSEN verworfen werden.

## 7. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 8. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-001, AI-DEV-005, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
