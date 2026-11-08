---
document_id: ATC-SPEC-GLOS-SEC-001
title: "GlobusOS Security Center — Technische Vollspezifikation"
version: 1.0.0-draft
status: draft
owner: A-TownChain-Okosystems
created: 2026-10-18
updated: 2026-10-18
scope:
  - GlobusOS userspace security services
  - ShivaCore kernel security boundary
  - package verification and updates
  - identity and wallet integration
---

# GlobusOS Security Center — Technische Vollspezifikation

**Status:** DRAFT · **Produktionsfreigabe:** NOT_READY  
**Geltungsbereich:** GlobusOS und dessen verbindliche Sicherheitsgrenzen zu ShivaCore, Identity, Package/Update, IPC und Wallet-Service.

## 1. Zweck und normative Sprache

Diese Spezifikation beschreibt Zielarchitektur, Sicherheitsinvarianten, Schnittstellen, Integrationspunkte, Prüfstrategie und Freigabekriterien des GlobusOS Security Centers. Sie ist ein Architektur- und Implementierungsvertrag, kein Nachweis, dass alle beschriebenen Funktionen bereits implementiert oder geprüft sind.

Die Begriffe **MUSS**, **DARF NICHT**, **SOLL**, **SOLLTE** und **KANN** sind normativ zu verstehen. Abweichungen von MUSS-Anforderungen blockieren die Sicherheitsfreigabe, bis sie begründet und formell genehmigt sind.

## 2. Status- und Evidenzgrenze

Der Repository-Status dokumentiert GlobusOS als Entwicklung / NOT_READY; ein unabhängiges Security Audit und die Hardwarevalidierung sind nicht abgeschlossen. Das Vorhandensein von Crates, Verträgen, Tests oder Workflows beweist weder eine vollständige Ende-zu-Ende-Integration noch Produktionsreife.

Die Architektur beschreibt `system/` als kanonische GlobusOS-Userspace-Basis und `modules/atc-shivacore/kernel/` als kanonischen Kernel-Quellpfad. ShivaCore bleibt Kernel und Trusted Computing Base (TCB); Security Center, Aurora, Blockchain- und Desktop-Logik bleiben außerhalb des Kernels.

