# A-TownChain / KAI-OS — Integriertes Technologie-Whitepaper

**Version:** 0.1.0-draft  
**Stand:** 19. Oktober 2026  
**Status:** Entwurf zur technischen, sicherheitsbezogenen und wirtschaftlichen Prüfung  
**Geltungsbereich:** ShivaCore Kernel, GlobusOS, A-TownChain Protocol, ATCLang und ATC-VM  
**Normative Leitlinie:** „Standalone First, Ecosystem Second“ · „Catalog First, Implementation Second“

> **Wichtiger Statushinweis:** Dieses Dokument beschreibt Architekturziele, spezifizierte Eigenschaften und offene Nachweise getrennt. Es ist weder ein Sicherheitsgutachten noch ein Prospekt, kein Testzertifikat und keine Bestätigung der Produktionsreife. Eine Architekturzeichnung oder ein grüner Build beweist allein weder vollständige Implementierung noch Sicherheit oder E2E-Funktion.

---

## 1. Zusammenfassung

A-TownChain / KAI-OS ist eine zusammenhängende Technologieinitiative mit fünf klar abgegrenzten Kernbereichen:

1. **ShivaCore Kernel** — ein Rust- und `no_std)-orientierter Capability-Microkernel als kleine Trusted Computing Base (TCB).
2. **GlobusOS** — ein natives Betriebssystem, dessen Systemfunktionen überwiegend als isolierte Userspace-Dienste ausgeführt werden.
3. **A-TownChain** — ein Blockchain-Protokoll mit definierten Transaktions-, Konsens-, Zustands- und Governance-Grenzen.
4. **ATCLang** — Sprache und Compiler-/Artefakt-Toolchain für deterministische Programme und Smart Contracts.
5. **ATC-VM** — die kanonische Ausführungsumgebung für validierte ATCB-Bytecode-Artefakte innerhalb des A-TownChain-Ausführungspfads.

Die Architekturthese lautet: Die Komponenten sollen über explizite, versionierte Verträge zusammenarbeiten, ohne ihre Sicherheitsautorität zu vermischen. Der Kernel kontrolliert Isolation und Kernel-Objekte; GlobusOS stellt Systemdienste bereit; A-TownChain verantwortet Protokollzustand und Konsens; ATCLang erzeugt kanonische Artefakte; ATC-VM verifiziert und führt Programme nach definierten Regeln aus.

Das Ökosystem ist nicht als ein monolithischer Kernel zu verstehen. Blockchain-Konsens, Netzwerkdienste, Dateisysteme, Treiber, KI-Agenten und Vertragsausführung gehören grundsätzlich in getrennte Services beziehungsweise eigene Laufzeitgrenzen. Die Integration wird über Registry, Standards, ABI-Verträge, Testvektoren und reproduzierbare Evidence gesteuert.

## 2. Status- und Evidenzmodell

Dieses Whitepaper unterscheidet folgende Statuswerte:

| Status | Bedeutung |
|---|---|
| `SPEC` | Eine Spezifikation oder Architekturentscheidung beschreibt das beabsichtigte Verhalten. |
| `IMPLEMENTED` | Code ist vorhanden; damit ist keine korrekte Funktion bewiesen. |
| `TESTED` | Ein definierter Test wurde ausgeführt; Geltungsbereich und Commit müssen angegeben sein. |
| `CI_VERIFIED` | Die relevanten CI-Prüfungen bestanden auf einem exakt identifizierten Commit. |
| `E2E_VERIFIED` | Ein Ende-zu-Ende-Pfad wurde mit nachvollziehbarer Evidence ausgeführt. |
| `VERIFIED` | Die für den konkreten Claim festgelegten Akzeptanzkriterien und unabhängigen Evidence-Anforderungen sind erfüllt. |
| `RESIDUAL` | Ein bekanntes Restrisiko oder eine offene Abweichung bleibt. |
| `BLOCKED` | Ein erforderlicher Nachweis, eine Entscheidung oder ein Gate fehlt. |
| `EXCLUDED` | Ein Bereich ist begründet aus dem jeweiligen Gate ausgeschlossen. |

**Evidence-Mindestanforderung:** Jede wesentliche Implementierungsbehauptung muss auf exakten Commit-SHA, Repository-Pfad, Symbol oder Datei, Blob-SHA, Workflow-Run, Job, Step, Exit-Code und relevante Logs beziehungsweise Artefakt-Hashes zurückführbar sein. Historische Testergebnisse sind kein Nachweis für den aktuellen Head. Unbekannte Zustände werden als `BLOCKED` behandelt. Ein grüner CI-Lauf allein begründet nicht den Status `VERIFIED`.

## 3. Architekturprinzipien und Systemgrenzen

### 3.1 Standalone First, Ecosystem Second

Jede Komponente muss innerhalb ihrer eigenen Zuständigkeit baubar, testbar und spezifizierbar bleiben. Die Ökosystemintegration darf keine zweite kanonische Implementierung erzeugen und keine lokalen Tests durch bloße Verknüpfung ersetzen.

### 3.2 Catalog First, Implementation Second

Vor der Implementierung werden Komponenten, Funktionen, Abhängigkeiten, normative Zuständigkeit, Schnittstellen und Evidenzanforderungen katalogisiert. Der Katalog ist kein Ersatz für Quellcode oder Tests; er macht Zuständigkeiten und Nachweislücken prüfbar.

### 3.3 Single Source of Truth (SSOT)

- **ShivaCore Kernel:** `globus-os/modules/atc-shivacore/kernel/`
- **GlobusOS:** das kanonische `globus-os`-Repository; Betriebssystemintegration und Userspace-Services.
- **A-TownChain Protocol / ATC-VM:** `a-townchain`, für die VM insbesondere `components/vm`.
- **ATCLang und ATCB v1:** `atclang`, einschließlich der kanonischen Bytecode-Spezifikation.
- **Normative Standards:** `atc-standards`.
- **Integration, Compliance, Architektur- und Evidence-Steuerung:** `a-townchain-ecosystem`; dieses Repository ist keine zweite Core-Implementation-SSOT.
- **ShivaCore-Spezifikationen und Governance:** `atc-shivacore`; kein zweiter kanonischer Kernel.

Legacy-, Demo- oder Referenzmaterial ist eindeutig zu kennzeichnen und darf nicht als aktive Implementierung oder Live-Drift-Quelle behandelt werden, sofern es nicht ausdrücklich in den Geltungsbereich aufgenommen wurde.

### 3.4 Vertrauensgrenzen

```text
Hardware / Firmware
        |
        v
