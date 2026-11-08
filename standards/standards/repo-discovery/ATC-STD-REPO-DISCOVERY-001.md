---
standard:
  id: ATC-STD-REPO-DISCOVERY-001
  title: "Content Discovery"
  version: "1.0.0"
  status: approved
  category: repo-discovery
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Repositories; Agenten, CI, Audits, Wartungszyklen"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-REPO-DISCOVERY-001 — Content Discovery (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Neue oder veraenderte Repository-Inhalte automatisch erkennen, klassifizieren,
bewerten und — falls erforderlich — in das ATC-Standardsystem zurueckfuehren.

Kontinuierlicher Kreislauf:
Repository → Discovery → Klassifizierung → Audit → Standard-Kandidat → Review →
Standard → Implementierung → erneuter Discovery-Scan.

## Erkennungsbereiche (Minimum)
Dateien (.rs/.ts/.py/.md/.yaml), Verzeichnisse (Module/Services/Packages), Code
(Funktionen/Klassen/APIs), Standards, Dokumentation (README/ADR/SPEC/GUIDE),
Konfiguration (Docker/CI/Terraform/K8s), Abhaengigkeiten (Libraries/Images),
Schnittstellen (REST/RPC/GraphQL/Events), Smart Contracts, Tests, Security
(Permissions/Auth/Policies), KI-Agenten (Tools/Prompts), Daten (Schemas/
Migrationen), Assets, Lizenzierung.

## Discovery-Signale (A-D)
A. Dateiendung (Typ) · B. Dateiname (README/ARCHITECTURE/SPEC/STANDARD/POLICY/
SECURITY/ADR-*) · C. Verzeichnis (docs/, standards/, contracts/, agents/, .github/,
terraform/, k8s/) · D. Inhalt — die Klassifizierung darf NICHT ausschliesslich
auf Dateinamen basieren (docs/foo.md kann ein Standard sein).

## Discovery-Zyklus (11 Stationen)
Bestand erfassen → Git-Aenderungen → neue Inhalte → klassifizieren →
Abhaengigkeiten → Standardsrelevanz → Risiko → Dokumentationsbedarf →
Cross-Repository-Auswirkung → Massnahmen → Discovery-Report.

## Baseline-System
Je Repository ein Content-Baseline-Snapshot (.atc/discovery/baseline.json:
Commit, Scan-ID, Zaehler fuer Files/Directories/Standards/Tests/Dependencies/
Workflows/Agents/Contracts). Naechster Scan vergleicht BASELINE → CURRENT →
DIFF → NEW CONTENT → CLASSIFICATION.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-010 | Je Repository Discovery-Baseline revisionssicher gefuehrt |
| REQ-RD-011 | Erkennung mindestens ueber die 15 Bereiche/4 Signale |
| REQ-RD-012 | Inhaltssignal (D) verpflichtend — Dateiname allein unzureichend |
| REQ-RD-013 | Discovery-Zyklus 11 Stationen je Scan durchlaufen |
| REQ-RD-014 | Scan-Ergebnis maschinenlesbar (RD-010-Format) |
| REQ-RD-015 | Discovery-Scan je relevantem Merge (RD-002-Kette) |
| REQ-RD-016 | Discovery laeuft im Wartungszyklus (REPO-MAINT-001 §16) |

## Implementierungsstatus
IMPLEMENTED (erster Zyklus) — Evidence: tools/discovery/scan.py (Baseline +
Diff + Klassifizierung), erster Scan auf atc-standards in .atc/discovery/.

## Security Considerations

Discovery-Baselines sind Inhaltsverzeichnisse ohne Dateiinhalte; keine Zugangsdaten erfassbar. Scans mit read-only Git-Zugriff.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: ATC-STD-201..203, REPO-AUDIT-002, REPO-MAINT-001, ERR-000 ·
INFORMATIVE: SCR-0046, .atc/discovery/
