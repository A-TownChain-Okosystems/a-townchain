# KAI-OS-READINESS-001 — Reifegradkatalog Betriebssystem, Ausführungsplattform und Node (KANDIDAT)

> **Status:** `CANDIDATE`, nicht registriert (atc-standards main ist branchgeschützt, PR blockiert durch unbestätigte Committer-Adresse)
> **Datum:** 11.11.2026 · Rev. 2 (Rev. 2: Kernel-SSOT und IPC-Modul eingeordnet, gemeinsamer Registrierungsblocker, Regel der getrennt prüfbaren Admission-Schritte, Architekturgrenzen)
> **Evidenzbasis:** `globus-os` main `4fccb823` (Commit 2026-11-11T00:23Z), `a-townchain` main `c2785d649`
> **Methode:** Lesen der Repos über GitHub-API und Baumlisten, kein Ausführen, kein Bauen, keine Messung
> **Zweck:** verhindern, dass ein funktionierender Kernel als fertiges Betriebssystem oder ein laufender Devnet-Node als Mainnet-Fähigkeit gilt

---

## 0. Warum dieser Katalog existiert

Die bisherige Bewertung war zu weit gefasst. „Technisch korrekt" ist eine Aussage über
einzelne Komponenten. „Einsatzbereit" ist eine Aussage über das Zusammenspiel. Zwischen
beiden liegt eine Kette von Abhängigkeiten, die man einzeln prüfen muss.

Dieser Katalog macht drei Dinge:

1. Er definiert die Statuswörter, damit sie nicht mehr mehrdeutig verwendet werden.
2. Er ordnet jeder Komponente eine single source of truth (SSOT) zu, ihre Abhängigkeiten,
   ihren Stand, den erforderlichen Negativtest und das Freigabe-Gate.
3. Er hält fest, welche Aussagen heute durch welche Evidenz gedeckt sind und welche nicht.

---

## 1. Statusvokabular (verbindlich)

| Status | Bedeutung | Zulässige Grundlage |
|---|---|---|
| `IMPLEMENTED` | Code existiert und ist baubar | Bau in frischem Klon, Commithash |
| `TESTED` | Testlauf lokal durchgeführt, Ergebnis dokumentiert | Testausgabe plus Commithash, Datum |
| `CI_VERIFIED` | Workflow grün auf dem Commit | Run-ID, Name, Ergebnis |
| `E2E_VERIFIED` | Vollständiger Pfad über alle beteiligten Komponenten | Lauf mit Protokoll, verkettete Zustandswurzeln |
| `RESIDUAL` | Teilweise vorhanden, für den Zweck nicht ausreichend | Fehlliste mit Datei- oder Symbolnachweis |
| `BLOCKED` | Voraussetzung fehlt vollständig; kein Pfad vorhanden | Nachweis des Fehlens (Baumliste, Abhängigkeiten) |
| `NOT TESTED` | Keine Aussage möglich, weil nie geprüft | Grund der Nichtprüfbarkeit |
| `PENDING EVIDENCE` | Behauptung existiert, Nachweis fehlt oder ist veraltet | letzter bekannter Nachweis mit Datum |

**R-1 (MUSS)** Ein Status wird nur mit benannter Evidenz vergeben: Commit, Datum, Prüfweg.
**R-2 (MUSS)** `BLOCKED` wird nie durch `NOT TESTED` ersetzt und umgekehrt. Fehlende
Prüfung ist keine Freigabe.
**R-3 (MUSS)** Ergebnisse werden nicht über Systemgrenzen hochgerechnet. Ein grüner
Kerneltest ist kein Nachweis für einen Node und umgekehrt.
**R-4 (MUSS)** `CI_VERIFIED` ersetzt kein `E2E_VERIFIED` und umgekehrt.
**R-5 (MUSS)** Ein Statuswechsel trägt Commithash und Prüfdatum; ältere Nachweise bleiben
mit ihrem Datum stehen.
**R-6 (MUSS)** Eine fachliche Einordnung ohne Einsicht in den Kandidaten oder die
Repository-Dateien ist kein Status. Verbindlich sind ausschließlich die Zeilen dieses
Katalogs; jede abweichende Tabelle ist Diskussionsbeitrag, nicht Freigabe.