Boot chain / HAL / platform boundary
        |
        v
ShivaCore Kernel (TCB)
  - capabilities / kernel objects
  - address spaces / threads / scheduling
  - IPC / traps / interrupts / timers
        |
        v
GlobusOS service space
  - storage / filesystem / networking / drivers
  - identity / wallet / security services
  - A-TownChain node and protocol services
  - ATCLang tooling / ATC-VM integration
        |
        v
Applications and authorized agents
```

Die Darstellung ist ein Zielmodell, keine Behauptung, dass alle Dienste bereits vollständig aus dem Kernel ausgelagert oder auf Hardware vollständig verifiziert sind.

## 4. ShivaCore Kernel

### 4.1 Rolle

ShivaCore ist als Rust-`no_std)-Capability-Microkernel konzipiert. Seine primäre Sicherheitsverantwortung ist die Durchsetzung von Prozess- und Speicherisolation, kontrolliertem Zugriff auf Kernel-Objekte, kontrollierter Interprozesskommunikation und privilegierten Übergängen.

Der Kernel soll keine KI-Entscheidungen treffen und keine Autorität über Blockchain-Konsens oder Smart-Contract-Geschäftslogik übernehmen. Diese Funktionen liegen in separaten Diensten beziehungsweise Protokollkomponenten.

### 4.2 Kernel-Minimalismus

Der TCB soll auf notwendige Primitive begrenzt bleiben, darunter:

- Threads, Scheduler und Prozessgrundlagen;
- Adressräume, virtuelle Speicherverwaltung und Mapping-Primitiven;
- Capability-/Kernel-Objektmodell und Rechteprüfung;
- IPC-Endpunkte und kontrollierte Nachrichtenübertragung;
- Traps, Interrupts, Timer und notwendige Hardwareabstraktion;
- ein klar spezifiziertes Syscall-ABI und Boot-Übergänge.

Komplexe Dateisysteme, Netzwerkprotokolle, GPU-/Audio-/USB-Stacks, Blockchain-Node, Vertragsausführung und KI-Agenten gehören in den Userspace. Ein Service erhält keine implizite Autorität allein dadurch, dass er Bestandteil von GlobusOS ist.

### 4.3 Sicherheitsinvarianten

Der Zielzustand verlangt mindestens:

