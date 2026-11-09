# Zero Trust Code Execution — Forensische Bestandsaufnahme

**Status:** Bestandsaufnahme. **Keine Codeänderung, kein neuer Standard, keine Implementierung.**
**Stand:** a-townchain `3c5b3af`, globus-os `77e3c8c` (`modules/atc-shivacore`)
**Datum:** 09.11.2026
**Zweck:** Erfassung der real existierenden Ausführungspfade, bevor irgendetwas gebaut wird
(Reihenfolge-Vorgabe aus der Architekturentscheidung vom 09.11.2026: Bestandsaufnahme → Authority → Admission → Enforcement).

## 1. Ergebnis in einem Satz

Es gibt heute **keinen einzigen Code-Admissionspunkt** im Ökosystem: der Kernel lädt ELF-Bytes aus der
Ramdisk ohne jede Prüfung, die ATC-VM führt strukturell geprüften Bytecode ohne Code-Identität aus, und
Prozesse starten ohne Capability-Voraussetzung. Die vorhandenen Bausteine (Capability-Table, Multi-Sig mit
`data_hash`, NX, VM-Verify plus Gas, Hash-Registry der Verträge) sind aber genau die Teile, aus denen eine
Authority zusammengesetzt werden kann.

## 2. Methode

Gelesen wurde der tatsächliche Code, nicht die Dokumentation: `kernel/src` (elf_loader, lkm, capability,
security, dma_auth, vmm, addr_space, process, user_sched, system, main), `service_space/src`,
`components/vm`, `components/node`, `ai/*`. Jede Aussage unten ist mit Datei und Zeile belegt.

## 3. Ausführungsflächen (Ist-Zustand)

| # | Fläche | Ort | Aufnahme heute | Code-Identität | Autorität |
|---|---|---|---|---|---|
| 1 | ELF-Programmlader | `kernel/src/elf_loader.rs` (1243 Z.), Aufruf `kernel/src/main.rs:339,372` | Ramdisk-Bytes werden geparst und geladen | **keine** (kein Hash, keine Signatur, keine Herkunft) | **keine** |
| 2 | Prozessstart | `kernel/src/process.rs:80`, `kernel/src/user_sched.rs:726` | `spawn(ptype, priority)` | keine | keine; `CapabilityTable` kommt in `user_sched.rs`/`system.rs` **nicht** vor |
| 3 | LKM / Kernelmodule | `kernel/src/lkm.rs` | Modell mit `std::collections`, also Host-Simulation | keine | kein `insmod`-Pfad im Boot |
| 4 | ATC-VM | `components/vm/src/vm.rs:101` `verify()` | strukturell: leer/`MAX_PROGRAM_OPS`/Storage/Stack/Sprungziele; Gas via `gas_cost` + `execute_state_transition_with_gas` | keine | keine Aufrufer-Autorität: wer Bytecode hat, führt ihn aus |
| 5 | ATC-Node | `components/node` | `0` Treffer für `atc-vm` in `Cargo.toml` → führt **keine** Verträge aus; prüft Blöcke/Konsens (`chain.rs:90`, `consensus.rs:73`) | – | – |
| 6 | Aurora AI | `ai/aurora-ai` | Frontend-Modul (atc-aistudio, React) plus Python-Werkzeuge; **keine** Tool-Runtime mit Allowlist im Betrieb | – | – |
| 7 | Genesis Engine | `ai/genesis-*` | Doku und Verträge; kein generierender Ausführungspfad | – | – |
| 8 | Vertrags-Registry | `tools/atc-sc-validator` | `source_sha256` + `abi_sha256`, Gate 16/16 | **ja, für Quellen** | bindet Quelle/ABI, **nicht** Ausführung |

Damit sind real nur **drei** Grenzen zu verdrahten: ELF-Lader, VM-`execute`, Prozess-Spawn. Alles andere
existiert noch nicht und braucht auch keinen nachträglichen Gate.

## 4. Die sieben Schutzschichten im Ist-Zustand

| Schicht | Zustand | Beleg / Lücke |
|---|---|---|
| Secure Boot | **TEIL** | BIOS- und UEFI-Images bauen, QEMU-Smoke läuft; keine Signaturkette, keine Messung |
| Code Signing | **FEHLT** | keine Signaturprüfung in `kernel/src` oder `service_space/src`; Ed25519 in `security.rs` signiert Multi-Sig-Anträge und DIDs, **nicht** Code |
| Code Integrity | **TEIL** | Hash-Registry für Vertragsquellen; für Binaries und Bytecode nichts |
| Execution Control | **FEHLT** | deny-by-default an keiner Grenze |
| Capability Security | **TEIL** | `capability.rs` (Rights `u16`, `create_typed`), Syscall-Gate `0x80`, `dma_auth.rs`; aber nicht als Voraussetzung der Codeaufnahme |
| Runtime Isolation | **TEIL** | Ring 3, eigener Adressraum, Daten NX (`vmm.rs:881`, `main.rs:1344`); W^X für Codesegmente nicht belegt, keine CFI |
| Continuous Attestation | **FEHLT** | `0` Treffer für `attest`, `tpm`, `measured_boot` in `kernel/src` |

