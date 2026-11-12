# Inventar und Konfliktmatrix: Smart-Contract-Ausfuehrung

**Datum:** 2026-11-12
**Stand:** `a-townchain` main `43b194c86` (Audit `ef3f2d5a8` / `a81217e4` als Grundlage)
**Zweck:** Vor jeder Codeaenderung feststellen, welche Implementierungen,
Spezifikationen und Wahrheiten es fuer dieselbe Sache gibt, und wo sie
auseinanderlaufen. Ergebnis ist die Grundlage fuer die Entscheidungen D1 bis D7
(`docs/decisions/AD-EXEC-D1-D7.md`) und fuer den Integrationsvertrag
`ATC-SMART-CONTRACT-EXECUTION-001`.

**Regel dieses Dokuments:** Es beschreibt, es entscheidet nicht. Jede Zeile der
Konfliktmatrix nennt die Entscheidung, die sie aufloest. Solange die Entscheidung
nicht gefallen ist, wird an der betroffenen Stelle kein Code geaendert.

## 1. Implementierungsstellen

| # | Stelle | Was sie definiert | Zustand | Tests |
|---|---|---|---|---|
| 1 | `ai/atclang/crates/atc-core/src/bytecode.rs` | Instruktionssatz (16 Arten) und kanonischer Encoder `ATCB` v1, `u16` BE Version, `u32` BE Anzahl, Immediate big-endian | implementiert, verifiziert | in `atc-core` enthalten |
| 2 | `ai/atclang/crates/atc-core/src/artifact.rs` | Artefakt `ATCA`: Nutzlast, Metadaten, SHA-256-Anhang | implementiert | ja |
| 3 | `ai/atclang/crates/atc-core/src/vm.rs` | Referenz-Interpreter, `i64`, geprueft, `MAX_STEPS` | implementiert | ja |
| 4 | `ai/atclang/crates/atc-core/src/storage.rs` | `StorageLayout` S1: 32-Byte-Slots, `LAYOUT_VERSION = 1`, `layout_hash` | implementiert, nur CLI-erreichbar (`main.rs:317`) | ja |
| 5 | `ai/atclang/crates/atc-core/src/abi.rs` | Vertrags-ABI: Felder, Events, Funktionssignaturen, `sha256` je Vertrag | implementiert | ja |
| 6 | `ai/atclang/crates/atc-core/src/native.rs` | native Vertrags-AST (`state`, `fn`, `capabilities`, `policies`) | Parser/Erkennung, kein Lowering | ja |
| 7 | `components/vm/src/artifact.rs` | Artefakt `ATCB`: Container-Byte `0x01`, Generation, `u32` LE Anzahl, SHA-256-Tail | implementiert | 33 + 34 + 26 |
| 8 | `components/vm/src/vm.rs` | Stack-Maschine `u64`, umlaufend, `GAS_TABLE_V1`, `Vec<u64>`-Speicher | implementiert | 27 (Gas) + weitere |
| 9 | `components/vm/src/ops.rs` | Textformat `.ops` (Assembler-Ausgabe) | implementiert | 12 (`exec_chain_e2e`) |
| 10 | `components/system/src/backend.rs` | Uebersetzer `Instruction` zu `Op`, `DIVERGENZEN`, Ablehnungen | implementiert | 13 |
| 11 | `components/system/src/system.rs` | Freigabe, Ausfuehrung, Zustand, Block; `DEFAULT_GAS = 50_000` | implementiert | 11 + 11 |
| 12 | `components/system/src/state.rs` | Zustand als `Vec<u64>`, `state_root` mit Domain `ATC-STATE-V1` | implementiert | in 46 enthalten |
| 13 | `components/contracts/exec_chain` | `.atc` (eigene Syntax) plus `assemble.py` zu `.ops` | produktiv benutzt, bypassed Compiler | Vektoren |
| 14 | `components/node/src/chain.rs` | Block, Payload als Zeichenkette, `produce`, `verify` | implementiert | 5 + 8 |
| 15 | `components/node/src/mempool.rs` | Signatur, DID-Bindung, Nonce, Duplikat | implementiert | enthalten |
| 16 | `components/node/src/consensus.rs` | Proposal, Vote, Quorum 2/3, Runden | implementiert | enthalten |
| 17 | `components/wallet/src/tx.rs` | Transaktion mit `gas_price`, `gas_limit`, `payload`, `TxType::Contract` | implementiert, Contract ungenutzt | enthalten |
| 18 | `components/wallet/src/keys.rs` | `address_from_pubkey`: `ATC:` plus 20-Byte-Prefix von SHA-256 (SEC-P0-005) | implementiert, verbindlich | enthalten |
| 19 | `components/node/src/did.rs` | DID = kanonische Adresse, Schluesselrotation, Epoch | implementiert | 4 |
| 20 | `components/exec-admission/src/lib.rs` | `ExecutionPolicy`, Rollen, Widerruf, Obergrenzen, Audit | implementiert, nur in `atc-system` benutzt | in 46 enthalten |