1. Jeder Prozess besitzt einen eigenen, korrekt eingerichteten Adressraum.
2. Ein Scheduler-Kontextwechsel aktiviert tatsächlich den Adressraum des Zielprozesses.
3. User-Prozesse können weder fremde User-Seiten lesen/schreiben noch Kernel-Speicher unautorisiert erreichen.
4. Capability-Prüfungen erfolgen an der maßgeblichen Kernel-Objektgrenze und sind nicht durch alternative Pfade umgehbar.
5. Ein fehlerhafter User-Prozess kann isoliert beendet werden, ohne andere Prozesse unnötig zu beeinträchtigen.
6. Ein Kernel-Fault wird von einem User-Fault unterschieden; Fehlerframes werden abhängig vom Privilegübergang korrekt behandelt.
7. Page-Table-Änderungen, Rollback, Mapping-Lebensdauer und Objektfreigabe sind konsistent und race-sicher.
8. Syscall-Trap-Pfad und Prozessdispatcher sind durch reale Tests verbunden, nicht nur über Unit-Test-Harnesses simuliert.

### 4.4 Boot und Plattform

Die Zielkette umfasst Firmware-/Bootgrenze, Kernel-Einstieg, erforderliche Hardwareinitialisierung und Übergabe an einen Root-Server beziehungsweise `globus-init`. BIOS- und UEFI-Pfade, Secure Boot, TPM, ACPI und PCI/ECAM sind separat nachzuweisen. Eine erfolgreiche QEMU-BIOS-Ausführung ist kein Beleg für UEFI- oder reale Hardwarekompatibilität.

### 4.5 Status und offene Nachweise

Die Architektur ist ein Entwicklungsziel; aktuelle Produktionsreife wird nicht behauptet. Insbesondere müssen Kernel- und OS-Status anhand des aktuellen kanonischen Repository-Heads geprüft werden. Offene oder zuletzt dokumentierte Nachweisbereiche umfassen unter anderem Page-Table-Mutation/Rollback, Mapping-/Object-Lifetime, vollständige Syscall-/Dispatcher-Integration, Userspace-Isolation, Secure-Boot-/UEFI-Evidence und die geplante Verlagerung historisch im Kernel-Crate liegender Blockchain- oder Bridge-Module.

**Akzeptanz-Gate:** Exakter Commit, reproduzierbarer Build, BIOS- und UEFI-Bootnachweise, definierte CPU-Profile, Negativtests für Isolation und Capabilities, Fault-Handling-Tests, Artefakt-Hashes und unabhängige Review-Evidence. Bis zur Erfüllung bleibt der betreffende Claim `BLOCKED` oder `RESIDUAL`.

## 5. GlobusOS

### 5.1 Ziel und Betriebsmodell

GlobusOS ist das native Betriebssystem des Ökosystems, nicht bloß eine Desktop-Anwendung unter einem bestehenden Betriebssystem. Es soll auf ShivaCore aufbauen und Systemdienste als getrennte Komponenten ausführen. Windows- und Linux-Gastsysteme können während der Entwicklung über Virtualisierung wie QEMU untersucht werden; ein Gastbetriebssystem ist dabei kein Ersatz für den nativen Bootpfad.

### 5.2 Service Space

Dienste werden als eigenständige Prozesse mit minimal erforderlichen Capabilities vorgesehen. Dazu gehören langfristig:

- Dateisystem und Storage;
- Netzwerk und Netzwerkgeräte;
- Geräte- und Grafikdienste;
- Identitäts-, Schlüssel- und Wallet-Dienste;
- Update-, Paket- und Security-Center-Funktionen;
- A-TownChain-Node- und Entwicklerdienste;
- Aurora-AI-Runtime und Agenten.

Jeder Dienst benötigt einen definierten IPC-Vertrag, Fehler- und Neustartstrategie, Capability-Profil, Paket-/Update-Identität und testbare Sicherheitsgrenze. Ein Dienstkatalog muss zwischen geplant, implementiert, getestet und verifiziert unterscheiden.

### 5.3 Update- und Plattformvertrauen

Die Zielarchitektur erfordert signierte Pakete und Updates, überprüfbare Herkunft, sichere Schlüsselverwaltung, Rollback-Schutz und klare Recovery-Prozesse. Paket-Signaturprüfung allein beweist nicht, dass der vollständige Installationspfad, der Trust Store oder das Update-Rollback korrekt abgesichert sind.

### 5.4 Kompatibilität und Leistung

