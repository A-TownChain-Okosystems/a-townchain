# GlobusOS / ShivaCore — Microkernel-Migrationsinventur

- **Auditdatum:** 2026-10-18
- **Repository:** `A-TownChain-Okosystems/globus-os`
- **Audit-Baseline:** `3bf000143ec9bfad79d0bacc36b99b8111178b95` (`main`, aus der aktuellen Repository-Suche ermittelt)
- **Arbeitsbranch:** `audit/microkernel-migration-inventory-2026-10-18`
- **Kernel-Source-of-Truth:** `modules/atc-shivacore/kernel/`
- **Ergebnis:** `PARTIAL_SEPARATION_CONFIRMED / MIGRATION_BLOCKED_PENDING_BOUNDARY_WORK`
- **Umfang dieses Commits:** Bestandsaufnahme und maschinenlesbarer Status; keine Produktionspfade werden ohne Implementierungs- und Integrationstests entfernt.

## 1. Executive Findings

GlobusOS enthält bereits getrennte Rust-Crates für IPC, Security, Process, Memory, VFS, Netzwerk-Policy, Devices, Services, Graphics, Audio, Package, Update und Runtime. Außerdem existiert `modules/atc-shivacore/service_space/` mit ausgelagerten Domainmodulen. Das belegt vorhandene Struktur und teilweise Code-Auslagerung, **nicht** automatisch separate Prozesse, getrennte Adressräume oder eine sichere Laufzeitgrenze.

Die direkte Quellprüfung fand folgende konkrete Grenzprobleme:

1. **ATCFS bleibt im aktiven Kernel-Crate.** `kernel/src/lib.rs` exportiert `atcfs`; `kernel/src/kernel_init.rs` importiert und erzeugt `AtcFileSystem`. `kernel/src/atcfs.rs` beschreibt seine Hashfunktion selbst als vereinfachte, nicht produktive SHA3-Implementierung. Das ist ein Produktionsrisiko für Content-IDs und kein akzeptabler kryptografischer Integritätsnachweis.
2. **VFS bleibt im aktiven Kernel-Crate.** `kernel/src/lib.rs` exportiert `vfs`; `kernel/src/kernel_init.rs` hält ein `Vfs`-Feld und erzeugt es beim Boot. Es gibt parallel `system/vfs/`. Vor der Entfernung muss geklärt und getestet werden, welche Kernelmechanismen nötig sind und welche Dateisystem-/Namespace-Logik in den Dienst gehört.
3. **Kernel-Security-Modul hat zu breite Verantwortung.** `kernel/src/security.rs` dokumentiert Multi-Signature, Audit-Log, Reputation, Rate-Limiting und Secure-Channel-Funktionen; nicht alle gehören in die TCB. Im Kernel verbleiben sollen nur Enforcement-Mechanismen, nicht die gesamte Policy-/Audit-Domainlogik.
4. **Service Space hängt direkt vom Kernel-Crate ab.** `modules/atc-shivacore/service_space/Cargo.toml` enthält `shivacore = { path = "../kernel" }`. Die aktuelle Quelltextsuche bestätigt direkte Kernel-Imports in `service_space/src/atcnet.rs` (`shivacore::ats1000::NetworkStack`), `remote_caps.rs` (`shivacore::capability::Rights`), `p2p_secure.rs` (`shivacore::security::TokenBucket`) und `knowledge_graph.rs` (`shivacore::capability::{CapId, CapabilityTable, Pid, ResourceType, Rights}`). `service_init.rs` erwähnt `KernelState::boot` nur in Kommentaren; ein aktiver Aufruf wurde in dieser Datei nicht gefunden. Mehrere hash-abhängige Module verwenden bereits `libshivacore::simple_hash`, aber dieser Kompatibilitätshash ist ausdrücklich nicht kryptografisch. Diese direkten Imports sind die wichtigsten strukturellen Kopplungen, die vor einer echten Prozessgrenze aufzulösen sind.
5. **Ein ABI-Crate ist bereits vorhanden, die End-to-End-Nutzung ist nicht bewiesen.** `system/shivacore-api/` definiert versionierte Syscall-/Capability-/IPC-Typen. Der sichtbare Kernel-Dispatcher prüft ABI-Version, Payloadgröße und Capability für bestimmte IDs, aber dieser Quellbefund beweist noch keine vollständige echte IPC-Übertragung, Kernel-Entry-Ausführung oder Isolation im gebooteten System.
6. **Treiberbasis ist teilweise im User-Space-Modell abgebildet.** `system/devices/` enthält PCI, NVMe, IOMMU, Interrupt-, Block- und Ethernet-Module. Ob diese Hardware tatsächlich durch einen separaten Prozess betrieben wird, mit widerrufbaren MMIO/DMA-/Interrupt-Rechten läuft und auf QEMU/Hardware getestet wurde, ist in dieser Inventur nicht nachgewiesen.
7. **Legacy-/Architekturdokumentation driftet.** `modules/atc-shivacore/ARCHITECTURE.md` listet Domainmodule im Kernel-Dateibaum, die nicht alle durch `kernel/src/lib.rs` exportiert werden; dieselbe Dokumentation enthält ältere Aussagen zu Domainfunktionen im Kernel. Nicht exportiert bedeutet nicht automatisch unerreichbar: Aufrufstellen, Cargo-Abhängigkeiten, Build-Konfiguration und Tests müssen vor dem Löschen durchsucht werden.

