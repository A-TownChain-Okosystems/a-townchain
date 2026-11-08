---
standard:
  id: ATC-STD-REGISTRY-001
  title: "ATC Registry Management Standard — Verbindliche SSOT-Verwaltung aller ATC-Registries: Registry-Inventar mit Zuständigkeiten, Single-Source-of-Truth-Prinzip, Generatoren statt Handarbeit, Cross-Registry-Konsistenz über Validator-Gates, Prozess für neue Registries, Manipulationsschutz"
  version: "1.1.0"
  status: approved
  category: governance-core
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 08.09.2026, 00:36 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-STDDEV-001
    - ATC-STD-TAXONOMY-001
    - ATC-STD-AUDIT-001
  related_standards:
    - ATC-STD-CHANGE-001
    - ATC-STD-VERSION-001
    - ATC-STD-REPO-AUDIT-001
    - ATC-STD-MASTER-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-REGISTRY-001 — ATC Registry Management Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 08.09.2026, 00:36 UTC+2);
> normativ in Kraft ab 08.09.2026, §30-eingefroren (ATC-STD-000). SCR-0025 akzeptiert.
> **Familie:** Standards Governance Core (FAM-43). **Kopplungen:** TAXONOMY-001
> (Meta-Registry), STDDEV-001 (Standard-Einträge), CHANGE-001 (Änderungskanal),
> AUDIT-001 (Evidence), REPO-AUDIT-002 (Health-Report als Registry-Nutzer).

## Abstract

ATC-STD-REGISTRY-001 regiert die Verwaltung aller maschinenlesbaren ATC-Registries:
Jedes Governance-Anliegen hat GENAU eine Single Source of Truth; Generatoren statt
Handarbeit; Cross-Registry-Konsistenz wird je CI-Lauf durch Validator-Gates erzwungen;
neue Registries folgen einem definierten Prozess. Der Standard kodifiziert das im
atc-standards-Repository etablierte Registry-System (10 aktive Registries, S-Gates)
als normative Regel — das Rückgrat der maschinenlesbaren Governance.

Schlüsselwörter: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE (empfohlen),
DARF/KANN (optional) — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Alle ATC-Registries im atc-standards-Repository (registry/ und schemas/)
sowie alle zukünftigen Registries des Ökosystems; ihre Erstellung, Änderung,
Konsistenzsicherung und Abschaltung.
**Gilt nicht:** Fachliche Inhalte der Registries (bestimmen die jeweiligen
Standards); Datenbanken von Anwendungen (KaiOsTodo & Co. folgen ihren eigenen
Governance-Regeln).

## §1 Zweck

Registries sind das Gedächtnis der Governance. Widersprüchliche oder verwaiste
Registries wären Widersprüchlichkeit auf Maschinenebene — dieser Standard macht
die Registry-Landschaft widerspruchsfrei (REQ-RM-001).

## §2 Registry-Inventar (SSOT-Zuständigkeiten)

Jedes Anliegen MUSS GENAU eine SSOT-Registry haben (REQ-RM-002):

| Registry | SSOT für | Generator/Prüfung |
|---|---|---|
| standards.yaml | Standard-Einträge (Status/Version/Mandat) | S-Gates, Manifest-Gate |
| versions.yaml | Versions-/Freigabe-Historie | S-Gates |
| dependencies.yaml | Standard-DAG (Abhängigkeiten) | S-Gates |
| categories.yaml | ID-Bereiche/Familien-Definitionen | S-24 (via taxonomy.yaml) |
| taxonomy.yaml | Taxonomie (Domains/Familien/Kategorien) | tools/taxonomy/gen_taxonomy.py, S-24 |
| framework.yaml | 43-Familien-Katalog (433 Slots) | tools/framework/gen_framework.py, S-21 |
| protocol-registry.yaml | 26 ATC-PROTO-Familien | tools/protocol/gen_protocol_registry.py, S-23 |
| repo-audit-checks.yaml | CHECK-001..064 (Audit-Katalog) | tools/repo-audit/gen_checks.py, S-22 |
| findings.yaml | Findings F-NNN (global) | S-Gates, AUDIT-001 |
| milestones.yaml | Meilenstein-Registry ATC-M-NNN | S-20 |
| naming-conventions.schema.json | ID-/Datei-Pattern | S-16 |

