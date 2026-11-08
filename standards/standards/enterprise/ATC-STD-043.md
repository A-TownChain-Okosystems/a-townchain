---
standard:
  id: ATC-STD-043
  title: "Artifact Integrity Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories und die Organisation gesamt"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
----

# ATC-STD-043 — Artifact Integrity Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-043 · Integritäts-Metadaten kritischer Artefakte · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-039" — Slots 040/042 belegt
> (Legacy), §37 ergab 043.

## Abstract

ATC-STD-043 definiert die verbindlichen Integritäts-Metadaten jedes
veröffentlichten kritischen Artefakts: Artifact ID, Version, Hash, Build ID,
Source Revision, Build Environment, SBOM, Signature, Timestamp, Provenance.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle veröffentlichten kritischen Artefakte (Node-Binaries, Libraries,
Container Images, Release-Pakete).

## §1 Artefakt-Record (REQ-STD-001, MUST)

Jedes kritische Artefakt MUSS einen maschinenlesbaren Record führen:

```
artifact:
  id: ATC-NODE
  version: 1.4.0
  source_revision: <commit-sha>
  hash: <sha256>
  build_id: <build-run-id>
  build_environment: <toolchain/container>
  sbom: <verweis>
  provenance: <verweis>
  signature: <verweis>
  timestamp: <utc>
```

## §2 Provenance & Verifikation (REQ-STD-002, MUST)

Der Record MUSS die Kette zu ATC-STD-041 (Reproducible Builds) schließen:
Source Revision MUSS auflösbar, der Hash MUSS prüfbar, Provenance MUSS der
Build-Umgebung zuordenbar sein. Unvollständige Records = kein Release
(GATE-Zeile Supply Chain).

## §3 Ablage (REQ-STD-003, MUST)

Artefakt-Records MÜSSEN unter evidence/releases/ versioniert abgelegt und
der Org-Registry zuordenbar sein.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — 9-Feld-Artefakt-Record MUSS je kritischem Artefakt existieren.
- id: REQ-STD-002 — Provenance-Kette MUSS zu ATC-STD-041 geschlossen und prüfbar sein.
- id: REQ-STD-003 — Records MÜSSEN unter evidence/releases/ versioniert sein.

## Compliance

Prüfung: GATE-Zeilen SBOM/Provenance/Integrity je Release; Record-Audit.

## Security Considerations

- Record-Fälschung: Records MÜSSEN selbst signiert/geschützt sein.
- Artefakt-Rückrufe (Revoke) MÜSSEN über die Records nachvollziehbar sein
  (ATC-STD-035 §2).

## Implementierungsstatus

**Status: SPECIFIED** — ab erstem kritischen Release des Rebuilds. SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) —
  Artefakt-Record-Schema, Provenance-Pflicht, Ablage. CANDIDATE.

## References

- ATC-STD-041 — Reproducible Builds · ATC-STD-019 — Supply Chain
- ATC-GATE-SEC-001 — Release-Gate