## 2. Bewertungsregeln

- **IMPLEMENTED** bedeutet ausschließlich, dass Quellcode/Vertrag existiert.
- **PARTIAL** bedeutet, dass Teile vorhanden sind, aber Integration, echte Ausführung oder Grenznachweis fehlt.
- **BLOCKED** bedeutet, dass eine sichere Migration erst nach einer Abhängigkeit oder einem Sicherheitsnachweis möglich ist.
- **NOT_VERIFIED** bedeutet, dass keine belastbare, SHA-gebundene Ausführungs- oder Integrationsevidenz im Rahmen dieses Audits erhoben wurde.
- **RESIDUAL** markiert ein konkret erkanntes, noch nicht behobenes Risiko.
- Kein Status in dieser Inventur behauptet Produktionsreife. Die vorhandenen Unit-Testzahlen aus dem Audit vom 2026-10-11 gelten nicht als Nachweis für den aktuellen Baseline-SHA.

## 3. Komponenten-Inventur

| Komponente / Bereich | Aktueller Pfad / Befund | Ziel-Ausführungsmodus | Status | Nächster verbindlicher Nachweis |
|---|---|---|---|---|
| Capability Enforcement | `kernel/src/capability.rs`; `system/security/` | Enforcement im Kernel, Policy-Administration als User-Space-Dienst | PARTIAL | Widerruf, Delegation, Attenuation, Cross-process-Negativtests |
| IPC Mechanismen | `kernel/src/ipc.rs`; `system/ipc/`; `system/shivacore-api/` | Kernel-Primitiv + versioniertes User-Space-Protokoll | PARTIAL | echte IPC-Integration über getrennte Adressräume, Größenlimits, Backpressure, Abbruch |
| Syscall ABI | `kernel/src/syscall.rs`; `system/shivacore-api/` | Kernel-Dispatcher + stabile ABI | PARTIAL | Entry-Pfad, Argument-/Pointervalidierung, echte Dispatch-Semantik, Negativtests |
| Process / Scheduler | `kernel/src/process.rs`, `scheduler.rs`; `system/process/` | minimale Kernel-Lifecycle-/Scheduling-Primitiven plus Service Manager | PARTIAL | echter Context-Switch, Prozesslebenszyklus, Quotas, Crash-Recovery |
| Adressraum / Memory | `kernel/src/memory_manager.rs`, `vmm.rs`, `userspace.rs`; `system/memory/` | Seitentabellen und Schutzrechte im Kernel; Policy/Accounting im Dienst | PARTIAL | zwei isolierte Prozesse, User-/Kernel-Permissions, Mapping-Lifetime und Rollback |
| User-Space-Ausführung | `kernel/src/userspace.rs`, `elf_loader.rs`, `user_io.rs` | privilegierte Übergangsmechanismen im Kernel; Loader/ELF-Policy außerhalb, soweit sicher | BLOCKED | tatsächlicher Ring-3-Start, Copy-in/out-Validierung, fehlerhafte Pointer und Fault-Tests |
| ATCFS / Content IDs | `kernel/src/atcfs.rs`, in `lib.rs` exportiert und in `kernel_init.rs` gestartet | Storage-/Dateisystemdienst im User Space | RESIDUAL / BLOCKED | erst kryptografisch korrekte Hashimplementierung oder sichere Neuimplementierung, dann User-Space-Migration und Persistenztests |
| VFS / Namespace | `kernel/src/vfs.rs`, in `lib.rs` exportiert und in `kernel_init.rs` gestartet; `system/vfs/` | VFS-/Mount-/Namespace-Dienst im User Space; Kernel nur sichere IPC-/Memory-Mechanismen | RESIDUAL / BLOCKED | Referenzinventur aller Aufrufstellen, Capability-gesteuerter VFS-RPC, Traversal-/Rechte-/Crashtests |
| Block-Storage | `system/devices/`, `system/vfs/`; zusätzlich unexportierte Kerneldatei `block.rs` laut Architekturregister | Block-Gerätedienst und Storage-Prozesse | PARTIAL | reale Treiber-/IPC-Integration, I/O-Fehler und Timeout-Tests |
| PCI/USB/NVMe/Netzwerkgeräte | `system/devices/`; `modules/atc-drivers/` | User-Space-Treiber; Kernel vergibt beschränkte MMIO/DMA/Interrupt-Ressourcen | PARTIAL | Hardwarezuweisung, IOMMU/DMA-Isolation, IRQ-Delegation und Widerruf |
| Netzwerk-Stack | `modules/atc-shivacore/service_space/src/{atcnet,net,tcpip,p2p,p2p_secure,sockets}.rs`; `system/net/` | User-Space-Netzwerkdienste | PARTIAL | Kernel-Abhängigkeit entfernen, echte NIC-IPC, Protokoll- und Fuzztests |
| Blockchain / Consensus / P2P | Service-Space `blockchain.rs`, `consensus.rs`, `genesis*.rs`, `gossip_bridge.rs`, `mempool.rs`, `vm.rs`, `contract.rs` | User-Space-Dienste / getrennte VM | PARTIAL | Service-Space darf keine Kernel-Interna importieren; Isolation und Ressourcenlimits |
| AI / Aurora | Service-Space `ai.rs`; `system/aurora-core/`, `system/runtime/` | User-Space Runtime / Agent-Sandbox | PARTIAL | Tool-Aufrufe über explizite Rechte, CPU-/Memory-/IPC-Quotas, Audit und Widerruf |
| DID / Identity / Remote Capabilities | Service-Space `did.rs`, `remote_caps.rs`, Identity-Crates | Identitäts-/Policy-Dienste; Kernel setzt lokale Capability-Regeln durch | PARTIAL | Replay-/Confused-Deputy-Tests und Nachweis, dass User-Space nicht selbst privilegieren kann |
| Kernel Security Domain | `kernel/src/security.rs`, `kernel/src/module_security.rs`; `system/security/` | nur notwendige Enforcement-/Integritätsmechanismen im Kernel; Audit-/Policy-Domain als Dienst | RESIDUAL | Funktionsinventur, Call-Site-Analyse, Zuständigkeitsmatrix und Fail-closed Tests |
| Service Manager / Init | `system/services/`; `service_space/src/service_init.rs`; `kernel_init.rs` | User-Space-Supervisor; Kernel initialisiert nur unverzichtbare Primitive | PARTIAL | Startreihenfolge, Dependency-Graph, Crash-Neustart, Quotas und Ausfalltests |
| Package / Update / Recovery | `system/package/`, `system/update/` | User-Space-Dienste; Kernel schützt nur Schreib-/Mapping-Grenzen | PARTIAL | verifizierte Bytes == aktivierte Bytes, TOCTOU-, Rollback- und Schlüsselwiderrufstests |
| Graphics / GPU / Audio / Input | `system/graphics/`, `system/audio/`, `system/devices/` | User-Space-Server/Treiber | PARTIAL | Berechtigungsmodell für Bildschirm/Kamera/Mikrofon/Input und Hardwaretests |
| Timer / Interrupt / HAL / Boot | `kernel/src/timer.rs`, `interrupts.rs`, `hal.rs`, Bootmodule | minimal privilegierte Kernel-/HAL-Mechanismen | PARTIAL | konkrete Hardware-/QEMU-Nachweise, sichere IRQ-Zuordnung und Bootbelege |
| Logging / Diagnostics / Audit | `system/diagnostics/`; Kernel-Diagnostik | User-Space-Sammlung; Kernel liefert begrenzte Events | PARTIAL | Queue-Limits, Tamper-Evidence, Ausfallverhalten und keine geheimen Daten in Logs |
| Trusted Computing Base (TCB) | `kernel/src/lib.rs`, `kernel_init.rs`, Kernel-Dependencies | Scheduler, Speicher-/Adressraum-Schutz, IPC, Capability-Enforcement, Interrupt-/HAL-/Boot-Minimum | NOT_VERIFIED | aus Call-Sites/Imports/Features erzeugte TCB-Liste, `unsafe`-Review, Boot- und Isolationsevidenz |

