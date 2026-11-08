---
standard:
  id: ATC-STD-002
  title: "Standards Family ID & Artifact Identifier Architecture"
  version: "1.0.0"
  status: approved
  category: governance
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Standards, Standard-IDs und Artefakt-Identifikatoren"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
----

# ATC-STD-002 — Standards Family ID & Artifact Identifier Architecture (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf 11.09.2026 (SCR-0096): numerische,
> dauerhaft reservierte Standards-Familien. Die ID beschreibt die Domäne, die
> konkrete Nummer den einzelnen Standard. Bis zur §9-Freigabe bleibt die
> Allokation nach ATC-STD-000 §37 wirksam. **Scope:** ATC-STD-002 ·
> Familien-Struktur + Artefakt-Identifikatoren · **Governance:** ATC-STD-000

## Abstract

ATC-STD-002 definiert die verbindliche Familien-Struktur des Standards-IDs
(10 Domänen-Familien 000–900) und die ID-Architektur der Artefakt-Klassen
(REQ, ADR, F, SCR, GATE, TC, SCHEMA, SPEC, CTRL). Damit entsteht eine saubere
Traceability-Kette: Familie → Standard → Requirement → Implementierung →
Test → Gate → Evidence.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle Standard-IDs und abgeleiteten Artefakt-Identifikatoren. Die
Allokationsausführung (nächste freie Nummer je Bereich) bleibt ATC-STD-000 §37.

## §1 Domänen-Familien (REQ-STD-001, MUST)

Die Nummernbereiche sind dauerhaft reserviert (Bereichs-Reservierung nach
ATC-STD-000 §7.7, entschieden per SCR-0096):

| Familie | Domäne | Zweck | Bereich |
|---|---|---|---|
| 000 | Governance & Meta | Standard-Governance, Lifecycle, Review, Change Control | ATC-STD-000–099 |
| 100 | Architecture | Systemarchitektur, Layer, Komponenten, Schnittstellen | ATC-STD-100–199 |
| 200 | Repository & Git | Repository-Struktur, Branching, Commits, Tags, Releases | ATC-STD-200–299 |
| 300 | Software Engineering | Coding, Testing, CI/CD, Dependencies, Qualität | ATC-STD-300–399 |
| 400 | Security | Security Engineering, Threats, Hardening, Vulnerability Mgmt | ATC-STD-400–499 |
| 500 | Data & Storage | Datenmodelle, Schemas, Storage, Backups, Migration | ATC-STD-500–599 |
| 600 | Blockchain & Protocol | ATC-L1, Konsens, Transaktionen, Smart Contracts | ATC-STD-600–699 |
| 700 | AI & Agents | Aurora, AI, Agenten, Modelle, AI-Safety | ATC-STD-700–799 |
| 800 | OS & Runtime | ShivaCore, GlobusOS, Runtime, Kernel, HAL | ATC-STD-800–899 |
| 900 | Integration & Interoperability | APIs, Bridges, Cross-Chain, externe Systeme | ATC-STD-900–999 |

**Regel (REQ-STD-001a):** Die Hunderterstelle bestimmt die Standards-Familie;
die letzten zwei Stellen identifizieren den Standard innerhalb dieser Familie
(ATC-STD-417 → Familie 400, Standard 17). Neue Standards MÜSSEN im Bereich
ihrer Domäne allokiert werden. SSOT der Familien-Definitionen:
registry/families/FAMILY-000.yaml … FAMILY-900.yaml (§6).

## §2 Kein Recycling (REQ-STD-002, MUST)

Zurückgezogene Standards bleiben historisch mit ihrer ID verbunden.
Verbindliche Reservierungsregeln (Owner-Härtung 11.09.):

1. Eine ID wird NIEMALS wiederverwendet.
2. Die ID bleibt bei DEPRECATED und RETIRED erhalten.
3. Eine ID DARF nur einem Standard zugeordnet werden.
4. Ein Standard DARF seine ID nach Veröffentlichung NICHT ändern.
5. Änderungen erfolgen über `version`, nicht über eine neue ID.
6. Ein komplett neuer Standard erhält eine NEUE ID.
7. Ein Family-Wechsel eines bestehenden Standards erzeugt KEINE stille
   Umnummerierung (Ausnahmeprozess: Nachfolge-Standard mit neuer ID + explicit
   `superseded_by`).
8. Die Registry ist die Single Source of Truth.
9. Freie IDs DÜRFEN reserviert werden (Registry-Zeile Status `reserved`).
10. Gelöschte Dateien geben eine bereits vergebene ID NIEMALS erneut frei.

