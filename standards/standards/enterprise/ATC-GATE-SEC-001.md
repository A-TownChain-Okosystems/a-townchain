---
standard:
  id: ATC-GATE-SEC-001
  title: "Continuous Security & Technology Assurance Gate"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories (Release-Readiness) und die Organisation gesamt"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
    - ATC-STD-019
    - ATC-STD-020
    - ATC-STD-021
    - ATC-STD-022
    - ATC-STD-024
    - ATC-STD-026
    - ATC-STD-028
    - ATC-STD-029
----

# ATC-GATE-SEC-001 — Continuous Security & Technology Assurance Gate (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Continuous Assurance 11.09.2026
> (SCR-0094). Meta-Gate: verbindet 018–029 zu einem Release-Readiness-Gate.
> Bis zur §9-Freigabe nicht wirksam. **Scope:** ATC-GATE-SEC-001 ·
> Gate-Zeilen, Security-Status-Definition, Evidence Store, Org-Audit ·
> **Governance:** ATC-STD-000

## Abstract

ATC-GATE-SEC-001 ist der Meta-Gate des Assurance-Systems: Ein Repository gilt nur
dann als RELEASE READY, wenn alle anwendbaren Gate-Zeilen PASS sind. Der Gate
definiert die verbindliche Security-Status-Definition (»sicher vor bekannten
Angriffen« IST evidenzbasiert, versioniert — nie pauschal) und den zentralen
Evidence Store.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für Releases aller ATC-Repositories und den organisationsweiten Audit.
Nicht-Gegenstand: Einzelinhalte der Zeilen (regeln die jeweiligen Standards).

## §1 Gate-Zeilen (REQ-GATE-001, MUST)

| Zeile | Quelle | Bedingung |
|---|---|---|
| Technology Currency | ATC-STD-018 §7/§8 | Review CURRENT / TCS ≥ 75 |
| EOL Detection | ATC-STD-018 §2 | eol_status CLEAR |
| Dependency Security | ATC-STD-019 §1/§8 | Scan PASS |
| Known Vulnerabilities | ATC-STD-018 §3 | vulnerability_status CLEAR (kein offenes Critical/High) |
| Known Bugs | ATC-STD-024 §1 | keine offenen P0/P1 |
| Security Tests | ATC-STD-018 §5 | Suite PASS |
| Regression Tests | ATC-STD-026 §1 | vollständig verankert |
| Secret Scanning | ATC-STD-018 §5 | Scan PASS |
| SBOM | ATC-STD-019 §2 | vorhanden & aktuell |
| Supply Chain | ATC-STD-019 §3-§7 | Gate PASS |
| Threat Model* | ATC-STD-029 | vorhanden & aufgefrischt |
| Attack Surface* | ATC-STD-028 | Inventar vollständig |
| Fuzzing* | ATC-STD-018 §5 | wo anwendbar: PASS |
| Security Review* | ATC-STD-018 §5 | unabhängig, aktuell |

* = kritikalitätsabhängig (C1/S4-Pflicht, sonst Best-Effort mit Dokumentation).

## §2 Release-Readiness (REQ-GATE-002, MUST)

ALL REQUIRED GATES = PASS → RELEASE READY. Andernfalls: FAIL → NO RELEASE.
Critical/High-Findings können NICHT durch Score oder Dokumentation kompensiert
werden (TCS 95 mit offener Critical CVE = FAIL; ATC-STD-018 §8).

## §3 Security-Status-Definition (REQ-GATE-003, MUST)

Verbindliche Ausweise (ersetzen pauschale Aussagen; »Unser System ist sicher« IST
UNZULÄSSIG):

| Status | Bedeutung (nur wenn) |
|---|---|
| VERIFIED | relevante Vulnerabilities geprüft, Controls vorhanden, automatisierte Checks erfolgreich, relevante Angriffsklassen getestet, kritische Findings geschlossen/mitigiert, Security Evidence vorhanden |
| PARTIALLY_VERIFIED | Teil der Anforderungen erfüllt — Lücken dokumentiert |
| NOT VERIFIED | notwendige Nachweise fehlen |
| VULNERABLE | bekannte ungepatchte relevante Schwachstelle |
| COMPROMISED | Hinweise auf tatsächliche Kompromittierung |

## §4 Evidence Store (REQ-GATE-004, MUST)

Zentraler Evidence-Bereich im atc-standards-Repository:

evidence/security/ · evidence/technology/ · evidence/dependencies/ ·
evidence/vulnerabilities/ · evidence/fuzzing/ · evidence/audits/ ·
evidence/penetration-testing/ · evidence/releases/

