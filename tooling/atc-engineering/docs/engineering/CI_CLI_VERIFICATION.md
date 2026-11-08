# CI, CLI und SHA-gebundene Verifikation

Status: Dokumentationsvorschlag; Änderungen müssen durch PR-Review und die für den Branch geltenden Checks bestätigt werden.

## Zweck und Begriffe

- **CLI (Command-Line Interface):** ein lokal oder in einer Automatisierung ausgeführtes Programm, das Argumente, Optionen, Umgebungsvariablen und Exit-Codes verarbeitet.
- **CI (Continuous Integration):** automatisierte Prüfungen auf einem konkreten Commit, zum Beispiel Formatierung, Build, Tests, statische Analyse und Evidenzvalidierung.
- **CD (Continuous Delivery/Deployment):** die kontrollierte Bereitstellung geprüfter Artefakte. Ein erfolgreicher Build allein autorisiert weder Release noch Deployment.
- **Verifikation:** die Aussage, dass definierte Anforderungen durch geeignete, nachvollziehbare Nachweise für exakt den geprüften Stand erfüllt sind.

CI und CLI sind unterschiedliche Dinge: Die CLI startet ein Werkzeug; CI führt vereinbarte Befehle reproduzierbar aus und sammelt deren Ergebnisse.

## CI/CD-Pipeline in atc-engineering

Die kanonischen Workflow-Definitionen liegen unter `.github/workflows/`. Die tatsächlichen Schritte der Workflows sind maßgeblich; diese Dokumentation ersetzt sie nicht.

### Engineering CI

`.github/workflows/engineering-ci.yml` wird bei Push auf `main`, Pull Requests und manuellem Dispatch ausgeführt. Der Workflow definiert unter anderem:

1. Rust-Toolchain 1.90.0 mit `rustfmt` und `clippy`.
2. `cargo fmt --all -- --check`.
3. Regressionstests für Build-Evidenz und JSON-Schema-Serialisierung.
4. `cargo test --locked --workspace --all-features`.
5. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.
6. Release-Build mit `cargo build --locked --release`.
7. Audit-Gate und Prüfung des Gate-Reports.
8. Abgleich von erwartetem SHA und ausgechecktem SHA sowie Bindung an Repository, Workflow-Ref, Run-ID und Attempt.
9. SHA-256-Prüfsummen für Binärdatei, Gate-Report und Evidenzmanifest.
10. Upload des Evidenzartefakts `atc-engineering-gate-evidence` mit 30 Tagen Aufbewahrung.

### Fleet Audit

`.github/workflows/fleet-audit.yml` ist das organisationsweite Repository-Audit. Sein Umfang und seine Trigger richten sich nach `docs/FLEET_GATE_POLICY.md`. Der Fleet-Status ist von den PR-spezifischen Engineering-CI-Prüfungen zu unterscheiden. Nicht zugängliche Repositories dürfen nicht stillschweigend als bestanden gelten.

### Sicherheits- und Governance-Prüfungen

Workflow-Berechtigungen, Geheimnisschutz, Gate-Report-Semantik, Provenienz und Trennung der Freigabeverantwortung sind Teil der Prüfung. Die konkrete Abdeckung muss aus Workflow-Definitionen, Testfällen und Laufnachweisen abgeleitet werden; ein in einer Roadmap genanntes Ziel ist noch kein implementierter oder verifizierter Kontrollmechanismus.

## CLI-Referenz

### Rust-Entwicklung und CI lokal reproduzieren

Im Repository-Root:

```bash
cargo fmt --all -- --check
cargo test --locked --workspace --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo build --locked --release
cargo run --locked -p atc-audit-cli -- . --gate
```

Die Toolchain sollte mit der CI übereinstimmen. Lokale Ergebnisse sind nützlich zur Diagnose, ersetzen aber nicht die erforderlichen GitHub-Actions-Nachweise des finalen PR-Head-SHA.

### `atc-audit-cli`

Allgemeine Form:

```text
atc-audit-cli [ROOT] [OPTIONS]
```

Wenn kein `ROOT` angegeben ist, verwendet die aktuelle Implementierung `.`. Die folgenden Optionen werden in `crates/atc-audit-cli/src/main.rs` verarbeitet:

