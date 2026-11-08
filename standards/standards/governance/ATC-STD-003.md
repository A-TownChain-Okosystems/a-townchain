---
standard:
  id: ATC-STD-003
  title: "Governance Determinism & Source-of-Truth Matrix Standard (SSOT-Matrix, State-ID, Ein-Zahl-Regel, README-Regeln, ATC-STD-000-Bootstrap, Agent-Autoritaetsgrenzen)"
  version: "1.0.0"
  status: approved
  category: governance
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf 11.09.) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: "2026-09-11"
  review_date: "2027-09-11"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards:
    - ATC-STD-018
    - ATC-STD-020
    - ATC-STD-IMPLEMENTATION-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories und Org-Governance-Dokumente"
----

# ATC-STD-003 — Governance Determinism & Source-of-Truth Matrix (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, §30-eingefroren) — §9-FREIGEGEBEN via Owner-Direktive 11.09.2026
> (Governance-Audit: »Deine Governance erzeugt selbst neue Konsistenzprobleme« — mehrfache
> widerspruchliche Zahlenzustaende in einem Dokument). Kernsatz: **Eine Informationsklasse,
> genau eine autoritative Quelle — alle anderen Ebenen referenzieren, sie speichern nicht.**

## Abstract

ATC-STD-003 macht die Governance selbst deterministisch: Er definiert die verbindliche
Source-of-Truth-Matrix (welche Quelle für welche Informationsklasse autoritativ ist), die
Ein-Zahl-Regel (maschinenverbindliche Standards-Zahl existiert ausschließlich als Registry-Derivat
mit State-ID, Registry-SHA-256 und generated_at), README-Regeln (generierte Dateien enthalten nur
CURRENT STATE; historische Zahlen ausschließlich in STATUS.md/CHANGELOG.md/audits/), die
Referenzpflicht für .github (keine eigenen Zahlen), die Autoritätskaskade Registry > Evidence >
Dokumentation, den Bootstrap-Mechanismus für ATC-STD-000 (EXEMPT befristet mit Sunset), sowie die
Autoritätsgrenzen von KI-Agenten (Executor/Auditor/Maintainer — nie Approver).

## Scope

**Gilt:** Alle generierten Sichtflächen (README/STATUS/AGENT_MANIFEST/Index), alle Zahlenbehauptungen
über Standards/Organisation, das Zusammenspiel von atc-standards, .github-Hub, REALITY_STATUS und
Repository-READMEs; Autoritätskette und State-ID-Schema.

**Gilt nicht:** Fachinhalte der Standards selbst; Implementierungsstatus-Erhebung (ATC-STD-
IMPLEMENTATION-001); Release-Zustände (GitHub Releases).

## §1 SSOT-Matrix (REQ-STD-001, MUSS)
Informationsklasse → autoritative Quelle: Standard-ID/-Text/-Metadaten → registry/standards.yaml +
standards/*.md; Standard-Version → registry/versions.yaml; Abhängigkeiten → registry/dependencies.yaml;
Implementierungsstatus → registry/standard-implementation.yaml; Findings → registry/findings.yaml;
Repository-Inventar/Capability-Ownership → registry/repositories.yaml; Org-Agent-Regeln →
.github/AGENTS.md; technische Realität (Evidence-Historie, append-only) → REALITY_STATUS.md;
Architekturentscheidungen → DECISIONS_REGISTER; Release-Zustand → GitHub Release + Registry.

## §2 Ein-Zahl-Regel & State-ID (REQ-STD-002, MUSS)
Die maschinenverbindliche Antwort auf »Wie viele Standards existieren?« ist ausschließlich
der STATE-Block (generiert): `state.id` (Format `ATC-STATE-<Datum>-<Registry-SHA8>`), `generated_at`,
`registry_sha256`, `standards_total/approved/candidate/draft`, `standard_files`, `families`.
README-Zahlen, REALITY_STATUS-Abschnittszahlen, Agenten-Speicher sind NIEMALS bindend.

## §3 README-Regeln (REQ-STD-003, MUSS)
Generierte READMEs enthalten ausschließlich CURRENT STATE. Historische Zahlen gehören in
STATUS.md/CHANGELOG.md/audits/ und dürfen nie in generierte READMEs zurückfließen. Ein generiertes
README, das mehrere Zeitzustände enthält, ist NON-COMPLIANT (Vollregeneration statt Patch-Regime).

## §4 Referenzpflicht für abgeleitete Repositories (REQ-STD-004, MUSS)
.github und alle anderen Repositories speichern KEINE eigenen Standards-Zahlen. Erlaubt sind nur
Verweise: registry_count + registry_commit + registry_sha256 + Stand der Ableitung. Definition:
`STANDARD_REGISTRY = atc-standards/registry/standards.yaml` — alle anderen referenzieren.

## §5 Autoritätskaskade (REQ-STD-005, MUSS)
Bei Widersprüchen: (1) Registry (normativ) > (2) Evidence/REALITY_STATUS (verifizierte technische
Realität, Historie append-only) > (3) sonstige Doku (README/ROADMAP/Wiki). REALITY_STATUS ist
Evidence-Quelle, keine konkurrierende normative Quelle; normative Festlegungen erfolgen nur über
Registry/Standards.

## §6 ATC-STD-000-Bootstrap (REQ-STD-006, MUSS)
ATC-STD-000 ist als Verfassung Momentaufnahme-EXEMPT: sie durch denselben Prozess, den sie
regelt. Verbindlicher Mechanismus: Bootstrap-Konstitution (v1.x) → normale Governance ab der
nächsten MAJOR (COMPAT-001-Gate). EXEMPT ist befristet dokumentiert (Sunset mit ATC-STD-000 v1.3.0:
vollständige Validator-Einbindung wie jeder andere Standard) und darf für KEIN anderes Dokument
übernommen werden.

## §7 Slot-Prinzip (REQ-STD-007, MUSS)
No standard because a slot exists. Kette: Problem → Requirement → Gap-Analysis →
Standard-Proposal → Review → ID-Allokation. Freie Slots sind Kapazität, kein Auftrag.

## §8 APPROVED ≠ IMPLEMENTED (REQ-STD-008, MUSS)
Normativer Status (APPROVED) und Implementierungsstatus (enforced/implemented/specification_only)
bleiben strikt getrennt (SCR-0048); niemals darf aus APPROVED auf Umsetzung geschlossen werden.

## §9 Agenten-Autoritätsgrenzen (REQ-STD-009, MUSS)
Aurora und alle KI-Agenten sind Executor/Auditor/Maintainer, nie Approver: AI Detection →
Evidence → Finding → Change Request → Automated Validation → **Human Authorization** → Merge →
Registry Update. Kritische Standards (S4/konsenskritisch) erfordern 2-of-N-Menschenfreigabe.
Ein Validator-PASS bestätigt technische Konformität, niemals normative Autorität.

## §10 State-Snapshot (REQ-STD-010, MUSS)
Jeder Audit/Scan dokumentiert den geprüften Zustand als State-ID (Registry-SHA + Zeitpunkt),
damit Aussagen exakt einem Zustand zugeordnet werden können.

## §11 Freigabe
FREIGEGEBEN 11.09.2026 via Owner-Direktive (Governance-Audit-P0-Liste). Priorität P0.

## §30 Freeze & Change-Control
§30-eingefroren; Änderungen ausschließlich via ATC-STD-UPDATE-001.

## Security Considerations
Der Standard verhindert Fehlsteuerung durch veraltete Zahlen (Agenten handeln nach falschen
Ist-Ständen) und Self-Approval-Loops in der Normierung.
