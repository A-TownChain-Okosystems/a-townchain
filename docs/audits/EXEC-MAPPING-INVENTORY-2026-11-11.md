# EXEC-MAPPING-INVENTUR — wo im ShivaCore-Kernel ausführbarer Speicher entsteht

> **Datum:** 11.11.2026 · **Art:** read-only Bestandsaufnahme, keine Codeänderung
> **Basis:** `globus-os` main `4fccb823`, 95 Rust-Dateien in `modules/atc-shivacore`, davon 48 in `kernel/src`; `a-townchain` main `1dd634b6c`
> **Auftrag:** vor der Integration des Enforcement-Punkts jede Stelle erfassen, an der ausführbare Speicherseiten entstehen (Owner-Vorgabe 11.11.2026)

---

## 0. Kurzfassung

Es gibt **zwei Mapping-Pfade** im Kernel. Der modellierte Pfad in `vmm.rs` hat Wächter
(W^X, HHDM-Policy, transaktionaler Rücklauf). Der Pfad, der das Nutzerabbild tatsächlich
installiert (`userspace.rs`), geht **daran vorbei**: er ruft `OffsetPageTable::map_to` direkt
mit rohen `PageTableFlags`.

Daraus folgen drei Befunde, die den Plan ändern:

1. **Daten- und Stackseiten sind heute ausführbar.** Sie werden `WRITABLE` und **ohne**
   `NO_EXECUTE` abgebildet. Der vom Owner gefürchtete „alternative Ausführungspfad" existiert
   also bereits, unabhängig von jedem Loader.
2. **Vier Aufrufstellen** installieren Nutzerabbilder (`main.rs` 782, 847, 948, 987). Keine
   davon kennt eine Freigabe. Ein Gate an einer Stelle allein lässt drei offen.
3. **`components/exec-admission` ist nicht kernel-tauglich wie es ist:** `std`, `String` im
   Ablehnungspfad, `Vec<u8>` in der Anfrage, und es hasht nicht selbst. Der Kernel muss den
   Hash über die Bytes bilden, die er abbildet.

---

## 1. Alle Stellen, an denen ausführbarer Speicher entsteht

| # | Stelle | Was passiert | Wächter heute | Admission |
|---|---|---|---|---|
| 1 | `kernel/src/userspace.rs:197` `map_user_binary` | installiert das Nutzerabbild: Code, Daten, Stack | nur Bereichsprüfung (`204-210`, `253-255`), `entry` im Fenster | **keine** |
| 2 | `userspace.rs:212` `map_region` → `:233` `mapper.map_to` | rohe `PageTableFlags` je Seite | kein `MappingFlags::validate`, kein HHDM-Test | **keine** |
| 3 | `userspace.rs:257-264` Codesegment | `USER_ACCESSIBLE \| WRITABLE`, also **W+X während der Installation** | danach entzogen (siehe 4) | **keine** |
| 4 | `userspace.rs:306-318` `mapper.update_flags` | Code verliert `WRITABLE` → final `PRESENT \| USER_ACCESSIBLE` | keine; generischer Rechteheber | **keine** |
| 5 | `userspace.rs:268-275` Datensegment | `USER_ACCESSIBLE \| WRITABLE`, **kein `NO_EXECUTE`** | keine | **keine** |
| 6 | `userspace.rs:280-287` Stack | `USER_ACCESSIBLE \| WRITABLE`, **kein `NO_EXECUTE`** | keine | **keine** |
| 7 | `main.rs:782, 847, 948, 987` | vier Aufrufe von `map_user_binary` (Smoke-Pfad, Ramdisk-Init, weiterer Prozess, zweiter Bootpfad) | – | **keine** |
| 8 | `kernel/src/spawn.rs:61` `plan` / `:83` `plan_retained` | löst Namen auf, parst ELF, prüft `ET_EXEC` und Einstiegsfenster | strukturell (`ElfParser`) | **keine**; kein Syscall-Arm, also vorbereitet und ungenutzt |
| 9 | `kernel/src/elf_loader.rs:277` `mgr.load_binary` | Modellpfad `UserspaceManager`, 1243 Zeilen Parser und Signale | `is_executable()` ist ELF-Segment-Semantik, keine Rechtspolitik | **keine**; kein Hash, keine Signatur |
| 10 | `kernel/src/vmm.rs:336` `map_page`, `:355` `map_page_inner`, `:579` `map_owned` | **validierter** Pfad: `MappingFlags::validate` (`134-140`) verbietet W+X, `validate_target_policy` (`462-473`) erzwingt HHDM = Supervisor/RW/NX | ja | keine, aber Wächter vorhanden |
| 11 | `kernel/src/allocator.rs:40` | Kernel-Heap | `map_to` mit Kernel-Flags | nicht nötig (kein Nutzerpfad) |
| 12 | `kernel/src/dma_auth.rs:150` `map_window` | Gerätefenster | eigene Prüfungen | nicht nötig (kein Ausführungsrecht) |
| 13 | `legacy/kernel-src-uncompiled/page_fault.rs:372`, `cow.rs:175/464` | weitere `map_page(..., executable)`-Pfade | – | **nicht gebaut** (14 Dateien, 20 891 LOC); bei Reaktivierung ein fünfter Pfad |
| 14 | `kernel/src/lkm.rs` | Modul-Laden | `std::collections` → Host-Simulation, kein echter `insmod`-Pfad | Stufe „dynamisches Modul" existiert heute nicht |