## 4. P0/P1-Migrationsblocker

### P0 — zuerst lösen

1. Erzeuge eine symbol- und call-site-basierte Liste aller aktiven Kernel-Exports, direkten Aufrufe, Cargo-Abhängigkeiten und Feature-gesteuerten Pfade. Dokumentationslisten allein reichen nicht.
2. Belege echte Adressraumisolation und Capability-Enforcement über mindestens zwei getrennte Prozesse, einschließlich unautorisierter IPC-, Memory- und Handle-Zugriffe.
3. Entkopple `shivacore-service-space` von `shivacore`. Ersetze Kernel-Interna nur durch die schmale, versionierte `libshivacore`-ABI bzw. explizite Service-Ports; keine ABI-Fassade darf privilegierte Implementierungen offenlegen.
4. Behandle `atcfs.rs` als Sicherheits-/Integritätsrisiko: die aktuelle Hashfunktion ist im Quelltext ausdrücklich vereinfacht und kein SHA3. Keine Content-ID darf als kryptografisch sicher gelten, bis eine echte getestete Implementierung verwendet wird.
5. Plane die VFS-Migration, ohne Bootfähigkeit oder Capability-Prüfungen zu umgehen. Der bestehende Kernel-Bootpfad instanziiert ATCFS und VFS; Entfernen ohne Ersatz-/Integrationspfad wäre ein Funktionsregressionsrisiko.

