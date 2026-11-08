---
standard:
  id: ATC-STD-019
  title: "Dependency & Supply Chain Security Standard"
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
    - ATC-STD-016
    - ATC-STD-017
    - ATC-STD-019
    - ATC-STD-020
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-019 — Dependency & Supply Chain Security (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, §30-eingefroren) — Owner-Direktive 10.09.2026; bewusst getrennt
> von ATC-STD-018 (018 = Technologie-Aktualität + Security Assurance; 019 = Supply-Chain-
> Integrität), damit 018 kein überladener Alles-Security-Standard wird.

## Abstract

ATC-STD-019 definiert verbindliche Software-Supply-Chain-Sicherheit für alle ATC-Repositories:
Dependencies und Lockfiles, SBOM-Pflicht, Dependency Pinning mit Integritätsprüfung,
Artefakt-Signierung und Provenance, Reproducible Builds, Registry-Sicherheit gegen Dependency
Confusion/Typosquatting und Build-Sicherheit. Bewusst getrennt von ATC-STD-018, damit
Technology Currency und Supply Chain je prüfbar bleiben; beide münden im Supply-Chain-Gate.

## Scope

**Gilt:** Software-Supply-Chain aller 27 Repositories: Dependencies, SBOM, Provenance, Package-
Integrität, Lockfiles, Reproducible Builds, Dependency Pinning, Artefakt-Signierung, Registry- und
Build-Sicherheit.

**Gilt nicht:** Vulnerability-SLAs und Technologie-Aktualität (ATC-STD-018); Incident-Abläufe
(ATC-STD-020).

## §1 Dependencies & Lockfiles (REQ-STD-001, MUSS)
Vollständige Lockfiles je Manifest; Abweichung Lock↔Manifest = CI-FAIL; kein Floating von
Versionsranges in Release-Zweigen.

## §2 SBOM (REQ-STD-002, MUSS)
Jedes Release-Artefakt mit maschinenlesbarer SBOM (z. B. CycloneDX/SPDX); SBOM-Regeneration je
Dependency-Änderung; Ablage als Evidence.

## §3 Dependency Pinning & Integrity (REQ-STD-003, MUSS)
Pinning mit Integritätsprüfung (Checksums/Signaturen); npm: --ignore-scripts für CI-Installs wo
möglich; Cargo: --locked; pip: Hash-Pinning für kritische Pipelines.

## §4 Artifact Signing & Provenance (REQ-STD-004, MUSS)
Release-Artefakte signiert; Provenance-Dokumentation (Build-Kontext, Quell-SHA); keine
unkomprimierten Downloads aus Drittquellen ohne Hashabgleich.

## §5 Reproducible Builds (REQ-STD-005, MUSS wo anwendbar)
Deterministische Builds mit dokumentiertem Reproduktionsschritt (Referenz: kanonische
Serialisierungs-/Determinismus-Konvention der ATC-Protokolle).

## §6 Registry Security (REQ-STD-006, MUSS)
Nur verifizierte Registries; Internal-Namespace-Schutz gegen Dependency Confusion; Typosquatting-
Screening bei neuen Dependencies; scopename-Gate für @atc-Namespaces.

## §7 Build Security (REQ-STD-007, MUSS)
CI mit least-privilege Permissions (Referenz: Org-Permissions-Bundle SCR-0060), kein Secret-
ECHO, heredoc-Pinning kritischer Actions (Schutz gegen Tag-Mutation); Workflow-Push nur mit
workflow-Scope (GH013-Regel).

## §8 Supply-Chain-Gate (REQ-STD-008, MUSS)
Kombiniert mit ATC-STD-018 §6:Supply-Chain-Verstöße (fehlende SBOM, unpinned Dependency,
unsigned Artefakt) blockieren Merge/Release.

## §9 Freigabe
FREIGEGEBEN 10.09.2026 via Owner-Direktive. Priorität P1.

## §30 Freeze & Change-Control
§30-eingefroren; Änderungen ausschließlich via ATC-STD-UPDATE-001.

## Security Considerations
Supply-Chain ist Angriffsvektor Nr. 1 — deshalb eigener Standard statt Anhang. Verstöße sind
P0-taugliche Findings gemäß ATC-STD-REPO-AUDIT-002.