## 5. Angriffsmatrix gegen den heutigen Stand

| Angriff | Heute blockiert? | Warum |
|---|---|---|
| Fremder Code eingeschleust | **nein** | Ramdisk-ELF wird ohne Prüfung geladen (#1) |
| Signierte Software manipuliert | **nein** | es wird nichts signiert oder geprüft |
| Schadcode über Updates | **nein** | kein signierter Update-Pfad im Kernel |
| Manipulierte Treiber | **teilweise** | kein echter LKM-Loader; `dma_auth.rs` prüft Device-Caps, nicht Treiberherkunft |
| Schadcode durch Aurora AI | **heute n/a** | keine Tool-Runtime — und damit auch keine Allowlist für später |
| Generierter Schadcode | **heute n/a** | kein Ausführungspfad |
| Speicherbasierte Einschleusung | **teilweise** | NX für Daten gesetzt; W^X der Codesegmente nicht nachgewiesen, keine CFI |
| Privilegienausweitung | **teilweise** | Syscall-Gate prüft Rechte; Prozessstart bindet keine Capabilities |
| Dynamisches Nachladen / JIT | **fehlt** | weder geregelt noch verboten |
| Kompromittierte Abhängigkeiten | **fehlt im Betrieb** | Dependabot ist Repo-Ebene, kein Runtime-Bezug, keine Build-Provenance |

## 6. Wiederverwendbare Bausteine (kein Neubau nötig)

- `security.rs` (1161 Z.): `MultiSigProposal` mit `data_hash`, Signatursammlung, Audit-Log, Reputation.
  Wird in `kernel_init.rs:156` initialisiert, ist im Betrieb aber **ungenutzt** (kein Aufruf außerhalb
  `security.rs`/`lkm.rs`) — für "jeder Freigabe- und Blockiervorgang protokolliert" muss das erst angebunden werden.
- `capability.rs` (537 Z.): Rights-Modell und `create_typed` — die Rechte-Seite der `ExecutionPolicy` liegt vor.
- `dma_auth.rs` (369 Z.): Muster für "eine Cap autorisiert genau ihr Objekt, kein Erweitern" — direkt
  übertragbar auf "eine Freigabe autorisiert genau diesen Hash".
- Registry-Gate: `source_sha256`/`abi_sha256` mit 16/16 PASS — das Prüf­muster für eine Code-Registry existiert.
- VM `verify()` plus Gas: die Strukturprüfung steht, es fehlt nur die Vorschaltung.

## 7. Offene Punkte, die vor dem Standard entschieden werden müssen

1. **Root of Trust:** Ohne TPM oder gemessene Boot-Kette hat die Signaturprüfung keine verankerte Wurzel.
   Das ist eine Hardware- und Boot-Entscheidung, keine Codearbeit.
2. **Wo lebt die Registry, und wer prüft sie?** Liegt sie auf der Ramdisk, prüft sie sich selbst — Zirkularität.
3. **Widerruf ohne Netz:** Der Boot muss ohne Revocation-Quelle entscheiden können (Offline-Policy, Fristen).
4. **Determinismus:** Betrifft Admission die Vertragsausführung, muss die Entscheidung im Konsens reproduzierbar sein.
5. **Verhältnis zu ATC-LIC:** Das Lizenz-Gate sitzt an derselben Grenze wie das Ausführungs-Gate. Reihenfolge
   und Zuständigkeit sind ungeklärt; zwei Gates an einer Grenze ohne Vorrangregel sind ein Fehlerrisiko.
6. **Signer-Autorität:** Wer darf signieren, und wie wird diese Liste selbst autorisiert?

## 8. Empfohlene Reihenfolge (bestätigt)

1. `ATC-SEC-EXEC-001` als **Kandidat** schreiben (Felder, Prüfreihenfolge, Fehlerfälle, Logpflicht), Ratifizierung separat.
2. Kernel-Admission am ELF-Lader — der einzige real existierende Nachladepfad.
3. VM-Admission (Bytecode-Hash plus Signer) — klein, weil `verify()` schon steht.
4. Prozessstart an Capabilities binden.
5. Attestation-Wurzel entscheiden (TPM gegen gemessene Kette), bevor Code dafür entsteht.

## 9. Grenzen dieser Aufnahme

Gelesen wurde Code, kein Laufzeitverhalten. Es gab keinen dynamischen Test gegen Testnet oder Mainnet, keinen
Pentest und keinen Versuch, die genannten Grenzen real zu umgehen. Die Aussagen gelten für die oben genannten
Commits; spätere Änderungen können sie verschieben.