### P1 — nach P0

- Treiber-Service-IPC einschließlich MMIO-/DMA-/IRQ-Rechten und IOMMU-Fähigkeit.
- Block-/Storage-/Netzwerkdienste als tatsächlich isolierte Prozesse mit begrenzten Queues und Ressourcen.
- Supervisor für Service-Start, Neustart, Beendigung, Abhängigkeiten und sichere Recovery.

### P2/P3

- Update-/Paketpfad mit Signaturprüfung exakt der aktivierten Bytes.
- Aurora AI, ATC-VM, Blockchain und UI über User-Space-Service-Verträge, Sandbox und widerrufbare Capabilities.

## 5. Evidenzlücken und offene Risiken

- **Nicht erhoben:** vollständige symbolische Call-Site-Suche über alle Rust-Dateien, alle Features/Targets, `unsafe`-Inventur und transitive Abhängigkeitsanalyse.
- **Nicht nachgewiesen:** separater Prozess/Adressraum für jeden Service, gebootete Ring-3-Ausführung, QEMU-/Hardware-Driver-IPC und IRQ-/DMA-Isolation.
- **Nicht ausgeführt:** Tests, Clippy, rustfmt, Build und CI auf dem Audit-Branch oder auf einem aktuellen Commit. Frühere Testergebnisse werden nicht übertragen.
- **Nicht umgesetzt in diesem Commit:** Entfernen von Kernel-Dateisystem-/Netzwerk-/Blockchainpfaden. Eine solche Änderung ohne vorherige Call-Site-Inventur wäre unsicher.
- **Offener konkreter Risikofund:** `atcfs.rs` nennt seinen Hash selbst vereinfacht/nicht produktiv.
- **Offener Architekturblocker:** Service Space hängt direkt von `shivacore` ab und verwendet laut vorherigem Audit Kernel-Interna.

## 6. Akzeptanzkriterien für jede künftige Migration

Eine Komponente wechselt erst zu `VERIFIED`, wenn (a) produktiver Code statt Stub existiert, (b) die tatsächliche User-Space-Ausführung belegt ist, (c) Capability-/IPC-Checks integriert sind, (d) positive und negative Tests einschließlich Crash-/Ressourcenfehlern bestehen, (e) Boot-/Integrations-/Regressionstests auf derselben SHA erfolgreich sind, (f) die TCB-/Abhängigkeitsänderung dokumentiert ist und (g) CI-Nachweise auf genau dieser SHA vorliegen.

## 7. Folgearbeit

Dieser PR ist die Phase-1-Baseline. Nach Review sollen die eigentlichen Codeänderungen in kleinen, getrennten PRs erfolgen: **P0.1 ABI-/Service-Space-Entkopplung**, **P0.2 IPC-/Adressraum-/Capability-Nachweise**, **P0.3 ATCFS-Integritätskorrektur und VFS-Migrationsplan**. Alte Kernelimplementierungen erst entfernen, wenn deren Ersatz, Call-Sites und Regressionstests nachgewiesen sind.
