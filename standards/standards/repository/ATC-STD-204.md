---
standard:
  id: ATC-STD-204
  title: "ATC-STD-204 — Dependency & Interface Standard"
  version: "1.0.0"
  status: approved
  category: repository
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-204 — Dependency & Interface Standard (v1.0.0, APPROVED)
> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026 (ATC-STD-000 §9); normativ in Kraft, Übergangsfrist für Nachregistrierung: 30 Tage (§9) | **Datum:** 07.09.2026 | **Autor:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-204 | **Scope:** Alle 26 Repositories der Organisation
> **Referenzen:** AD-026 (Dependency Graphs), AD-030/031 (Registry), ATC-STD-000, ATC-STD-201/-202/-203, Externe Bewertung 07.09.2026 (F-001 Dependency Governance P0, F-002 Interface Governance P0, F-005 Versioning P1)

---

## Abstract

ATC-STD-204 regelt, wie die 26 Repositories der Organisation voneinander
abhängen dürfen und über welche Interfaces sie kommunizieren. Kernprinzip:

> **Kein Repository importiert ein anderes Repository implizit.**
> Jede Abhängigkeit ist deklariert, versioniert und über eine registrierte
> Schnittstelle gekapselt.

Damit verwandelt der Standard die Repository-Sammlung von einem losen Geflecht
in einen maschinenlesbaren Dependency Graph mit versionierten Interfaces
(Findings F-001 und F-002 der externen Bewertung).

## 1. Kanonische Quellen (SSOT-Regel)

| Objekt       | Kanonische Datei                        | Regel                                  |
|--------------|------------------------------------------|----------------------------------------|
| Repository-Dependencies | `registry/dependencies.yaml` | Nur dort deklarierte `uses:`-Kanten existieren |
| Interfaces   | `registry/interfaces.yaml` (NEU)        | Kein Eintrag = kein offizielles Interface |
| Repo-Metadaten | `registry/repositories.yaml`            | ATC-STD-202 §Registration              |
| Standard-Versionen | `registry/versions.yaml`            | ATC-STD-000 §13                        |

Abgeleitete Doku (README, Wiki, Notion) darf diese Registrys zitieren,
aber niemals widersprechen (Registry schlägt Doku, F-005-Regel).

## 2. Dependency-Deklaration (F-001)

### 2.1 Deklarationspflicht

Jede Nutzung eines Repositories durch ein anderes MUSS als Kante in
`registry/dependencies.yaml` deklariert sein:

```yaml
repositories:
  atc-sdk: {uses: [atclang, a-townchain]}
```

### 2.2 Verbot impliziter Abhängigkeiten

- **Verboten:** direkter Import/Copy von Code aus einem anderen Repo ohne
  Deklaration ("Repository A importiert zufällig Repository B").
- **Verboten:** Querschnitts-Imports, die den Layer-Regeln von ATC-STD-202
  widersprechen (z.B. L0-Repo importiert L5-Repo).
- **Erlaubt:** Abhängigkeit nur in Richtung deklarierter Interfaces (§3).

### 2.3 Zyklusfreiheit

Der Graph MUSS azyklisch sein (DAG). `atc-std-validator` prüft bei jedem
Registry-Commit auf Zyklen und lehnt zyklische Kanten ab.

### 2.4 Änderungsprozess

Neue/entfernte Kanten = SCR (ATC-STD-000 §SCR) mit Begründung, Owner-Freigabe
und Impact-Analyse der betroffenen Repos.

## 3. Interface-Registry (F-002)

### 3.1 Interface-Eintrag

Jede deklarierte Abhängigkeit MUSS über mindestens ein registriertes Interface
laufen. Eintrag in `registry/interfaces.yaml`:

```yaml
interfaces:
  - id: IFC-0001
    title: "ATCLang Bytecode Format"
    provider: atclang
    consumers: [atc-vm]
    api_version: "1.0"
    compatibility: stable
    security_level: S4
    owner: ShivaCoreDev
    test: tests/interfaces/ifc-0001/          # verbindliche Test-Suite
    status: active
```

