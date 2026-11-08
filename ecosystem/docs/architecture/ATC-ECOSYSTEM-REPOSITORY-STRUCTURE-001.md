# ATC-ECOSYSTEM-REPOSITORY-STRUCTURE-001

- **Dokument-ID:** `ATC-ECOSYSTEM-REPOSITORY-STRUCTURE-001`
- **Version:** 1.0.0
- **Status:** `PROPOSED / NOT VERIFIED`
- **Geltungsbereich:** `A-TownChain-Okosystems/a-townchain-ecosystem`
- **Architekturprinzip:** Standalone First, Ecosystem Second

## 1. Zweck und normative Grenzen

Dieses Dokument beschreibt die Zielstruktur des Ecosystem-Control-Plane-Repositories. Es bündelt Architekturentscheidungen, Standardsreferenzen, SSOT-Zuordnungen, Katalogverträge, Integrationsregeln, Compliance-Mappings und Verifikationsnachweise.

Es ist **kein** Nachweis, dass alle unten aufgeführten Verzeichnisse oder Validatoren bereits existieren. Vor Änderungen muss der aktuelle Repository-Baum inventarisiert und gegen bestehende Dateien, Workflows, Issues und PRs abgeglichen werden. Bestehende autoritative Dateien sind zu erweitern oder wiederzuverwenden; keine parallele Registry, kein zweiter Katalog und kein zweites Manifest anlegen.

Fachliche Produktionsimplementierungen bleiben in den kanonischen Repositories. Importierte Subtrees und Integrations-Snapshots verschieben keine SSOT-Verantwortung.

## 2. Abgleich mit dem kanonischen Layer-Modell

Die vorhandene Master-Architektur `ARCHITECTURE.md` und Issue #71 definieren das Ökosystem-Modell **L0–L7 plus X**. Dieses Dokument führt kein konkurrierendes L1–L6-Modell ein.

- **L0–L7:** Architektur- und Verantwortungsdomänen.
- **X:** Cross-Layer Control Plane; kein zusätzlicher Ausführungslayer.
- **Repository-Verzeichnisstruktur:** organisatorische Ablage und nicht mit Architektur-Layern gleichzusetzen.
- **Statusdimensionen:** Architektur, Implementierung, Tests, CI-Evidence und E2E-Evidence getrennt halten.

Security, Privacy, Observability, Governance und Evidence sind querschnittlich zu modellieren, soweit sie mehrere Layer betreffen.

## 3. Zielstruktur

Die Struktur ist ein Zielbild. Jeder neue Pfad benötigt eine klar benannte Zuständigkeit und darf bestehende SSOT-Artefakte nicht duplizieren.

```text
a-townchain-ecosystem/
├── README.md
├── ARCHITECTURE.md
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── LICENSE
├── CHANGELOG.md
├── .github/
│   ├── CODEOWNERS
│   ├── pull_request_template.md
│   ├── ISSUE_TEMPLATE/
│   └── workflows/
│       ├── validate-catalog.yml
│       ├── validate-manifest.yml
│       ├── validate-architecture.yml
│       ├── validate-dependencies.yml
│       ├── validate-security.yml
│       ├── validate-evidence.yml
│       └── validate-release.yml
├── governance/
│   ├── policies/
│   ├── roles-and-responsibilities/
│   ├── change-management/
│   ├── release-policy/
│   ├── merge-policy/
│   └── exceptions/
├── architecture/
│   ├── overview/
│   ├── system-context/
│   ├── layers/
│   ├── component-boundaries/
│   ├── deployment/
│   ├── security-architecture/
│   └── decisions/
├── standards/
│   ├── registry/
│   ├── protocols/
│   ├── cryptography/
│   ├── serialization/
│   ├── interfaces/
│   ├── compatibility/
│   └── schemas/
├── catalog/
│   ├── README.md
│   ├── schemas/
│   ├── components/
│   ├── functions/
│   ├── layers/
│   ├── metadata/
│   └── exclusions/
├── registry/
│   ├── repository-manifest/   # nur wenn dies kein Duplikat des bestehenden Manifests ist
│   ├── repository-owners/
│   ├── component-ssot/
│   ├── dependency-lock/
│   └── compatibility-matrix/
├── integration/
│   ├── contracts/
│   ├── dependency-graph/
│   ├── build-graphs/
│   ├── adapters/
│   ├── compatibility-tests/
│   └── end-to-end/
├── compliance/
│   ├── privacy/
│   ├── security/
│   ├── licensing/
│   ├── supply-chain/
│   └── control-mappings/
├── evidence/
│   ├── schemas/
│   ├── runs/
│   ├── artifacts/
│   ├── attestations/
│   └── reports/
├── validation/
│   ├── catalog/
│   ├── manifest/
│   ├── architecture/
│   ├── dependencies/
│   ├── security/
│   └── evidence/
├── scripts/
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── contract/
│   ├── regression/
│   └── fixtures/
├── docs/
│   ├── getting-started/
│   ├── architecture-guides/
│   ├── operations/
│   ├── troubleshooting/
│   └── glossary.md
└── reports/
    ├── catalog/
    ├── drift/
    ├── compatibility/
    └── compliance/
```

