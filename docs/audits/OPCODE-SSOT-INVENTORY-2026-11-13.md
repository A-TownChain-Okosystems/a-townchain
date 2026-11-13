# OPCODE-SSOT-INVENTUR — drei Opcode-Vokabulare im ATC-Stack

> **Datum:** 13.11.2026 · **Art:** read-only Bestandsaufnahme (E2-A/E2-B), keine Codeaenderung
> **Basis:** `a-townchain` main `d5e5e53` · `atclang` main `516482a` · `atc-vm` main `21132a49`
> **Auftrag:** Owner-Freigabe E2 (ATC-BC-001 als einzige normative Opcode-Quelle), Teilschritte E2-A und E2-B
> **Grenze:** Vor der Inventur wurden keine Opcode-Werte geaendert und keine Implementierung als fehlerhaft deklariert.

---

## 0. Kurzfassung

Im Stack existieren **drei** getrennte Opcode-Vokabulare, nicht zwei:

1. **ATC-BC-001** (frozen): 19 Opcodes, big-endian, Container `ATCB` + u16-Version + u32-Count, keine Checksum.
2. **ATC-BC-002** (Kandidat, ausdruecklich nicht normativ): 19 Opcodes `0x01..0x13`, little-endian, Container mit Generationsbyte und SHA-256-Tail.
3. **`.ops`-Textformat** der VM (`Push 1`, `Add`, `Halt`): kein Byteformat, keine ABI, 19 Namen in `components/vm`, 16 Namen im Einzelrepo `atc-vm`.

Die beiden Binaertabellen belegen **sieben gemeinsame Bytewerte** (`0x01`, `0x02`, `0x03`,
`0x10`, `0x11`, `0x12`, `0x13`) und **keiner** davon bedeutet dasselbe. `0x10` ist `Add`
nach BC-001 und `Halt` nach BC-002. Dazu kommt ein dritter, verschobener Kontrollfluss im
Legacy-Encoder des Compilers (`Return` als `0x30`, in BC-001 `Jump`), der heute nur von
Tests benutzt wird.

Der Live-Pfad des Monorepos versiegelt Vertraege als **Generation 1 der VM**, also mit der
BC-002-Belegung. Diese Generation 1 ist **nicht** ATC-BC-001, obwohl beide „v1" heissen.
Das ist der Kern der Abweichung, den E2-C aufloesen muss.

Kein Statusclaim: die unten genannten Testzahlen sind lokale Laeufe. Ein `CI_VERIFIED`
gibt es fuer keinen dieser Staende, weil GitHub Actions im Konto nicht startet.

---

## 1. Auftrag und Grenzen

E2-A verlangt eine Inventur aller Opcode-Definitionen, E2-B eine Abweichungsmatrix
zwischen Standard, ATCLang-Compiler und ATC-VM. Beides ist hier dokumentiert.

Nicht Teil dieses Dokuments:

- keine Aenderung an Bytewerten, Tabellen oder Encodern,
- keine Aussage darueber, welche Implementierung „falsch" ist,
- keine Ratifizierung von ATC-BC-002 (das ist eine Owner-Entscheidung, siehe E2-C).

## 2. Inventur — wo Opcodes definiert sind

