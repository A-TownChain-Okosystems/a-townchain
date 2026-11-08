---
standard:
  id: ATC-STD-TAXONOMY-001
  title: "ATC Standards Taxonomy & Family Creation Standard — Meta-Governance: vierstufige Taxonomie (Domain→Familie→Kategorie→Standard), kontrollierte Familien-/Kategorie-Erstellung (ATC-FAM-REQ/ATC-CAT-REQ/ATC-TCR), Lifecycle, automatische ID-Vergabe, TAX-CHECK-001..018, maschinenlesbare Taxonomie-Registry"
  version: "1.0.0"
  status: approved
  category: taxonomy
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 08.09.2026, 00:27 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-FRAMEWORK-001
    - ATC-STD-AUDIT-001
    - ATC-STD-DESC-001
    - ATC-STD-VERSION-001
  related_standards:
    - ATC-STD-MILESTONE-001
    - ATC-STD-REPO-AUDIT-001
    - ATC-STD-AOS-001
    - ATC-STD-PROTOCOL-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-TAXONOMY-001 — Standards Taxonomy & Family Creation (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 08.09.2026, 00:27 UTC+2);
> normativ in Kraft ab 08.09.2026, §30-eingefroren (ATC-STD-000). SCR-0024 akzeptiert.
> **Ebene:** Meta-Governance (P0). **Familie:** Standards Governance Core (FAM-43).
> **Maschinenlesbar:** registry/taxonomy.yaml (SSOT der Taxonomie, generiert von
> tools/taxonomy/gen_taxonomy.py, Validator S-24 mit TAX-CHECK-Untermenge).

## Abstract

ATC-STD-TAXONOMY-001 kontrolliert nicht einzelne Standards, sondern die
Standards-Taxonomie selbst: wie neue Familien, Kategorien, Namensräume und
Abhängigkeiten entstehen, verschoben, zusammengeführt, aufgeteilt, umbenannt und
archiviert werden. Damit kann das Standardsystem kontrolliert und praktisch
unbegrenzt wachsen — ohne inkonsistente IDs, Doppelvergaben oder verwaiste
Namensräume. Bestehende Standards sind grandfathered: Ihre IDs bleiben unverändert
(§30); die Taxonomie-Registry bildet den Bestand in die vierstufige Hierarchie ab.

Schlüsselwörter: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE (empfohlen),
DARF/KANN (optional) — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Die gesamte Standards-Taxonomie des Ökosystems: Domains, Familien,
Kategorien, Namensräume, ID-Vergaben und taxonomische Änderungen — inkl. der
Befugnis von KI-Agenten, neue Familien/Kategorien vorzuschlagen (§12).
**Gilt nicht:** Inhalte einzelner Standards (dafür deren eigene Governance);
Retroaktives Umnummerieren des Bestands (§30-verboten).

## §1 Zweck

Der Standard definiert verbindlich, wie innerhalb von ATC (a) neue Standard-Familien
erstellt, (b) neue Kategorien erstellt, (c) bestehende Kategorien erweitert,
(d) Kategorien verschoben, (e) Familien zusammengeführt, (f) Familien aufgeteilt,
(g) Kategorien umbenannt und (h) veraltete Familien/Kategorien archiviert werden —
sowie wie (i) IDs und Namensräume vergeben und (j) Abhängigkeiten zwischen Familien
definiert werden (REQ-TAX-001).

## §2 Hierarchie (vier Ebenen)

```
DOMAIN    → großer fachlicher Bereich
FAMILY    → zusammenhängendes Standardsystem
CATEGORY  → konkretes Themengebiet (feiner als Familien)
STANDARD  → konkrete normative Regel
```
Vollständige ID-Notation für Standards NEUER Familien nach diesem Standard:
`ATC-STD-<FAMCODE>[-<CATEGORY>]-NNN` (lesbar: ATC·STD·Familie·Kategorie·Nummer).
Bestehende Standards behalten ihre IDs (Grandfathering, REQ-TAX-002).

## §3 Neue Familie erstellen

