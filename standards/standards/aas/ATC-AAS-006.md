---
standard:
  id: ATC-AAS-006
  title: "ATC-AAS-006 — Agent Context Standard"
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

# ATC-AAS-006 — Agent Context Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-006 (Agent Context Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Kontext-Hierarchie

Organization → Repository → Project → Issue → Task → Files → Code → Tests.
Der Agent bindet jede Aussage an die niedrigste verfügbare, verifizierte Ebene.

## 2. Wahrheits-Priorität (bei Konkurrenz)

```
ATC Standards > Repository Governance > Architecture > Requirements >
Issue > Task > Existing Code > Agent Assumption
```

Niedrigere Quelle darf höhere Quelle nie stillschweigend überschreiben —
Widerspruch → Konfliktverfahren (ATC-AAS-013).

## 3. Verifikationsklasse je Aussage

Jede Agenten-Aussage trägt implizit die Klassifikation aus ATC-AAS-012
(FACT/EVIDENCE/INFERENCE/ASSUMPTION/UNKNOWN); `context.yaml` benennt die
für die aktuelle Task maßgeblichen Quellen je Hierarchie-Ebene.

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

1. Bei widersprüchlichen Informationen aus verschiedenen Kontextquellen MUSS die definierte Wahrheits-Priorität strikt angewendet werden.
2. Alle verarbeiteten Aussagen MÜSSEN eindeutig einer Verifikationsklasse (K1-K4) zugeordnet werden.
3. Aussagen der Klasse K4 (ungeprüfte Annahmen) MÜSSEN vor der Verwendung in kritischen Logikpfaden verifiziert werden.
4. Der Kontextspeicher MUSS bei Kontextfenster-Überschreitung nach dem LRU-Prinzip bereinigt werden, wobei normative Regeln erhalten bleiben MÜSSEN.
5. Veraltete Kontextdaten (> 24 Stunden ohne Re-Validierung) MÜSSEN vor ihrer Weiterverwendung neu geprüft werden.
6. Kontext-Updates MÜSSEN atomar erfolgen und dürfen bestehende validierte Fakten nicht unbegründet überschreiben.

## 5. Metriken & Akzeptanzkriterien

- 100% aller im Kontext gespeicherten Fakten MÜSSEN eine zugewiesene Verifikationsklasse besitzen.
- 0 Entscheidungen, die auf unverifizierten Klasse-K4-Aussagen basieren, MÜSSEN zulässig sein.
- Konfliktauflösungszeit im Kontext MUSS unter 20ms liegen.
- LRU-Bereinigungen MÜSSEN normative Vorgaben zu 100% schützen.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS über Validator-Läufe (S-01 bis S-25) verifiziert werden. Befunde werden als Findings F-001 bis F-005 nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 kategorisiert. Kontextfehler MÜSSEN umgehend gemeldet werden.

## 7. Security Considerations

Kontextinjektionen und verfälschte Kontextdaten können zu fehlerhaftem Verhalten führen. Der Agent MUSS Eingaben aus externen Quellen stets validieren. Manipulationen am Kontext MÜSSEN den Notstopp auslösen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, ATC-AAS-012, ATC-AAS-013, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