---

## 2. Komponentenmatrix

| # | Komponente | SSOT | Abhängigkeiten | Stand | Status | Evidenz (11.11.2026) |
|---|---|---|---|---|---|---|
| 1 | Kernel (ShivaCore) | `globus-os` `modules/atc-shivacore/kernel` | Untyped-Speicher, Paging, ACPI | 48 Dateien in `kernel/src`, 733 Tests | `TESTED` (QEMU) | `4fccb823`, Testlauf laut Projektbericht |
| 2 | Boot BIOS/UEFI | `globus-os` `modules/atc-shivacore/boot` | Kernel, Ramdisk | beide Images booten bis Scheduler | `TESTED` (QEMU) | `4fccb823`, Ramdisk-Start bestätigt |
| 3 | Prozessisolation | Kernel `paging`, `scheduler` | CR3-Wechsel je Prozess | eigener Adressraum je Prozess | `TESTED` | Gegenprobe: Fremdzugriff endet im Seitenfehler, andere Prozesse laufen weiter |
| 4 | Fehlerpfade | Kernel `fault`, `syscall` | IDT, Ring 3 | Page Fault, Protection Violation, Invalid Opcode behandelt | `TESTED` | Fehler beendet den Prozess, nicht die Maschine |
| 5 | Capabilities / Syscall-ABI 2.0 | Kernel `capability.rs` | Prozessraum | Capability-Prüfung vorhanden | `IMPLEMENTED` | Modul vorhanden und im Syscall-Pfad wirksam; eine vollständige Admission ist damit **nicht** belegt |
| 6 | IPC-Modul (Syscall-Seite) | Kernel `ipc.rs` | Capability-Prüfung | Modul vorhanden | `IMPLEMENTED` | Rechteprüfung vor der Zustellung; `IpcReceive` meldet fehlenden Transport statt Bytes zu behaupten |
| 6a | IPC-Transport / Service Space im Image | `globus-os` `service_space` | Ramdisk, ELF-Loader, Init | kein Transport | `BLOCKED` | `service_space` ist keine Abhängigkeit des Kernel-Crates; die Ramdisk trägt kein Programm, das ihn startet |
| 7 | Persistenz / Block-Device | **keine** | – | kein Treiber im Baum | `BLOCKED` | 48 `kernel/src`-Dateien, kein `ata`, `ahci`, `nvme`, `virtio-blk`, `block` |
| 8 | Dateisystem | Kernel `vfs.rs`, `atcfs.rs` | Block-Device (7) | Struktur vorhanden | `RESIDUAL` | Quelle für Bytes fehlt, damit ist `vfs`/`atcfs` nicht nutzbar |
| 9 | `globus-init` | **keine** | Dateisystem (8), Prozessstart | nicht vorhanden | `BLOCKED` | kein Pfad im Baum; was läuft, ist `init_program` aus der Ramdisk, gestartet vom Kernel |
| 10 | Service-Lifecycle | – | `globus-init` (9) | nicht vorhanden | `BLOCKED` | ohne Init kein Start, Stopp, Neustart, Abhängigkeitsordnung |
| 11 | Code-Admission im Betriebssystem | **keine** | Kernel, Artefaktformat | kein Durchsetzungspunkt | `BLOCKED` | im Kernel kein Modul für `tpm`, `attest`, `measure`, `verify`, `sign`; der ELF-Loader prüft weder Signatur noch Herkunft |
| 11a | Admission-Entscheidungslogik (Userspace) | `a-townchain` `components/exec-admission` | Artefaktformat, Signaturprüfer | 21 Tests, 12 unterscheidbare Fehlercodes | `IMPLEMENTED` | Stufen liefern getrennte Codes (AuthorityUnavailable, CapabilityNotGranted, ComponentNotAllowed, ExpiredOrRolledBack, IntegrityMismatch, IsolationUnavailable…); genutzt von `components/system` |
| 11b | Durchsetzungspunkt (Enforcement) | **keine** | 11a, Kernel, Bootpfad | keiner vorhanden | `BLOCKED` | keine Aufrufstelle vor dem ersten Instruktionsabruf; ein grünes Entscheidungsergebnis ohne Aufrufer verhindert nichts |
| 12 | Secure Update / Rollback | **keine** | Admission (11b), Persistenz (7) | nicht vorhanden | `BLOCKED` | keine Updatekomponente im Baum |
| 13 | Attestation / Measured Boot | **keine** | Messkette, TPM oder gleichwertig | nicht vorhanden | `BLOCKED` | siehe 11, keine Messstelle |
| 14 | Kernel-SSOT | `globus-os` | – | Monorepo ohne Kernelkopie | `IMPLEMENTED` | `a-townchain` Baum: 13 487 Dateien, 0 Treffer für `kernel/globus-os/` oder `kernel/kai-os/`; Doku-Drift im Spiegel ist davon getrennt und keine zweite Implementierungswahrheit |
| 15 | ATC-VM (Crate) | `a-townchain` `components/vm` | Gasmodell, Fehlercodes | Workspace-Member vorhanden | `PENDING EVIDENCE` | letzter grüner Lauf 09./10.11., heute nicht erneut ausgeführt |
| 16 | Node-Transaktionspipeline | `a-townchain` `components/node` | VM (15), System, Konsens | speichert Transaktionen ohne Ausführung | `BLOCKED` | Node-Abhängigkeiten: `atc-algorithm`, `atc_wallet`, `secp256k1`, `serde`, `serde_json`, `sha2`; keine VM, kein Zustandsspeicher |
| 17 | Node-Konsens / Persistenz | `a-townchain` `components/node` | Kette im Speicher | Devnet-Konsens im Prozess | `RESIDUAL` | Kette nur im `Vec`, Grenze 64 Blöcke, kein Storage-Dep |
| 18 | Hashstandard für Konsensflächen | `a-townchain` `standards/atc/candidates/ATC-HASH-002-CANDIDATE.md` | SHA-256, Domain Separation | Rev. 3.2 auf main | `PENDING REGISTRY` | Registrierung blockiert (Branchschutz 409, PR 403) |
| 19 | Mainnet-Fähigkeit | – | 7, 8, 9, 11, 15, 16, 17, 18 | – | `BLOCKED` | Summe der Blocker, nicht eigene Komponente |
| 20 | Hardware-Reife | – | Kernel, Treiber | kein Test außerhalb QEMU | `NOT TESTED` | alle Nachweise sind QEMU |
| 21 | CI | – | Committer-Adresse | zahlreiche Workflows | `BLOCKED` | `startup_failure` in push-getriggerten Workflows (11.11.2026) |