## §3 SSOT-Prinzip

(1) Kein Governance-Anliegen DURCH zwei Registries; (2) keine Parallel-Struktur
statt Registry-Erweiterung; (3) prozessuale Rekorde (z. B. AUD-Records) werden als
Typ in der Fach-Registry geführt, nicht als eigene Dateisammlung (REQ-RM-003 —
Lehrling aus ATC-STD-999: MAUD ist AUD-Record-Typ MASTER, keine Parallelstruktur).

## §4 Generatoren statt Handarbeit

Für jede strukturierte Registry MIT Generator (framework, taxonomy, protocol,
repo-audit-checks) MÜSSEN Änderungen am Generator + Regeneration erfolgen —
direkte Hand-Edits an der generierten Datei sind VERBOTEN (Generator-Kommentar in
der Datei dokumentiert; REQ-RM-004). Registries OHNE Generator (standards.yaml,
versions.yaml, dependencies.yaml) werden redaktionell via SCR gepflegt und vom
Validator geprüft.

## §5 Cross-Registry-Konsistenz

Je CI-Lauf MÜSSEN die Konsistenz-Gates grün sein (REQ-RM-005): jede standards.yaml-
Kategorie ist in taxonomy.yaml aufgelöst (S-24); jede FRAMEWORK-Slot-Referenz ist
registriert (S-21); jeder Standard ist im Agent-Manifest gebunden (Manifest-Gate);
Versionen/Status von standards.yaml ↔ versions.yaml ↔ Standard-Datei stimmen
überein. Ein Gate-FAIL blockiert Commits in den Freigabeprozess (STDDEV-001 §5).

## §6 Neue Registry erstellen

Prozess (REQ-RM-006): (1) Nachweis, dass kein bestehendes SSOT-Anliegen abgedeckt
ist → (2) SCR mit Zuständigkeitsdefinition → (3) Schema/Felddefinition (YAML/
JSON, maschinenlesbar) → (4) Generator-Pflicht bei strukturiertem Inhalt →
(5) Validator-Gate-Anbindung (neues S-Gate oder Erweiterung) → (6) Owner-Freigabe
→ (7) Aufnahme ins Inventar (§2). Jede neue Registry MUSS im Inventar stehen.

## §7 Maschinenlesbarkeit

YAML/JSON verpflichtend; UTF-8; Kommentare nur für Herkunfts-/Regenerierungs-
Hinweise (kein prozessualer Inhalt in Kommentaren); keine Geheimnisse in Registries
(REQ-RM-007). Feldnamen folgen den Naming-Conventions (S-16).

## §8 Änderungs- und Manipulationsschutz