| # | Artefakt | Ort | Inhalt | Normativ |
|---|---|---|---|---|
| 1 | ATC-BC-001 v1.0.0-FROZEN | `atclang/specs/bytecode/SPEC.md` §2/§3 | 19 Opcodes, big-endian, 10-Byte-Kopf, keine Checksum | **ja, frozen** |
| 2 | Compiler kanonisch | `atclang/crates/atc-core/src/bytecode.rs` (`encode_canonical`, `decode_bytecode`) | exakt Tabelle 1, BE, unbekannter Opcode = Fehler | Implementierung von 1 |
| 3 | Compiler Legacy-Encoder | `bytecode.rs` (`Instruction::encode`, `Bytecode::encode`) | little-endian, Kontrollfluss verschoben | **nein**, im Code als LEGACY/NON-CANONICAL markiert |
| 4 | Compiler-Aussencontainer | `atclang/crates/atc-core/src/artifact.rs` | Magic `ATCA`, je Funktion ein eingebettetes ATCB-v1-BE | Implementierung, kein eigener Katalog |
| 5 | ATC-BC-002 Kandidat | `a-townchain/standards/atc/candidates/ATC-BC-002-CANDIDATE.md` | 19 Opcodes `0x01..0x13`, LE, Generationsbyte, SHA-256-Tail | **nein**, „KANDIDAT · nicht normativ" |
| 6 | VM-Binaerformat | `a-townchain/components/vm/src/artifact.rs` | implementiert Tabelle 5, Selbstbezeichnung „ATC-BC-002, Kandidat" | nicht normativ |
| 7 | VM-Spec-Overview | `a-townchain/components/vm/docs/specs/SPEC-OVERVIEW.md` | behauptet Bindung an ATC-BC-001 | Doku gegen Code |
| 8 | Uebersetzer-Backend | `a-townchain/components/system/src/backend.rs` | bildet `Instruction` per **Namen** auf `atc_vm::vm::Op` ab | Bruecke 2→6 |
| 9 | Einzelrepo-VM | `A-TownChain-Okosystems/atc-vm` main `21132a49` | Textformat `.ops`, 15 Ops, kein `artifact.rs` | zweite VM-Version |

Alle sechs gemeinsamen Quelldateien zwischen `atc-vm` (Repo) und `components/vm`
(Monorepo) haben unterschiedliche Blob-SHAs. Nur `components/vm` kennt das Binaerartefakt.

## 3. ATC-BC-001 (frozen, normative Referenz)

| Opcode | Instruction | Operand (big-endian) |
|---:|---|---|
| `0x01` | ConstI64 | i64 |
| `0x02` | LoadLocal | u16 |
| `0x03` | StoreLocal | u16 |
| `0x10` | Add | - |
| `0x11` | Sub | - |
| `0x12` | Mul | - |
| `0x13` | Div | - |
| `0x14` | Neg | - |
| `0x15` | Eq | - |
| `0x16` | Ne | - |
| `0x17` | Lt | - |
| `0x18` | Gt | - |
| `0x19` | Le | - |
| `0x1A` | Ge | - |
| `0x20` | Call | function:u16, argc:u16 |
| `0x21` | Return | - |
| `0x30` | Jump | i16 |
| `0x31` | JumpIfFalse | i16 |
| `0x40` | Pop | - |

Container: `ATCB` + format_version u16 BE (`1`) + instruction_count u32 BE, 10 Byte Kopf,
kein Hash im Container. Ablehnungsregeln REQ-BC-F1..F6 (Magic, Version, Count-Ueberlauf,
unbekannter Opcode, Truncation, Trailing Bytes) sind im Standard normativ und im Compiler
(`decode_bytecode`) umgesetzt.

## 4. ATC-BC-002 (Kandidat, nicht normativ)

| Byte | Befehl | Operand (little-endian) |
|---:|---|---|
| `0x01` | Push | u64 |
| `0x02` | Add | - |
| `0x03` | Sub | - |
| `0x04` | Mul | - |
| `0x05` | Div | - |
| `0x06` | Dup | - |
| `0x07` | Swap | - |
| `0x08` | Eq | - |
| `0x09` | Lt | - |
| `0x0A` | Load | u32 |
| `0x0B` | Store | u32 |
| `0x0C` | Caller | - |
| `0x0D` | Jump | u32 |
| `0x0E` | JumpIfZero | u32 |
| `0x0F` | JumpIfNotZero | u32 |
| `0x10` | Halt | - |
| `0x11` | Assert | - (v2) |
| `0x12` | Revert | u32 (v2) |
| `0x13` | RevertIf | - (v2) |

