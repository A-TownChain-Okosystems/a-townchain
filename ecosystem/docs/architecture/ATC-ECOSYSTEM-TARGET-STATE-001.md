# ATC-ECOSYSTEM-TARGET-STATE-001 — Ökosystemweiter Soll-Zustand

> **Dokument-ID:** ATC-ECOSYSTEM-TARGET-STATE-001  
> **Version:** 0.1.0  
> **Status:** DRAFT / NOT VERIFIED  
> **Geltungsbereich:** A-TownChain, GlobusOS, ShivaCore, Aurora AI, ATCLang, Genesis Engine, Wallet/SDK, Standards, Deployment und Ökosystem-Governance  
> **Autoritative Ablage:** A-TownChain-Okosystems/a-townchain-ecosystem  
> **Zweck:** zentraler Soll-Katalog und überprüfbarer Soll-/Ist-Abgleich; keine zweite Implementierungs-SSOT.

## 1. Verbindliche Lesart

Dieses Dokument legt fest, wie der vollständige Ökosystem-Soll-Zustand erfasst und nachgewiesen wird. Es ist **kein** Beleg dafür, dass die beschriebenen Funktionen bereits implementiert oder verifiziert sind.

- **Soll:** freigegebene Architektur, Standards, Funktionsverträge, Sicherheitsinvarianten und Abnahmekriterien.
- **Ist:** Zustand einer kanonischen Quelle an einem vollständigen Commit-SHA.
- **Evidence:** konkrete Tests und Prüfläufe mit Run → Job → Step → Exit-Code/Logs sowie Artefakten und Hashes.
- **Abweichung:** Differenz zwischen freigegebenem Soll und beobachtetem Ist.
- **Status:** IMPLEMENTED, TESTED, CI_VERIFIED, E2E_VERIFIED, VERIFIED, RESIDUAL, BLOCKED, UNRESOLVED oder begründet EXCLUDED.

Ein grüner CI-Run, ein vorhandenes Dokument, ein geschlossener Issue oder ein gemergter PR allein beweist keine vollständige Verifikation. Fehlende Pflichtnachweise bleiben BLOCKED oder UNRESOLVED. VERIFIED gilt nur für die konkrete Anforderung, den konkreten Commit und die tatsächlich ausgeführten Abnahmekriterien.

## 2. SSOT- und Governance-Grenzen

| Zuständigkeit | Kanonische Quelle / Regel |
|---|---|
| Ökosystem-Inventar, Soll-/Ist-Korrelation, Driftbefunde, Cross-Repo-Evidence | a-townchain-ecosystem |
| Normative Standards und technische Verträge | atc-standards; nur registrierte und freigegebene Normen sind verbindlich |
| Blockchain-Kern, ATC-VM, Algorithmus/Konsens und Wallet | jeweilige kanonische Pfade in a-townchain bzw. atc-wallet |
| ShivaCore-Kernel | globus-os/modules/atc-shivacore/kernel/ |
| GlobusOS und OS-Dienste | kanonische Module in globus-os |
| Aurora AI | aurora-ai |
| ATCLang | kanonische ATCLang-Implementierungsquelle; exakter Pfad im Inventar nachzuweisen |
| Genesis Engine | genesis-engine |
| SDKs und Tooling | jeweilige registrierte SSOT, nicht ein duplizierter Umbrella-Snapshot |

Importierte Subtrees sind Integrations-Snapshots. Sie dürfen die Quell-SSOT nicht ersetzen. Keine zweite VM, kein zweiter Konsens-/Wallet-Kern und keine zweite Kernelimplementierung im Ecosystem-Control-Plane.

## 3. Layer-Modell: bestehender Registerstand und dokumentiertes Zielmodell

**Zielmodell für die weitere Architekturplanung:** `L0–L9 + X0–X36`. Dieses Zielmodell ist im organisationsweiten `AGENT_MANIFEST.md` von `A-TownChain-Okosystems/.github` dokumentiert (Commit `2d97437a263ba080bf2e2ca171cdfe13b8ff15f0`, Abschnitt 9). `X0` ist ausdrücklich Teil des Cross-Layer-Namensraums. Die frühere Katalogfassung `L0–L9 + X1–X36` ist damit als Zielvorschlag überholt.