(Kopplung: ATC-STD-000 §7.3/§7.9, §36 ID-Immutabilität.)

## §3 Bestands-Mapping & Grandfathering (REQ-STD-003, MUST)

Bestehende IDs SIND unveränderlich (§36 ID-Immutabilität) und werden nicht
renummeriert. Bestandskonformität: 000 Governance ✓ (ATC-STD-000, 002),
100 Architecture ✓ (ATC-STD-100 Language Architecture), 200 Repository ✓
(ATC-STD-201–215), 300 Software Engineering ✓ (ATC-STD-300/314).

**Datenmodell (REQ-STD-003a): family_id ≠ standard_id.** Die Domänenkennung
und die Standard-Identität sind im Datenmodell explizit getrennt — 400 ist die
Domänenkennung, ATC-STD-417 die unveränderliche Identität des Standards:

```
family_id: 400
family: SECURITY

standard_id: ATC-STD-417
title: Security Baseline Standard
status: DRAFT
version: 1.0.0
```

**Dokumentierte Legacy-Ausnahme:** die Enterprise-Assurance-Familie
ATC-STD-016–043 (SCR-0087–0095) ist inhaltlich Security/Assurance-Domäne,
lebte aber vor diesem Standard im 000-Bereich (§37-Sequenzialvergabe).
Diese IDs bleiben unverändert (Grandfathering); SICHERHEITS-Standards
ab jetzt MÜSSEN im 400-Bereich allokiert werden. Präfix-Familien
(ERR-, ZKP-, V2S-, TUD-, REPO-DISCOVERY-, CI-, IMPLEMENTATION-,
REPO-MAINT-) behalten ihren Präfix-Namespace; neue Präfix-Familien per SCR.

## §4 Artefakt-Klassen (REQ-STD-004, MUST)

Artefakt-Klassen werden NICHT mit der Standards-Familiennummer vermischt —
sie sind eigene Klassen mit standard-bezogener ID-Struktur:

| Klasse | Format (neu) | Beispiel |
|---|---|---|
| Requirement | REQ-<std>-NNN | REQ-417-001 |
| Test Case | TC-<std>-NNN | TC-417-001 |
| Gate (standard-scoped) | GATE-<std>-NNN | GATE-417-001 |
| Architecture Decision | ADR-<std>-NNN | ADR-417-001 |
| Security Control/Check | ATC-CTRL-<std>-NNN | ATC-CTRL-417-001 |
| Spec/Schema | SPEC-<std>-NNN / SCHEMA-<std>-NNN | SCHEMA-417-001 |

Abgrenzung zu bestehenden, weitergeltenden Systemen (MUSS dokumentiert
bleiben):

- **SCR** bleibt der org-weite SEQUENZIELLE System Change Request
  (SCR-0001…) — NICHT standard-bezogen. Der von der Owner-Referenz
  vorgesehene standard-bezogene Security Control bekommt die eigene Klasse
  ATC-CTRL-<std>-NNN (Kollisionsvermeidung mit SCR-0001…0096).
- **Findings** laufen im Format Pn-CAT3-NNNN (ATC-STD-030 §2); F-NNN-Legacy
  migriert bei Berührung. Ein drittes format F-<std>-NNNN wird NICHT
  eingeführt.
- **Org-Gates** behalten ATC-GATE-<DOM>-NNN (z. B. ATC-GATE-SEC-001);
  standard-scoped Gate-Instanzen DÜRFEN GATE-<std>-NNN nutzen.
- **AD-NNN** (DECISIONS_REGISTER) bleibt grandfathered (ATC-STD-000 §7.9);
  neue Architecture Decisions DÜRFEN ADR-<std>-NNN nutzen.
- **REQ-Format (Owner-Korrektur):** REQ-417-001 statt REQ-STD-417-001 — REQ
  drückt bereits die Artefakt-Klasse aus, 417 verweist auf den Standard.
  Per-Standard-Namespaces der Bestandsstandards (REQ-STD-001…NNN je Datei,
  REQ-TUD-…, REQ-RM-…) bleiben gültig; Migration erfolgt bei nächster
  Major-Revision des jeweiligen Standards.
- **Vierdimensionale Identität:** Family (400) · Standard (ATC-STD-417) ·
  Artifact (REQ/SPEC/ADR/TC/SCR/GATE) · Instance (-001).
- **SCR-Doppelformat (dokumentiert):** SCR-NNNN (4-stellig) = org-weiter
  System Change Request (SCR-0001…); SCR-<std>-NNN (3+3) = standard-scoped
  Security Check gemäß Owner-Kette §5. Beide Muster sind lexikalisch
  unterscheidbar; Validator-Regeln trennen sie strikt. (Owner-Alternative
  ATC-CTRL-<std>-NNN aus SCR-0096-Entwurf bleibt als Alias dokumentiert.)

