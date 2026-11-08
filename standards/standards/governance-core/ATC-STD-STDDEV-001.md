---
standard:
  id: ATC-STD-STDDEV-001
  title: "ATC Standards Development Standard — Verbindlicher Lebenszyklus für Standards: Erstellung im Hausformat, Review, §9-Freigabe, §30-Einfrierung, Wartung (MINOR/MAJOR), Review-Zyklen, Deprecation und Retirement — die Prozessnorm, mit der Standards selbst entstehen"
  version: "1.0.0"
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
    - ATC-STD-DESC-001
    - ATC-STD-TAXONOMY-001
    - ATC-STD-REGISTRY-001
    - ATC-STD-AUDIT-001
  related_standards:
    - ATC-STD-VERSION-001
    - ATC-STD-CHANGE-001
    - ATC-STD-MD-001
    - ATC-STD-AI-DECISION-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-STDDEV-001 — ATC Standards Development Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 08.09.2026, 00:36 UTC+2);
> normativ in Kraft ab 08.09.2026, §30-eingefroren (ATC-STD-000). SCR-0025 akzeptiert.
> **Familie:** Standards Governance Core (FAM-43). **Kopplungen:** ATC-STD-000
> (§9/§30), DESC-001 (Standard-Lifecycle), TAXONOMY-001 (Meta-Governance),
> REGISTRY-001 (SSOT), CHANGE-001 (Dachnorm).

## Abstract

ATC-STD-STDDEV-001 ist die Prozessnorm, mit der ATC-Standards selbst entstehen:
ein verbindlicher Lebenszyklus von der Idee über SCR, Entwurf im Hausformat,
Validierung, Owner-Review und §9-Freigabe bis zur §30-Einfrierung — einschließlich
Wartung (MINOR/MAJOR), jährlichem Review-Zyklus, Deprecation und Retirement. Er
kodifiziert die Praxis der Standard-Erstellung des atc-standards-Repositories als
normative Regel und schließt damit die letzte prozessuale Lücke des Governance Core.

Schlüsselwörter: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE (empfohlen),
DARF/KANN (optional) — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Alle ATC-Standards (neu und bestehend), ihre Erstellung, Freigabe,
Wartung, Review, Deprecation und Retirement — inkl. der Rollen von KI-Agenten
als Autor-Entwerfer.
**Gilt nicht:** Inhaltliche Gestaltungsfreiheit einzelner Standards (der Standard
bestimmt den Prozess, nicht den Inhalt); Repositories ohne Standards-Charakter.

## §1 Zweck

Standards sind die Verfassung des Ökosystems — ihre Entstehung MUSS so kontrolliert
sein wie ihre Wirkung. Dieser Standard definiert den verbindlichen Weg von der
Idee zur normativen Regel (REQ-SD-001).

## §2 Rollen und Autorität

- **Owner (Michael):** Einziges §9-Freigabe-Organ; ohne seine Freigabe ist kein
  Standard APPROVED oder normativ (Human Gate, REQ-SD-002).
- **Antragsteller:** Jeder (Mensch oder Agent) DARF Standard-Ideen via SCR
  einbringen; die Entscheidung liegt beim Owner.
- **Autor-Agent:** DARF Standards entwerfen und ausarbeiten; DARF NICHT freigeben,
  einfrieren oder als APPROVED deklarieren (REQ-SD-003; AI-DECISION-001).
- **Standards Governance:** Validator-Suite + Registry als objektive Prüfinstanz
  (S-Gates); sie prüft, der Owner entscheidet.

## §3 Standard-Lebenszyklus

```
IDEA → SCR → DRAFT → VALIDATED → REVIEWED → APPROVED (§9) → EFFECTIVE (normativ)
     → MAINTAINED (MINOR/MAJOR) → DEPRECATED → RETIRED
```
Statuswechsel nur vorwärts; Ausnahmen (Rollback) nur via SCR mit Owner-Freigabe
(REQ-SD-004). Mapping auf DESC-001-Lifecycle und Registry-Status: draft ↔ DRAFT,
approved ↔ APPROVED/EFFECTIVE. RETIRED-Standards bleiben §30-lesbar, sind aber
nicht mehr normativ bindend (REQ-SD-005).

