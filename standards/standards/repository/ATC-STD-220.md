---
standard:
  id: ATC-STD-220
  title: "Repository File Admission & Placement Standard (Master der Subfamilie 220-229)"
  version: "1.0.0"
  status: approved
  category: repository
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle Repositories der Organisation A-TownChain-Okosystems"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-002
    - ATC-GOV-001
----

# ATC-STD-220 — Repository File Admission & Placement (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Entwurf 11.09.2026 (SCR-0099). Subfamilie
> 220–229 (Familie 200 Repository/Git gem. ATC-STD-002 §1; Owner-Referenz
> „ATC-STD-1XX" als inhaltliche Familien-Kennung interpretiert und der
> Repository-Domaene zugeordnet). Leitgrundsatz: **Presence is not
> permission.** Dass eine Datei technisch committet werden KANN, bedeutet
> nicht, dass sie das Recht BESITZT, Bestandteil des Repositories zu sein.

## Abstract

Der Standard definiert die formale Aufnahmepruefung fuer Repository-Dateien
(Admission), ihre Klassifikation, ihren Berechtigungsgrund (Entitlement), die
autoritative Quelle (Authority) und den Ausnahmeprozess. Jede Datei MUSS vor
Aufnahme bestehen: Eine Datei DARF nur dann Bestandteil eines Repositories
sein, wenn ihr Zweck, ihre Herkunft, ihre Zustaendigkeit, ihr Speicherort und
ihr Lifecycle eindeutig begruendet werden koennen.

Schluesselwoerter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 (ATC-STD-000 §10).

## Scope

**Gilt:** Repository File Admission & Placement fuer ALLE Repositories der
Organisation A-TownChain-Okosystems — Aufnahmepruefung, Klassifikation,
Entitlements, autoritative Quelle, generierte Dateien, Ausnahmen (Familie
200 Repository/Git, Subfamilie 220-229).

**Gilt nicht:** Inhaltliche Qualitaetskriterien von Dateien (Standards-),
Branch-Protection und Rulesets (ATC-GOV-001 Kap. 12) sowie Secret-Scanning
von GitHub (ergaenzend, ATC-STD-038).

## §1 Entscheidungsmodell (REQ-220-001, MUST)

Jede Datei MUSS die zehnspaltige Pruefung bestehen:

| Pruefung | Frage | Ergebnis |
|---|---|---|
| Purpose | Warum existiert die Datei? | MUSS beantwortet sein |
| Ownership | Welches Modul/Projekt besitzt sie? | MUSS eindeutig sein |
| Necessity | Benoetigt fuer Build/Runtime/Tests/Docs/Governance/Entwicklung? | MUSS begruendet sein |
| Placement | Liegt sie am richtigen Ort? | MUSS korrekt sein |
| Authority | Autoritative Quelle oder Duplikat? | MUSS definiert sein |
| Lifecycle | Wer aktualisiert/loescht sie, wann? | MUSS definiert sein |
| Security | Secrets, Credentials, PII, gefaehrliche Artefakte? | DARF keine unzulaessigen Inhalte enthalten |
| Reproducibility | Reproduzierbar erzeugbar? | Generierte Dateien grundsaetzlich nicht committen (Ausnahme §4) |
| Licensing | Herkunft/Lizenz geklaert? | MUSS geklaert sein |
| Consistency | Widerspricht sie anderen Artefakten? | DARF nicht widersprechen |

## §2 Admission-Status (REQ-220-002)

- **ADMITTED** — Datei ist Bestandteil des Repository-Vertrags.
- **CONDITIONAL** — erlaubt unter definierter Bedingung (z. B.
  `.env.example` erlaubt; `.env` mit echten Secrets nicht; `Cargo.lock`
  abhaengig vom Repo-Typ; deklarierte generierte Sichten §4).
- **QUARANTINED** — gefunden, Berechtigung ungeklaert → Review PFLICHT.
- **REJECTED** — darf nicht im Repository sein: `.env`, `*.pem`, `*.key`,
  `credentials.json`, `node_modules/`, `dist/`, `build/`, `target/`,
  `__pycache__/`, `*.pyc`, `.venv*/`, `backup*.zip`, `PRIVATE_KEYS*`
  (je nach Repository-Kontext; Liste nicht abschliessend — DEFAULT-REJECT
  gilt).

## §3 File Classification Taxonomy (REQ-220-003)

SOURCE · CONFIGURATION · BUILD · TEST · DOCUMENTATION · GOVERNANCE ·
SECURITY · SCHEMA · INTERFACE · PROTOCOL · DATA · MIGRATION · TOOLING ·
CI/CD · GENERATED · TEMPORARY · LOCAL · SECRET · VENDOR · ARCHIVE.

Beispiele: `src/main.rs` → SOURCE/ADMITTED · `README.md` →
DOCUMENTATION/ADMITTED · `.github/workflows/` → CI-CD/ADMITTED ·
`target/` → GENERATED/REJECTED · `.env` → SECRET/REJECTED ·
`backup.zip` → ARCHIVE/REJECTED.

## §4 Autoritative Quelle & Generated Artifacts (REQ-220-004)

Existiert dieselbe Information mehrfach (z. B. `docs/ARCHITECTURE.md`,
`ARCHITECTURE.md`, `wiki/ARCHITECTURE.md`), MUSS die autoritative Quelle
definiert sein: README = Entry Point/Ueberblick · docs/ = technische Doku ·
spec|standards/ = normative Spezifikation · wiki/ = erklaerend/operativ.
**Eine Kopie ERHAELT NIEMALS automatisch den Status einer autoritativen
Quelle.** Registry + Repository schlagen Wiki (ATC-STD-000 §1).

