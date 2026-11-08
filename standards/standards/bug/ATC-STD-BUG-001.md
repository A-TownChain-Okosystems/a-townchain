---
standard:
  id: ATC-STD-BUG-001
  title: "ATC-STD-BUG-001 — Bug Finding Standard"
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

# ATC-STD-BUG-001 — Bug Finding Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-BUG-001…004 (Bug- & Konsistenz-Lebenszyklus) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Kernprinzip:** Trennung Finding → Dokumentation → Fix → Synchronitätsprüfung
> **Scope:** Code-Repositories, Wiki-Repositories, Spezifikationen, Tests und technische Doku des Oekosystems. Nicht gilt: Feature-Anfragen ohne Fehlercharakter\n>\n> **Verweise:** ATC-STD-000 (§7 IDs, §24 Registry), ATC-STD-201/202/203, registry/findings.yaml, change-requests/

---

## Abstract

Dieser Standard definiert, wie ein Fehler reproduzierbar und technisch
belastbar identifiziert wird. Ein Bug ist ein **Finding**, erst wenn der
Pflichtprozess vollstaendig durchlaufen und alle Pflichtfelder belegt sind.

## 1. Pflichtprozess (REQ-STD-101: MUST)

```
OBSERVE  →  REPRODUCE  →  ISOLATE  →  CLASSIFY  →  VERIFY  →  CREATE FINDING
```

| Stufe | Bedeutung | Pflicht |
|---|---|---|
| OBSERVE | Fehler beobachten (Log, Test, Report, Audit) | MUST |
| REPRODUCE | Fehler reproduzieren (bei reproduzierbaren Bugs) | MUST |
| ISOLATE | Betroffene Datei/Komponente/Commit isolieren | MUST |
| CLASSIFY | Severity S0-S4 klassifizieren | MUST |
| VERIFY | Gegen Referenz/Spec pruefen — ist es ueberhaupt ein Fehler? | MUST |
| CREATE FINDING | Registry-Eintrag F-NNN anlegen | MUST |

Ein Fehler, der nicht isoliert und verifiziert ist, ist eine Beobachtung,
kein Finding. **Vermutungen sind keine Findings** (REQ-STD-102: MUST).

## 2. Pflichtfelder je Finding (REQ-STD-103: MUST)

| Feld | Pflicht |
|---|---|
| Finding-ID (F-NNN) | Ja |
| Repository | Ja |
| Branch/Commit | Ja |
| betroffene Datei | Ja |
| betroffene Komponente | Ja |
| Fehlerbeschreibung | Ja |
| Reproduktionsschritte | Ja |
| erwartetes Verhalten | Ja |
| tatsaechliches Verhalten | Ja |
| technische Ursache | Wenn bekannt |
| Severity | Ja |
| Evidence/Log | Wenn verfuegbar |
| Testfall zur Reproduktion (TEST-NNN) | Bei reproduzierbaren Bugs |

## 3. Severity-Skala (REQ-STD-104)

| ID | Bedeutung | Beispiele |
|---|---|---|
| S0 | Critical | Konsensbruch, kritische Sicherheitsluecke, Datenverlust, Schluesselkompromittierung, Protokollintegritaet gefaehrdet |
| S1 | High | Schwerer Funktionsfehler, Sicherheitsrelevant ohne S0-Kriterium |
| S2 | Medium | Funktionsfehler mit Umgehung, inkonsistente Doku |
| S3 | Low | Geringfuegiger Fehler, Kosmetik mit Nutzerwirkung |
| S4 | Informational | Beobachtung, Verbesserungshinweis ohne Fehlerwirkung |

**Abgrenzung (REQ-STD-105):** Diese Bug-Severity S0-S4 ist VON den
S-Klassen S0-S4 der ATC-STD-202 (Repository-Security-Klassifizierung) zu
unterscheiden. Kontext trennt eindeutig: Bug-Severity lebt auf Findings
(`severity: S1`), S-Klassen auf Repositories (.atc/repository.yaml).

## 4. Registry und IDs (REQ-STD-106)

- Findings erhalten fortlaufende IDs **F-NNN**, kanonisch gefuehrt in
  `registry/findings.yaml` (atc-standards-Repo, AD-030).
- F-NNN sind **immutable** (ATC-STD-000 §7): nie umbenennen, nie
  wiederverwenden. Aufloesung = Statusaenderung, keine Loeschung.
- Reproduktionstests erhalten **TEST-NNN** (Regressionstest des
  Bug-Lebenszyklus; abgegrenzt von TC-NNN der Conformance-Suiten).

## 5. Verweise

ATC-STD-BUG-002 (Dokumentation), ATC-STD-BUG-003 (Fix-Lifecycle),
templates/finding.template.md, registry/findings.yaml.\n\n## Requirements\n\n- id: REQ-STD-101\n  title: "Pflichtprozess OBSERVE→CREATE FINDING mit allen Stufen"\n  severity: MANDATORY\n- id: REQ-STD-102\n  title: "Vermutungen sind keine Findings — Isolation + Verifikation vor Registrierung"\n  severity: MANDATORY\n- id: REQ-STD-103\n  title: "Pflichtfelder je Finding vollstaendig belegt"\n  severity: MANDATORY\n- id: REQ-STD-104\n  title: "Severity-Klassifizierung S0-S4 je Finding"\n  severity: MANDATORY\n- id: REQ-STD-105\n  title: "Bug-Severity von S-Klassen (ATC-STD-202) getrennt halten"\n  severity: MANDATORY\n- id: REQ-STD-106\n  title: "F-NNN fortlaufend, immutable, kanonisch registry/findings.yaml; TEST-NNN fuer Reproduktionstests"\n  severity: MANDATORY\n

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