Familien werden über `ATC-FAM-REQ-NNN` beantragt (REQ-TAX-003) mit Feldern:
request_id, name, code, reason, parent_domain, priority, status. Vor Erstellung
MUSS die Pflichtprüfung abgeschlossen sein: existierende passende Familie? ·
Erweiterbarkeit einer bestehenden Familie? · eindeutige Abgrenzung? · ausreichender
Scope? · Überschneidungen? · Abhängigkeiten bekannt? · eigener Namensraum
erforderlich? · mindestens ein initialer Standard? (REQ-TAX-004).

## §4 Mindestkriterium für neue Familien

Eine Familie SOLLTE NICHT für einen einzigen isolierten Standard angelegt werden.
**Empfehlung (verbindlich dokumentiert je Request):** Eine Familie wird erstellt,
wenn mindestens 3 logisch zusammengehörende Standards erwartet werden ODER eine
eigenständige Governance-/Technologie-Domäne vorliegt (REQ-TAX-005). Beispiel einer
rechtfertigenden Familie: 5 Standards Identity/Lifecycle/Decision/Audit/Security.

## §5 Neue Kategorie erstellen

Kategorien werden über `ATC-CAT-REQ-NNN` beantragt (REQ-TAX-006): request_id,
family, name, code, reason, status. Kategorien sind feiner als Familien und MÜSSEN
eindeutig einer Familie zugeordnet sein.

## §6 Unterschied Familie vs. Kategorie

Domain = großer fachlicher Bereich · Family = zusammenhängendes Standardsystem ·
Category = konkretes Themengebiet · Standard = konkrete normative Regel. Kategorien
OHNE Standards sind zulässig, aber als PROPOSED zu führen (REQ-TAX-007).

## §7 Namenskonventionen

Jede Familie erhält einen stabilen, global eindeutigen Code (z. B. AIA); jede
Kategorie einen innerhalb der Familie eindeutigen Code (z. B. IDENTITY). Codes
MÜSSEN mit der Familiengenehmigung gleichzeitig als Naming-Pattern registriert
werden (TAX-CHECK-012; S-02-Präfixliste) — sonst ist der Code nicht verwendbar
(REQ-TAX-008).

## §8 Registry (Single Source of Truth)

Die Taxonomie lebt maschinenlesbar in `registry/taxonomy.yaml` (SSOT der
Taxonomie; generiert von tools/taxonomy/gen_taxonomy.py), referenziert die
Standard-Einträge via registry/standards.yaml (SSOT der Standardeinträge) und wird
je CI-Lauf vom Validator S-24 geprüft (REQ-TAX-009). Änderungen nur via
ATC-TCR + SCR. Die vierstufige Struktur (domains → families → categories) folgt
dem Registry-Format des Owner-Entwurfs.

## §9 Automatische ID-Vergabe

IDs DÜRFEN NICHT frei erfunden werden. Verbindliche Kette (REQ-TAX-010):
Request (FAM-/CAT-REQ) → Taxonomy Validator (TAX-CHECK-001..018) → Name Check →
Duplicate Check → Code Allocation → Registry → ID Assignment. Doppelvergaben
(gleiche ID für verschiedene Standards) sind dadurch strukturell ausgeschlossen.

## §10 Taxonomy Change Request

Taxonomie-Änderungen erfolgen über `ATC-TCR-NNN` mit Typen (REQ-TAX-011):
CREATE_FAMILY · CREATE_CATEGORY · RENAME_CATEGORY · MERGE_FAMILY · SPLIT_FAMILY ·
MOVE_FAMILY · RETIRE_FAMILY · RETIRE_CATEGORY. Jeder TCR MUSS Impact-Analyse
(betroffene Standards, Registries, Referenzen) und Owner-Freigabe enthalten;
MERGE/SPLIT/RENAME berühren KEINE bestehenden Standard-IDs (§30) — nur
Taxonomie-Metadaten (REQ-TAX-012).

## §11 Lifecycle für Familien und Kategorien

```
PROPOSED → ANALYZED → APPROVED → ACTIVE → DEPRECATED → RETIRED
```
Nur ACTIVE Familien dürfen neue Standards aufnehmen; DEPRECATED Familien nehmen
keine neuen Standards auf, bestehende bleiben gültig; RETIRED Familien sind
archiviert (REQ-TAX-013). Sprünge im Lifecycle sind verboten (MILESTONE-001-Analogie).