**Der Ist-/Registry-Stand bleibt davon getrennt:** Der derzeitige Layer-Architekturentwurf und das Registry-SSOT auf `main` deklarieren **L0–L10 und X1–X8**. Die Manifest-Dokumentation dokumentiert das Zielmodell für die Planung, ersetzt aber keine normative Architektur-/Standards-Freigabe und migriert Registry, Mapping oder Validatoren nicht automatisch. Es darf weder eine bereits erfolgte Registry-Migration noch eine normative Freigabe behauptet werden.

Nachweisbarer Registerstand bei der Prüfung:
- Architekturdatei: [ATC-ECOSYSTEM-LAYER-ARCHITECTURE-001.md](ATC-ECOSYSTEM-LAYER-ARCHITECTURE-001.md), Blob-SHA `0c9c4e2d811125131011c1bec68aae744f620b0a`.
- Layer-Registry: [ATC-LAYER-REGISTRY-001.yaml](layers/ATC-LAYER-REGISTRY-001.yaml), Blob-SHA `045a45098f51408b6fb99e75639afc997d733eb0`.
- Component mapping: [ATC-LAYER-MAPPING-001.yaml](layers/ATC-LAYER-MAPPING-001.yaml), Blob-SHA `3f58d6a9b0e3ed794e75bd84b2f08d6028363693`.
- Alle drei Quellen kennzeichnen sich als Draft bzw. owner review pending; sie sind ein nachgewiesener Repository-Stand, aber keine hiermit behauptete normative Freigabe.

### Vorläufige Bestandsaufnahme – bestehendes Register

| ID-Bereich | Dokumentierter Stand | Status in diesem PR |
|---|---|---|
| L0–L10 | 11 Hauptschichten, darunter L10 Integration/Deployment/Ops | Bestehender Draft; Abgleich mit Zielmodell und Migration ausstehend |
| X1–X8 | 8 Cross-Cutting-Layer | Bestehender Draft; Zielkonflikt offen |
| X0 | Im bestehenden Register nicht als kanonische Cross-Cutting-ID belegt | Im dokumentierten Zielmodell enthalten; Registry-Migration ausstehend |
| X9–X36 | Im bestehenden Register nicht als kanonische Cross-Cutting-IDs belegt | Im dokumentierten Zielmodell enthalten; nicht als bereits registriert oder freigegeben behaupten |
| X35 | In den Projektanforderungen für Installer/Release/Deployment vorgesehen | Zuordnung und Autorität gegen Register/Standards zu klären |
| X36 | In den Projektanforderungen für Systemkomposition und E2E-Abschluss vorgesehen | Zuordnung und Autorität gegen Register/Standards zu klären |
| L10 | Im bestehenden Registry-SSOT enthalten | Entfernen/Umnummerieren nur nach autorisierter Architekturentscheidung |

### Erforderliche Freigabe und koordinierte Migration

Das Zielmodell für die Planung ist `L0–L9 + X0–X36`; die normative Freigabe und die Migration sind jedoch separate, noch offene Schritte. Vor Änderungen am Registry-SSOT muss der zuständige Architektur-/Standards-Owner mindestens dokumentieren:
1. die Verantwortlichkeiten von L0–L9 und X0–X36 sowie die Behandlung der bisherigen L10-Verantwortlichkeiten;
2. stabile Definitionen und Eigentümer für X0–X36 einschließlich der bestehenden X1–X8-Definitionen;
3. die Referenzabbildung für Komponenten, Funktionen, Sub-Layer und bestehende Katalogeinträge;
4. die koordinierte Änderung von Architekturvertrag, Registry, Mapping, Katalogen, Validatoren und gegebenenfalls `atc-standards`.

Bis die Freigabe und Migration nachgewiesen sind, lautet der Status **TARGET MODEL RECORDED / REGISTRY MIGRATION PENDING**. Das Zielmodell darf für die weitere Architekturplanung verwendet werden, aber weder als bereits normativ genehmigt noch als in Registry/Validatoren umgesetzt dargestellt werden.

## 4. Pflichtschema für jede Soll-Anforderung

Jede Anforderung wird als maschinenlesbarer Datensatz geführt und mindestens mit folgenden Feldern verbunden:

