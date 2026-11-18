---
standard:
  id: ATC-OS-SEC-UPDATE-001
  title: "Blockchain Automatic Security Updates — Pflicht zur automatisierten, kryptografisch abgesicherten Sicherheitsaktualisierung"
  version: "1.0.0"
  status: candidate
  category: security
  authority: A-TownChain Ecosystems
  owner: "Michael Wroblewski (Owner-Entwurf 18.11.2026)"
  created: "2026-11-18"
  updated: "2026-11-18"
  normative: true
  applies_to: "Alle sicherheitskritischen Blockchain-Komponenten von KAI-OS / GlobusOS"
  supersedes: []
  superseded_by: null
  effective_date: null
  review_date: "2027-05-18"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-OS-SEC-UPDATE-001 — Blockchain Automatic Security Updates (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE. Owner-Entwurf vom 18.11.2026, als verbindliche
> Sicherheitsrichtlinie vorgesehen. Normativ wirksam mit der §9-Freigabe nach
> ATC-STD-000; bis dahin gilt der Standard als Entwurf mit Anwendungspflicht fuer
> neue Komponenten, aber ohne Bestandsschutz-Wirkung.

---

## Abstract

Sicherheitsluecken in Blockchain-Komponenten duerfen nicht davon abhaengen, dass ein
Betreiber zur richtigen Zeit manuell patcht. Dieser Standard macht die automatische
Sicherheitsaktualisierung zur **Pflicht** und zieht gleichzeitig eine harte Grenze:
**Automatische Aktualisierung bedeutet nicht, dass ungepruefter Code automatisch
ausgefuehrt werden darf.** Jedes Update muss authentifiziert, auf Integritaet geprueft,
kompatibel und autorisiert sein, bevor es laeuft. Der Updateprozess erhaelt niemals
Zugriff auf Wallet-Schluessel, Seed-Phrasen oder ATC-Bestaende.

## 1. Zweck (Purpose)

Der Standard legt fest:

1. **welche** Komponenten automatisch Sicherheitsupdates erhalten (REQ-BSU-001),
2. **wie** ein Update authentifiziert, geprueft und eingespielt wird (REQ-BSU-004..007),
3. **was** der Updateprozess niemals beruehren darf (REQ-BSU-009),
4. **wer** entscheiden darf (BSUA gegenueber ShivaCore, REQ-BSU-010),
5. **wie** ein Fehlschlag behandelt wird, ohne eine bekannte Schwachstelle wieder zu
   oeffnen (REQ-BSU-008, REQ-BSU-014, REQ-BSU-015).

## 2. Geltungsbereich (Scope)

Gilt fuer alle sicherheitskritischen Blockchain-Komponenten von KAI-OS und GlobusOS,
insbesondere:

- A-TownChain Node und Konsenskomponenten
- Transaktionsvalidierung
- Mempool
- Kryptografische Bibliotheken
- Signatur- und Schluesselverwaltungsfunktionen
- Wallet-Sicherheitskomponenten
- Blockchain-Kommunikation und P2P-Netzwerk
- ATC-Staking und Validator-Software
- Persistenz- und Datenintegritaetskomponenten

Nicht im Geltungsbereich: reine Anwendungs- und UI-Pakete ohne Kettenbezug; fuer sie
gilt ATC-STD-UPDATE-001.

## 3. Begriffe und Definitionen

| Begriff | Bedeutung |
|---|---|
| **BSUA** | Blockchain Security Update Authority. Fuehrt Updates aus und prueft Signaturen, entscheidet aber nicht ueber Rechte. |
| **ShivaCore Security Authority** | Einzige Instanz fuer Rechte- und Systemaenderungsentscheidungen. |
| **Sicherheitsupdate** | Update, das eine bekannte Schwachstelle (CVE/GHSA/ATC-VULN) schliesst oder eine Integritaetsluecke behebt. |
| **Funktionsupdate** | Update mit neuem Verhalten ohne Sicherheitsbezug. |
| **Protokollupdate** | Aenderung an Konsensregeln, Transaktionsformat, State-Transition oder Gaspraechnung. |
| **Koordinierte Aktivierung** | Protokollupdates werden zu einem vorab festgelegten Aktivierungspunkt (Hoehe oder Zeit) netzweit gleichzeitig wirksam, nie einzeln pro Validator. |
| **Freigegebenes Update** | Update mit gueltiger Signatur eines autorisierten Signierers, unveraendertem Artefakt und zulaessiger Version. |
| **Sicherer Wiederherstellungsmodus** | Zustand, in dem ein Knoten nicht am Konsens teilnimmt, statt eine bekannt unsichere Version zu starten. |

## 4. Normative Anforderungen

### 4.1 Umfang und Prioritaet

- **REQ-BSU-001** — Jede Komponente aus §2 MUSS einen automatisierten,
  kryptografisch abgesicherten Updatepfad besitzen. Ein rein manueller Pfad ist
  unzulaessig.
- **REQ-BSU-002** — Sicherheitsupdates MÜSSEN gegenueber Funktionsupdates
  priorisiert werden. Bei Konflikt gilt: Sicherheit vor Funktionsumfang.
- **REQ-BSU-003** — Die Updateklasse MUSS je Komponente festgelegt und dokumentiert
  sein (Tabelle in §5). Die Klasse bestimmt das Verhalten, nicht der Betreiber.

### 4.2 Updateklassen

| Blockchain-Komponente | Updateverhalten |
|---|---|
| Konsens-Sicherheitskorrekturen | Automatisch, priorisiert |
| Transaktionsvalidierung | Automatisch |
| Kryptografische Bibliotheken | Automatisch |
| P2P-Sicherheitskomponenten | Automatisch |
| Mempool-Sicherheitskorrekturen | Automatisch |
| Wallet-Sicherheitskomponenten | Automatisch |
| Validator-Sicherheitskorrekturen | Automatisch |
| Neue Blockchain-Funktionen | Kontrolliertes Update |
| Protokollaenderungen | Koordinierte Aktivierung |

Die Klassen sind maschinenlesbar hinterlegt in
`components/update-authority/policy/update-classes.yaml`. Diese Datei ist die
ausfuehrbare Form der Tabelle; sie wird nur zusammen mit diesem Standard geaendert.

- **REQ-BSU-011** — Protokoll- und Konsensaenderungen DUERFEN NICHT auf einem
  einzelnen Validator zu einem beliebigen Zeitpunkt aktiviert werden. Die Aktivierung
  MUSS ueber einen koordinierten Aktivierungspunkt mit vorab veroeffentlichter Hoehe
  oder Zeit erfolgen, damit keine Netzwerkgabel durch gemischte Protokollversionen
  entsteht.

### 4.3 Schutzmechanismen

- **REQ-BSU-004 (Authentifizierung)** — Nur kryptografisch autorisierte Updates
  DUERFEN akzeptiert werden. Die Signatur MUSS ueber die kanonischen Bytes des
  Manifests geprueft werden und einer eigenen Domain zugeordnet sein
  (`ATC-UPDATE-v1`), damit eine Signatur aus einem anderen Kontext (Admission, DID,
  Transaktion) nicht als Update-Signatur durchgeht.
- **REQ-BSU-005 (Integritaetspruefung)** — Vor dem Einspielen MUESSEN Signatur,
  SHA-256 des Artefakts, Groesse, Version und Herkunft geprueft werden. Weicht der
  beobachtete Hash vom signierten Hash ab, MUSS das Update abgelehnt werden.
- **REQ-BSU-006 (Automatische Installation)** — Freigegebene Sicherheitsupdates
  MUESSEN ohne Nutzerinteraktion installiert werden. Ein Betreiber darf nicht der
  Engpass fuer einen Patch sein.
- **REQ-BSU-007 (Funktionspruefung)** — Nach dem Einspielen MUESSEN Node-,
  Wallet-Schnittstellen- und Konsensfunktionen geprueft werden. Erst ein
  bestandener Health-Check macht das Update "erfolgreich".
- **REQ-BSU-008 (Rollback und Wiederherstellung)** — Bei fehlerhafter Installation
  MUSS der vorherige funktionsfaehige Zustand wiederherstellbar sein. Rollback ist
  jedoch NICHT unbedingt zulaessig: Wuerde der Rueckschritt eine bekannte kritische
  Schwachstelle erneut oeffnen, MUSS statt Rollback der sichere
  Wiederherstellungsmodus greifen (REQ-BSU-014).

### 4.4 TUF-Metadaten

- **REQ-BSU-012** — Die Updateinfrastruktur MUSS die Schutzprinzipien des
  Update Framework (TUF) umsetzen: getrennte Rollen `root`, `targets`, `snapshot`,
  `timestamp`, Schwellenwerte fuer Signaturen, Ablaufdaten je Metadatendatei,
  Schluesselrotation ueber `root` sowie Schutz gegen kompromittierte
  Signaturschluessel.
- **REQ-BSU-013** — Die Infrastruktur MUSS Schutz gegen Replay und Downgrade
  bieten: Ein bereits installierter Stand wird nicht erneut eingespielt, ein
  Rueckschritt ist nur mit ausdruecklicher Freigabe (Manifest-Flag **und**
  Autoritaets-Allowlist) zulaessig. Veraltete Repository-Metadaten werden abgelehnt
  (Ablaufpruefung).

### 4.5 Trennung von Vermoegenswerten

- **REQ-BSU-009** — Automatische Updates DUERFEN folgende Daten NIEMALS veraendern,
  lesen oder an Dritte uebertragen: private Wallet-Schluessel, Seed-Phrasen,
  Wallet-Bestaende, Staking-Zuordnungen, Transaktionshistorie,
  Validator-Identitaeten. Ein kompromittierter Updateprozess darf keinen Zugriff auf
  Wallet-Schluessel erhalten. Software ist updatefaehig, Schluesselmaterial ist es
  nicht.

### 4.6 Zuständigkeit

- **REQ-BSU-010** — Die BSUA ueberwacht installierte Versionen, erkennt kritische
  Sicherheitsupdates, fuehrt sie aus, prueft Signaturen und Vertrauen, ueberwacht die
  Aktualitaet, dokumentiert alle Vorgaenge und alarmiert bei nicht aktualisierbaren
  Nodes. Sie ist KEINE zweite Kernel-Autoritaet: die Rechteentscheidung verbleibt
  bei ShivaCore.

### 4.7 Fehlerverhalten

- **REQ-BSU-015** — Das System MUSS sich gemaess §8 verhalten. Jede Abweichung ist
  ein Finding nach ATC-STD-BUG-001.
- **REQ-BSU-014 (Quarantaene)** — Nach einem fehlgeschlagenen Einspielen MUSS die
  Komponente unter Quarantaene gestellt werden; weitere Updates dieser Komponente
  werden abgelehnt, bis die Quarantaene ausdruecklich aufgehoben wird.
- **REQ-BSU-019 (Fail-closed)** — Jeder unklare Zustand ist eine Ablehnung, niemals
  eine Freigabe. Kein Trust-on-first-use: unbekannte Komponenten und unbekannte
  Signierer werden abgelehnt, nicht registriert.

### 4.8 Nachweise und Betrieb

- **REQ-BSU-016** — Nicht aktualisierbare oder dauerhaft blockierte Nodes MUESSEN
  alarmiert und der Sicherheitszustand dokumentiert werden.
- **REQ-BSU-017** — Jede Update-Entscheidung MUSS genau einen revisionsfaehigen
  Audit-Eintrag erzeugen, auch bei Ablehnung (Komponente, Version, Ergebnis, Grund,
  Signierer, Zeitpunkt).
- **REQ-BSU-018** — Die BSUA MUSS die Aktualitaet installierter Blockchain-Versionen
  fortlaufend ueberwachen und Abweichungen von der Freigabeliste melden.
- **REQ-BSU-020** — Ausnahmen sind nur als befristete, begruendete und
  revisionsfaehige Ausnahme zulaessig (ATC-STD-000 §8); sie sind nie permanent.

## 5. Referenz-Durchsetzung

Die Pruefordnung ist ausfuehrbar implementiert in
`components/update-authority` (Rust, Crate `atc-update-authority`): reine
Entscheidungslogik ohne Netzwerk, ohne Schreibzugriff, ohne Signaturberechnung.

| Pruefschritt | Code | Anforderung |
|---|---|---|
| 1 Manifest lesbar | `UpdCode::ManifestUnreadable` | REQ-BSU-005 |
| 2 Komponente bekannt | `UpdCode::UnknownComponent` | REQ-BSU-019 |
| 3 keine Quarantaene | `UpdCode::Quarantined` | REQ-BSU-014 |
| 4 Groesse plausibel | `UpdCode::SizeInvalid` | REQ-BSU-005 |
| 5 Hash stimmt | `UpdCode::IntegrityMismatch` | REQ-BSU-005 |
| 6 Signatur gueltig | `UpdCode::SignatureInvalid` | REQ-BSU-004 |
| 7 nicht abgelaufen | `UpdCode::Expired` | REQ-BSU-013 |
| 8 Boot-ABI ausreichend | `UpdCode::BootAbiTooOld` | REQ-BSU-007 |
| 9 Version zulaessig | `UpdCode::VersionRejected` | REQ-BSU-013 |
| 10 nicht widerrufen | `UpdCode::Revoked` | REQ-BSU-012 |
| 11 akzeptiert | `UpdCode::Accepted` | REQ-BSU-006 |

Die Kryptografie liegt bewusst NICHT im Crate: die Signaturpruefung kommt ueber den
Trait `SignatureVerifier` aus der Runtime. Das Crate behauptet keine Krypto, es
erzwingt Reihenfolge und Entscheidung.

## 6. Schutzgrenzen (Trust Boundaries)

| Einheit | Darf | Darf nicht |
|---|---|---|
| Blockchain Update Manager (BSUA) | Software aktualisieren, Signaturen pruefen, Versionen ueberwachen | Schluessel lesen, Rechte erteilen, Kernel-Autoritaet sein |
| ShivaCore Security Authority | Systemaenderungen pruefen und autorisieren | Software-Updates selbst einspielen |
| Software | aktualisiert werden | Schluesselmaterial enthalten oder veraendern |
| Wallet-Schluessel | separat geschuetzt, ausserhalb des Updatepfads | vom Updateprozess erreichbar sein |

## 7. Compliance und Pruefungen

| Gate | Pruefung | Nachweis |
|---|---|---|
| GATE-BSU-01 | Updatepfad vorhanden und automatisiert | Konfiguration je Komponente |
| GATE-BSU-02 | Signatur- und Hashpruefung aktiv | Test mit manipuliertem Artefakt |
| GATE-BSU-03 | Anti-Replay und Anti-Downgrade | Test mit gleicher und aelterer Version |
| GATE-BSU-04 | Quarantaene nach Fehlschlag | Test mit fehlerhaftem Apply |
| GATE-BSU-05 | Rollback oder sicherer Wiederherstellungsmodus | Test mit bekannt unsicherer Vorgaengerversion |
| GATE-BSU-06 | Kein Schluesselzugriff im Updatepfad | Rechte- und Pfadpruefung |
| GATE-BSU-07 | Audit-Eintrag je Entscheidung | Audit-Log-Nachweis |
| GATE-BSU-08 | Koordinierte Protokollaktivierung | Aktivierungsplan mit Hoehe/Zeit |

## 8. Verhalten bei fehlgeschlagenen Updates

| Situation | Systemreaktion |
|---|---|
| GitHub nicht erreichbar | Wiederholungsversuche, alternative Quelle |
| Signatur ungueltig | Update ablehnen |
| Paket manipuliert | Update blockieren und Sicherheitsalarm |
| Update fehlerhaft | Rollback |
| Kritische Sicherheitsluecke | Update priorisieren, Sicherheitsmodus nach Richtlinie |
| Protokollversion inkompatibel | Keine unkoordinierte Aktivierung |
| Update dauerhaft nicht verfuegbar | Betreiber warnen und Sicherheitszustand dokumentieren |
| Rollback wuerde kritische Schwachstelle oeffnen | Kein Rollback, sicherer Wiederherstellungsmodus |

## 9. Ausnahmen

Ausnahmen nach ATC-STD-000 §8: befristet, begruendet, mit Owner-Genehmigung und
Ablaufdatum. Eine Ausnahme von REQ-BSU-009 (Schluesseltrennung) ist unzulaessig.

## 10. Referenzen (References)

### NORMATIVE Referenzen

- ATC-STD-000 — Governance Root
- ATC-STD-UPDATE-001 — Change Control fuer Artefakt-Updates
- ATC-STD-020 — Vulnerability Management (VULN-Records, ACCEPT/MITIGATE)
- ATC-STD-022 — Security Patch Management (SLA-Pfade)
- ATC-STD-033 — Vulnerability Intelligence · ATC-STD-036 — Emergency Security Change
- ATC-STD-041 — Reproducible Builds · ATC-STD-043 — Artifact Integrity
- ATC-GATE-SEC-001 — Security Meta-Gate
- ATC-STD-IMPLEMENTATION-001 — Implementierungsstatus

### INFORMATIVE Referenzen

- The Update Framework (TUF) — Metadaten- und Rollenmodell
- NIST SSDF (SP 800-218) — sichere Software-Lieferkette
- `components/update-authority` — Referenz-Durchsetzung der Pruefordnung

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED (CANDIDATE, §9-Freigabe offen) |
| Referenz-Implementierung | `components/update-authority` — Entscheidungslogik mit 17 Tests, HEAD-gebunden nachzuweisen |
| Nicht implementiert | TUF-Metadaten-Rollen, Health-Check, Rollback-Orchestrierung, koordinierte Aktivierung (Gates BSU-05..08) |

## Security Considerations

- Die Trennung von Updatepfad und Schluesselmaterial ist die zentrale
  Schutzmassnahme: ein kompromittierter Updateprozess darf keine ATC-Werte
  erreichen (REQ-BSU-009).
- Kein Trust-on-first-use und Fail-closed verhindern, dass ein unbekannter Signierer
  durch bloßes Auftauchen vertrauenswuerdig wird.
- Domain-Trennung der Signatur (`ATC-UPDATE-v1`) verhindert Cross-Context-Replay.
- Keine Zugangsdaten in Manifesten, Audit-Logs oder Update-Artefakten; Secrets nur
  ueber Umgebungsvariablen.

## Changelog

### 1.0.0 — 2026-11-18

- Initial Release (Owner-Entwurf Michael Wroblewski, 18.11.2026)
- 20 normative Anforderungen (REQ-BSU-001..020), 8 Gates (GATE-BSU-01..08)
- Updateklassen-Matrix, fuenf Schutzmechanismen, Trust-Boundary-Modell,
  Fehlerverhalten-Matrix inklusive sicherem Wiederherstellungsmodus
- Referenz-Durchsetzung auf `components/update-authority` abgebildet

## References

NORMATIV: ATC-STD-000, ATC-STD-UPDATE-001, ATC-STD-020, ATC-STD-022,
ATC-STD-033, ATC-STD-036, ATC-STD-041, ATC-STD-043, ATC-GATE-SEC-001,
ATC-STD-IMPLEMENTATION-001
INFORMATIV: TUF, NIST SP 800-218, `components/update-authority`