**Nicht vorhanden:** ein `mmap`- oder `PROT_EXEC`-Syscall. `syscall.rs` kennt kein
Speicher-Mapping. Stufe 2 des Geltungsbereichs ist damit heute nur kernelintern erreichbar
(über 1 bis 7), nicht vom Nutzer.

---

## 2. Der eigentliche Befund: der reale Pfad umgeht die Wächter

`vmm.rs` prüft, dass W und X nie gleichzeitig gesetzt sind (`MappingFlags::validate`), und dass
HHDM-Abbildungen Supervisor, beschreibbar und NX sind. `userspace.rs` benutzt diese Typen
nicht. Es rechnet mit `PageTableFlags` aus der `x86_64`-Crate und ruft `map_to` direkt:

```
userspace.rs:263  PageTableFlags::USER_ACCESSIBLE | PageTableFlags::WRITABLE   // Code, W+X
userspace.rs:274  PageTableFlags::USER_ACCESSIBLE | PageTableFlags::WRITABLE   // Daten
userspace.rs:286  PageTableFlags::USER_ACCESSIBLE | PageTableFlags::WRITABLE   // Stack
```

`PageTableFlags::empty()` bedeutet: kein `NO_EXECUTE`. Damit sind Daten und Stack
**beschreibbar und ausführbar**, solange das NX-Bit nicht gesetzt wird. Ein Sprung in einen
String, in einen Puffer oder auf den Stack ist damit ein gültiger Ausführungspfad, ganz ohne
Loader. Ein Admission-Gate am ELF-Pfad würde daran nichts ändern.

**Vorbehalt dieser Messung:** ob `EFER.NXE` aktiv ist, wurde in diesem Lauf nicht geprüft. Ist
NX deaktiviert, ist ohnehin jede Seite ausführbar; dann ist der Befund noch schärfer, nicht
schwächer. Beides gehört in den ersten Test des nächsten Schnitts.

Zweiter Teil desselben Befunds: während der Installation ist die **Codeseite** W+X
(`263`), erst danach wird W entzogen (`311-318`). Das ist ein legitimes Installationsmuster,
aber es bedeutet, dass im Fenster zwischen `map_to` und `update_flags` beschreibbarer
ausführbarer Speicher existiert. Wer dieses Fenster nutzen kann, braucht kein Gate zu umgehen.

---

## 3. Eignung von `components/exec-admission` für den Kernel

Geprüft am Code (`components/exec-admission/src/lib.rs`, `Cargo.toml`):

| Punkt | Befund | Folge für den Kernel |
|---|---|---|
| `no_std` | **nein** (`#![no_std]` fehlt, `std::` kommt vor) | `no_std + alloc`-Variante nötig, sonst nicht einbindbar |
| Abhängigkeiten | **keine** | gut: keine Krypto-Kante, keine Fremdlast |
| Hashberechnung | **keine** (`sha2`/`blake3` kommen nicht vor) | der Kernel muss `observed_hash` über die tatsächlich abgebildeten Bytes bilden |
| Anfragefelder | `code_hash`, `observed_hash`, `policy_bytes: Vec<u8>`, `signature: Option<&[u8]>`, `now_us` | `Vec` braucht den Kernel-Allokator; besser als Slice übergeben |
| Ablehnungsdetail | `Decision.detail: String` | **Allokation im Ablehnungspfad**, im Kernel ungeeignet (Allokationsfehler bei knappem Speicher genau dann, wenn abgelehnt wird) |
| Policy-Felder | `signer_identity: String`, `source_provenance: Option<String>`, `generation_proof: Option<String>` | ebenfalls `String`; für den Kernel als feste Bytefelder oder Index in eine Tabelle |
| Fehlercodes | 11 unterscheidbare `AdmCode`-Varianten mit `rank()` = Prüfschritt | **passt** zur Forderung „eigenständiger Grund je Stufe" |