- Identität: requirement_id, Titel, präzise Sollbeschreibung und Version.
- Normquelle: Repository, exakter Pfad sowie freigegebene Version oder Commit-SHA.
- Architektur: Layer, stabile Komponenten-ID und Funktions-ID; ungeklärte IDs bleiben UNRESOLVED.
- Ownership: kanonisches Repository, exakter Pfad und Symbol.
- Abnahmekriterien: stabile Test-ID, erwartetes beobachtbares Ergebnis und negativer Fehler-/Missbrauchsfall.
- Abhängigkeiten und Sicherheitsinvarianten.
- Evidence-Pflichten: exakter Commit-SHA, Workflow-Run/Job/Step, Exit-Code/Logs und Artefakt-Hashes.
- Beobachteter Zustand: geprüfter Commit, Resultat, offene Lücken und Status.
- Ausnahme: genehmigter Verweis, Verantwortlicher und Prüfzeitpunkt.

Das Schema muss maschinenlesbar als versioniertes JSON- oder YAML-Schema registriert und validiert werden. Dieser Abschnitt allein ist noch kein implementiertes oder validiertes Schema.

## 5. Ökosystem-Inventar und Soll-/Ist-Abgleich

Das Inventar muss alle aktiven, archivierten, externen und ausdrücklich ausgeschlossenen Repositories erfassen. Pro Repository sind mindestens festzuhalten:

- Repository-Name, Status, Default-Branch, verantwortliche Domäne und SSOT-Rolle.
- Vollständiger geprüfter Commit-SHA und Root-Tree-SHA.
- kanonische Komponenten/Funktionen sowie zugehörige Katalog-IDs.
- Snapshot-Commit und Differenz zum kanonischen Quell-Commit, sofern importiert.
- Workflows, verpflichtende Checks, letzte relevante Runs und deren SHA-Bindung.
- offene Architektur-, Sicherheits-, Katalog- und Evidence-Befunde.
- genehmigte Ausnahmen mit Begründung, Verantwortlichem und Review-Datum.

Ein unvollständiger Repository-Census darf nicht als vollständig gemeldet werden. Das demo-repository bleibt vom Live-Drift-Gate ausgeschlossen, sofern die bestehende Governance dies festlegt; die Ausnahme ist explizit und nachvollziehbar zu führen.

## 6. Verifikations- und Release-Gates

| Gate | Abnahmekriterium | Fail-closed-Verhalten |
|---|---|---|
| G0 – Inventory | alle relevanten Quellen erfasst oder als offen/excluded begründet | fehlende Quellen → BLOCKED |
| G1 – Catalog | stabile IDs, keine ungeklärten Dubletten, alle Lücken sichtbar | fehlende/mehrdeutige Zuordnung → BLOCKED |
| G2 – SSOT | jede Implementierung einer autoritativen Quelle zugeordnet | konkurrierende Kernimplementierung → Merge-Block nach Policy |
| G3 – Contract | Inputs, Outputs, Fehler, Invarianten und Rechte festgelegt | fehlender Pflichtvertrag → UNRESOLVED |
| G4 – Component | unit-, negative-, security- und determinism tests nach Vertrag | fehlender oder fehlgeschlagener Test → nicht bestanden |
| G5 – Integration | Cross-Repo-Schnittstellen mit exakten SHAs geprüft | nur Einzel-CI grün → keine E2E-Aussage |
| G6 – System/E2E | vollständige, dokumentierte Systempfade auf unterstützten Profilen | ungetesteter Pflichtpfad → BLOCKED |
| G7 – Release/Deployment | signierte Artefakte, Herkunft, Upgrade/Rollback, Installer und Recovery belegt | fehlender Supply-Chain-/Rollback-Nachweis → Release blockiert |
| G8 – Evidence | Claim ↔ Commit ↔ Run/Job/Step ↔ Logs ↔ Artefakt-Hash nachvollziehbar | fehlende Bindung → BLOCKED |
| G9 – Governance | Ausnahmen, Reviews und Freigaben gültig; keine offenen kritischen Befunde | kritischer offener Befund → Merge/Release blockiert |

Ein Gate kann nur die Kriterien bestätigen, die es tatsächlich ausgeführt hat. CI-Erfolg und Produktionsreife bleiben getrennte Aussagen.

## 7. Mindest-Evidence-Vertrag

Jeder Verifikationsbefund enthält:

