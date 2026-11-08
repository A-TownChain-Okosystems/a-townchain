---
standard:
  id: ATC-STD-BUG-004
  title: "ATC-STD-BUG-004 — Repository Synchronization & Merge Gate Standard"
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

# ATC-STD-BUG-004 — Repository Synchronization & Merge Gate Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-BUG-001…004 (Bug- & Konsistenz-Lebenszyklus) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Kernprinzip:** Trennung Finding → Dokumentation → Fix → Synchronitätsprüfung
> **Scope:** Alle nicht-trivialen Changes mit Doku-/Spec-/Architektur-Relevanz. Nicht gilt: Rein interne Agenten-Protokolle ohne Produktverzug\n>\n> **Verweise:** ATC-STD-000 (§7 IDs, §24 Registry), ATC-STD-201/202/203, registry/findings.yaml, change-requests/

---

## Abstract

Die Grundregel der Dual-Repo-Architektur (AD-030):

> Code darf nicht dauerhaft einen anderen normativen Zustand darstellen
> als Wiki, Specification oder Architecture Repository.

Dieser Standard macht die Synchronitaetspruefung zum **Merge Gate**.

## 1. Change Impact Analysis (REQ-STD-131: MUST vor jedem nicht-trivialen Merge)

```
CODE CHANGE → CHANGE IMPACT ANALYSIS
   ├─ API geaendert?            ├─ Protokoll geaendert?
   ├─ Datenstruktur geaendert? ├─ Verhalten geaendert?
   ├─ Configuration geaendert? └─ Security-Modell geaendert?
        → WIKI IMPACT CHECK → SPECIFICATION CHECK
        → CROSS-REPOSITORY CHECK → RESULT (SYNC-Status)
```

Normative Dokumentation (Standards, Spec, Architektur-Entscheidungen)
und Implementierungsdokumentation (Wiki, README, Modul-Doku) sind dabei
getrennt zu bewerten: Normativ gilt was in atc-standards bzw. dem
DECISIONS_REGISTER steht (ATC-STD-000 §31).

## 2. Repository Consistency Matrix (REQ-STD-132: MUST)

| Quelle | Gegenpruefung |
|---|---|
| Code | Wiki |
| Code | Specification |
| Code | Architecture |
| Code | API Documentation |
| Code | Tests |
| Wiki | Specification |
| Wiki | Architecture |
| Specification | Tests |
| API Docs | Implementierung |
| Version | Changelog |

## 3. Konsistenzstatus (REQ-STD-133)

| Status | Bedeutung | Konsequenz |
|---|---|---|
| SYNC-OK | Code und Doku stimmen ueberein | Merge frei |
| SYNC-DRIFT | Code und Doku unterscheiden sich | CHANGE REQUIRED |
| SYNC-MISSING | Codeaenderung vorhanden, noetige Doku fehlt | BLOCKED |
| SYNC-STALE | Doku beschreibt aeltere Implementierung | CHANGE REQUIRED |
| SYNC-CONFLICT | Wiki und Code widersprechen sich | BLOCKED + Finding |
| SYNC-BLOCKED | Pruefung nicht abschliessbar (Infos fehlen) | BLOCKED |

## 4. Merge Gate (REQ-STD-134: MUST — normativ)

> Kein Change gilt als vollstaendig implementiert, solange nicht
> nachgewiesen wurde, dass die betroffenen Wiki-, Specification-,
> Architecture- und Test-Artefakte den tatsaechlich implementierten
> Codezustand widerspiegeln.

```
Code Change → Bug/Fix-ID → Tests → Documentation Impact Analysis
  → Wiki↔Code-Check → SYNC-OK? ──JA──→ APPROVED → MERGE
                              └─NEIN─→ BLOCKED  → CHANGE REQUIRED
```

Agenten-Pflicht (REQ-STD-135: MUST): Jeder von einem KI-/Entwickler-Agenten
geschlossene Fix dokumentiert einen SYNC-NNN-Record je gepruefter Beziehung
der Konsistenzmatrix und schliesst mit AUD-NNN ab.

Beispiel einer vollstaendigen Kette:

```
F-017 ── SCR-021 (Fix) ── TEST-044 (Regression)
     └─ SYNC-012 (Code↔Wiki, Code↔Spec, Code↔Tests) ── AUD-031 (Final)
```

## 5. Automatisierung

Heute manuell/agentengesteuert; Automatisierungsgrad steigt mit
governance-ci (ATC-STD-201 V-12) und kuenftigen Sync-Pruefwerkzeugen
(Backlog, siehe registry/findings.yaml). Der Gate-Anspruch gilt
unabhaengig vom Automatisierungsgrad (REQ-STD-136: MUST).

## 6. Verweise

AD-030 (kanonische Standards-Heimat), AD-017 (sync_modules.py,
Sync-Punkte), ATC-STD-BUG-001/002/003, AGENT_MASTERRULES REGEL 3
(Code↔Doku-Abgleich).\n\n## Requirements\n\n- id: REQ-STD-131\n  title: "Change Impact Analysis vor jedem nicht-trivialen Merge"\n  severity: MANDATORY\n- id: REQ-STD-132\n  title: "Consistency Matrix geprueft je betroffener Beziehung"\n  severity: MANDATORY\n- id: REQ-STD-133\n  title: "SYNC-Status ermittelt und dokumentiert"\n  severity: MANDATORY\n- id: REQ-STD-134\n  title: "Merge Gate: keine Implementierung ohne Synchronitaets-Nachweis"\n  severity: MANDATORY\n- id: REQ-STD-135\n  title: "Agenten dokumentieren SYNC-NNN je Pruefung und AUD-NNN als Abschluss"\n  severity: MANDATORY\n- id: REQ-STD-136\n  title: "Gate-Anspruch unabhaengig vom Automatisierungsgrad"\n  severity: MANDATORY\n

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