Die bestehende Arbeit zur Paket-Signaturprüfung in [PR #84](https://github.com/A-TownChain-Okosystems/globus-os/pull/84) adressiert eine wichtige Teilfunktion. Laut PR-Beschreibung bleiben Schlüsselbereitstellung, Rotation/Widerruf und die Durchsetzung an jedem Installations-Einstiegspunkt offene Integrations- und Review-Aufgaben. Der PR-Text hält außerdem fest, dass die dort hinzugefügten Tests noch nicht ausgeführt wurden. Daraus wird hier kein CI-PASS abgeleitet.

## 3. Ziele und Nicht-Ziele

### 3.1 Ziele

- Einheitliche, fail-closed Sicherheitsentscheidungen für privilegierte GlobusOS-Operationen.
- Explizite Identitäten, Fähigkeiten (Capabilities), Richtlinien und Vertrauensanker.
- Kryptografische Prüfung exakt der Artefaktbytes, die anschließend installiert oder aktiviert werden.
- Sichere Schlüsselrotation, Widerruf, Update, Recovery und Wallet-Autorisierung.
- Nachvollziehbare, datensparsame Sicherheitsereignisse und belastbare Freigabeevidenz.
- Prüffähige Schnittstellen und reproduzierbare Negativtests an den Systemgrenzen.

### 3.2 Nicht-Ziele

- Das Security Center ersetzt weder Kernel-Enforcement noch eine unabhängige Sicherheitsprüfung.
- Es ist kein Malware-Scanner und garantiert keine Erkennung unbekannter Schadsoftware.
- Es darf der KI, Desktop-Shell oder Blockchain-Laufzeit keine implizite privilegierte Autorität verleihen.
- Eine Benutzeroberfläche oder ein Status-Dashboard gilt nicht als Sicherheitskontrolle, wenn der zugrunde liegende Dienst die Entscheidung nicht erzwingt.

## 4. Bedrohungsmodell und Vertrauensgrenzen

### 4.1 Zu schützende Werte

- Integrität von Kernel, Boot-Kette, Systemdiensten, Paket- und Update-Artefakten.
- Vertraulichkeit und Integrität von Identitäts-, Schlüssel-, Wallet- und Recovery-Material.
- Korrekte Capability-Zuweisung, IPC-Autorisierung und Policy-Auswertung.
- Integrität von Audit-Ereignissen, Trust-Store und Sicherheitsstatus.
- Verfügbarkeit sicherheitskritischer Recovery- und Update-Pfade.

### 4.2 Angreiferannahmen

Das Design MUSS mindestens mit kompromittierten oder fehlerhaften Userspace-Diensten, manipulierten Paketen, nicht vertrauenswürdigen Signierern, widerrufenen Schlüsseln, wiederholten/veralteten Metadaten, Downgrade-Versuchen, gefälschten IPC-Requests, unvollständigen Audit-Ausgaben und Ausfällen von Update-/Storage-Komponenten rechnen. Netzwerkantworten, Paketmetadaten, UI-Eingaben und KI-Ausgaben sind nicht vertrauenswürdig, bis sie validiert und autorisiert wurden.

### 4.3 Autoritätsmodell

1. Firmware/Bootloader: Vertrauensanker nur im Rahmen tatsächlich unterstützter und geprüfter Plattformfunktionen.
2. ShivaCore-TCB: erzwingt Adressraumisolation, Capability- und IPC-Grenzen, Scheduling und Low-Level-Ressourcenzugriff.
3. GlobusOS-Sicherheitsdienste: Identität, Policy, Paketprüfung, Update, Audit und koordinierte Reaktion.
4. Anwendungen, Desktop, Aurora und Blockchain-Dienste: nicht vertrauenswürdig als Autoritätsquelle; sie erhalten nur explizit gewährte Fähigkeiten.

Eine Userspace-Prüfung allein ersetzt keine vom Kernel erzwungene Capability-Grenze.

## 5. Sicherheitsprinzipien und Invarianten

- **Deny by default:** Nicht erkannte, unvollständige oder widersprüchliche Eingaben werden abgelehnt.
- **Least privilege:** Jede Identität erhält nur zweckgebundene, begrenzte Fähigkeiten.
- **No ambient authority:** Privilegien werden nicht aus Prozessnamen, UI-Zustand oder bloßer Erreichbarkeit abgeleitet.
- **Kernel enforcement:** Sicherheitskritische Ressourcen und IPC werden an der maßgeblichen Grenze autorisiert.
- **Exact-byte verification:** Verifiziertes Objekt und installiertes Objekt müssen dieselben Bytes repräsentieren.
- **Fail closed:** Unbekannter Schlüssel, Widerruf, Hash-/Signaturfehler, unvollständige Policy oder nicht verfügbarer zwingender Audit-Pfad blockieren die geschützte Operation.
- **Separation of duties:** Schlüsselpflege, Freigabe und Aktivierung sollen nicht durch einen einzigen unkontrollierten Pfad delegiert werden.
- **Evidence before readiness:** Quellcodepräsenz, ein einzelner Test oder ein historischer Workflow-Run reichen nicht für PRODUCTION_READY.

## 6. Zielarchitektur und Verantwortlichkeiten

| Bereich | Verantwortung | Sicherheitsanforderung |
|---|---|---|
| `system/security/` | Policy-Auswertung, Sicherheitsentscheidungen, gemeinsame Verträge | Deterministisch, fail-closed, versionierte Policy |
| `system/identity/` | Identität, Authentisierung, Recovery-Bindung | Keine Autorisierung allein durch selbst deklarierte Request-Felder |
| `system/ipc/` | Nachrichten-/Endpoint-Grenze | Absender, Ziel, Capability, Operation und Payload validieren |
| `system/package/` | Paketmetadaten und Artefaktprüfung | Digest und Signatur über kanonisch definierte Daten und exakte Bytes |
| `system/update/` | Staging, Aktivierung, Rollback | Nur verifizierte unveränderliche Artefakte aktivieren |
| `system/wallet-service/` | Wallet-Autorisierung und Signiergrenze | Keine Schlüssel-/Grant-Übergabe als untrusted Request-Behauptung akzeptieren |
| `system/diagnostics/` | Sicherheitszustand sichtbar machen | Beobachtung ist keine Autorisierung; keine Geheimnisse in Logs |
| `modules/atc-shivacore/kernel/` | Kernel-TCB | Capability-/IPC-Enforcement muss kernel-seitig verbindlich sein |

Ein neues `system/security-center/`-Crate ist erst nach Prüfung von Workspace-Konventionen, bestehenden Crates und Abhängigkeiten zulässig. Bestehende Implementierungen sind zu erweitern, nicht durch parallele oder konkurrierende Autoritätsquellen zu duplizieren.

## 7. Komponentenanforderungen

### 7.1 Policy Engine

- Policies MÜSSEN versioniert, typisiert, deterministisch und auf einen klar definierten Kontext begrenzt sein.
- Jede Entscheidung MUSS mindestens Ergebnis (ALLOW/DENY), Regel-/Policy-Version, Begründungscode und erforderliche Audit-Klassifikation liefern.
- Fehler, fehlende Felder, unbekannte Aktionen und unauflösbare Konflikte führen zu DENY.
- Policy-Änderungen benötigen Authentisierung, Berechtigung, Validierung und ein Audit-Ereignis.

### 7.2 Identität und Capability Enforcement

- Identität, Capability, Policy und Objektberechtigung sind getrennte Prüfschritte.
- Requests DÜRFEN nicht selbst einen `Grant`, eine Vertrauensstufe oder eine erfolgreiche Vorautorisierung behaupten.
- Capability-Erteilung und -Widerruf müssen an der autoritativen Stelle erfolgen; untrusted Userspace-Daten dürfen keinen Kernel-Grant erzeugen.
- Widerruf und Ablauf müssen bei nachfolgenden Operationen wirksam sein; gecachte Entscheidungen benötigen definierte Invalidierungsregeln.

### 7.3 Paket- und Artefaktprüfung

Vor Installation MUSS der tatsächliche Installationspfad folgende Prüfungen erzwingen:

1. Metadaten syntaktisch und semantisch validieren; kanonische Signaturrepräsentation eindeutig festlegen.
2. Digest der konkreten Artefaktbytes neu berechnen und exakt vergleichen.
3. Signatur gegen explizit vertrauenswürdigen, für diesen Zweck zugelassenen Schlüssel prüfen.
4. Schlüsselstatus, Widerruf, Gültigkeitszeitraum, Algorithmus- und Policy-Version prüfen.
5. Paketname, Version, Zielplattform, Abhängigkeiten und zulässige Downgrade-/Rollback-Regeln validieren.
6. Fehler oder nicht verfügbare zwingende Trust-Daten führen zum Abbruch ohne Aktivierung.

Der Verifier MUSS ein unveränderliches `VerifiedArtifact` oder ein typisiertes Fehlerresultat liefern. Der Installer MUSS genau die verifizierten Bytes verwenden. Zwischen Verifikation und Installation darf kein Pfadwechsel, Austausch, erneutes ungesichertes Herunterladen oder anderer TOCTOU-Austausch möglich sein. Wenn Bytes kopiert werden müssen, ist der Digest der Installationskopie erneut zu prüfen.

### 7.4 Schlüsselverwaltung und Widerruf

- Trust Anchors benötigen Herkunft, Zweck, Algorithmus, Version und Lebenszyklusstatus.
- Rotation muss authentisiert, versioniert, nachvollziehbar und gegen Replay geschützt sein.
- Widerruf muss monotone Versionierung oder gleichwertigen Replay-Schutz nutzen.
- Widerrufene, unbekannte, abgelaufene oder zweckfremde Schlüssel werden abgelehnt.
- Offline-Verhalten, veraltete Trust-Snapshots und Notfallrotation sind explizit zu spezifizieren; unsicheres Fail-open ist verboten.
- Private Schlüssel dürfen nicht in Logs, Diagnosedumps oder ungeschützten Konfigurationen erscheinen.

### 7.5 Update und Recovery

Der Update-Ablauf SOLL als explizite Zustandsmaschine modelliert werden:

`Idle → Downloaded → Verified → Staged → Activated → Healthy`

Fehler vor Aktivierung führen zu `Rejected` oder `Aborted`. Fehler nach Aktivierung müssen einen definierten Recovery-/Rollback-Pfad auslösen, sofern die Plattform dies sicher unterstützt. Der Rollback darf keine widerrufenen Schlüssel, verbotenen Downgrades oder ungeprüften Artefakte legitimieren. Zustandstransitionen müssen atomar oder crash-konsistent sein.

### 7.6 Wallet-Sicherheit

- Signieroperationen benötigen eine unabhängig geprüfte Identität, Capability und Policy.
- Ein Aufrufer darf weder durch selbst gelieferten `Grant` noch durch frei gewählte Rollen seine Autorisierung begründen.
- Private Schlüssel und Recovery-Material bleiben hinter dem autoritativen Schlüssel-/Wallet-Service.
- Signierabsicht, Ziel, Netzwerk/Chain-Domain und Transaktionsdaten sind vor dem Signieren zu validieren.
- Audit-Daten dürfen Schlüssel, Seed-Phrasen, vollständige geheime Payloads oder sensible Authentisierungsdaten nicht enthalten.

### 7.7 IPC und Dienste

Jeder sicherheitsrelevante IPC-Aufruf muss Quelle, Ziel, Operation, Capability, Schema/Version und Größen-/Ressourcenlimits validieren. Unbekannte Nachrichtenversionen, ungültige Handles, übergroße Payloads und fehlende Berechtigungen werden abgelehnt. Der Security Center Service darf keinen alternativen Weg um das Kernel-Enforcement eröffnen.

### 7.8 Audit und Security Posture

Sicherheitsereignisse sollen strukturierte, stabile Ereignis-IDs, Zeit-/Sequenzbezug, auslösenden Dienst, Aktion, Entscheidung und Begründungscode enthalten. Geheimnisse und unnötige personenbezogene Daten sind auszuschließen. Manipulationserkennung, Aufbewahrung, Zugriff und Export sind zu definieren. Wenn Audit-Verfügbarkeit für eine Operation vorgeschrieben ist, muss diese Operation bei Audit-Ausfall fehlschlagen.

Der aggregierte Sicherheitsstatus MUSS zwischen `PASS`, `FAIL`, `UNKNOWN`, `PENDING` und `NOT_APPLICABLE` unterscheiden. Fehlende Daten dürfen nie stillschweigend als PASS interpretiert werden.

## 8. Schnittstellenvertrag (konzeptionell)

Die folgenden Signaturen sind Designverträge und keine Behauptung, dass diese APIs bereits im Repository existieren:

- `evaluate_policy(context) -> Decision`
- `authorize_operation(identity, capability, operation, object) -> AuthorizationResult`
- `verify_artifact(metadata, exact_bytes, trust_snapshot) -> VerifiedArtifact | VerifyError`
- `stage_verified_artifact(verified_artifact) -> StagingResult`
- `activate_verified_update(staged_update) -> ActivationResult`
- `revoke_key(key_id, authenticated_revision) -> RevocationResult`
- `emit_security_event(event) -> AuditResult`
- `get_security_posture() -> SecurityPosture`

Typisierte Fehlerklassen sollen mindestens `InvalidMetadata`, `DigestMismatch`, `InvalidSignature`, `UntrustedSigner`, `RevokedSigner`, `ExpiredTrustSnapshot`, `DowngradeRejected`, `PolicyDenied`, `CapabilityDenied`, `ArtifactChanged`, `AuditUnavailable`, `StagingFailed` und `ActivationFailed` abdecken. Fehler dürfen nicht in generische Erfolgspfade umgewandelt werden.

## 9. Mindestbedrohungen und Negativtests

| ID | Angriff / Fehlerfall | Erwartetes Ergebnis |
|---|---|---|
| GSC-TH-001 | Artefakt nach Signaturprüfung geändert | Installation blockiert; Digest-Abweichung protokolliert |
| GSC-TH-002 | Fehlende oder falsch formatierte Metadaten | Fail closed |
| GSC-TH-003 | Unbekannter Signierschlüssel | Fail closed |
| GSC-TH-004 | Widerrufener Schlüssel / veralteter Trust-Stand | Fail closed gemäß Widerrufs- und Offline-Policy |
| GSC-TH-005 | Signatur gültig, aber falsche Paket-/Versionsdaten | Fail closed |
| GSC-TH-006 | Downgrade auf verbotene Version | Fail closed |
| GSC-TH-007 | IPC-Request mit gefälschtem Grant | Keine Autorisierung |
| GSC-TH-008 | Capability fehlt oder wurde widerrufen | Operation abgelehnt |
| GSC-TH-009 | Audit-Dienst fällt bei auditpflichtiger Operation aus | Operation blockiert |
| GSC-TH-010 | Update-Abbruch oder Stromausfall in jeder Transition | Kein ungeprüfter/inkonsistenter Zustand |
| GSC-TH-011 | Wallet-Request versucht Schlüssel oder Grant einzuschleusen | Ablehnung; kein Geheimnisverlust |
| GSC-TH-012 | Unbekannter Policy-Status oder fehlerhafte Antwort | UNKNOWN/DENY, niemals PASS |

Tests müssen gültige Pfade und negative Fälle abdecken. Wo möglich sind Property-/Fuzz-Tests für Parser, Metadaten, IPC-Schemas und Zustandsmaschinen zu ergänzen.

## 10. CI- und Validierungsvertrag

Die dokumentierten Workspace-Basiskommandos sind:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Zusätzliche Release-Gates müssen Abhängigkeits-/Advisory-Prüfung, Signatur- und Widerrufs-Negativtests, Paketinstallations-Integration, Update-/Recovery-Tests, Kernel-/IPC-Autorisierung und Geheimnis-Leak-Prüfungen umfassen. Jeder CI-Nachweis ist an den vollständigen Commit-SHA, Workflow-Run, Job, Step und Exit-Status zu binden. Ein historischer Run oder ein anderer Head-SHA zählt nicht als Nachweis für den aktuellen Stand.

Kein Gate darf zur Herstellung eines grünen Builds entfernt, abgeschwächt oder übersprungen werden, ohne formelle Risikoentscheidung und dokumentierte Ausnahme.

## 11. Verbindliche Abnahmekriterien

| ID | Kriterium | Freigabebedingung |
|---|---|---|
| GSC-AC-001 | Alle Installations-Einstiegspunkte identifiziert | Code- und Call-Site-Inventar geprüft; keine Umgehung |
| GSC-AC-002 | Exakte Artefaktbytes verifiziert | Verifier-/Installer-Bindung und TOCTOU-Negativtest bestanden |
| GSC-AC-003 | Signatur-/Digestfehler blockieren | Automatisierte Negativtests bestanden |
| GSC-AC-004 | Trust Store und Widerruf abgesichert | Rotation, Replay, Widerruf und Offline-Verhalten getestet |
| GSC-AC-005 | Kernel-Capability-Enforcement belegt | Manipulierte/fehlende Grants werden an der autoritativen Grenze abgewiesen |
| GSC-AC-006 | IPC-Grenzen geprüft | Schema-, Handle-, Capability- und Größen-Negativtests bestanden |
| GSC-AC-007 | Wallet-Autorisierung belegt | Kein selbst deklarierter Grant; keine Schlüssel-Leaks |
| GSC-AC-008 | Update und Recovery geprüft | Zustandsübergänge, Abbruch und Rollback reproduzierbar getestet |
| GSC-AC-009 | Audit ist datensparsam und belastbar | Geheimnis-Leak-Tests und Manipulations-/Ausfallverhalten geprüft |
| GSC-AC-010 | Policy ist deterministisch und fail-closed | UNKNOWN/Fehlerfälle ergeben keinen Allow |
| GSC-AC-011 | Release-CI ist grün | Alle erforderlichen Gates auf exakt dem freizugebenden SHA erfolgreich |
| GSC-AC-012 | TCB- und Servicegrenzen dokumentiert | Architektur- und Code-Review abgeschlossen |
| GSC-AC-013 | Unabhängiges Security Audit | Findings klassifiziert; blockierende Findings geschlossen oder formell akzeptiert |
| GSC-AC-014 | Hardware-/Boot-Evidenz wo erforderlich | Reproduzierbare Plattform-/QEMU-/Hardwarebelege, getrennt nach jeweiligem Kriterium |

Ein Kriterium darf nur mit verlinkter Evidenz als erfüllt markiert werden. Nicht untersuchte Punkte bleiben `UNKNOWN` oder `PENDING`, nicht PASS.

## 12. Umsetzung in Phasen

1. **Phase 0 — Inventar und Baseline:** alle Installationspfade, Schlüsselquellen, Capability- und Wallet-Call-Sites sowie bestehende Tests dokumentieren; PR #84 und aktuelle CI anhand des exakten Heads prüfen.
2. **Phase 1 — Paket-Trust:** Trust-Store, Widerruf, exakte Bytes und verbindliche Installer-Integration fertigstellen; negative Integrationstests hinzufügen.
3. **Phase 2 — Kernel/IPC/Identity/Wallet:** Autorisierungsgrenzen prüfen, gefälschte Grants ausschließen und Widerruf wirksam machen.
4. **Phase 3 — Update/Audit/CI:** crash-konsistente Update-/Recovery-Pfade, datensparsames Audit und verpflichtende Release-Gates ergänzen.
5. **Phase 4 — Release-Kandidat:** vollständige SHA-gebundene CI-Evidenz, unabhängiges Audit und erforderliche Hardware-/Boot-Nachweise zusammentragen.

## 13. Governance, Änderungen und Freigabe

Änderungen an Trust-Root-Format, Signaturdomäne, Policy-Semantik, Kernel-Capability-Modell, Wallet-Autorisierung oder Update-Rollback-Verhalten sind sicherheitskritische Architekturänderungen und benötigen explizite Review- und Migrationsevidenz. Sicherheitsausnahmen müssen begründet, befristet, verantwortet und auditierbar sein.

**Aktueller Status dieser Spezifikation:** DRAFT.  
**Aktueller Produktionsstatus des Projekts:** NOT_READY.  
Diese Dokumentation implementiert keine Sicherheitskontrolle und bestätigt keine CI-Ergebnisse. Eine Freigabe ist erst nach erfüllten, evidenzgebundenen Abnahmekriterien zulässig.