## §12 KI-Agenten dürfen Familien vorschlagen — nicht erzeugen

Ein ATC-Entwicklungsagent DARF feststellen („GAP DETECTED"), dass für eine neue
Technologie keine passende Standards-Familie existiert. Er DARF dann NICHT einfach
Dateien erzeugen, sondern MUSS die verbindliche Kette durchlaufen (REQ-TAX-014):
GAP DETECTED → CLASSIFICATION → FAMILY/CATEGORY CHECK → TAXONOMY REQUEST
(ATC-FAM-REQ) → IMPACT ANALYSIS → APPROVAL (Owner-Human-Gate, AI-DECISION-001) →
CREATE FAMILY/CATEGORY → REGISTER (taxonomy.yaml via SCR) → CREATE STANDARDS.
Damit erweitern KI-Agenten das Standardsystem, ohne dessen Governance zu umgehen.
Erste Anwendung: Die Beispiel-Familie „AIA (AI Agents)" des Owner-Entwurfs überlappt
mit der existierenden AAS-Familie (ATC-AAS-001..025) — TAX-CHECK-008/010
(Duplicate/Overlap) WÜRDEN den Request zurückweisen bzw. auf MERGE lenken. Diese
Prüfung ist als dokumentierter Negativfall im Standard verankert.

## §13 TAX-CHECK-001..018 (Maschinenprüfungen)

TAX-CHECK-001 Domain exists · 002 Family name unique · 003 Family code unique ·
004 Category name unique · 005 Category code unique · 006 Parent exists ·
007 Scope defined · 008 Duplicate family check · 009 Duplicate category check ·
010 Overlap analysis · 011 Dependency analysis · 012 Naming compliance ·
013 Registry consistency · 014 Reference integrity · 015 Lifecycle state valid ·
016 Owner assigned · 017 Approval recorded · 018 Documentation generated.
Validator S-24 implementiert die maschinenprüfbare Untermenge je CI-Lauf
(001, 002, 003, 004, 005, 006, 013, 014, 015); die volle Checkliste ist Pflichtteil
jedes FAM-/CAT-REQ/TCR (REQ-TAX-015).

## §14 Standards Governance Core (FAM-43)

Der Standards Governance Core besteht aus fünf Standards, die die Infrastruktur
bilden, mit der alle zukünftigen ATC-Standards, Familien und Kategorien kontrolliert
entstehen (REQ-TAX-016): **ATC-STD-TAXONOMY-001** (dieser Standard, Meta-Governance)
· **ATC-STD-STDDEV-001** (Standards-Development-Standard; GEPLANT) ·
**ATC-STD-REGISTRY-001** (Registry-Management; GEPLANT) · **ATC-STD-CHANGE-001**
(Change-Control-Dachnorm; GEPLANT — konsolidiert SCR-Prozess ATC-STD-000 §19–33
mit UPDATE-001/COMPAT-001 zu einer Dachnorm; Abgrenzung bei Erstellung klären) ·
**ATC-STD-AUDIT-001** (bestehend, APPROVED). Katalog-Familie FAM-43.

## Requirements (normativ)

- **REQ-TAX-001** (§1): Zehn taxonomische Operationen verbindlich geregelt.
- **REQ-TAX-002** (§2): Vierstufige Hierarchie; Bestand grandfathered (keine
  Retro-Nummerierung, §30).
- **REQ-TAX-003** (§3): Familienerstellung nur via ATC-FAM-REQ-NNN.
- **REQ-TAX-004** (§3): Acht-Punkte-Pflichtprüfung vor jeder Familienerstellung.
- **REQ-TAX-005** (§4): Mindestkriterium (≥3 erwartete Standards oder eigenständige
  Domäne) je Request dokumentiert.
- **REQ-TAX-006** (§5): Kategorieerstellung nur via ATC-CAT-REQ-NNN mit Familienzuordnung.
- **REQ-TAX-007** (§6): Leere Kategorien sind PROPOSED; Bedeutungstrennung verbindlich.
- **REQ-TAX-008** (§7): Codes global (Familie) bzw. familienweit (Kategorie) eindeutig;
  Naming-Pattern-Registrierung mit Familiengenehmigung.
- **REQ-TAX-009** (§8): registry/taxonomy.yaml ist Taxonomie-SSOT; S-24 prüft je CI-Lauf.
- **REQ-TAX-010** (§9): ID-Vergabe nur über die siebenstufige Kette — keine freie
  ID-Erfindung.
- **REQ-TAX-011** (§10): Taxonomie-Änderungen nur via ATC-TCR-NNN mit Impact-Analyse
  und Owner-Freigabe.
- **REQ-TAX-012** (§10): MERGE/SPLIT/RENAME ändern nie bestehende Standard-IDs.
- **REQ-TAX-013** (§11): Lifecycle ohne Sprünge; nur ACTIVE Familien nehmen Standards auf.
- **REQ-TAX-014** (§12): KI-Agenten: GAP → Anfrage → Human-Gate → Registrierung —
  niemals direkte Dateierzeugung.
- **REQ-TAX-015** (§13): TAX-CHECK-001..018 Pflicht je Request; S-24 automatisiert die
  maschinenprüfbare Untermenge.
- **REQ-TAX-016** (§14): Governance Core = TAXONOMY-001 + STDDEV-001 + REGISTRY-001 +
  CHANGE-001 + AUDIT-001 (FAM-43); drei davon GEPLANT mit Slot-Reservierung.

## Security Considerations

Die Taxonomie-Registry ist Angriffsfläche für Namespace-Hijacking: Wer Familien Codes
zuweist, kontrolliert ID-Räume. Daher: Code-Vergabe nur über die siebenstufige Kette
(§9), Owner-Human-Gate je Familiengenehmigung (§12), S-24 prüft Eindeutigkeit und
Registry-Konsistenz je CI-Lauf. Manipulation an taxonomy.yaml wäre ein P0-Angriff auf
die Meta-Governance — nur via TCR + SCR; §30-Analogie für TAX-IDs.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.0.0** (2026-09-08): Initial Release — Owner-Entwurf (14 Abschnitte):
  vierstufige Taxonomie (Domain→Familie→Kategorie→Standard), ATC-FAM-REQ/ATC-CAT-REQ/
  ATC-TCR-Anfrageketten, Familien-Mindestkriterium, Lifecycle (PROPOSED→RETIRED),
  automatische ID-Vergabe (siebenstufige Kette), KI-Agenten-GAP-Prozess mit
  Human-Gate (inkl. dokumentiertem Negativfall AIA vs. AAS), TAX-CHECK-001..018,
  Standards Governance Core FAM-43 (TAXONOMY-001 + STDDEV-001/REGISTRY-001/CHANGE-001
  GEPLANT + AUDIT-001). Maschinenlesbar: registry/taxonomy.yaml (SSOT, initialer
  Bestands-Abbild: 5 Domains, 32 Familien aus categories.yaml) + Validator NEU S-24
  (Negativtest verifiziert). Grandfathering: keine Retro-Nummerierung (§30).
  SCR-0024; §9-Freigabe Michael Wroblewski 08.09.2026, 00:27 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- registry/taxonomy.yaml + tools/taxonomy/gen_taxonomy.py (SSOT)
- ATC-STD-000 (§7 ID-Räder, §19–33 SCR, §30), ATC-STD-DESC-001 (Standard-Lifecycle)
- ATC-STD-FRAMEWORK-001 (§6.3 Namensraum-Regel, Katalog FAM-43)
- ATC-STD-AUDIT-001 (Governance Core), ATC-STD-AI-DECISION-001 (Human Gates)
- registry/categories.yaml (32 ID-Bereiche/Familien des Bestands)
- ATC-STD-MILESTONE-001 (Lifecycle-Analogie), ATC-STD-VERSION-001

*ATC-STD-TAXONOMY-001 v1.0.0 · Owner-Entwurf Michael Wroblewski · Aurora (Superagent) · 08.09.2026 · SCR-0024*