## §4 Hausformat-Pflicht

Jeder Standard MUSS dem Hausformat entsprechen (REQ-SD-006): YAML-Frontmatter
(vollständige Standard-Metadaten), Titel mit Version/Status, Status-Blockquote
mit Freigabe-Nachweis, Abstract mit RFC-2119-Schlüsselwörtern, Scope (Gilt/Gilt
nicht), §-nummerierte Abschnitte, normative Requirements mit registrierten
REQ-<FAM>-NNN-IDs, Security Considerations, Standard-interner Changelog,
References, Fußzeile. Struktur-Basis: DESC-001 + MD-001; Validierung via
Validator-Suite (ALL COMPLIANT als EFFECTIVE-Voraussetzung, REQ-SD-007).

## §5 Erstellungsprozess (10 Schritte)

(1) Idee + Recherche (Bestands- und Katalog-Check: TAXONOMY-001 Pflichtprüfung)
→ (2) SCR anlegen (change-requests/SCR-NNNN.md) → (3) Entwurf im Hausformat →
(4) Maschinelle Validierung (Validator-Suite, betroffene S-Gates) → (5) Selbst-
Review des Autors → (6) Owner-Review → (7) §9-Freigabe (Human Gate) →
(8) Registry-Synchronisation (standards.yaml, versions.yaml, Manifest) →
(9) Dokumentations-Sync (STATUS, README, CHANGELOG) → (10) Commit/Push mit
Commit-Trailer + §30-Einfrierung (REQ-SD-008). Schritte DÜRFEN NICHT übersprungen
werden; die Reihenfolge 7→8→9→10 ist zwingend (kein EFFECTIVE ohne APPROVED).

## §6 §9-Freigabe und Sammelfreigaben

Die Freigabe MUSS im Chat des Owners dokumentiert sein (Zeitstempel, Umfang).
Sammelfreigaben MEHRERER Standards in einem Akt sind zulässig; gebündelte
FRAMEWORK-Patches werden mit genehmigt. Die Freigabe MUSS in versions.yaml als
Approval-Eintrag nachgewiesen werden (REQ-SD-009).

## §7 Änderungen nach der Freigabe

- **PATCH:** redaktionelle Korrekturen ohne semantische Änderung; via SCR,
  versions.yaml-Eintrag, Validator.
- **MINOR:** neue Anforderungen (REQ-IDs) oder Klarstellungen ohne Breaking
  Change; via SCR + Validator; Review-Zeitpunkt aktualisieren (REQ-SD-010).
- **MAJOR:** inhaltliche Brüche oder Neuordnung; via SCR + UPDATE-001 (MAJOR-
  Pfad) + COMPAT-001-Pflicht-Gate (UPD-G04) + Owner-Freigabe (REQ-SD-011).
- Änderungen an §30-eingefrorenen Standards sind NUR als neue Version mit
  gekennzeichnetem Changelog zulässig — niemals stille Nachträglichkeit.

## §8 Review-Zyklus

Jeder APPROVED-Standard MUSS ein `review_date` führen (Initial: +365 Tage).
Zum Review-Zyklus MUSS geprüft werden: weiterhin aktuell? Widersprüche zu
jüngeren Standards? Aufnahme neuer REQs? Ergebnis: Verlängerung (MINOR) oder
Revision — ein Standard ohne Review-Ergebnis nach Ablauf +90 Tage ist
Governance-Finding (BUG-005, F-NNN, P3) (REQ-SD-012).

## §9 Deprecation und Retirement

DEPRECATED: Standard wird als abgelöst markiert (Nachfolger MUSS benannt sein);
Bestand bleibt normativ bis zur Retirement-Freigabe. RETIRED: archiviert,
§30-lesbar, nicht bindend; Einträge in Registry/Manifest mit Kennzeichnung.
Beides nur via SCR + Owner-Freigabe (REQ-SD-013).

## §10 KI-Agenten als Autoren

