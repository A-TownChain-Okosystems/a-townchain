---
standard:
  id: ATC-STD-016
  title: "Repository Artifact & File Inventory Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-10"
  updated: "2026-09-10"
  normative: true
  effective_date: "2026-09-10"
  review_date: "2027-09-10"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards:
    - ATC-STD-017
    - ATC-STD-201
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "atc-standards (Phase 1) und alle ATC-Repositories (Phase 2)"
----

# ATC-STD-016 — Repository Artifact & File Inventory (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, §30-eingefroren) — Owner-Direktive 10.09.2026, ~15:05 UTC+2
> (Deep-Dive-Nachgang): verbindlicher Inventur-Standard als Partner von ATC-STD-017.
> ATC-STD-016 beantwortet »Welche Dateien existieren?« — ATC-STD-017 beantwortet
> »Welche davon sind noch gültig und was passiert mit den anderen?«
> Änderungen ausschließlich via ATC-STD-UPDATE-001 (SCR/MINOR/MAJOR).

## Abstract

ATC-STD-016 verlangt für das atc-standards-Repository (Phase 1) und perspektivisch alle
ATC-Repositories (Phase 2) ein maschinenlesbares, GENERIERTES Artefakt-Inventar als
Single Source of Truth. Das Inventar ist die Datengrundlage für die Klassifizierung
und Lifecycle-Entscheidungen nach ATC-STD-017 und für Repository-Audits (REPO-AUDIT-001).

## Scope

**Gilt:** Der Gegenstandsbereich dieses Standards im ATC Enterprise Standards Framework
(FAM-01 Enterprise & Governance, Slot ATC-STD-016) — verbindlich für das Repository atc-standards
(Phase 1) und alle ATC-Repositories (Phase 2 gemäß §Roadmap im SCR-0087).

**Gilt nicht:** Automatisierte Löschung ohne Audit-Verfahren; inhaltliche Fachnormen
anderer Familien; externe Archivsysteme außerhalb der Git-Historie.

## §1 Zweck

Verbindliche, reproduzierbare Bestandsaufnahme aller relevanten Dateien eines Repositories —
Voraussetzung für Traceability, Lifecycle-Steuerung und Löschschutz (ATC-STD-017 §7).

## §2 Geltungsbereich

Alle Dateiklassen gemäß ATC-STD-017 §2 (Dokumentation, Standards, Schemas, Konfigurationen,
Quellcode, Tests, Scripts, CI/CD, Assets, Beispiele, Templates, Reports, generierte
Artefakte, historische Drafts, Root-Level-Dateien).

## §3 Inventar-SSOT (REQ-STD-001)

MUSS: Artefakt-Inventar als `registry/artifacts.yaml` (atc-standards) bzw. `.atc/artifacts.yaml`
(Produkt-Repositories, Phase 2). Das Inventar MUSS generatorisch erzeugt werden
(REQ-STD-003) — manuell gepflegte Bestände sind verboten (Drift-Gefahr).

## §4 Manifest-Schema (REQ-STD-002)

JEDER Inventar-Eintrag MUSS das Schema erfüllen:

```yaml
artifact:
  path: standards/example.md          # MUSS: Repositorie-relativer Pfad
  status: ACTIVE                      # MUSS: Klasse gemäß ATC-STD-017 §3
  owner: standards                    # MUSS: Owner (Rolle/Repo/Standard)
  standard_id: ATC-STD-XXX            # SOLLTE: normative Zuordnung
  successor: null                     # SOLLTE: Nachfolgeartefakt (bei DEPRECATED/OBSOLETE)
  references: [README.md, registry.yaml]  # KANN: bekannte Referenzen
  last_reviewed: 2026-09-10            # MUSS: Datum des letzten Klassifizierungs-Reviews
```

## §5 Generierungspflicht (REQ-STD-003)

MUSS: Das Inventar MUSS von einem Tool (Phase 1: `tools/artifacts/gen_artifacts.py`)
aus dem Dateisystem + Referenzanalyse generiert werden. Regeneration nach jeder
strukturverändernden Änderung (SCR) sowie im CI-Gate (ATC-STD-017 §8).

## §6 Klassifizierungskopplung (REQ-STD-004)

MUSS: Jedes Inventar-Artefakt trägt eine Klasse gemäß ATC-STD-017 §3 (ACTIVE, LEGACY,
DEPRECATED, ORPHANED, OBSOLETE, DUPLICATE, GENERATED, ARCHIVED, UNKNOWN). Der Generator
setzt UNKNOWN als ehrlichen Default; manuelle Verfeinerung über Klassifizierungs-Review.

## §7 Aktualisierung & Drift (REQ-STD-005)

MUSS: CI vergleicht Inventar-Regeneration gegen den eingecheckten Stand (Drift-Check,
analog Views-Drift-Gate). Drift ohne SCR = WARN (Phase 1) / FAIL (Phase 2, nach
Owner-Freigabe gemäß ATC-STD-017 §8 Stufe 2).

## §8 Compliance & Verifikation

Nachweise: Generator-Lauf (CI-Log), Inventar-Datei (SSOT), Klassifizierungs-Report
(ATC-STD-017 §8 WARN-Liste). Verifikation via Repository-Audit (REPO-AUDIT-001) und
Governance-CI.

## §9 Freigabe

FREIGEGEBEN am 10.09.2026, ~15:05 UTC+2, durch Owner-Direktive (Alexander Wroblewski)
im Rahmen der ATC-STD-017-Etablierung (Governance-Kette ATC-STD-000 → 016 → 017).
Priorität: P1.

## §30 Freeze & Change-Control

Dieser Standard ist §30-eingefroren. Änderungen ausschließlich via ATC-STD-UPDATE-001:
PATCH (redaktionell), MINOR (Erweiterung, z. B. Schema-Felder), MAJOR (Breaking,
COMPAT-001-Gate erforderlich).

## Security Considerations

Das Inventar ist reine Metadaten-Sicht; es enthält keine Inhalte von Dateien. Löschschutz
und Audit-Records obliegen ATC-STD-017 — das Inventar liefert nur die belastbare Basis
(»Nicht referenziert« ist ein Audit-Signal, kein Löschkriterium).
