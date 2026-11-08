# ARCHITECTURE — atc-standards

Das Repository ist die KANONISCHE Standards-Heimat des A-TownChain-
Oekosystems (AD-030). Die Architektur folgt ATC-STD-000 (Verfassung) und
dem §28-Soll-Layout des Owners.

```text
atc-standards/
├── governance/          Verfassung + Prozess-Doku (CHANGE_CONTROL, APPROVAL_PROCESS)
├── approval/            Formale Review-Pakete (ATC-STD-000 v1.0.0)
├── change-requests/     SCR-Registry-Dokumente (SCR-0001…)
├── standards/           ATC-Standards je Kategorie (repository: 201/202/203)
├── atc/                 ATC-Standards: ATC-01…99, ATC-0001…0008 (Legacy, vollstaendig)
├── ats/                 ATS-Standards: ATS-1000…1007 (ShivaOS), ATS-LIC
├── licensing/           ATC-LIC, ATVM License Gate, IP-License-Specs, BaFin-Richtlinie
├── registry/            standards.yaml (Herzstueck §25), categories/versions/
│                        lifecycle/dependencies/repositories/teams + STANDARDS_REGISTRY (Legacy)
├── schemas/             standard/requirement/change-request + repository-Schemas
├── templates/           STANDARD/REQUIREMENT/SCR + repository/workflows
├── tools/               atc-std-validator (S-01…S-15), atc-repo-audit (V-01…V-16)
├── docs/                REPOSITORY_STANDARD, governance-Index
└── .atc/                Repo-Metadaten (repository/compliance/lifecycle/ownership)
```

## Governance-Fluss

```text
ATC-STD-000 (Verfassung, §36 Final Model)
    -> Registry (§25: kein Eintrag = kein Standard)
    -> Standards je Kategorie (§26)
    -> Validator (§24, tools/) + CI
    -> Implementierungen -> Compliance -> Production
```

## Tool-Verkettung

- atc-std-validator: prueft jeden Standard gegen die Verfassung; CI laeuft
  bei jedem Push ueber alle vier STD-Standards.
- atc-repo-audit: Repository-Selbst-Compliance (ATC-STD-201ff), R3-Gate.
- Beide stdlib-only, CI-faehig (Exit-Codes).