Registry-Änderungen NUR via SCR (CHANGE-001); Commits mit Agent-Trailer; Git-
Historie ist Audit-Trail. Generierte Dateien tragen den Generator-Nachweis; ein
Hand-Edit an generierter Datei ist ein P1-Finding (BUG-005). Registry-Zustände vor
Freigaben dürfen nicht vorweggenommen werden (kein „approved" vor §9) (REQ-RM-008).

## §9 Abschaltung von Registries

RETIRE nur via SCR + Owner-Freigabe; Daten MÜSSEN migriert oder §30-lesbar
archiviert werden; Konsumenten (Gates, Doku) MÜSSEN vorher umgestellt werden
(REQ-RM-009).

## Requirements (normativ)

- **REQ-RM-001** (§1): Widerspruchsfreie Registry-Landschaft als Ziel.
- **REQ-RM-002** (§2): SSOT-Inventar verbindlich; jede Registry eine Zeile.
- **REQ-RM-003** (§3): Keine Doppel-SSOT, keine Parallelstrukturen.
- **REQ-RM-004** (§4): Generierte Registries nur via Generator ändern.
- **REQ-RM-005** (§5): Konsistenz-Gates je CI-Lauf; FAIL blockiert Freigabe.
- **REQ-RM-006** (§6): 7-Schritte-Prozess für neue Registries (Generator +
  Gate + Inventar Pflicht).
- **REQ-RM-007** (§7): YAML/JSON, UTF-8, keine Secrets, Naming-konform.
- **REQ-RM-008** (§8): SCR-only-Änderungen; Hand-Edits an generierten Dateien
  sind P1-Findings; kein Status-Vorgriff vor §9.
- **REQ-RM-009** (§9): RETIRE mit Migration/Archiv + Konsumenten-Umstellung.
- **REQ-RM-010** (§10): Schema-Validierungspflicht: Jede Registry-Datei MUSS gegen ein korrespondierendes JSON/YAML-Schema validiert werden.
- **REQ-RM-011** (§10): Backup und Immutabilität: Ältere Registry-Zustände MUESSEN nach jedem Freigabe-Tag in der Git-Historie unveränderlich archiviert bleiben.
- **REQ-RM-012** (§10): Verwaiste Eintragsvermeidung: Einträge in Registries MUESSEN bei Löschung des Zielobjekts auf DEPRECATED oder RETIRED gesetzt werden.
- **REQ-RM-013** (§10): Sync-Intervall: Automatische Registry-Synchronisation MUSS bei jedem PR-Merge ausgeführt werden.


## §10 Metriken & Akzeptanzkriterien

1. **Schema-Konformitätsrate:** MUSS 100% aller Registries umfassen.
2. **Cross-Registry-Konsistenz:** MUSS in 100% der CI-Läufe (Gates S-16 bis S-24) bestanden werden.
3. **Generator-Integritätsquote:** MUSS 100% Manuelle Hand-Edit-Abwesenheit bei generierten Registries garantieren.
4. **Synchronisations-Latenz:** MUSS nach SCR-Freigabe unter 5 Minuten liegen.

## §11 Compliance & Verifikation

Die Einhaltung der Registry-Standards MUSS kontinuierlich in den Validator-Läufen S-01 bis S-25 (insbesondere S-16, S-20, S-21, S-22, S-23, S-24) und Repository-Audits nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 verifiziert werden. Fehlerhafte Registries erzeugen Findings F-001 bis F-999 und blockieren den Pipeline-Status.

## Security Considerations

Registry-Manipulation wäre ein Direktangriff auf die maschinenlesbare Governance:
falsche „approved"-Status müssten von keinem Menschen mehr geprüft werden. Schutz:
SCR-Pflicht + Git-Historie als Audit-Trail, Generator-Erzwingung mit Regenerierungs-
Kommentar, Konsistenz-Gates je CI-Lauf, §9-Vorgriffs-Verbot. Secrets in Registries
sind verboten — Zugangsdaten gehören in verschlüsselte Stores. Registry-Integritätsverletzungen MUESSEN sofort als Severity S1 eingestuft werden. Unbefugte Änderungen an Schemas oder Generatoren MUESSEN den CI-Build abbrechen.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.1.0** (2026-09-08): Additive Elaboration SCR-0041 — neue Regeln (§10, REQ-RM-010..013), Metriken, Compliance & Verifikation, keine bestehende Regel geändert.
- **1.0.0** (2026-09-08): Initial Release — Kodifizierung des etablierten
  Registry-Systems: SSOT-Inventar (10 Registries + Naming-Schema mit Generator/Gate-
  Tabelle), SSOT-Prinzip mit AUD-Typ-Lehre, Generator-Pflicht, 5 Konsistenz-Gates,
  7-Schritte-Prozess für neue Registries, Maschinenlesbarkeits- und Schutzregeln,
  RETIRE-Prozess. 9 REQ-RM. SCR-0025; §9-Freigabe Michael Wroblewski 08.09.2026, 00:36 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- **NORMATIVE:** ATC-STD-000, ATC-STD-REGISTRY-001, registry/*.yaml (Inventar), schemas/naming-conventions.schema.json
- **INFORMATIVE:** SCR-0041, ATC-STD-BUG-001..005
- tools/framework|taxonomy|protocol|repo-audit/gen_*.py (Generatoren)
- ATC-STD-TAXONOMY-001 (Meta), ATC-STD-STDDEV-001 (Erstellung), ATC-STD-CHANGE-001
- ATC-STD-AUDIT-001 (Evidence), ATC-STD-BUG-005 (Findings), ATC-STD-999 (AUD-Typ)
- Praxis-Nachweis: S-20..S-24 + Manifest-Gate, SCR-0019..0024

*ATC-STD-REGISTRY-001 v1.0.0 · Owner-Entwurf Michael Wroblewski · Aurora (Superagent) · 08.09.2026 · SCR-0025*