Agenten-Sessions zu Standard-Entwicklungen fallen unter AOS-001 (Session-Record
Pflicht). Der GAP-Prozess von TAXONOMY-001 ist Pflicht, wenn die Idee eine neue
Familie/Kategorie berührt. Agenten MÜSSEN jeden Entwurf als DRAFT kennzeichnen
und die §9-Entscheidung ausdrücklich dem Owner vorbehalten (REQ-SD-014).

## Requirements (normativ)

- **REQ-SD-001** (§1): Verbindlicher Lebenszyklus für alle Standards.
- **REQ-SD-002** (§2): §9-Freigabe ausschließlich durch den Owner.
- **REQ-SD-003** (§2): Autor-Agenten entwerfen, freigeben dürfen sie nicht.
- **REQ-SD-004** (§3): Lifecycle vorwärts; Rollback nur via SCR + Owner.
- **REQ-SD-005** (§3): RETIRED = §30-lesbar, nicht normativ bindend.
- **REQ-SD-006** (§4): Hausformat-Pflicht (Frontmatter, RFC-2119, §-Struktur).
- **REQ-SD-007** (§4): ALL COMPLIANT ist EFFECTIVE-Voraussetzung.
- **REQ-SD-008** (§5): 10-Schritte-Prozess, Reihenfolge zwingend.
- **REQ-SD-009** (§6): Freigabe-Dokumentation + versions.yaml Approval-Eintrag.
- **REQ-SD-010** (§7): PATCH/MINOR via SCR + Validator; Review-Datum pflegen.
- **REQ-SD-011** (§7): MAJOR nur via UPDATE-001 + COMPAT-001 + Owner.
- **REQ-SD-012** (§8): review_date +365 Tage; Überfälligkeit ist Finding.
- **REQ-SD-013** (§9): Deprecation/Retirement mit Nachfolger-Nennung + Owner.
- **REQ-SD-014** (§10): Agenten-Entwürfe immer DRAFT; GAP-Prozess bei
  Familien-/Kategorie-Bezug; Session-Records nach AOS-001.

## Security Considerations

Die Standard-Erstellung ist Meta-Angriffsfläche: Wer Standards schreibt,
prägt das Ökosystem. Schutz: §9-Human-Gate als einzige Freigabe-Instanz,
Validator-Suite als objektive Strukturprüfung, §30 gegen stille Nachträglichkeit,
SCR-Vollständigkeit (kein Standard ohne Antrag), Session-Records (AOS-001) als
Autoren-Nachweis. Manipulation der versions.yaml Approval-Einträge wäre ein
Angriff auf die Freigabe-Historie — Git-Historie + Validator decken es auf.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.0.0** (2026-09-08): Initial Release — Kodifizierung der etablierten
  Erstellungspraxis (SCR-0001..0025) als normativer Prozess: 10-Schritte-
  Erstellungsprozess, Rollenmodell mit §9-Human-Gate, Lifecycle IDEA→RETIRED,
  Hausformat-Pflicht, PATCH/MINOR/MAJOR-Wartung mit COMPAT-Kopplung, 365-Tage-
  Review-Zyklus mit Finding-Regel, Deprecation/Retirement, KI-Autoren-Regeln
  (AOS-001/TAXONOMY-001). 14 REQ-SD. SCR-0025; §9-Freigabe Michael Wroblewski 08.09.2026, 00:36 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- ATC-STD-000 (§9 Freigabe, §30 Einfrierung), ATC-STD-DESC-001 (Standard-Lifecycle)
- ATC-STD-TAXONOMY-001 (§3/§12 GAP-Prozess), ATC-STD-REGISTRY-001 (SSOT)
- ATC-STD-CHANGE-001 (Dachnorm), ATC-STD-VERSION-001, ATC-STD-UPDATE-001
- ATC-STD-COMPAT-001 (MAJOR-Gate), ATC-STD-AI-DECISION-001, ATC-STD-AOS-001
- Praxis-Nachweis: SCR-0016..0025 (Freigabe-Kette mit versions.yaml-Approval)

*ATC-STD-STDDEV-001 v1.0.0 · Owner-Entwurf Michael Wroblewski · Aurora (Superagent) · 08.09.2026 · SCR-0025*