| Option | Wirkung |
|---|---|
| `--requirements` | Ermittelt Zielanforderungen; unvollständige Pflichtanforderungen blockieren mit Exit-Code 1. |
| `--repair` | Führt die implementierte begrenzte Reparaturschleife aus und auditiert erneut. |
| `--max-repair-iterations N` | Setzt das Iterationslimit für Reparatur/Engineering; ungültiger oder fehlender Wert fällt aktuell auf 8 zurück. |
| `--engineer` | Führt den Engineering-Loop aus und schreibt Evidenz gemäß der implementierten Policy. |
| `--gate` | Erzeugt einen `GATE_REPORT` und beendet sich mit Exit-Code 1, wenn das Gate nicht freigibt. |
| `--atclang-inspect` | Inspiziert eine ATCLang-Quelldatei bzw. den Pfad. |
| `--atclang-create` | Erstellt eine ATCLang-Quelldatei. |
| `--atclang-edit` | Ersetzt den Inhalt einer ATCLang-Quelldatei. |
| `--atclang-compile` | Ruft den konfigurierten ATCLang-Compiler auf. |
| `--input PATH` | Ein-/Quelldatei für ATCLang-Operationen. |
| `--output PATH` | Ausgabepfad für Erstellen/Kompilieren. |

ATCLang-Optionen und Audit-Optionen können im Programm kombiniert sein; vor komplexen Aufrufen sollte deshalb die konkrete Implementierung geprüft werden. Die Umgebungsvariablen `ATC_REQUESTED_STATE`, `ATC_CODER`, `ATC_VALIDATOR`, `ATC_AUDITOR`, `ATC_RELEASE_AUTHORITY` und `ATC_HUMAN_APPROVAL` beeinflussen den Gate-Kontext. `ATC_SOURCE` setzt Quelltext für Create/Edit; `ATCLANG_COMPILER` überschreibt den Compiler-Namen. Umgebungsvariablen sind Eingaben, keine unabhängigen Beweise für tatsächliche Freigabe oder Rollentrennung.

Beispiele:

```bash
cargo run --release -p atc-audit-cli -- /path/to/repository
cargo run --release -p atc-audit-cli -- /path/to/repository --requirements
cargo run --release -p atc-audit-cli -- /path/to/repository --repair --max-repair-iterations 12
cargo run --release -p atc-audit-cli -- /path/to/repository --gate
```

### Exit-Codes

Die Exit-Codes beziehen sich auf die aktuelle CLI-Implementierung, nicht auf beliebige Shell-Befehle:

| Code | Bedeutung |
|---|---|
| `0` | Der aufgerufene Pfad ist regulär beendet; beim Audit bedeutet dies keine Findings und beim Gate eine erlaubte Entscheidung. |
| `1` | Inhaltliches Gate-/Anforderungs-Ablehnungsergebnis oder Audit-Findings. |
| `2` | Ein behandelte Eingabe-, Parsing-, Datei-, Compiler- oder Ausführungsfehler in den entsprechenden Pfaden. |

Für jeden anderen Befehl gilt dessen eigene Exit-Code-Semantik. Ein Exit-Code 0 sagt nur aus, dass dieser konkrete Befehl erfolgreich beendet wurde; er beweist nicht automatisch vollständige funktionale Korrektheit.

## GitHub CLI und Actions-Diagnose

Die folgenden Beispiele setzen eine installierte und authentifizierte GitHub CLI (`gh`) voraus:

```bash
gh auth status
gh pr list --repo A-TownChain-Okosystems/atc-engineering
gh pr view <PR-NUMMER> --repo A-TownChain-Okosystems/atc-engineering
gh run list --repo A-TownChain-Okosystems/atc-engineering
gh run view <RUN-ID> --repo A-TownChain-Okosystems/atc-engineering
gh run view <RUN-ID> --repo A-TownChain-Okosystems/atc-engineering --log-failed
```

Bei einem Fehler: zuerst den betroffenen Workflow-Run und dessen getesteten SHA feststellen, danach Job, fehlgeschlagenen Step, Exit-Code und Log prüfen. Ursache und Infrastrukturfehler getrennt dokumentieren. Fehlende Berechtigung, nicht verfügbare Logs oder nicht ausgeführte Pflichtjobs sind kein Pass.