**Zum CI-Status:** Eine unbestätigte E-Mail-Adresse ist ein Konto- und Infrastrukturproblem.
Sie ist kein Nachweis eines Kernelfehlers, aber ebenso wenig ein bestandener CI-Nachweis.
Bis zur Bestätigung bleibt jede Aussage auf lokale Läufe beschränkt (`TESTED`, nie
`CI_VERIFIED`).

---

## 2a. Gemeinsamer Registrierungsblocker (administrativ, nicht technisch)

Zwei Kandidaten warten auf denselben Vorgang. Das ist **ein** Blocker, nicht zwei Fehler.

| Kandidat | Technischer Stand | Registrierung | Ursache |
|---|---|---|---|
| `ATC-HASH-002` Rev. 3.2 | Normtext auf `a-townchain` main | `BLOCKED` | PR-Anlage 403, Committer-Adresse unbestätigt |
| `KAI-OS-READINESS-001` | Kandidat auf `a-townchain` main | `BLOCKED` | derselbe |

Belege: `atc-standards` main ist branchgeschützt (direktes Schreiben 409, „Changes must be made
through a pull request, 14 of 14 required status checks"). PR-Anlage liefert 403, „At least one
email address must be verified". Die push-getriggerten Workflows enden deshalb in
`startup_failure`, damit kann keiner der 14 Checks grün werden. Der Deadlock ist
administrativ und wird durch die Kontoverifizierung gelöst, nicht durch Code.

**B-1 (MUSS)** Nach der Verifizierung: beide vorhandenen Branches prüfen, je einen PR mit genau
der vorgesehenen Änderung, Checks dokumentieren, Review und Merge nach Branchschutz, danach
Merge-Commit und tatsächlichen Registereintrag auf `atc-standards/main` prüfen.
**B-2 (MUSS)** Keine vorgezogene Statusänderung auf `VERIFIED` oder `REGISTERED`. Ein grüner
CI-Lauf allein ist keine Registrierung.

## 3. Abhängigkeitskette

```
Kernel (1-4)
  ├─ Block-Device (7) ──> Dateisystem (8) ──> globus-init (9) ──> Service-Lifecycle (10)
  │                                                        └──> IPC-Transport (6)
  ├─ Code-Admission (11) ──> Update/Rollback (12)
  │                     └──> Attestation / Measured Boot (13)
  └─ Capabilities (5)

parallel, eigene Domäne:
ATC-VM (15) ──> Node-Transaktionspipeline (16) ──> State Root ──> Konsens/Block (17)
ATC-HASH-002 (18) ──> alle Konsensflächen (16, 17 und VM)

Mainnet (19) = 7 + 8 + 9 + 11 + 15 + 16 + 17 + 18 im Zusammenspiel, auf Hardware (20), mit CI (21)
```

**K-1 (MUSS)** Die Reihenfolge ist eine Abhängigkeitsordnung, keine Wunschliste. Eine
Komponente gilt erst als erledigt, wenn ihre Vorbedingung erledigt ist.
**K-2 (MUSS)** Die Node-VM-Integration läuft parallel, weil sie eine eigene Architekturdomäne
ist. Sie ist kein Ersatz für die Betriebssystemarbeit und umgekehrt.

---

## 4. Code-Admission: Anforderungen an den fehlenden Pfad

Der wichtigste offene Sicherheitspunkt. Prozessisolation schützt vor einem bereits laufenden
Prozess. Sie beantwortet nicht, wer überhaupt Code einbringen darf.

**A-1 (MUSS) — Reihenfolge der Prüfungen.** Sechs getrennte Schritte, keiner darf einen
anderen ersetzen:

| # | Prüfung | Frage |
|---|---|---|
| 1 | Herkunft | aus welchem Artefakt, von welchem Erzeuger, mit welchem Commithash |
| 2 | Signatur | ist die Signatur gültig gegen einen zugelassenen Schlüssel |
| 3 | Vertrauenskette | führt der Schlüssel zu einer autorisierten Wurzel |
| 4 | Artefaktbeschreibung | stimmen Hash, Format, Version mit dem zugelassenen Datensatz überein |
| 5 | Widerruf | ist der Erzeuger oder das Artefakt zurückgezogen |
| 6 | Autorisierung | darf dieses Artefakt mit diesen Capabilities hier laufen |

**A-2 (MUSS)** Signaturgültigkeit ist keine Ausführungsberechtigung. Ein gültig
signiertes Artefakt eines widerrufenen Erzeugers wird abgelehnt.
**A-3 (MUSS)** Jede Ablehnung wird protokolliert: Artefaktkennung, Grund, Prüfschritt.
Stille Ablehnung ist unzulässig.
**A-4 (MUSS)** Der Admission-Pfad liegt vor dem ersten Instruktionsabruf, nicht danach.
Eine Prüfung nach dem Start ist keine Admission.
**A-5 (MUSS) — getrennt prüfbar und auditierbar.** Die sechs Schritte dürfen nicht in einem
einzigen generischen `verify()` verschwinden. Jeder Schritt hat eine eigene Funktion, ein
eigenes Ergebnis und einen eigenen Grund im Protokoll, damit eine Ablehnung sagt, **welcher**
Schritt sie verursacht hat. Ein Sammelaufruf, der nur „ungültig" zurückgibt, ist nicht
konform, auch wenn er formal alle Prüfungen enthält.
**A-6 (GRUNDSATZ, dauerhaft in der Sicherheitsarchitektur).** Eine gültige Signatur ist keine
Ausführungsberechtigung. Herkunft, Integrität, Vertrauenskette, Widerruf,
Capability-Autorisierung und Ausführungsentscheidung bleiben getrennte Fragen.
**A-7 (MUSS) — Architekturgrenzen.** ShivaCore entscheidet über die Zulässigkeit der
Ausführung. GlobusOS stellt den kontrollierten Prozess- und Service-Lifecycle bereit. Aurora AI
und Genesis Engine bleiben eigenständige Domänen ohne privilegierte Sonderfreigabe. Externe
Kryptografie- oder Attestierungs-Frameworks dürfen Referenz und Adapter sein, aber nicht
unbemerkt zur maßgeblichen Sicherheitsinstanz werden.

---

## 5. Node-VM-Integration: Zielpfad

```
Transaktionseingang
  -> kanonische Transaktionsvalidierung
  -> ATC-VM, deterministische Ausführung
  -> State Transition, Gas, Receipt, State Root
  -> Konsens und Blockfinalisierung
```

**N-1 (MUSS)** Der Node integriert die kanonische VM. Es entsteht keine zweite
Ausführungslogik im Node, auch nicht als „vereinfachte" Variante für Devnet.
**N-2 (MUSS)** Der Node übernimmt Transaktionspipeline, Zustandsverwaltung und
Konsenskoordination, nicht Ausführungsregeln.
**N-3 (MUSS)** Ohne VM darf der Node keine Transaktion als ausgeführt melden. Ein Pfad, der
Transaktionen ohne Ausführung in Blöcke schreibt, ist als `BLOCKED` zu führen, nicht als
vereinfachter Betrieb.

---

## 6. Negativtests je Komponente (Pflichtliste)

Ein positiver Test zeigt, dass etwas funktioniert. Ein Negativtest zeigt, dass etwas
Unzulässiges nicht funktioniert. Beide sind nötig.

| # | Komponente | Negativtest |
|---|---|---|
| 1 | Prozessisolation | fremden Speicher lesen, muss im Seitenfehler enden, andere Prozesse laufen weiter |
| 2 | Kernel-Fehlerpfad | ungültiger Opcode in Ring 3 beendet den Prozess, nicht die Maschine |
| 3 | Block-Device | Lese- und Schreibfehler werden gemeldet, kein stiller Datenverlust |
| 4 | Dateisystem | beschädigte Struktur führt zu Ablehnung, nicht zu falschen Daten |
| 5 | `globus-init` | Dienst mit fehlender Abhängigkeit startet nicht, statt halb zu starten |
| 6 | IPC | Zugriff ohne passende Capability wird abgelehnt und protokolliert |
| 7 | Admission | unsigniert, falsch signiert, widerrufen, unbekannt: je eigene Ablehnung |
| 8 | Update | manipuliertes Update wird abgelehnt, vorheriger Stand bleibt lauffähig |
| 9 | Attestation | Abweichung im Messwert wird erkannt, nicht toleriert |
| 10 | Hash/Domains | Blatt als Knoten, leer gegen Ein-Blatt, Hochziehen gegen Duplizieren |
| 11 | Signatur | High-S, nicht kanonische Form, fremder Proposer, doppelter Schlüssel im Set |
| 12 | VM | Schrittobergrenze greift ohne Gas, Rollback vollständig, deterministisch |
| 13 | Node | Node ohne VM lehnt Ausführung ab, statt sie zu behaupten |
| 14 | Konsens | doppelte Transaktions-ID, falsche Höhe, falscher Vorgänger werden abgelehnt |

---

## 7. Prioritätenfolge

| # | Arbeitspaket | Abschlusskriterium | Status heute |
|---|---|---|---|
| 1 | Kernel-SSOT konsolidieren | ein autoritativer Kernel, Spiegel ohne eigene Implementierung | `TESTED`, Monorepo clean; Mirror-Doku-Drift offen |
| 2 | Block-Device-Abstraktion und Treiber | Datenträger lesen und schreiben, Fehlerfälle getestet | `BLOCKED` |
| 3 | `globus-init` | kontrollierter Prozess- und Dienststart | `BLOCKED` |
| 4 | IPC und Service Space | nachweisbarer, geschützter Kommunikationspfad | `BLOCKED` |
| 5 | Native Code-Admission | Signatur, Herkunft, Widerruf, Capability geprüft | `BLOCKED` |
| 6 | Update und Measured Boot | Manipulationserkennung, sichere Aktualisierung, Attestation | `BLOCKED` |
| 7 | Node-VM-Integration | deterministische Ausführung, reproduzierbarer State Root | `BLOCKED` |
| 8 | Hardware-Validierung | Tests auf realer Hardware, Kompatibilitätsmatrix | `NOT TESTED` |

---

## 8. Verbindliche Gesamtbewertung (11.11.2026)

| Bereich | Status | Begründung |
|---|---|---|
| Kernel | `TESTED` | QEMU-Nachweise, 733 Tests, Commithash `4fccb823` |
| Kernel-SSOT | `IMPLEMENTED` | keine zweite Kernelkopie im Monorepo; Doku-Drift im Spiegel separat |
| Kernel-IPC-Modul | `IMPLEMENTED` | Modul vorhanden, Rechteprüfung vor Zustellung; Transportintegration offen |
| Capability-Infrastruktur | `IMPLEMENTED` | Modul vorhanden; vollständige Admission nicht belegt |
| IPC-Transport | `BLOCKED` | Service Space nicht im Image, Ramdisk ohne Programm |
| Prozessisolation | `TESTED` | konkrete negative Gegenprobe vorhanden |
| GlobusOS als Betriebssystem | `RESIDUAL` | wesentliche Betriebssystemfunktionen fehlen (7, 8, 9, 10) |
| Code-Admission | `BLOCKED` | kein nativer Admission-Pfad |
| Node-VM-Integration | `BLOCKED` | keine nachgewiesene Vertragsausführung |
| Mainnet | `BLOCKED` | keine vollständige Ausführungs-, Persistenz- und Konsenskette |
| Hardware | `NOT TESTED` | ausschließlich QEMU |
| CI | `BLOCKED` | `startup_failure` in push-getriggerten Workflows |

**Satz für die Projektdokumentation:** Eine ernstzunehmende Kernel-Grundlage ist gebaut, ein
einsatzbereites KAI-OS noch nicht. Das ist kein Rückschritt, sondern die Präzisierung des
Reifegrads. Jede Komponente trägt ihren eigenen Status; die Summe ergibt keinen höheren.

---

## 9. Offene Punkte

1. Registry-Eintrag `KAI-OS-READINESS-001` (blockiert, Branchschutz und Committer-Adresse).
2. Statuszeile für `components/vm` und `components/wallet` heute neu erheben (Testlauf fehlt für 11.11.).
3. Mirror-Doku-Drift `a-townchain-os-docs` gegen Kanon schließen.
4. Kompatibilitätsmatrix für Hardware (CPU, Chipsatz, Firmware) als eigenes Dokument.
5. Erster Implementierungsschnitt für Code-Admission, ohne ABI-Vorgriff: Artefaktbeschreibung
   (Format, Version, Hash, Erzeuger), getrennte Prüfstufen nach A-5, Protokollsatz je Stufe.
   Vorbedingung ist eine Owner-Entscheidung über Ablageort und ABI-Bezug, weil eine
   Admission-Schnittstelle die Syscall-Oberfläche berührt.
6. Definition, ab welchem Nachweis `E2E_VERIFIED` vergeben werden darf (Vorschlag: verkettete Zustandswurzeln über mindestens zehn Blöcke plus reproduzierbarer Bau).

---

## 10. Was dieses Dokument nicht ist

Keine Freigabe, keine Messung, kein Testbericht. Es ist ein Katalog, der festhält, was belegt
ist und was nicht, damit die Bewertung nicht wieder zu weit gefasst wird.