Ergebnis: Die **Entscheidungslogik ist wiederverwendbar**, ihre **Darstellung nicht**. Der
Kernel braucht dieselbe Logik mit Slice-Eingaben, festen Fehlercodes und ohne `String` im
Ablehnungspfad. Ein zweites Regelwerk ist damit nicht nötig, nur eine zweite Signatur
derselben Regeln.

---

## 4. Reihenfolge für den ersten Implementierungsschnitt

Die Reihenfolge folgt aus den Befunden, nicht aus Bequemlichkeit:

| Gate | Arbeit | Warum an dieser Stelle |
|---|---|---|
| G-1 | Daten- und Stackseiten mit `NO_EXECUTE` abbilden (`userspace.rs:274`, `286`), NXE-Zustand beim Boot prüfen und protokollieren | ohne NX ist jedes Admission-Gate wirkungslos, weil Daten schon ausführbar sind |
| G-2 | `userspace.rs` auf die validierten Typen aus `vmm.rs` führen oder gleichwertige Prüfung einbauen (W^X, HHDM, Bereich) | der reale Pfad umgeht heute alle Wächter |
| G-3 | einen einzigen Nutzer-Mapping-Punkt schaffen, den alle vier Aufrufstellen (`main.rs:782/847/948/987`) benutzen | vier offene Türen sind keine Tür |
| G-4 | `observed_hash` im Kernel über die abzubildenden Bytes berechnen, nie aus dem Aufruf übernehmen | TOCTOU: Prüfung muss dieselben Bytes treffen, die ausgeführt werden |
| G-5 | `exec-admission` als `no_std + alloc`-Variante mit Slice-Eingaben und festen Codes bereitstellen, `detail` als Code statt `String` | sonst Allokation im Ablehnungspfad |
| G-6 | Admission-Aufruf **vor** dem ersten Mapping mit Ausführungsrecht, Protokoll je Stufe, fail-closed | die eigentliche Durchsetzung |
| G-7 | `update_flags` als Rechteheber unter dasselbe Gate stellen (später `mmap`, Module, JIT) | Stufen 3 bis 5 des Geltungsbereichs |

**G-1 vor G-6 ist keine Geschmacksfrage:** ein Gate am ELF-Pfad, während Daten und Stack
ausführbar bleiben, dokumentiert eine Grenze, die es nicht gibt.

---

## 5. Testmatrix des ersten Schnitts

Die elf Gegenproben des Owners, ergänzt um zwei, die aus dieser Inventur folgen:

| Test | Erwartung |
|---|---|
| signiertes, autorisiertes ELF | Ausführung erlaubt |
| unbekannter Herausgeber | abgelehnt, eigener Grund |
| manipuliertes ELF | `IntegrityMismatch` |
| widerrufener Schlüssel | `Revoked` |
| abgelaufene oder zurückgerollte Version | `ExpiredOrRolledBack` |
| fehlende Capability | `CapabilityNotGranted` |
| direkter Loader-Bypass | vom Kernel verhindert |
| direkte Ausführungsanforderung (`mmap`-Äquivalent) | Admission erforderlich |
| Änderung zwischen Prüfung und Laden | abgelehnt (Hash über abgebildete Bytes) |
| Fehler in einer Stufe | keine Freigabe, kein Teilprozess |
| Protokollprüfung | eigener Grund je Stufe |
| **neu: Sprung in Datenseite** | Seitenfehler (NX), nicht Ausführung |
| **neu: Sprung in Stackseite** | Seitenfehler (NX), nicht Ausführung |

Die letzten beiden sind heute **nicht** erfüllt. Sie sind der Grund, warum G-1 zuerst kommt.

---

## 6. Was diese Inventur nicht ist

Keine Änderung, kein Testlauf, keine Messung am laufenden System. Sie ist die Landkarte für
den Schnitt, den der Owner entschieden hat: Kernel-Enforcement, fail-closed, vor der Freigabe
ausführbaren Speichers.
