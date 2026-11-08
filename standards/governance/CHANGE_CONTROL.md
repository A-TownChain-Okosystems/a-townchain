# Change Control — SCR-Verfahren (operationalisiert)

Geltung: ATC-STD-000 §20 (Change Management). Dieses Dokument macht den
SCR-Prozess operativ: Nummerierung, Lebenszyklus, Registry-Pflichten.

## SCR-Lebenszyklus

```
PROPOSED -> REVIEW -> DECIDED (ACCEPTED | REJECTED | OBSOLETE) -> IMPLEMENTED -> CLOSED
```

- **PROPOSED:** SCR-Dokument in change-requests/ angelegt, Decision: PENDING.
- **REVIEW:** Impact-Analysen (Technical/Security/Architecture/Compatibility/
  Migration) vervollstaendigt; einfache SCRs duerfen Analysen buendeln.
- **DECIDED:** Owner-Entscheidung dokumentiert (Datum + Vermerk).
- **IMPLEMENTED:** Umsetzung per Commits referenziert; Version des betroffenen
  Standards gemaess ATC-STD-000 §21-21 (Breaking = MAJOR).
- **CLOSED:** Registry-Zeile aktualisiert, SCR archiviert im Verzeichnis.

## SCR-Registry

| SCR | Betroffener Standard | Thema | Status | Entscheidung |
|---|---|---|---|---|
| SCR-0001 | ATC-STD-000 | ID-Allokationsprozess je Bereich | PROPOSED | PENDING |
| SCR-0002 | ATC-STD-000 | L↔R-Compliance-Mapping | OBSOLETE | Aufgeloest durch Formalfassung §23 (07.09.) |
| SCR-0003 | ATC-STD-000 | §34-Integritaetsumsetzung (Protected main, CODEOWNERS, Signed Tags) | CLOSED | ACCEPTED (Option B) 07.09. — Implemented: Branch-Protection, Tags, Scanning |
| SCR-0004 | ATC-STD-000 | Rollen- und Berechtigungsmodell | CLOSED | ACCEPTED 07.09. — Implemented: §14.1 in v1.1.0 |

## Regeln

1. Aenderungen an STABLE-Standards NUR via SCR (ATC-STD-000 §20). Fuer
   DRAFT/CANDIDATE-Standends sind Revisionen ohne SCR zulaessig (dokumentiert
   im Changelog des Standards), bis der Status APPROVED erreicht ist.
2. SCR-IDs fortlaufend SCR-XXXX, keine Wiederverwendung.
3. Jedes SCR benutzt templates/SCR_TEMPLATE.md + change-request.schema.yaml.
4. Emergency-Aenderungen (ATC-STD-000 §32) bekommen ein nachgeholtes SCR mit
   Vermerk 'Emergency' — die normale Governance wird nicht dauerhaft umgangen.