Container: `ATCB` + Containerformat `0x01` + **Generationsbyte** + u32 LE Count, danach
Strom, danach SHA-256-Tail ueber alle vorhergehenden Bytes. Der Kandidatentext haelt
ausdruecklich fest: „aendert ATC-BC-001 nicht"; ein Artefakt deklariert seine Generation,
und der Verifier lehnt v2-Befehle in einem v1-Artefakt ab.

## 5. `.ops` — Textformat, kein Byteformat

`components/vm/src/ops.rs` parst ein Zeilenformat (`Push 1`, `Load 0`, `Jump 4`, `Halt`,
`Assert`, `Revert 5`, `RevertIf`) und akzeptiert 19 Namen. Das Einzelrepo `atc-vm` kennt
davon 16 (ohne `Assert`, `Revert`, `RevertIf`). Das Format ist ein Assembler-Eingabeformat
fuer Fixtures und den EXEC-CHAIN-Pfad; es definiert **keine** Bytebelegung und darf nicht
als ABI oder als Opcode-Quelle referenziert werden.

## 6. Abweichungsmatrix B-1: numerische Kollision

| Byte | ATC-BC-001 (frozen) | ATC-BC-002 (Kandidat) | gleiche Bedeutung |
|---|---|---|---|
| `0x01` | ConstI64 (i64) | Push (u64) | nein |
| `0x02` | LoadLocal | Add | nein |
| `0x03` | StoreLocal | Sub | nein |
| `0x10` | Add | Halt | nein |
| `0x11` | Sub | Assert | nein |
| `0x12` | Mul | Revert | nein |
| `0x13` | Div | RevertIf | nein |

Summe: 7 gemeinsame Bytes, 0 identische Bedeutungen, 12 nur in BC-001, 12 nur in BC-002.

## 7. Abweichungsmatrix B-2: Namenskollision der Generationen

ATC-BC-002 nennt seine Basisbefehle „Generation 1" (`0x01..0x10`). Diese Generation 1 ist
eine eigene Belegung derselben Bytes und **nicht** ATC-BC-001.

- Die VM prueft die Generation nur fuer `0x11..0x13` (`ist_v2_opcode`); `0x01..0x10` gelten
  in beiden Generationen.
- Schutz heute ist allein der Container: BC-001 schreibt als viertes Byte `0x00` (hohes
  Byte der u16-Version `1`), BC-002 schreibt `0x01` (Containerformat). Ein BC-001-Strom
  scheitert daher an der Containerpruefung, bevor Bytes gedeutet werden.
- Ohne diesen Containerunterschied wuerde ein BC-001-Strom stillschweigend als andere
  Befehlsfolge ausgefuehrt. Diese Abhaengigkeit ist implizit und wird in E2-C explizit
  gemacht.

## 8. Abweichungsmatrix B-3: Container und Integritaet

| Merkmal | ATC-BC-001 | ATC-BC-002 | `.ops` |
|---|---|---|---|
| Magic | `ATCB` | `ATCB` | - |
| Kopf | 10 B (Magic, u16 BE, u32 BE) | 10 B (Magic, Container, Generation, u32 LE) | Kopfkommentare |
| Endianness | big | little | - |
| Integritaet | keine im Container (ATCA-Schicht) | SHA-256-Tail | - |
| Unbekannter Opcode | Fehler | Fehler `E-010` | Fehler `UnknownOp` |
| Falsche Generation | - | Fehler `E-011` | - |

## 9. Abweichungsmatrix B-4: Legacy-Encoder im Compiler

| Instruction | BC-001 | Legacy-Encoder | Folge |
|---|---|---|---|
| Return | `0x21` | `0x30` | kollidiert mit BC-001 `Jump` |
| Pop | `0x40` | `0x31` | kollidiert mit BC-001 `JumpIfFalse` |
| Jump | `0x30` | `0x40` | kollidiert mit BC-001 `Pop` |
| JumpIfFalse | `0x31` | `0x41` | ausserhalb des Katalogs |

