---
approval:
  id: APPROVAL-DECISION-2026-09-11
  standard: ATC-STD-000 (§9 Lifecycle-Freigabe)
  type: Sammelfreigabe
  owner: Michael Wroblewski
  date: '2026-09-11'
  trigger: "Owner-Chat 'Freigabe' 11.09.2026 (nach Vorlage der wartenden CANDIDATE-Standards)"
  scr: SCR-0092
  agent: ATC-AI-ARCH-001
----

# §9-Sammelfreigabe — 38 CANDIDATE-Standards (11.09.2026)

Owner-Freigabe per Chat-Bestaetigung ("Freigabe") auf die Vorlage der
wartenden CANDIDATE-Standards (For-you-Note 10.09.: 36 + seither 2 neue:
ATC-STD-TUD-001, ATC-STD-CI-001 war enthalten). Wirkung: CANDIDATE →
APPROVED gemaess ATC-STD-000 §9; die Standards sind damit normativ
verbindlich (Registry + Repository als SSOT).

## Freigegebene Standards (38)

### Verfassung / Repository-Governance
- ATC-STD-202 v1.2.0 — Repository Naming & Classification (26→27 Repos,
  SCR-0005/AD-046-Rollen)
- ATC-STD-REPO-MAINT-001 v1.0.0 — Repository Maintenance & Lifecycle (FAM-46)
- ATC-STD-IMPLEMENTATION-001 v1.0.0 — Standard-Implementierungs-Matrix
- ATC-STD-CI-001 v1.0.0 — CI-Dependency-Governance (SCR-0054/ATC-ERR-0002)

### Error-Knowledge-Familie (ERR-000..015, 16 Standards)
- ATC-STD-ERR-000..015 v1.0.0 — Error Master + 15 Muster-Standards
  (ATC-ERR-0001-Kette, kein lokaler Fix ohne Systemverifikation)

### Repository-Discovery-Familie (001..010)
- ATC-STD-REPO-DISCOVERY-001..010 v1.0.0 — 10 Discovery-Standards

### Legacy-Nummernkreis (5)
- ATC-STD-114, 212, 213, 214, 215, 314 v1.0.0 — Registry-Luecken-Backfill
  (S-24-Taxonomie-konform)

### Familien-Dach
- ATC-STD-V2S-000 v1.0.0 — Vision-to-Software Lifecycle Master (FAM-45;
  Phasen 001..026 folgen getrennt)

### Technology Uniqueness (heute, SCR-0091)
- ATC-STD-TUD-001 v1.0.0 — Technology Uniqueness & Differentiation
  (FAM-51; inkl. der heutigen Prior-Art-Klassifikationen: 2x NOVEL,
  15x DIFFERENTIATED, 0x PENDING)

## Umsetzung

- registry/standards.yaml: status candidate → approved (38 Eintraege)
- Frontmatter der 38 Standard-Dateien: status → approved
- registry/versions.yaml: je 1 Approval-Eintrag (X.Y.Z-Approval, 11.09.2026)
- Registry-Fix im Zuge der Freigabe: ATC-STD-TUD-001 war versehentlich
  unter legacy_series: einsortiert und ist jetzt korrekt unter standards:
  (Regex-Validatoren sahen ihn, YAML-Parse nicht — consolidate Lesson:
  Append immer vor dem ersten Nicht-standards-Key, Einrueckung 2 Spaces)

## Ausstehend (bewusst NICHT Teil dieser Freigabe)

- 12 DRAFT-Standards (AI-GOV-Familie SCR-0062 etc.) — DRAFT, nicht CANDIDATE
- V2S-001..026 — noch nicht gebaut (SCR-0042-DoD)
- GPG-Signierung — wartet auf Release-Key (AUD-001 F-006-Familie)

scr: SCR-0092 · agent: ATC-AI-ARCH-001