Kompatibilität muss pro Hardware- und Firmwareprofil belegt werden. Leistungsmessungen benötigen reproduzierbare Workloads, Hardware-/VM-Profil, Build-SHA, Messmethodik und Baseline. Es werden keine ungemessenen Leistungs- oder Kompatibilitätsversprechen abgegeben.

## 6. A-TownChain Protocol

### 6.1 Protokollrolle

A-TownChain definiert die Regeln für Transaktionen, Zustandsübergänge, Blockverarbeitung, Konsensintegration und ATC-Wertbewegung. Die Spezifikation des Protokolls ist von einem tatsächlich betriebenen Netzwerk, ökonomischer Sicherheit und produktiver Adoption zu unterscheiden.

### 6.2 Dokumentierte Parameterbasis

Die derzeit dokumentierte Parameterbasis umfasst:

| Parameter | Dokumentierter Wert | Aussagegrenze |
|---|---|---|
| Chain ID | `658467` | Identifikator laut Spezifikation; kein Beweis für aktiven Mainnet-Betrieb |
| Maximale Supply | `360,000,000 ATC` | Protokollparameter; Ausgabe-/Burn-/Genesis-Regeln separat zu verifizieren |
| Dezimalstellen | `18` | Einheitenrepräsentation |
| Ziel-Blockzeit | `360 Sekunden` | Zielparameter; tatsächliche Verteilung ist zu messen |
| Halving-Intervall | `360,000 Blöcke` | Emissionsparameter; vollständiger Emissionsplan muss formal getestet sein |
| Transaktions-Domain | `ATC-TX-DOMAIN-V2` | Domain-Separation-Parameter |

Die vollständigen Regeln für Genesis, Gebühren, Blockvalidierung, Fork-Verhalten, Validator-Anreize und Recovery müssen gegen den kanonischen Code und die normativen Standards abgeglichen werden.

### 6.3 Signaturen und Transaktionssicherheit

Der dokumentierte Transaktionssignaturpfad verwendet secp256k1 ECDSA mit RFC6979/SHA-256, Low-S-Normalisierung und kompaktem `r||s)-Format mit komprimierten öffentlichen Schlüsseln. Ed25519 ist in dieser Transaktionssignaturdefinition nicht als Signierer festgelegt. Die Implementierung muss Canonicalization, Domain Separation, Nonce-Handling, Replay-Schutz und negative Signaturvektoren testen.

### 6.4 Konsens und Mining

Die dokumentierte AI-Mining-Variante A kombiniert AI-Compute-Mining mit unabhängigen Proof-of-Stake-Validatoren; beide Rollen sollen in ATC vergütet werden. Diese Designbeschreibung ist kein Nachweis wirtschaftlicher Sicherheit. Vor einem produktiven Start sind Angriffsmodell, Sybil-/Mehrheitsangriffe, Slashing beziehungsweise Sanktionsregeln, Validator-Verteilung, Emissionssimulationen, Finalität und Wiederherstellungsverhalten offenzulegen und zu testen.

Cross-chain-Funktionalität gilt nicht als verfügbar, solange Bridge-Design, Vertrauensannahmen, Nachrichtenvalidierung, Replay-Schutz, Schlüsselverwaltung, Notfallsteuerung und unabhängige Security-Reviews nicht abgeschlossen und evidenzbasiert verifiziert sind.

### 6.5 Token- und Wirtschaftsmodell

Die dokumentierte maximale Supply ist nur ein Teil des ökonomischen Modells. Für eine belastbare Investorenbewertung müssen insbesondere Genesis-Zuteilung, Team-/Treasury-Anteile, Vesting, Fördermittel, Validator-Emissionen, Gebühren, Governance-Rechte, Liquiditätsannahmen und etwaige Sperrfristen explizit entschieden und versioniert werden. In diesem Dokument werden keine nicht belegten Finanzierungsziele, Bewertungen, Renditen oder Tokenzuteilungen erfunden.

## 7. ATCLang

### 7.1 Rolle und Pipeline

ATCLang ist die Sprach- und Compiler-Toolchain für Programme, deren Ausführung an die kanonische ATC-VM gebunden ist. Der vorgesehene logische Pfad lautet:

```text
ATCLang source (.atc)
  -> Lexer / Parser / AST / type and semantic checks
  -> canonical ATC-IR
  -> IR verification
  -> module / ABI validation
  -> canonical ATCB v1 artifact
  -> independent structural verification
  -> canonical ATC-VM
  -> deterministic execution and state transition