## §5 Traceability-Kette (REQ-STD-005, MUST)

Jede Kette MUSS auflösbar sein (vollständige 11-Stufen-Kette):

FAMILY → ATC-STD → REQ → SPEC → ADR → IMPLEMENTATION → TEST CASE →
SECURITY CHECK → GATE → EVIDENCE → RELEASE

Beispiel: Family 400 → ATC-STD-417 → REQ-417-001/002, SPEC-417-001,
ADR-417-001, SCR-417-001, TC-417-001/002 → GATE-417-001 → EVIDENCE →
RELEASE. (DTC-Kette nach ATC-STD-300; Evidence Store nach
ATC-GATE-SEC-001 §4.) Die Registry bleibt Allokations-Autorität
(ATC-STD-000 §7.8).

## §6 Registry-Struktur (REQ-STD-006, MUST)

Die Registry gliedert sich in Familien-Definitionen und Standard-Records:

```
registry/
├── families/            # SSOT der Familien (normativ)
│   ├── FAMILY-000.yaml
│   ├── FAMILY-100.yaml
│   └── … FAMILY-900.yaml
└── standards/           # normalisierte Standard-Records (je Standard eine Datei)
    ├── ATC-STD-000.yaml
    ├── ATC-STD-001.yaml
    └── …
```

Übergangs-SSOT: registry/standards.yaml bleibt die maschinenlesbare
Allokations-Autorität (ATC-STD-000 §7.8), bis sämtliche Tools die
per-Standard-Records lesen; registry/standards/*.yaml werden daraus
generiert (Generator: tools/gen_views/gen_registry_records.py) und MÜSSEN
NICHT manuell gepflegt werden. Familie-Dateien SIND normativ (keine
Ableitung). Migration der Tool-Kette = Nachfolge-SCR.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — 10 Domänen-Familien MÜSSEN als dauerhaft reservierte Bereiche gelten; Allokation MUSS in der Domäne erfolgen.
- id: REQ-STD-002 — Recycling IST VERBOTEN; Lücken bleiben historische Marker.
- id: REQ-STD-003 — Bestands-IDs SIND unveränderlich; Legacy-Ausnahmen (016–043) MÜSSEN dokumentiert bleiben; Security ab jetzt in 400.
- id: REQ-STD-004 — Artefakt-Klassen MÜSSEN eigene ID-Strukturen nutzen; Abgrenzungen (SCR, Findings, Org-Gates, AD) MÜSSEN eingehalten werden.
- id: REQ-STD-005 — Traceability-Kette MUSS durchgehend auflösbar sein (11 Stufen, vierdimensionale Identität).
- id: REQ-STD-006 — Registry-Struktur MUSS aus families/ (normativ) und standards/ (normalisiert) bestehen; Reservierungsregeln 1–10 MÜSSEN gelten.

## Compliance

Prüfung: Allokations-Audit bei jedem neuen Standard (Registry-Validator,
S-17-Doppelvergabe), Familien-Zuordnung in Registry-Zeile; Schema-Sync
(schemas/naming-conventions.schema.json, ATC-STD-000 §7.10) als
Implementierungsschritt des Assurance-Rollouts.

## Security Considerations

- ID-Verschleierung/Doppelvergabe unterläuft Traceability: Registry-First
  (§7.8) MUSS greifen.
- Artefakt-IDs von Security-Controls (ATC-CTRL) können Angriffsvektoren
  offenlegen: Zugriffsbeschränkung analog evidence/vulnerabilities.

## Implementierungsstatus

**Status: SPECIFIED** — Familien-Tabelle normativ; Schema-/Validator-Sync
(naming-conventions.schema.json: neue Klassen TC/ADR/CTRL/SPEC/SCHEMA)
als Nachfolge-SCR offen. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): SCR-0096 Initial + Owner-Härtung (gleicher Tag):
  Hunderterstellen-Regel, 10 Reservierungsregeln, Datenmodell-Trennung
  family_id/standard_id, REQ-417-001-Format (Owner-Korrektur), SCR-Doppelformat
  dokumentiert, 11-Stufen-Traceability-Kette, Registry-Struktur families/ +
  standards/. CANDIDATE.

## References

- ATC-STD-000 — Governance Root (§7.7 Namespace Allocation, §7.9 Reserved
  Identifiers, §36/§37)
- ATC-STD-300 — DTC Traceability Chain · ATC-STD-030 §2 — Finding-Format
- ATC-GATE-SEC-001 §4 — Evidence Store