Nicht Teil dieser Kette und deshalb nicht bewertet: `components/storage`
(Content-Addressing, kein Vertragszustand), `components/mining/src/executor.rs`
(PoW-Versuche), `apps/atc-marketplace`, `modules/atc-governance`.

## 2. Wahrheiten je Domaene

| Domaene | Kandidaten fuer die eine Wahrheit | Bewertung |
|---|---|---|
| Instruktionsstrom | `ai/atclang/specs/bytecode/SPEC.md` (ATC-BC-001, `SPEC-FROZEN`, normativ seit 06.10.2026) und `standards/atc/candidates/ATC-BC-002-CANDIDATE.md` | BC-001 ist normativ und eingefroren. BC-002 ist Kandidat und aendert BC-001 ausdruecklich nicht. |
| Artefakt-/Envelopeformat | BC-001 (kein Envelope, nur Strom) gegen `components/vm/src/artifact.rs` (Envelope mit Generation und Hash) | **zwei Formate unter demselben Namen und derselben Version** (Konflikt K-01) |
| Speicher/Storage | G11-Entscheidung (Slots fester Breite, ABI-definiert, `layout_hash`), `atc-core/src/storage.rs`, VM-`Vec<u64>`, `state.rs` | drei Modelle, keines an das andere angebunden (K-04) |
| Gas | `GAS_TABLE_V1` (19 Opcodes), `DEFAULT_GAS = 50_000`, `tx.gas_price`/`tx.gas_limit`, ATC-14 Gastabelle (`deploy 50000 + 10/byte`, `call`, `create 32000`) | vier Quellen, keine Verbindung (K-05) |
| Identitaet | `address_from_pubkey` (SEC-P0-005), DID aus derselben Ableitung, `caller: u64` in der VM | Kette einheitlich, VM-Aufrufer ausserhalb (K-06) |
| Zustandswurzel | `state.rs` (Domain `ATC-STATE-V1`, SHA-256 ueber Woerter) und Block-`payload` (Zeichenkette) | Wurzel existiert, geht aber nicht in die Blockgueltigkeit ein (K-07) |
| Ausfuehrung im Block | `atc-system` (Bibliothek) gegen `atc-node` (keine VM-Kante) | Ausfuehrung ist kein Konsensbestandteil (K-08) |
| Codeaufnahme | `components/exec-admission` plus `ATC-SEC-EXEC-001-CANDIDATE` | vorhanden, im Kettenpfad nicht verankert (K-09) |
| Frontend | ATCLang (`atc-core`) gegen EXEC-GATE (`.atc` plus `assemble.py` zu `.ops`) | zwei Sprachen, zwei Semantiken (K-10) |

## 3. Normative Anker

| Dokument | Status | Relevanz |
|---|---|---|
| `ai/atclang/specs/bytecode/SPEC.md` (ATC-BC-001) | `SPEC-FROZEN`, normativ seit 2026-10-06, SCR-0071 | traegt ATCB v1 |
| `standards/os/ATC-STD-600.md` | approved, 1.3.0 | Chain Identity, Genesis, Netz |
| `standards/atc/candidates/ATC-BC-002-CANDIDATE.md` | Kandidat, nicht normativ | `Assert`, `Revert`, Generationen, Ruecklauf |
| `standards/atc/candidates/ATC-SEC-EXEC-001-CANDIDATE.md` | Kandidat, nicht normativ | Aufnahme und Ausfuehrung von Code, ExecutionPolicy |
| `standards/atc/ATC-14-DETERMINISTIC_EXECUTION.md` | Doku-Stand "FINAL" (Spezifikation), ohne Registry-Eintrag | Determinismus, Wasm-Sandbox, Gastabelle |
| `standards/atc/candidates/ATC-NODE-001-CANDIDATE.md`, `ATC-ECOSYS-001-CANDIDATE.md` | Kandidaten | Knoten- und Oekosystemrahmen |
| `docs/CODEGEN_G11_SCOPE.md`, G11-Freigabe 08.11.2026 | Owner-Entscheid | Slots, u256 nativ, Artefaktkopf |
| `standards/atc/ATC-STD-SC-001..020` | freigegeben | Vertragsstandards, Gates SC-G0..G13, Registry |