Artefakte lassen sich über die Actions-Oberfläche des jeweiligen Runs oder die GitHub CLI abrufen. Vor ihrer Verwendung ist zu prüfen, ob sie zum richtigen Run/Attempt und Commit gehören. SHA-256-Prüfsummen belegen die Integrität der erfassten Bytes im Vergleich zur Prüfsumme; sie beweisen für sich allein weder die Korrektheit des Artefakts noch die Erfüllung funktionaler Anforderungen.

## Verifikationsvertrag: Evidenz am exakten Commit

Für jeden als verifiziert ausgewiesenen PR-Stand müssen die Nachweise mindestens enthalten:

- Repository und vollständiger 40-stelliger Commit-SHA.
- PR-Nummer und Head-SHA; der tatsächlich getestete SHA muss exakt mit dem zu verifizierenden SHA übereinstimmen.
- Workflow-Name/Ref, Run-ID und Attempt.
- Für jedes erforderliche Gate: Job, Step, Exit-Code und nachvollziehbarer Log- oder Artefaktverweis.
- Artefaktname, SHA-256-Prüfsumme und Provenienz, sofern ein Artefakt erzeugt wird.
- Negative Tests für die relevanten Ablehnungsbedingungen, sofern der Gate-Vertrag sie verlangt.
- Offene Befunde, ausgeschlossene Bereiche und externe Blocker.

Ein erfolgreicher Merge-Ref-Test oder ein grüner Run auf einem älteren Head-SHA ersetzt keinen Test des finalen PR-Heads. Nach einer neuen Commit-Änderung muss die Verifikation für den neuen SHA erneut geprüft werden.

## Statusmodell

| Status | Aussage |
|---|---|
| `IMPLEMENTED` | Änderung oder Kontrolllogik ist im Quellstand vorhanden. |
| `TESTED` | Bestimmte Tests wurden ausgeführt; Umfang und SHA müssen dokumentiert sein. |
| `CI_VERIFIED` | Die definierten CI-Prüfungen bestanden für den ausgewiesenen exakten SHA, mit nachvollziehbarer Run-/Job-/Step-Evidenz. |
| `E2E_VERIFIED` | Definierte End-to-End-Szenarien wurden für den ausgewiesenen Stand ausgeführt und bestanden. |
| `VERIFIED` | Die festgelegten fachlichen Abnahmekriterien sind durch vollständige, aktuelle und geeignete Evidenz erfüllt. |
| `RESIDUAL` | Ein bekannter Befund oder ein nicht abgeschlossener Teil bleibt bestehen. |
| `BLOCKED` | Erforderliche Evidenz, Zugriffsrechte, Voraussetzungen oder Freigaben fehlen. |

Diese Status sind nicht automatisch gleichbedeutend und dürfen nicht allein aus einem Badge oder einer Erfolgsmeldung abgeleitet werden. Fehlende, veraltete oder SHA-fremde Evidenz ergibt `BLOCKED` bzw. `RESIDUAL`, nicht `VERIFIED`.

## Abnahmekriterien für Dokumentations- und Engineering-PRs

- Änderungen sind eng abgegrenzt und über einen Review-PR eingereicht.
- Pflichtprüfungen wurden auf dem finalen PR-Head ausgeführt und ihr Status ist nachvollziehbar.
- Run → Job → Step → Exit-Code → Log/Artefakt ist dokumentiert.
- Geprüfter SHA, PR-Head-SHA und Artefakt-Provenienz stimmen überein.
- Relevante positive und negative Tests sind vorhanden und bestanden.
- Restrisiken, nicht getestete Pfade und externe Blocker sind explizit aufgeführt.
- Kein Gate wurde abgeschwächt, kein Test entfernt und kein fehlender Nachweis als Erfolg gewertet.
- Es erfolgt kein automatischer Merge; Review und erforderliche menschliche Freigabe bleiben bestehen.

**Freigaberegel:** Grün in CI ist notwendige Evidenz für die von CI ausgeführten Prüfungen, aber keine universelle funktionale Freigabe. `CI_VERIFIED` ist kein Ersatz für `VERIFIED`.