Grundsatz: Keine künstlichen PASS-Nachweise. Ein fehlender Test bleibt NOT
VERIFIED und MUSS NICHT durch Dokumentation ersetzt werden. Evidence MUSS
versioniert und einem Repository/Release zuordenbar sein.

## §5 Automatischer Organisations-Audit (REQ-GATE-005, SHOULD)

Zielarchitektur: ATC Assurance Engine prüft alle governed Repos fortlaufend in
den drei Achsen Technology/Security/Quality und speist einen Org-Compliance-
Score (BLOCKED bei P0/P1, ATC-STD-018 §8-Prinzip). Vollautomatisierung SOLLTE
über SCR-Planung erreicht werden; bis dahin läuft der Audit agentengestützt
(Aurora, AGENT_MANIFEST) mit denselben Gate-Zeilen.

## §6 Security Release Gate — Spaltenstruktur (REQ-GATE-006, MUST)

Das Release-Gate fasst die Assurance-Standards in drei Spalten:

| Technology | Security | Supply Chain |
|---|---|---|
| Dependencies | Vulnerability | SBOM |
| EOL | SAST | Provenance |
| Compatibility | Secrets | Integrity |
| Updates (ATC-STD-032) | Fuzzing* | Reproducibility (ATC-STD-041) |

ALL REQUIRED PASSED? → YES: RELEASE · NO: BLOCK. Die Spalten strukturieren
§1-Zeilen; die Release-Entscheidung bleibt an §2 gebunden.

## §7 ATC Assurance Framework — Endzustand (REQ-GATE-007, SHOULD)

Zielarchitektur (Owner-Endzustand): ATC STANDARDS speisen die drei Achsen
Governance/Technology/Security → ATC Assurance Engine prüft fortlaufend
Repository, Dependencies, Releases → Evidence Layer (evidence/) →
Compliance Gate → PASS: RELEASE / FAIL: BLOCK. Standards sind damit keine
Markdown-Sammlung, sondern ein ausführbares Framework.

## REQ-Matrix (normative Anforderungen)

- id: REQ-GATE-001 — Gate-Zeilen: 14 Zeilen MÜSSEN geprüft werden; kritikalitätsabhängige Zeilen MÜSSEN als solche markiert sein.
- id: REQ-GATE-002 — Release-Readiness: ALL REQUIRED = PASS sonst NO RELEASE; keine Kompensation durch Score/Dokumentation.
- id: REQ-GATE-003 — Security-Status: 5-Stufen-Definition MUSS verwendet werden; pauschale Sicherheitsaussagen sind unzulässig.
- id: REQ-GATE-004 — Evidence Store: Struktur MUSS bestehen; NOT VERIFIED MUSS NICHT überschrieben werden.
- id: REQ-GATE-006 — Release-Gate MUSS die 3-Spaltenstruktur Technology/Security/Supply Chain abbilden.
- id: REQ-GATE-007 — Endzustand: Framework-Architektur SOLLTE vollausgebaut sein (Engine, Evidence Layer, Gate).
- id: REQ-GATE-005 — Org-Audit: Assurance Engine SOLLTE alle Repos fortlaufend prüfen und einen BLOCKED-fähigen Score speisen.

## Compliance

Prüfung: Gate-Ausweis je Release (evidence/releases/), Org-Audit-Bericht;
Validator-Erweiterung (Gate-Zeilen-Maschine) als SCR-Nachfolge.

## Security Considerations

- Evidence Store enthält Schwachstellen-Details: Zugriffsbeschränkung je
  Unterverzeichnis; öffentlich nur Metadaten/Aggregat.
- Gate-Umgehung (Release ohne PASS) IST ein Governance-Finding P1 +
  Incident-Record (ATC-STD-020 §7).

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Evidence Store angelegt
(evidence/README.md), Gate-Maschine als Implementierungs-SCR offen. SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): SCR-0094 Initial (14 Gate-Zeilen, Release-Readiness-
  Regel, 5-Stufen-Security-Status, Evidence-Store, Org-Audit) + SCR-0095
  Erweiterung: 3-Spalten-Release-Gate (§6), Assurance-Framework-Endzustand
  (§7), REQ-GATE-006/007. CANDIDATE.

## References

- ATC-STD-000 — Governance Root
- ATC-STD-018/019/020/021/022/024/026/028/029 — Assurance-Familie
- ATC-STD-202 — Repository-Klassifizierung (C1/S4-Kritikalität)