Damit ist die Lage bei den Ankern eindeutig: **genau ein eingefrorener
Instruktionsstandard** existiert, und die VM-Implementierung folgt ihm nicht.

## 4. Konfliktmatrix

| ID | Konflikt | Auswirkung | Loest |
|---|---|---|---|
| K-01 | `ATCB` v1 bedeutet bei Compiler und VM verschiedene Bytes; Opcodes kollidieren (`0x01` ConstI64 gegen Push, `0x02` LoadLocal gegen Add, `0x10` Add gegen Halt) | kein Austauschformat; jede Seite lehnt die andere ab | D1 |
| K-02 | Die VM-Implementierung weicht vom eingefrorenen ATC-BC-001 ab, ohne dass eine normative Aenderung vorliegt | Normabweichung, nicht nur Namensfrage | D1 |
| K-03 | Es gibt keinen Aufrufbegriff: kein Selektor, keine Calldata, kein Rueckgabepuffer, keine Vertragsadresse | Vertraege sind nicht adressierbar, `ATC-8300` laeuft leer | D2 |
| K-04 | Drei Speichermodelle: Rahmen-lokale (Compiler), persistente Woerter (VM), ABI-Slots (S1) | jede Lokale ist Zustand, Slots sind nicht adressierbar | D3 |
| K-05 | Vier Gasquellen ohne gemeinsamen Vertrag | keine deterministische Abrechnung, kein Schutz gegen Ressourcenmissbrauch | D4 |
| K-06 | Identitaet: `ATC:`-Adresse und DID sind identisch, die VM kennt nur `u64` | keine Berechtigungspruefung im Vertrag | D6 |
| K-07 | Zustandswurzel existiert, steht aber nicht im Block und nicht in der Blockgueltigkeit | kein Nachweis gleicher Zustaende zwischen Validatoren | D5 |
| K-08 | Ausfuehrung passiert in einer Bibliothek, nicht im Knoten | Konsens validiert keine Ausfuehrung | D5 |
| K-09 | Aufnahme (Admission) ist im Kettenpfad nicht verankert, Devnet-Schluessel ist eine Konstante | Freigabe ist im Devnet keine Grenze | D5 (Verankerung), sonst SEC-EXEC-001 |
| K-10 | Zwei Frontends: ATCLang und EXEC-GATE (`.ops`) | zwei Semantiken fuer denselben Vertragsbegriff | D7 |
| K-11 | ATC-14 verlangt eine Wasm-Sandbox (ATC-21) und eine eigene Gastabelle; die Kette fuehrt eine eigene Stack-Maschine | Standard und Implementierung beschreiben verschiedene Laufzeitmodelle | D1 und D4 (Abgrenzung), sonst SCR auf ATC-14 |
| K-12 | Ueberlauf und Vorzeichen: `i64` geprueft gegen `u64` umlaufend | unterschiedliche Ergebnisse bei denselben Eingaben | D2 (Befehlssatz) |

## 5. Was aus dem Inventar folgt

1. **Es fehlt nicht an Implementierungen, es fehlt an Vertraegen.** Fuer jede
   Domaene existiert mindestens eine lauffaehige Umsetzung. Was fehlt, ist die
   Entscheidung, welche davon die normative ist.
2. **Der eingefrorene Standard ist die staerkste Karte.** ATC-BC-001 ist
   normativ und bereits durch Verifier und Tests gedeckt. Die VM-Abweichung ist
   die kleinere Baustelle, nicht der Standard.
3. **Kein Konflikt erfordert eine zweite VM oder einen zweiten Compiler.**
   Jeder Konflikt loest sich durch Zuordnung einer bestehenden Umsetzung zu einer
   bestehenden Wahrheit, nicht durch Neubau.
4. **Reihenfolge bleibt zwingend:** D1 und D7 zuerst, weil sie das Format
   bestimmen. D2 und D3 haengen daran, D6 ist unabhaengig entscheidbar, D4 setzt
   D2 voraus, D5 schliesst ab.