- claim_id, requirement_id, component_id und test_id.
- kanonisches Repository, vollständigen Commit-SHA, Pfad, Symbol und Blob-SHA soweit anwendbar.
- Referenz-SHA für den Vergleich, Zeitpunkt und Umgebung/Hardwareprofil.
- Workflow-Run-ID, Job-ID/-Name, Step, Exit-Code und relevante Logs.
- Artefaktname, unveränderliche Referenz und SHA-256-Hash.
- erwartetes Ergebnis, beobachtetes Ergebnis, Einschränkungen und Status.
- Verantwortlichen und Verweis auf einen offenen Befund bei Fehlschlag.

Keine SHAs, Testresultate, Artefakte, Freigaben oder Verantwortlichen dürfen erfunden oder aus benachbarten Commits übernommen werden. Wenn ein Nachweis nicht abrufbar ist, bleibt der Status offen.

## 8. Vollständigkeitskriterien

Der Soll-Zustand darf erst als katalogseitig vollständig bezeichnet werden, wenn:

- [ ] Layer-/Sublayer-Modell, ID-Namensräume und normative Konflikte autorisiert aufgelöst sind.
- [ ] Repository-Census vollständig ist; Ausschlüsse und archivierte Quellen sind begründet.
- [ ] jede bekannte Komponente und Funktion eine stabile ID und eine Zuständigkeit besitzt.
- [ ] jede Anforderung messbare Abnahmekriterien einschließlich negativer Fälle hat.
- [ ] jede bekannte Abhängigkeit/Schnittstelle als Vertrag oder als offene Lücke geführt wird.
- [ ] alle SSOT-Zuordnungen belegt oder explizit UNRESOLVED sind.
- [ ] Katalog, Layer-Register, Dependency Graph und Standards-Registry konsistent sind.
- [ ] Drift-, Dubletten-, Schema- und Evidence-Validierung automatisiert und getestet ist.
- [ ] die Abdeckung je Layer, Komponente, Funktion und Test offen ausgewiesen wird; unbekannte Werte werden nicht als 100 % gezählt.
- [ ] End-to-End- und Release-Nachweise auf exakten SHAs vorliegen, bevor das Gesamtsystem als verifiziert bezeichnet wird.

## 9. Umsetzungsreihenfolge

1. **Registry reconciliation:** aktuelles Layer-Register, Architekturvertrag, Standards-Registry und bestehende Kataloge auf Konflikte prüfen; keine stille Umnummerierung.
2. **Repository census:** aktive/archivierte/ausgeschlossene Repositories, Branches, Commit-SHAs und SSOT-Rollen festhalten.
3. **Catalog gap analysis:** fehlende Komponenten, Funktionen, Schnittstellen und Abnahmekriterien identifizieren.
4. **Schema + validator:** Soll-Anforderungen und Evidence maschinenlesbar validieren; negative Tests für fehlende Felder, falsche SHA-Bindung und doppelte IDs hinzufügen.
5. **Cross-repo reconciliation:** kanonische SHAs und importierte Snapshots vergleichen; Drift nicht automatisch als Implementierungsfehler klassifizieren.
6. **Verification matrix:** Pflicht-Workflows, Profile, Tests, E2E-Pfade und Release-Gates pro Anforderung zuordnen.
7. **Evidence-backed closure:** Befunde einzeln beheben und mit frischer, exakter Evidenz erneut prüfen.

## 10. Aktueller Dokumentstatus

**DRAFT / NOT VERIFIED / TARGET MODEL RECORDED / REGISTRY MIGRATION PENDING.** Dieses Dokument strukturiert den Soll-Zustand und die erforderliche Beweiskette. Das Zielmodell `L0–L9 + X0–X36` ist im Agent Manifest dokumentiert, aber die normative Freigabe, Behandlung der bisherigen L10-Verantwortlichkeiten, Registry-/Mapping-/Katalog-/Validator-Migration, tatsächliche Repository-Abdeckung und zugehörige CI-Nachweise bleiben separat zu prüfen und freizugeben.

### Referenzen

- [Ökosystem-README](../../README.md)
- [Layer-Architektur](ATC-ECOSYSTEM-LAYER-ARCHITECTURE-001.md)
- [Komponenten-/Funktionskatalog (JSON)](ATC-COMPONENT-FUNCTION-CATALOG-001.json)
- [Abhängigkeitsgraph (JSON)](ATC-DEPENDENCY-GRAPH-001.json)
- [Layer-Register und Mapping](layers/)
- [X18 – Ecosystem Integrity Entwurf](../knowledge-system/records/generated/ATC-KMS-ISSUE-163.json)
