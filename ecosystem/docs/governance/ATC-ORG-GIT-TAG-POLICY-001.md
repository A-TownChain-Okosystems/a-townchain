# Git-Tag- und Release-Policy

Status: Vorschlag zur Governance-Abnahme  
Policy-ID: ATC-ORG-GIT-TAG-POLICY-001  
Geltungsbereich: alle Repositories der Organisation `A-TownChain-Okosystems`

## 1. Grundsatz

Ein Git-Tag ist eine unveränderliche Versionsreferenz auf einen konkreten Commit, kein Ersatz für CI-Evidence und kein Nachweis von Produktionsreife. Das Ökosystem-Repository aggregiert Governance und Evidence; es vergibt keine Releases stellvertretend für kanonische Quell-Repositories.

## 2. Tag-Namensschema

- Stabile Releases: `vMAJOR.MINOR.PATCH` (SemVer), nur nach bestandenen Release-Gates.
- Vorabversionen: SemVer-Prerelease-Suffixe, z. B. `v1.2.0-rc.1`; als Pre-release markieren.
- Keine beweglichen Tags wie `latest`, `stable` oder `main` als Release-Ersatz.
- Ein Tag muss auf einen vollständigen Commit-SHA zeigen und in den Release-Evidence-Daten denselben SHA dokumentieren.

## 3. Release-Gates

Vor Erstellung oder Veröffentlichung eines Release-Tags müssen mindestens dokumentiert sein:

1. kanonisches Repository und verantwortlicher Maintainer,
2. exakter Commit-SHA (kein PR-Merge-Ref als Ersatz),
3. erforderliche CI-Checks für diesen SHA mit Run → Job → Step → Exit-Code/Log,
4. Security-/Dependency-Prüfungen entsprechend der Repository-Policy,
5. Kompatibilitäts- und Migrationshinweise, falls Schnittstellen oder Formate betroffen sind,
6. Release-Notes und bekannte Residuals,
7. unabhängige Freigabe gemäß Branch-/Review-Schutz.

`IMPLEMENTED`, ein grüner Einzeljob oder das Vorhandensein von Dateien ist allein kein Release-Gate. Bei fehlender Evidenz bleibt der Status `BLOCKED` oder `RESIDUAL`.

## 4. Unveränderlichkeit und Korrekturen

- Veröffentlichte Tags dürfen nicht verschoben oder gelöscht werden, um eine fehlerhafte Freigabe zu kaschieren.
- Bei einem fehlerhaften Release wird ein neues Patch-/Prerelease-Tag auf einen neuen Commit erzeugt und das alte Release als superseded/yanked dokumentiert.
- Annotierte und signierte Tags sind für Releases zu bevorzugen. Die Signaturprüfung muss Teil der Release-Evidence sein, sofern Signierung technisch eingerichtet ist.
- Ein Tag-Name allein beweist keine Signatur; Tag-Objekt und Ziel-Commit müssen geprüft werden.

## 5. Archivierte Repositories

Archivierte Repositories erhalten keine neuen Tags, solange kein expliziter Revival-/Maintenance-Beschluss mit Verantwortlichem, CI-Plan und Support-Zeitraum vorliegt.

## 6. Bestandsaufnahme vom 2026-10-11

Die maschinenlesbare Bestandsaufnahme liegt in `docs/governance/ATC-ORG-GIT-TAG-AUDIT-001.json`.

- 33 Repositories geprüft: 26 nicht archiviert, 7 archiviert.
- Git-Tags gefunden: `a-townchain-os:v1.0.0` und `atc-standards:v1.1.0`.
- Keine vorhandenen Tags wurden verschoben oder gelöscht.
- Es wurden keine neuen Release-Tags erzeugt: Für die 31 taglosen Repositories wurde kein vollständiger exakter-SHA-Release-Nachweis durch diesen Audit erbracht.
- Der Draft-Release `a-townchain-os:v2.0.0` bleibt ein Draft; er wird nicht veröffentlicht, bevor seine Release-Gates nachgewiesen sind.

## 7. Rollout

Diese Policy ist ein Vorschlag in einem offenen PR. Sie wird erst nach Review und Merge verbindlich. Anschließend sollte ein CI-Validator Tag-Format, kanonisches Repository, Release-Evidence-SHA und unveränderliche Tag-Nutzung prüfen. Der Validator darf fehlende Evidence nicht durch Annahmen ersetzen.