Aufrufer: ausschliesslich Tests innerhalb von `bytecode.rs`. Der Produktionspfad
(`Artifact::from_program` → `encode_canonical`) nutzt die kanonische Tabelle. Der
Legacy-Encoder ist damit ungenutzt, aber oeffentliche API ohne Katalogbindung.

## 10. Abweichungsmatrix B-5: Semantik der Bruecke

`components/system/src/backend.rs` uebersetzt `Instruction` per Namen nach `Op` (keine
Byte-Ebene). Im Code selbst als `DIVERGENZEN` gefuehrt:

| Thema | atc-core | atc-vm | Behandlung heute |
|---|---|---|---|
| Ueberlauf | geprueft, Fehler | umlaufend | offen |
| `Lt` | vorzeichenbehaftet | ohne Vorzeichen | offen |
| `Neg` | vorhanden | fehlt | Uebersetzung lehnt ab |
| `Pop` | vorhanden | fehlt | Uebersetzung lehnt ab |
| `Call` | vorhanden | fehlt | Uebersetzung lehnt ab |
| negative Konstante | `i64` | `u64`-Stapel | Uebersetzung lehnt ab |
| `Return` | Return | `Halt` | Abbildung |
| `JumpIfFalse` | bedingt | `JumpIfZero` | Abbildung |
| `Ne`/`Gt`/`Le`/`Ge` | eigene Opcodes | nur `Eq`/`Lt` | Expansionsfolgen |

## 11. Live-Pfad: welche Generation tatsaechlich versiegelt wird

`components/system/src/system.rs:254` erzeugt jedes Vertragsartefakt mit
`Artifact::new(GENERATION_V1, …)`. Der Live-Pfad arbeitet also mit der VM-Generation 1,
deren Belegung aus BC-002 stammt. In der Kette existieren damit Artefakte, die weder BC-001
noch ratifiziertes BC-002 sind. Persistierte `.atcb`-Dateien gibt es im Repo nicht; die
Artefaktbytes entstehen beim Deploy und werden ueber Envelope und Hash in den Block
gebunden.

## 12. Evidenz (lokale Laeufe, 13.11.2026)

| Pruefung | Kommando | Ergebnis |
|---|---|---|
| Compiler | `cargo +nightly-2026-08-01 test -p atc-core` | 219 gruen |
| System/Bruecke | `cargo +nightly-2026-08-01 test -p atc-system` | 81 gruen |
| VM | `cargo test -p atc-vm` | 169 gruen |
| Spec-Tabellenabgleich | Extraktion aus `SPEC.md` gegen `artifact.rs` | 19 bzw. 19 Codes, Deckung bestaetigt |
| Kollisionspruefung | Mengenschnitt der beiden Kataloge | 7 gemeinsame Bytes, 0 identische Bedeutungen |

Werkzeughinweis: `cargo` 1.83 kann `atc-core`/`atc-system` nicht bauen (`cpufeatures 0.3.1`
verlangt `edition2024`); verwendet wurde `nightly-2026-08-01`. `atc-vm` baut mit der
Standard-Toolchain.

Kein `CI_VERIFIED`: GitHub Actions startet im Konto nicht (`startup_failure`), zuletzt
belegt auf `atclang` `516482a`.

## 13. Status

| Punkt | Status | Einschraenkung |
|---|---|---|
| E1 Contract-Lowering fail-closed | IMPLEMENTED / TESTED | kein CI_Nachweis, PR-Anlage 403 |
| E2-A Inventur | DONE | read-only |
| E2-B Abweichungsmatrix | DONE | read-only |
| E2-C Entscheidung | OFFEN | siehe `OPCODE-SSOT-E2-C-RESOLUTION-2026-11-13.md` |
| E2-D Akzeptanztests | OFFEN | Set in E2-C definiert |
| E2-E Verifikation am Remote | OFFEN | - |
