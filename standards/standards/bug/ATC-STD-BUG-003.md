---
standard:
  id: ATC-STD-BUG-003
  title: "ATC-STD-BUG-003 — Bug Fix Lifecycle Standard"
  version: "1.0.0"
  status: approved
  category: bug
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: null
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-BUG-003 — Bug Fix Lifecycle Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-BUG-001…004 (Bug- & Konsistenz-Lebenszyklus) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Kernprinzip:** Trennung Finding → Dokumentation → Fix → Synchronitätsprüfung
> **Scope:** Alle Bug-Fixes ab Fix-Planung. Nicht gilt: Triviale S4-Dokumentationstypos ohne Verhaltensaenderung (begruendungspflichtig)\n>\n> **Verweise:** ATC-STD-000 (§7 IDs, §24 Registry), ATC-STD-201/202/203, registry/findings.yaml, change-requests/

---

## Abstract

Ein Bug-Fix ist nie "Code geaendert". Ein Fix durchlaeuft einen
kontrollierten Lifecycle und ist erst mit Erfuellung der Definition of
Done CLOSED.

## 1. Fix-Lifecycle (REQ-STD-121: MUST)

```
OPEN → ANALYZED → FIX PLANNED → IMPLEMENTED → UNIT TEST → INTEGRATION TEST
→ REGRESSION TEST → SECURITY CHECK → REVIEW → VERIFIED → CLOSED
```

- Stufen duerfen nicht uebersprungen werden; Ausnahmen nur bei S4 ohne
  Verhaltensaenderung (REQ-STD-122: MAY).
- REGRESSION TEST bedarf eines TEST-NNN, der den URSPRUENGLICHEN Fehler
  reproduziert und nach dem Fix bestanden hat (REQ-STD-123: MUST).

## 2. Change Request (REQ-STD-124)

Jeder **nicht-triviale** Fix erhaelt einen SCR-NNN (change-requests/,
Lebenszyklus gemaess ATC-STD-000 §26). Trivial = S4-Dokumentationstypo ohne
Verhaltens- und Security-Relevanz; Trivialitaet wird im Finding begruendet.

```
F-017
 └── SCR-021
      ├── Commit abc123 (fix(...) mit F/SCR-Referenz)
      ├── Tests [TEST-044]
      └── Wiki-/Spec-Update (SYNC-Pflicht, BUG-004)
```

Commits folgen Conventional Commits (ATC-STD-203) und referenzieren
F-NNN sowie SCR-NNN (REQ-STD-125: MUST).

## 3. Definition of Done (REQ-STD-126: MUST — alle Punkte)

- [ ] Ursache identifiziert
- [ ] Fix implementiert
- [ ] urspruenglicher Fehler reproduziert und Bestaetigung dass der Fix ihn behebt
- [ ] Regressionstest (TEST-NNN) vorhanden
- [ ] Tests erfolgreich
- [ ] Security-Auswirkungen geprueft
- [ ] Code Review durchgefuehrt
- [ ] Commit eindeutig referenziert
- [ ] Wiki/Spezifikation geprueft (BUG-004)
- [ ] Repository-Konsistenz geprueft (BUG-004)

## 4. Verweise

ATC-STD-BUG-001/002, ATC-STD-BUG-004 (Sync-Gate), ATC-STD-203
(Conventional Commits, Release), change-requests/SCR-0001…0004.\n\n## Requirements\n\n- id: REQ-STD-121\n  title: "Fix-Lifecycle 12 Stufen ohne Ueberspringen"\n  severity: MANDATORY\n- id: REQ-STD-122\n  title: "Ausnahme nur S4 ohne Verhaltensaenderung, begruendet"\n  severity: CONDITIONAL\n- id: REQ-STD-123\n  title: "Regressionstest TEST-NNN reproduziert den urspruenglichen Fehler"\n  severity: MANDATORY\n- id: REQ-STD-124\n  title: "SCR-NNN fuer nicht-triviale Fixes; Trivialitaet begruendet"\n  severity: MANDATORY\n- id: REQ-STD-125\n  title: "Conventional Commits mit F-NNN-/SCR-NNN-Referenz"\n  severity: MANDATORY\n- id: REQ-STD-126\n  title: "Definition of Done: alle 10 Punkte erfuellt vor CLOSED"\n  severity: MANDATORY\n

## Compliance

Geprueft wird per Review der Finding-/SCR-/Merge-Records gegen die oben deklarierten REQ-STD-Anforderungen
(Manual: Review-Chain gemaess ATC-STD-000 §26; automatisiert: Bestandteile
in atc-std-validator/atc-repo-audit, Ausbau dokumentiert in
registry/findings.yaml). Verstoss gegen MANDATORY = NON-COMPLIANT = GATE
BLOCKED (ATC-STD-BUG-004).

## Security Considerations

S0-Kriterien (Konsensbruch, Schluesselkompromittierung,
Protokollintegritaet) haben Vorrang vor allem Funktions-Backlog; der
Fix-Lifecycle erzwingt SECURITY CHECK vor REVIEW (BUG-003). Security-
Relevanz jedes Findings ist im Feld impact.security zu dokumentieren
(BUG-002).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Aenderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Mandat AD-040) |

## References

**NORMATIVE:** ATC-STD-000 (§7 IDs, §8 Struktur, §26 SCR, §33 Revisionen),
ATC-STD-202 (S-Klassen), ATC-STD-203 (Commits/Release),
registry/standards.yaml · registry/findings.yaml · change-requests/
**INFORMATIVE:** AD-017 (sync_modules.py), AD-030 (kanonische Heimat),
AGENT_MASTERRULES REGEL 1/3, docs/audits/