### 3.1 Strukturregeln

1. Die Liste ist kein Auftrag, sämtliche Verzeichnisse blind anzulegen. Leere Platzhalter ohne Vertrag sind nicht erforderlich.
2. Vorhandene Pfade und Dateien haben Vorrang, bis eine kontrollierte Migration beschlossen wurde.
3. Generierte Reports sind abgeleitete Artefakte; sie dürfen keine zweite autoritative Datenquelle werden.
4. Die Ablage von Workflow-Logs und Build-Artefakten muss Retention, Zugriffsschutz und Größe berücksichtigen. Dauerhafte Evidence referenziert unveränderliche Artefakte und Hashes; große kurzlebige Dateien müssen nicht ins Git-Repository eingecheckt werden.
5. Normative Spezifikationen verbleiben im zuständigen Standards-Repository, sofern die bestehende Governance nichts anderes festlegt. Das Ecosystem-Repository registriert und referenziert sie.

## 4. Mindestverträge

| Bereich | Mindestinhalt |
|---|---|
| Repository-Manifest | Repository-ID, vollständiger Name, Status/Scope, Default-Branch, beobachteter vollständiger Commit-SHA, Root-Tree-SHA, Beobachtungszeitpunkt und Evidenz |
| SSOT-Registry | stabile Komponenten-ID, kanonisches Repository und Pfad, Zuständigkeit, Geltungsbereich, Status und dokumentierte Ausnahmen |
| Komponentenvertrag | ID, Name, Layer/Sublayer, Owner, Schnittstellen, Abhängigkeiten, Standardsreferenzen, Version und Status |
| Funktionsvertrag | stabile Funktions-ID, Verhalten, Inputs/Outputs, Fehlersemantik, Invarianten, Limits, Source-Revision/Pfad/Symbol, Tests und Evidenz |
| Integrationsvertrag | Provider/Consumer, Protokoll/ABI-Version, Kompatibilitätsregeln, Fehler- und Sicherheitsverhalten sowie Testnachweise |
| Evidence-Vertrag | Scope, Commit-SHA, Workflow-Run, Job, Step, Exit-Code, Logs, Artefakt-Hashes, Ergebnis, Prüfer und bekannte Einschränkungen |
| Architekturentscheidung | Kontext, Entscheidung, Alternativen, Konsequenzen, Status, Verantwortlichkeit und referenzierte Nachweise |

Das vorhandene maschinenlesbare Komponenten-/Funktionsschema ist zuerst zu prüfen und zu erweitern, falls erforderlich. Kein neues Schema wird eingeführt, solange das vorhandene Schema denselben Vertrag abdecken kann.

## 5. SSOT-Invarianten

- Blockchain-Core und Protokollimplementierung: kanonisches Repository `a-townchain`.
- ATC-VM: `a-townchain/components/vm`.
- Algorithmus/Konsens: `a-townchain/components/algorithm`.
- ShivaCore-Kernel: `globus-os/modules/atc-shivacore/kernel/`.
- GlobusOS-System und Desktop: `globus-os`.
- Normative Standards: `atc-standards`.
- Ecosystem-Katalog, Cross-Repository-Integration, Governance und aggregierte Evidence: `a-townchain-ecosystem`.
- Aurora AI und ATCLang: konkrete kanonische Repository-/Pfadzuordnungen müssen im aktuellen Katalog und anhand der Quellen bestätigt werden; unbekannte Pfade bleiben `UNRESOLVED`.

Diese Liste beschreibt bekannte Architekturzuordnungen und ersetzt keine aktuelle SHA-gebundene Quellinventur.

## 6. CI/CD-Validierungsvertrag

Validatoren werden erst als verpflichtende Gates aktiviert, wenn sie implementiert, positiv und negativ getestet sowie reproduzierbar nachgewiesen sind.