Generierte Dateien: Grundregel NICHT committen. Ausnahme CONDITIONAL: Eine
generierte Sicht DARF committet sein, wenn das Repository die Generierung
deklariert (Generator + Regenerations-Kommando dokumentiert, z. B. Header
`GENERIERT aus …`, ATC-STD-002 §6-Muster). Undeklarierte generierte Dateien
sind REJECTED.

## §5 File Entitlement (REQ-220-005)

Jede Datei BENOETIGT mindestens ein legitimes Entitlement (Rechtsgrund):

A = Application · B = Build · T = Test · D = Documentation · G = Governance ·
S = Specification · C = Configuration · I = Interface · P = Protocol ·
O = Operations (Mehrfach moeglich, z. B. ATC-STD-000.md = S/G).

Beispiele: `src/main.rs` = A · `README.md` = D · `.github/workflows/test.yml`
= B/O · `tests/lexer.atc` = T. Dateien ohne legitimes Entitlement
(`random.txt`, `old-final-v2.zip`, `screenshot.png`, `backup/`, `test123/`)
HABEN KEINE Repository-Berechtigung.

## §6 Automatische Pruefung (REQ-220-006, MUST)

`atc audit files` (tools/file_admission/file_admission.py, CHECK-FILE-001):
Git-Diff → neue/geaenderte Dateien → Klassifikation → Admission Rules →
Security Scan → Duplicate/Authority Check → Placement Check → License
Check → Generated Artifact Check → **PASS / WARN / FAIL** (CI-taugliche
Exit-Codes). Exit 0 = PASS, 1 = FAIL, 2 = WARN-Only. Full-Scan-Modus fuer
Repository-Audits. Der Checker ist Gate-faehig (ATC-GOV-001 Kap. 11);
CI-Wiring in Workflow-Dateien = Owner-Aktion (GH013).

## §7 Ausnahmeprozess (REQ-220-007)

DEFAULT → REJECT unless justified → EXCEPTION REQUEST (ATC-EXC-NNN oder
dateilokaler Exception-Record `file_admission_exceptions.yaml`) → OWNER
REVIEW → SECURITY REVIEW (falls relevant) → APPROVAL → EXPIRY. Jede
Ausnahme MUSS reason, owner, approved, expires haben; KEINE permanente
unsichtbare Ausnahme (Kopplung ATC-GOV-001 Kap. 7).

Beispiel: `fixtures/test-dataset.bin`, status EXCEPTION, reason
"required for deterministic integration tests", owner atc-test, approved
true, expires 2027-03-01.

## §8 Subfamilien-Roadmap 220–229 (keine Schatten-IDs)

- 220 (dieser Standard) — File Admission & Placement Master
- 221 — File Classification (Taxonomie-Normierung, KI-Klassifikation)
- 222 — File Placement (Pfad-Matrix je Repo-Typ)
- 223 — Generated Artifact Policy (Registry-Generatoren, Lockfiles)
- 224 — Repository Exceptions (Exception-Records, Expiry-Enforcement)
- 225 — Repository Integrity Audit (`atc audit files`-Rollout, Berichte)

## REQ-Matrix

- REQ-220-001 — Zehnspaltige Pruefung MUSS je Datei bestehen.
- REQ-220-002 — Admission-Status MUSS ADMITTED/CONDITIONAL/QUARANTINED/REJECTED folgen; DEFAULT-REJECT.
- REQ-220-003 — Klassifikation MUSS aus der Taxonomie stammen.
- REQ-220-004 — Autoritative Quelle MUSS definiert; Kopien OHNE Authority-Status; generierte Dateien nur deklariert.
- REQ-220-005 — Jede Datei BENOETIGT mindestens ein Entitlement.
- REQ-220-006 — Automatischer Admission-Check MUSS existieren; CI-Exit-Codes verbindlich.
- REQ-220-007 — Exceptions MUSSEN befristet, benannt und reviewbar sein.

## Compliance

Pruefung: `python3 tools/file_admission/file_admission.py` (full) bzw.
`--diff` im CI. Evidence: audits/file-admission/<datum>-report.md.
Fail-Closed: REJECTED = FAIL; QUARANTINED ohne Review = FAIL/WARN je Modus.

## Security Considerations

- Security-Scan erkennt Secret-Muster (Keys, PEM-Blocks, Token-Präfixe);
  Vollstaendigkeit nicht garantiert — ergaenzt, ersetzt nicht Secret-
  Scanning von GitHub (Kopplung ATC-STD-038).
- Der Checker selbst ist Governance-Werkzeug: Aenderungen via SCR.

## Implementierungsstatus

**Status: IMPLEMENTED (PARTIAL)** — Checker implementiert und am
atc-standards-Repo pilotiert (Evidence: audits/file-admission/); CI-Wiring
in Workflow-Dateien ausstehend (Owner-Aktion GH013); Rollout auf 27 Repos
= Nachfolge-SCR. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf SCR-0099 — Entscheidungsmodell,
  Admission-Status, Taxonomie, Authority/Generated-Regeln, Entitlements,
  Checker CHECK-FILE-001, Ausnahmeprozess, Subfamilie 220–229. CANDIDATE.

## References

- ATC-STD-000 · ATC-STD-002 (Familien/IDs) · ATC-GOV-001 (Gates, Exceptions)
- ATC-STD-038 (Secrets) · ATC-STD-017 (verwaiste Artefakte) · ATC-STD-041/043