### 3.2 Pflichtfelder

`id`, `provider`, `consumers`, `api_version`, `compatibility`, `security_level`,
`owner`, `test`, `status`. Fehlt ein Feld, gilt das Interface als nicht
registriert.

### 3.3 ID-Allokation

Interface-IDs `IFC-NNNN` werden fortlaufend vergeben und niemals wiederverwendet
(ATC-STD-000 §37 ID-Allokationsprozess).

### 3.4 Verbindlichkeit

Ein Release eines Repos mit nicht-registrierten externen Interfaces ist
nicht freigabefähig (ATC-STD-203 Release-Gate). "Repository existiert" ist
kein Nachweis für ein stabiles Interface (F-007).

## 4. Dreifache Versionierung (F-005)

Jedes Repository trennt MASCHINENLESBAR drei Versionierungsdimensionen:

| Dimension          | Beispiel                  | Ändert                       |
|-------------------|---------------------------|------------------------------|
| Protocol Version   | ATC Protocol 1.0          | Netz-/Konsens-Regeln         |
| Specification Version | ATC-STD-xxx v1.2.0      | Normativer Standard-Text     |
| Implementation Version | atc-node 0.9.4         | Konkreter Repo-Release       |

**Regel:** `Protocol ≠ Specification ≠ Implementation`. Jedes Repo führt
eine `versions.yaml` im Wurzelverzeichnis mit den drei Feldern; Standard-
Versionen laufen zusätzlich über `registry/versions.yaml`. Breaking Changes
in einer Dimension erzeugen NIEMALS automatisch Änderungen in einer anderen.

## 5. Kompatibilitäts-Regeln

- Interfaces folgen **SemVer**: MAJOR = breaking, MINOR = additive, PATCH = fix.
- Breaking Change eines Interfaces erfordert: neue MAJOR-Version, SCR,
  Migrations-Plan für alle `consumers`, Parallelbetrieb ≥1 Minor-Zyklus.
- `compatibility`-Werte: `stable` (Änderungen nur per MAJOR), `evolving`
  (MINOR-Änderungen erlaubt), `experimental` (keine Garantie, nur R0/R1-Repos).

## 6. Integration-Test-Pflicht

Jede `provider`→`consumer`-Kante MUSS einen Integration-Test im
`test`-Pfad des Interface-Eintrags haben. Der Test läuft in der CI beider
Repos (ATC-STD-203 §2) und MUSS im Release-Gate grün sein.

## 7. Conformance-Level (Anbindung an ATC-STD-202)

| Compliance-Level | Anforderung                                    |
|------------------|------------------------------------------------|
| R0–R1            | Dependencies deklariert, Interface-Einträge können fehlen (experimental) |
| R2               | Alle genutzten Interfaces registriert, Integration-Tests vorhanden |
| R3+              | Zyklusfreiheit + SemVer-Konformenz validiert, keine experimental-Interfaces gegen R3-Repos |

## 8. Validator-Erweiterung

`tools/atc-std-validator` erweitert sich um drei Checks:
1. **DEP-001:** Jede `uses:`-Kante verweist auf registriertes Interface des Ziels.
2. **DEP-002:** Zyklusfreiheit des Repository-Graphen.
3. **DEP-003:** `versions.yaml` in jedem R2+-Repo mit drei Dimensionen vorhanden.

## 9. Inkrafttreten

APPROVED per Owner-Freigabe 07.09.2026 (ATC-STD-000 §9). Der Standard ist für alle
R2+-Repositories verbindlich; Übergangsfrist für Nachregistrierung bestehender
Kanten: 30 Tage.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Abhaengigkeits-Governance: Supply-Chain-Schutz — keine Zugangsdaten in Manifesten/Lock-Dateien; CVE-Blocker vor Merge; Lizenz-Review je neuer Dependency; Lockfile-Pinning verpflichtend.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203, ATC-STD-UPDATE-001 · INFORMATIVE: Dependabot-Rollout AUD-2026-0002