1. **Schema:** ungültige JSON-Dateien, fehlende Pflichtfelder, doppelte IDs und ungültige Referenzen erkennen.
2. **Manifest:** Repository-Scope und vollständige Commit-/Root-Tree-SHAs prüfen; veraltete oder fehlende Daten nicht als PASS werten.
3. **SSOT:** konkurrierende Produktionsquellen und ungültige kanonische Pfade erkennen.
4. **Katalog:** fehlende Komponenten/Funktionen und unaufgelöste Beziehungen sichtbar machen; keine Statushochstufung allein durch Katalogaufnahme.
5. **Dependencies:** fehlende Knoten, ungültige Kanten, Zyklen und inkompatible Versionen gemäß dokumentierter Regel prüfen.
6. **Contract/E2E:** tatsächliche Schnittstellen gegen versionierte Verträge testen.
7. **Evidence:** Run → Job → Step → Exit-Code/Logs sowie Artefakt-Hashes und exakte Quellrevision miteinander verknüpfen.
8. **Merge Gate:** kritische, ungelöste Pflichtabweichungen blockieren. Keine Abschwächung bestehender Branch-Protection-Regeln als Teil dieser Strukturänderung.

`demo-repository` bleibt gemäß bestehender Governance aus dem Live-Drift-Gate ausgeschlossen. Ausschlüsse müssen explizit und testbar sein.

## 7. Statussemantik

| Status | Bedeutung |
|---|---|
| `PROPOSED` | Entwurf; noch nicht als bestehender Zustand bestätigt |
| `MAPPED` | nachvollziehbar einem Katalogeintrag/einer Quelle zugeordnet |
| `IMPLEMENTED` | Implementierung nachgewiesen; Verifikation nicht automatisch abgeschlossen |
| `VERIFIED` | definierte Prüfungen für die exakte Revision erfolgreich und mit Evidence belegt |
| `RESIDUAL` | bekannte Restabweichung oder akzeptiertes Restrisiko |
| `BLOCKED` | Prüfung oder Freigabe durch ungelöste Voraussetzung verhindert |
| `UNRESOLVED` | Zuordnung oder Sachverhalt nicht belastbar geklärt |
| `EXCLUDED` | mit Policy/Begründung explizit aus dem Geltungsbereich ausgeschlossen |

Die kanonischen Statuswerte des bestehenden Schemas sind maßgeblich. Falls dort Werte fehlen, muss das Schema kontrolliert und kompatibel aktualisiert werden; keine zweite Statusdefinition neben dem Schema einführen.

## 8. Umsetzungsreihenfolge

1. **Baseline:** aktuellen Tree, vorhandene Dateien, aktive PRs/Issues, Manifest und vorhandene Workflows erfassen.
2. **Reconciliation:** Zielstruktur gegen `ARCHITECTURE.md`, Issue #71, bestehende Katalogdatei und die M0–M8-Planung in Issue #185 abgleichen.
3. **Gap-Liste:** bestehende, fehlende, doppelte und widersprüchliche Artefakte mit Pfad und Entscheidung dokumentieren.
4. **Schemas zuerst:** vorhandene Schemas und Validatoren prüfen; Änderungen klein und rückwärtskompatibel halten.
5. **Validatoren:** nur belegte, deterministische Prüfungen ergänzen und Negativtests hinzufügen.
6. **Evidence:** vollständige Commit-/Tree-SHAs, Run/Job/Step, Exit-Codes, Logs und Artefakt-Hashes erfassen.
7. **Gates:** nach erfolgreicher Reproduktion und Review schrittweise verbindlich aktivieren.
8. **Freigabe:** Draft bis zur Erfüllung der Akzeptanzkriterien; kein automatischer Merge.

## 9. Akzeptanzkriterien

- [ ] Zielstruktur gegen den aktuellen Repository-Baum und bestehende Issues/PRs abgeglichen.
- [ ] Kanonisches Layer-Modell L0–L7 + X beibehalten; kein widersprüchliches alternatives Modell eingeführt.
- [ ] Genau eine autoritative Katalog-/Schema-/Manifestquelle verwendet.
- [ ] SSOT-Zuordnungen für bekannte Komponenten klar, unbekannte Zuordnungen explizit `UNRESOLVED`.
- [ ] Keine produktiven Implementierungsduplikate im Ecosystem-Repository.
- [ ] Validatoren mit Positiv- und Negativtests sowie dokumentierten Eingaben/Ausgaben belegt.
- [ ] Fehlende oder veraltete Pflicht-Evidence führt zu `BLOCKED`, nicht PASS.
- [ ] `demo-repository` bleibt aus dem Live-Drift-Gate ausgeschlossen.
- [ ] Änderungen sind reviewbar, auf eine vollständige Commit-SHA gebunden und nicht automatisch gemergt.

## 10. Aktueller Status

`PROPOSED / NOT VERIFIED`. Dieses Dokument ist ein Zielbild. Es belegt weder die Existenz aller aufgelisteten Verzeichnisse noch die Implementierung oder Verifikation der beschriebenen Validatoren. Die Bestätigung erfordert eine vollständige Baseline- und Evidence-Prüfung.