```

Diese Pipeline ist der Architekturvertrag. Sie darf nicht als vollständig implementierter Ende-zu-Ende-Pfad ausgegeben werden, bevor alle Übergänge mit exaktem Commit, Conformance-Vektoren und reproduzierbaren Artefakten belegt sind.

### 7.2 Compiler-Anforderungen

Die Toolchain muss reproduzierbare Artefakte erzeugen und undefinierte beziehungsweise hostabhängige Semantik vermeiden. Erforderlich sind unter anderem:

- deterministische Tokenisierung, Parsing und Codegenerierung;
- explizite Typ- und Integersemantik;
- Validierung von Kontrollfluss und Sprungzielen;
- eindeutige Fehlerbehandlung und keine stillen semantischen Casts;
- kanonische Serialisierung ohne Zeitstempel, Pointer, Host-Metadaten oder Hash-Map-Reihenfolge;
- negative Tests für fehlerhafte Eingaben und Ressourcenerschöpfung;
- unabhängige beziehungsweise differentielle Konformitätstests zwischen Compiler, Verifier und VM.

Historische Dateizahlen, Testzahlen oder Parse-Raten sind kein aktueller Nachweis. Maßgeblich sind der aktive Rust-Quellbaum, die aktuellen Specs und SHA-gebundene CI-Evidence.

### 7.3 ATCB v1

Die normative Bytecode-Spezifikation `ATC-BC-001` in `atclang/specs/bytecode/SPEC.md` bezeichnet ATCB v1 als eingefrorenen Formatstand vom 6. Oktober 2026; der Registry-Pointer in `atc-standards` kennzeichnet ihn zugleich als **FROZEN / NOT VERIFIED**, solange das zugehörige Gate und die Owner-Freigabe nicht abgeschlossen sind.

Die dokumentierte Container-Geometrie lautet:

- vier ASCII-Bytes `ATCB`;
- Formatversion als Big-Endian-`u16`, derzeit Version 1;
- Instruction Count als Big-Endian-`u32`;
- nachfolgende Instruktionen und Operanden in Big-Endian;
- keine Checksum im Container; Integrität gehört zur separaten ATCA-Artefaktschicht;
- unbekannte Opcodes, abgeschnittene Operanden, ungültige Versionen und nachlaufende Bytes werden fail-closed abgewiesen.

Die kanonische Spezifikation nennt 19 Opcodes, darunter Konstanten, lokale Variablen, arithmetische und Vergleichsoperationen, Calls, Return, Sprünge und Pop. Der normative Opcode-Katalog und die Verifikationsregeln müssen aus dem aktuellen eingefrorenen Spec übernommen werden; Legacy-Encoding darf nicht als kanonisch bezeichnet werden.

### 7.4 Noch offene Ausführungsverträge

Der separate Vertrag `ATCB-ATVM-EXECUTION-001` ist als `0.1.0-DRAFT` gekennzeichnet. Er beschreibt unter anderem Funktionsrahmen, lokale Variablen, Aufrufsemantik, Integerverhalten und persistente Storage-Grenzen, beansprucht aber ausdrücklich keine vollständige Compiler-to-VM-Implementierung. Ein Draft darf den normativen Bytecode-Freeze nicht überschreiben.

## 8. ATC-VM

### 8.1 Zuständigkeit

Die kanonische VM-Implementierung liegt in `a-townchain/components/vm`. ATC-VM verifiziert und führt zulässige Bytecode-Artefakte nach den verbindlichen Regeln aus. Sie darf nicht durch eine zweite, abweichende VM-Implementierung im Integrationsrepository ersetzt werden. Referenzmodelle dürfen zu Tests dienen, müssen aber als solche gekennzeichnet sein.

### 8.2 Determinismus

Für dieselben gültigen Eingabeartefakte, denselben definierten Ausgangszustand und dieselben expliziten Kontextparameter muss die Ausführung dieselben beobachtbaren Ergebnisse erzeugen. Dazu gehören insbesondere:

- kanonische Dekodierung und fail-closed Ablehnung ungültiger Artefakte;
- festgelegte Integer-, Overflow- und Division-durch-Null-Semantik;
- stabile Regeln für Sprungziele, Stack und Funktionsaufrufe;
- keine unkontrollierten Host-Zeit-, Zufalls-, Pointer- oder Plattformdaten;
- deterministische Fehlercodes und Zustandsübergänge;
- Ressourcenlimits und Abbruchverhalten, die nicht von undefiniertem Hostverhalten abhängen.

### 8.3 Ressourcen- und Speichergrenzen

Die dokumentierte Projektbaseline nennt für die VM Grenzwerte von 65.536 Programmoperationen, 4.096 Stack-Einträgen und 65.536 Storage-Einheiten. Diese Werte sind als konfigurierte Protokoll-/Implementierungsparameter gegen die aktive SSOT zu bestätigen, bevor sie als Release-Garantie verwendet werden. Jeder Grenzwert benötigt Boundary- und Überlauftests sowie eine definierte Fehlerreaktion.

### 8.4 Storage und State Transition

Lokale Variablen eines Funktionsrahmens sind nicht mit persistentem Chain-Storage gleichzusetzen. Persistenter Zustand benötigt eine eigene Capability-/ABI-Grenze, kanonische Schlüssel- und Wertkodierung, definierte Missing-Key-Semantik, deterministische Kosten-/Gasregeln und eine atomare Zustandsübergangsregel. Der Ausführungspfad darf diese Regeln nicht umgehen, indem er interne Op-Enums oder nichtkanonische `.ops`-Repräsentationen als Artefaktformat verwendet.

### 8.5 Akzeptanzkriterien

- gültige und ungültige ATCB-Conformance-Vektoren;
- unabhängige strukturelle Verifikation;
- Differentialtests gegen eine Referenz, ohne die Referenz als Produktions-SSOT zu behandeln;
- deterministische Ergebnisse und Fehler bei identischen Eingaben;
- Grenz-, Fuzzing- und Malformed-Input-Tests;
- Gas-/Ressourcen- und Abbruchtests;
- Zustandsübergangs- und Persistenztests im kanonischen Chain-Kontext;
- Build-, Test- und Artefakt-Evidence auf exakt identifiziertem Commit.

Bis diese Evidenz vollständig ist, bedeutet ein eingefrorenes Format nicht, dass die gesamte VM-Ausführung oder der Mainnet-Pfad verifiziert ist.

## 9. Zusammenspiel der Komponenten

### 9.1 Beispielhafter Smart-Contract-Pfad

1. Ein Entwickler schreibt Quelltext in ATCLang.
2. Die Toolchain prüft Syntax, Typen und semantische Regeln.
3. Der Compiler erzeugt kanonische IR und ATCB v1.
4. Der Artefaktpfad bindet die kanonische Payload an Integritätsdaten.
5. Der Verifier prüft Struktur, Kontrollfluss und zulässige Ressourcen.
6. Die kanonische ATC-VM führt den Bytecode im definierten Ausführungskontext aus.
7. Der Chain-State-Transition-Pfad akzeptiert oder verwirft den resultierenden Übergang nach den Protokollregeln.
8. Ein GlobusOS-Node-Service kann diesen Ablauf bereitstellen, ohne die VM in den ShivaCore-TCB zu verschieben.

Jeder Schritt ist ein eigener Evidence-Punkt. Ein erfolgreiches Compiler-Beispiel beweist nicht automatisch die VM-Sicherheit, Konsenskorrektheit oder E2E-Funktion.

### 9.2 Systemdienste und Berechtigungen

GlobusOS-Services erhalten nur die notwendigen Capabilities. Der Wallet-Dienst signiert nicht aufgrund einer bloßen KI-Empfehlung. Aurora-AI-Agenten können Vorschläge erstellen, aber Aktionen wie Signieren, Übertragen, privilegierte Änderungen oder Gegenmaßnahmen benötigen eine passende Capability, Policy-Prüfung, Auditspur und gegebenenfalls explizite menschliche Freigabe.

### 9.3 Keine Autoritätsvermischung

- ShivaCore entscheidet über Kernel-Objekte, Isolation und privilegierte Kernel-Operationen.
- GlobusOS orchestriert Userspace-Dienste und Systemlebenszyklus.
- A-TownChain entscheidet nach Protokollregeln über Chain-Zustand und Konsens.
- ATCLang definiert Compiler-/Artefaktverhalten.
- ATC-VM definiert die erlaubte deterministische Ausführung.
- KI-Modelle sind keine Quelle von Kernel-, Konsens- oder Wallet-Autorität.

## 10. Sicherheit, Governance und Lieferkette

### 10.1 Secure Development Lifecycle

Jede sicherheitskritische Änderung benötigt Bedrohungsmodell, Review, positive und negative Tests, reproduzierbare Builds, Abhängigkeitsprüfung und Release-Artefakte. Für den Kernel sind Fault-Injection und Isolationsnachweise erforderlich; für die VM malformed-input-, Fuzzing- und Determinismustests; für das Protokoll adversariale Zustandsübergangs- und Konsenstests; für Compiler und Bytecode differential/conformance tests.

### 10.2 Supply Chain

Pakete und Releases sollen an Commit-SHA, Build-Workflow, Toolchain-Version, Abhängigkeitsinventar und Hashes gebunden sein. Signaturen und Hashes belegen Herkunft beziehungsweise Integrität nur im Rahmen eines vertrauenswürdigen Schlüssel- und Build-Prozesses; sie ersetzen keinen Code- oder Sicherheitsreview.

### 10.3 Standards und Änderungen

Normative Regeln gehören in die zuständige SSOT und Registry. Format- oder ABI-Änderungen müssen versioniert, rückwärtskompatibel bewertet und durch eine dokumentierte Änderungsfreigabe geführt werden. Ein Dokumentations- oder Ecosystem-Repository darf keine normative Kopie schaffen, die von der kanonischen Spezifikation abweicht.

### 10.4 Technology-Claims

Behauptungen wie „einzigartig“, „weltweit erstmals“, „revolutionär“ oder vergleichbare Neuheitsclaims benötigen belegbare Prior-Art-Recherche, Implementierungsnachweis, unabhängige Tests/Benchmarks, Sicherheitsreview und IP-Bewertung. Ohne vollständige Nachweiskette ist der Claim als `PENDING-EVIDENCE` zu führen und darf nicht als bestätigte Differenzierung veröffentlicht werden.

## 11. Geschäfts- und Adoptionsmodell

Mögliche Wertschöpfungspfade sind Entwicklerwerkzeuge, Unternehmensintegration, Support und Wartung, geprüfte Systempakete, Infrastruktur-/Validator-Dienste und optionale Plattformdienste. Dies sind Hypothesen, keine bestätigten Umsätze oder Product-Market-Fit-Nachweise.

Für eine belastbare Investorenbewertung fehlen in diesem Dokument bewusst unbestätigte Zahlen zu Marktanteilen, Kunden, Umsatz, Bewertung, Finanzierungsziel, Runway, Tokenverteilung und Rendite. Diese Daten müssen aus belastbaren Unternehmensunterlagen und einem geprüften Finanzmodell stammen.

## 12. Roadmap als Gate-Modell

Die Roadmap wird nicht über Wunschdaten, sondern über nachweisbare Reifegrade gesteuert:

| Gate | Erforderliche Ergebnisse | Freigabekriterium |
|---|---|---|
| G0 — Ownership & Catalog | SSOT, Registry, Verantwortliche, ABI-/Spec-Grenzen | Keine konkurrierenden Implementierungs-SSOTs |
| G1 — Reproducible Build | Pinbare Toolchains, reproduzierbare Artefakte, Hashes | Build reproduzierbar auf exaktem Commit |
| G2 — Component Tests | Unit-, negative-, boundary- und fuzz tests | Relevante Tests dokumentiert und grün |
| G3 — Conformance | Normative Vektoren, Verifier-/VM-Abgleich, ABI-Tests | Keine ungeklärten normativen Konflikte |
| G4 — Isolation & Security | Kernel-/Userspace-Negativtests, Threat Model, Review | Keine offenen kritischen Blocker |
| G5 — E2E | Boot, Systemdienst, Chain/Compiler/VM-Pfad | Ende-zu-Ende-Nachweise auf exaktem Commit |
| G6 — Release Readiness | Install/Update/Rollback, Recovery, Release-Signaturen | Release- und Sicherheitsfreigabe dokumentiert |

Der Status jedes Gates muss anhand aktueller Workflows und Artefakte aktualisiert werden. Dieses Whitepaper erklärt kein Gate pauschal für bestanden.

## 13. Risiken und Gegenmaßnahmen

| Risiko | Auswirkung | Gegenmaßnahme / Nachweis |
|---|---|---|
| Architekturabweichung zwischen Spec und Code | Unsichere oder nicht reproduzierbare Integration | SSOT-Reconciliation, normative Conformance-Tests |
| Kernel-Isolation unvollständig | Prozess- oder Kernel-Kompromittierung | CR3-/Adressraumtests, Fault-Tests, unabhängiges Review |
| Bytecode-/VM-Semantik driftet | Unterschiedliche Zustandsübergänge | Freeze-konforme Implementierung, Differentialtests |
| Konsens- oder Anreizmodell unzureichend | Ökonomischer Angriff oder Netzwerkausfall | Formale Spezifikation, Simulation, adversariale Tests |
| Supply-Chain-/Update-Schwäche | Kompromittierte Pakete oder Recovery | Signierte Releases, Rollback- und Installationspfadtests |
| Scope zu breit | Verzögerung und Wartungskosten | Phasen-Gates, priorisierte TCB-/E2E-Blocker |
| Überzogene Claims | Reputations-, Rechts- und Investorenrisiko | Claim-Registry, Evidence-Pflicht, Legal/IP-Review |

## 14. Offene Entscheidungen vor einer finalen Veröffentlichung

1. Aktuelle Head-SHAs und Status jedes kanonischen Repositories zum Veröffentlichungszeitpunkt festhalten.
2. ShivaCore-Spezifikationsentscheidungen und Architekturabweichungen abschließen oder ausdrücklich als offen ausweisen.
3. GlobusOS-Boot-, Installations-, Update-, Recovery- und Hardware-Evidence konsolidieren.
4. A-TownChain Genesis-, Emissions-, Gebühren-, Konsens- und Validatorregeln vollständig gegen die aktive SSOT abgleichen.
5. ATCB v1 Registry-/Owner-Gates abschließen und den Unterschied zwischen Format-Freeze und Implementierungsverifikation erhalten.
6. Compiler-to-VM- und Chain-State-Transition-E2E-Pfad nachweisen.
7. Finanzierungsmodell, Budget, Runway, Tokenverteilung und Vesting durch verantwortliche Eigentümer freigeben.
8. Team, Eigentumsrechte, Open-Source-Lizenzen, Drittanbieterrechte und IP-Risiken rechtlich prüfen.
9. Sicherheitsreview und gegebenenfalls externe Audits mit Scope, Commit und Findings dokumentieren.
10. Alle nicht nachgewiesenen Claims als `OPEN`, `BLOCKED` oder `PENDING-EVIDENCE` markieren.

## 15. Glossar

- **ABI:** Application Binary Interface — versionierter Vertrag zwischen Komponenten.
- **ATCB:** Kanonisches Bytecode-Containerformat für ATC-VM.
- **ATC-VM:** Kanonische virtuelle Maschine für ATCB-Artefakte.
- **Capability:** Nicht implizit vorhandenes, explizit zugewiesenes Recht auf ein Objekt oder eine Operation.
- **E2E:** Ende-zu-Ende-Verifikation über eine vollständige Komponentenkette.
- **SSOT:** Single Source of Truth — kanonischer Ort für eine Implementierung oder normative Definition.
- **TCB:** Trusted Computing Base — Teil des Systems, dessen Korrektheit für die Sicherheitsgarantien vorausgesetzt wird.

## 16. Disclaimer

Dieses Dokument ist ein technischer und strategischer Entwurf. Es ist weder Anlageberatung noch Angebot zum Kauf oder Verkauf von Token oder Wertpapieren, kein Prospekt und keine Garantie für Funktion, Sicherheit, Rendite, Netzwerkbetrieb oder zukünftige Liefertermine. Protokollparameter, Architekturziele und Spezifikationsstände sind nicht mit verifizierter Implementierung gleichzusetzen. Vor einer öffentlichen Investorenverwendung sind technische, rechtliche, finanzielle und IP-bezogene Prüfungen erforderlich.

---

## Referenzquellen und maßgebliche Zuständigkeiten

- ShivaCore-Kernel-SSOT: [globus-os/modules/atc-shivacore/kernel](https://github.com/A-TownChain-Okosystems/globus-os/tree/main/modules/atc-shivacore/kernel)
- GlobusOS: [globus-os](https://github.com/A-TownChain-Okosystems/globus-os)
- A-TownChain: [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain)
- ATCLang und ATCB v1: [atclang](https://github.com/A-TownChain-Okosystems/atclang)
- Normative Standards: [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards)
- Architekturgrenze ShivaCore: [SHIVACORE_KERNEL_ARCHITECTURE.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/architecture/SHIVACORE_KERNEL_ARCHITECTURE.md)
- ATCLang Source-of-Truth-Grenze: [ATCLANG_COMPILER.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/architecture/ATCLANG_COMPILER.md)
- Normative Bytecode-Spezifikation: [ATC-BC-001 / ATCB v1](https://github.com/A-TownChain-Okosystems/atclang/blob/main/specs/bytecode/SPEC.md)
- ATCB Registry-Pointer: [ATCB-v1-POINTER.md](https://github.com/A-TownChain-Okosystems/atc-standards/blob/main/references/standards/ATCB-v1-POINTER.md)
