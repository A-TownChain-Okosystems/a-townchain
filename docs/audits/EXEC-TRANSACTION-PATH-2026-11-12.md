# Transaktionspfad Wallet → RPC → Node → Konsens → ATC-VM → Speicherung

**Erhebung:** 12.11.2026 · **Basis:** `a-townchain` HEAD `82ae098d` (Arbeitsbaum: nur
`docs/decisions/exec-decision-register.yaml`, `tools/decision_register.py` und Tests geändert,
kein Code) · **Methode:** Quelltext gelesen, Zeilenangaben aus dieser Fassung; `cargo check`
für `modules/atc-blockchain/kernel` ausgeführt (Beleg unten) · **Kein Code geändert.**

Evidenzklassen nach der Dokuregel: **IMPLEMENTED** (Code vorhanden) · **TESTED** (durch Tests
belegt) · **NORMATIVE** (freigegebener Standard) · **RESEARCH** (Kandidat) · **LEGACY/UNVERIFIED**
(Behauptung ohne Nachweis). `IMPLEMENTED` heißt nicht `VERIFIED` und nicht `CI_VERIFIED`.

---

## 1. Ergebnis in einem Satz

Es gibt **keinen durchgehenden Transaktionspfad**. Der Pfad ist an fünf Stellen unterbrochen,
und die einzige Stelle, an der Kontostände überhaupt verändert würden, existiert nicht:
`TxLedger` führt Nonces und bereits gesehene IDs, **keine Salden**.

```
Wallet                Node                                        System/VM
------                ----                                        ---------
Transaction           RPC (nur lesend)                            ATCE-Hülle
signiert     ──X──>   [kein Ingress]                              Admission
                      Mempool (vorhanden, ungenutzt)  ──X──>       ATCB
                      Chain.produce_tx_block                        VM
                      Consensus (leere Blöcke)                      Receipt
                      TxLedger (Nonces, keine Salden)  ──X──>      ContractState
                      [keine Persistenz]  ──X──>                    [im Aufrufer]
```

---

## 2. Der Pfad in acht Etappen

| # | Etappe | zuständiges Crate (Repo `a-townchain`) | Beleg | Zustand |
|---|---|---|---|---|
| 1 | Schlüssel, Adresse, Signatur | `components/wallet` (`atc_wallet`) | `tx.rs:11-27`, `keys.rs` | IMPLEMENTED · TESTED |
| 2 | Einreichung ans Netz | `components/node` (`atc-node`) | `rpc.rs:165-186` | **fehlt** |
| 3 | Annahme / Prüfung / Mempool | `components/node` | `mempool.rs:91-131`, `:193-244` | IMPLEMENTED · TESTED, **nicht verdrahtet** |
| 4 | Blockbildung | `components/node` | `chain.rs:102-245` | IMPLEMENTED, Payload = nur Tx-IDs |
| 5 | Konsens / Finalität | `components/node` (`consensus.rs`), kanonisch `components/algorithm` | `consensus.rs:1-20`, `:298-380` | IMPLEMENTED, Devnet, in-process |
| 6 | Verteilung / Annahme fremder Blöcke | `components/node` | `gossip.rs:1-25`, `chain.rs:249-309` | IMPLEMENTED, nur Pull |
| 7 | Ausführung (Vertrag) | `components/system`, `components/exec-admission`, `components/vm` | `system.rs:330-495` | IMPLEMENTED · TESTED, **nicht angeschlossen** |
| 8 | Dauerhafte Speicherung | — | `did.rs:273-280`, `components/storage` | **fehlt** (nur DID-Registry als JSON) |

### 2.1 Wallet (Etappe 1) — vorhanden und vollständig spezifiziert

`components/wallet/src/tx.rs` definiert die kanonische Transaktion:

```rust
pub const NUMERIC_CHAIN_ID: u64 = 658467;
pub const TX_DOMAIN_V2: &[u8] = b"ATC-TX-DOMAIN-V2";
pub struct Transaction {
    chain_id, tx_type, sender_did, recipient_did, amount: u128,
    gas_price, gas_limit, nonce, timestamp, payload: Vec<u8>, poh_hash: [u8;32],
}
```

Signiert wird ein domänengetrennter Digest (`signing_bytes`, `tx.rs:50`), die ID ist
`SHA-256("ATC-TX-ID-V2" ‖ Felder)` (`tx.rs:64`). `verify_bound` prüft Signatur **und** die
Bindung `sender_did` an die Adresse des öffentlichen Schlüssels (`TxError::SenderNotBound`).
Beträge sind `u128`, fest 16 Byte big-endian — die 2026 identifizierten Kodierungsfehler sind
hier geschlossen. **TESTED** über `components/node/tests/tx_block_tests.rs`.

Aber: `components/wallet/src` enthält **keinen Netzwerkcode** — kein `TcpStream`, keinen
RPC-Client. Die Wallet kann eine Transaktion erzeugen und signieren, aber nicht einreichen.

### 2.2 RPC (Etappe 2) — nur lesend

`components/node/src/rpc.rs` bedient zeilenbasiert (`MAX_RPC_REQUEST_BYTES = 8 KiB`, `rpc.rs:9`)
und JSON-RPC. Die vollständige Methodenliste (`rpc.rs:169-186`):

`chain_id`, `boot_hash`, `peers`, `ping`, `did_count`, `height`, `consensus`, `did_epoch`
sowie `DID <did>`, `DID_COUNT`, `DID_EPOCH`, `HEIGHT`, `BEST_HASH`, `CONSENSUS`.
Unbekannte Methoden → `-32601`.

**Es gibt keine Methode, die eine Transaktion annimmt.** Kein `sendRawTransaction`, kein
`eth_sendTransaction`, kein HTTP-POST. Der einzige Ingress des Prozesses ist der Gossip-Port
(Blöcke) und der RPC-Port (Anfragen). Kein Transport trägt eine `Transaction`.

### 2.3 Mempool (Etappe 3) — vorhanden, ohne Aufrufer

`Mempool::admit` (`mempool.rs:91`) prüft in dieser Reihenfolge: Kapazität, Ketten-ID, Signatur
und DID-Bindung (`IdentityPolicy::verify`, `:280`), unbekannte DID (`UnknownDid`, strikter
Modus), Schlüsselabweichung (`KeyMismatch`), Doppel-ID, Nonce-Sprung/-Unterschreitung.
`TxLedger::check_block` (`:193`) wiederholt dieselbe Prüfung für die Blockebene.

Ein repo-weiter Aufruf von `Mempool::admit` außerhalb der Tests existiert nicht. Die
Devnet-Binary instanziiert nur einen `TxLedger` (`main.rs:64-65`) und einen leeren Tx-Slice.
**Der Mempool ist eine geprüfte, aber unbenutzte Schnittstelle.**

### 2.4 Blockbildung (Etappe 4) — Block trägt nur IDs

```rust
pub struct Block { pub height: u64, pub prev_hash: [u8;32], pub payload: String, pub hash: [u8;32] }
```

`payload` ist bei Transaktionsblöcken die **kommagetrennte Liste der Transaktions-IDs**
(`encode_tx_ids`, `chain.rs:363`), nicht die Transaktion selbst. Ein Block ist damit **nicht
selbsttragend**: Wer ihn empfängt, kann Betrag, Empfänger, Signatur und Gebühr nicht aus dem
Block prüfen, sondern muss die Transaktionen auf einem zweiten Kanal bekommen — den es nicht
gibt. `import_tx_block` (`:218`) nimmt deshalb `height` **und** `txs` als getrennte Argumente.

Weitere Eigenschaften des Ist-Blocks: kein Zeitstempel, kein Transaktionswurzel-Hash, kein
`state_root`, keine Validator-Daten, keine Signatur; Hash = `atc_hash("ATC-BLOCK-V1|height|prev|payload")`
(`chain.rs:25-35`, ATC-HASH-001, FNV-basiert, laut Crate-Doku „nicht kryptoanalysiert").
Devnet-Cap: 64 Blöcke (`chain.rs:103`). `'|'` und `';'` sind im Payload verboten (Wire-Format).

### 2.5 Konsens (Etappe 5) — Runde vorhanden, Daemon finalisiert leere Blöcke

`consensus.rs` implementiert: deterministische Proposer-Wahl (Weighted Round Robin über die
Höhe), Blockhash **vor** dem Anhängen (`preview_hash`), Vote je Validator über den **vollen
32-Byte-Blockhash** mit Domäne `ATC-CONSENSUS-VOTE-V1` (`:32-44`), Quorum bei 2/3 der Power,
danach `commit_payload` (Block + Ledger). Das Crate dokumentiert die Grenzen selbst
(`consensus.rs:12-20`): Vote-Austausch **nicht** im Gossip, Runden nur innerhalb eines
Prozesses, kein Slashing, kein Reorg, kein View-Wechsel bei Timeout.

Der laufende Devnet-Prozess ruft die Runde so auf (`main.rs:85`):

```rust
engine.run_round(&mut kette_locked, &[], &mut ledger)
```

mit leerem Transaktions-Slice, alle 2 Sekunden. Ergebnis: die Kette wächst mit **leeren
Blöcken**; `height` und `last_hash` über RPC sind echt, aber ohne Inhalt.

PoW existiert **nicht** (kein `proof_of_work`, kein `difficulty` in `components/algorithm`
oder `components/node`). PoH existiert als `Tick`-Kette (`algorithm/src/poh.rs:44-86`), wird im
Node-Pfad aber nicht zur Blockbindung verwendet. Proposer-Wahl: `algorithm/src/selection.rs:17`.

### 2.6 Verteilung (Etappe 6) — Pull, Adoption nur mit Validator

`gossip.rs` bietet nur `STATUS` und `BLOCKS <from>`; Blöcke werden mit `|`/`;` serialisiert und
**vor** jeder Übernahme vollständig geprüft (Hashes, Höhen, Verkettung, Genesis-Bindung). Der
einzige Annahmeweg ist `Chain::adopt_verified` (`chain.rs:286`) bzw. `import_block`
(`:249`) mit `I-001` kein Validator → Ablehnung, `I-002` Blockhash, `I-003` Verkettung,
`I-004` Kapazität, `I-005` Payload. Kein Push, keine Signaturen, kein TLS, keine Auth.

### 2.7 Vertragsausführung (Etappe 7) — vollständig, aber isoliert

Der Vertragspfad ist der am besten abgesicherte Teil und hat mit dem Transaktionspfad
**keinen Berührungspunkt**:

- `System::deploy_container` (`system.rs:330`): ATCE-Hülle prüfen → `into_artifact` →
  Policy, Signatur, Registry.
- `System::execute` (`:388`): Stufe 0 Hülle (leer `E-001`, ATCB `E-007`), 0b Bytecode-Hash
  gegen `code_hash`, 0c innere Bytes gegen Hülle, dann Freigabe und Ausführung; `Receipt` mit
  `stage`, `code`, `offset`.
- `ContractState` (`system/state.rs`): Wortspeicher, Wurzel = `SHA-256("ATC-STATE-V1\0" ‖ code_hash ‖
  Wortzahl ‖ Wörter)`.

**Kein Crate hängt von `atc-system` ab** (`grep atc-system --include=Cargo.toml` liefert nur das
Manifest selbst). `atc-vm` und `atc-exec-admission` erscheinen ausschließlich als
Workspace-Mitglieder (`Cargo.toml:17,27,28`), nicht als Abhängigkeit eines anderen Crates.
Damit gilt: `System::execute` wird von **keinem** Node-, RPC- oder CLI-Pfad aufgerufen.
`ContractState` gehört dem Aufrufer, `*state = after` ist die einzige Übernahme; es gibt keinen
Speicher, gegen den `state_root_before` (`system.rs:396`) geprüft werden könnte.

### 2.8 Speicherung (Etappe 8) — nicht vorhanden

`components/storage` ist ein Content-Addressing-Blobstore (`cas.rs`, `chunking.rs`,
`store.rs`, `retrieval.rs`, zusammen 312 Zeilen) — kein Ketten- oder Kontozustand.
Die einzige dauerhafte Datei im Node ist die DID-Registry (`did.rs:273` `save`, `:277` `load`,
JSON). Blöcke, Ledger-Nonces, Vertragszustand und Registry überleben keinen Neustart.

---

## 3. Die fünf Brüche

| ID | Bruch | Folge |
|---|---|---|
| B-1 | Wallet → Netz: kein Ingress (kein RPC-Verb, kein Client in der Wallet) | Eine signierte Transaktion kann das Gerät nicht verlassen |
| B-2 | Mempool → Daemon: `admit` wird nirgends aufgerufen | Prüfcode ist vorhanden, aber nicht Teil des Betriebs |
| B-3 | Konsens → Inhalt: `run_round(..., &[], ...)` | Finalisiert werden leere Blöcke; Höhe wächst ohne Transaktionen |
| B-4 | Block → Kontostand: `TxLedger` führt keine Salden, keine Gebühren, keine Emission | „Alice −100, Bob +100" existiert an keiner Stelle im Code |
| B-5 | Transaktion → VM: niemand ruft `System::execute`; `atc-system` hat keinen Konsumenten | Vertragsausführung ist keine Zustandsänderung der Kette, sondern ein Bibliotheksaufruf |

Ergänzend, kein Bruch im engeren Sinn, aber Voraussetzung für B-4/B-5: es gibt **keine
dauerhafte Zustandsautorität** (P1d-B1/B2), also auch keinen Commit-Punkt, an den ein
Transaktions- oder Vertragseffekt gebunden werden könnte.

---

## 4. Drei Blockmodelle — SSOT-Verletzung sichtbar

| Ort | Blockmodell | Status |
|---|---|---|
| `components/node/src/chain.rs` | `height, prev_hash, payload:String, hash` | läuft im Devnet, **kein** `timestamp`/`tx_root`/`state_root` |
| `modules/atc-blockchain/kernel/src/blockchain.rs` | `id, height, parent_hash, proposer_did, timestamp, poh_hash, transactions, tx_root, state_root, gas_used, total_fees, signature` | **kompiliert nicht** (Beleg unten), `UNVERIFIED_BUILD` |
| `components/system` | kein Block; `Deployment` + `Receipt` + Audit | keine Kette |

Beleg für das dritte Modell (lokal ausgeführt):

```
$ cd modules/atc-blockchain/kernel && cargo check --offline
error[E0432]: unresolved import `crate::mempool` ... (17 Fehler)
error: could not compile `atc-blockchain` (lib) due to 17 previous errors
```

Ursache: das Manifest setzt `[lib] path = "src/blockchain.rs"`, der Code referenziert
`crate::mempool::{MemoryPool, StateDb, TxValidator}` und `ConsensusEngine`, die in
`kernel/src/` nicht existieren (dort liegt nur `blockchain.rs`, 57 Zeilen).
`ecosystem/integration` (ebenfalls `UNVERIFIED_BUILD`) hängt an genau diesem Crate.

Das vollständige Blockmodell mit `tx_root`, `state_root`, `proposer_did` und `signature`, das in
der Dokumentation beschrieben wird, existiert also **nur in einem nicht baubaren Crate** — und
das laufende Modell hat keines dieser Felder. Beides zusammen ist die SSOT-Verletzung: dieselbe
Funktionalität („was ist ein Block") hat drei Ausprägungen, von denen keine die andere ablöst.

Zusätzlich: `apps/atc-node` trägt denselben Crate-Namen `atc-node` wie `components/node`
(`apps/atc-node/Cargo.toml:2`), ist kein Workspace-Mitglied (P1d-A: Parallelkopie stillgelegt)
und wird nie gebaut.

---

## 5. SSOT-Grenzen (Soil und Ist)

| Funktionalität | kanonischer Ort (Soil) | konkurrierende Orte (Ist) |
|---|---|---|
| Transaktionsformat | `components/wallet/src/tx.rs` | `modules/atc-blockchain/kernel` (eigene `Transaction`) |
| Signaturprüfung | `atc_wallet::tx::verify_bound` | Kernel-eigene Prüfung (`TxValidator`, nicht baubar) |
| Mempool/Annahme | `components/node/src/mempool.rs` | Kernel-`MemoryPool`, Brücke (keine) |
| Blockmodell | `components/node/src/chain.rs` | Kernel-`Block`, `apps/atc-node` (stillgelegt) |
| Blockhash | `atc_hash` = ATC-HASH-001 (`components/algorithm/src/hash.rs:113`) | Kernel-`simple_hash` |
| Konsens | `components/algorithm` (kanonisch, laut Crate-Doku) | `components/node/src/consensus.rs` (Rundenlogik), `validator.rs` |
| Wirtschaftspolitik | `components/algorithm/src/economics.rs` | — (Konstanten korrekt, aber nur vom Mining-Reward gelesen) |
| Vertragsausführung | `components/system` + `atc-vm` + `atc-exec-admission` | — (keine Konkurrenz, aber kein Konsument) |
| Zustandswurzel | `components/system/src/state.rs` (je Vertrag) | keine Systemwurzel vorhanden |
| Persistenz | **nicht definiert** | DID-Registry (JSON), CAS-Blobs (anderes Objekt) |

---

## 6. Abgleich mit der vorgelegten Beschreibung

| Behauptung | Befund im Code | Klasse |
|---|---|---|
| Block enthält Timestamp, Transactions, Transaction Root, Validator Data, Block Hash | Node-Block hat nur `height`, `prev_hash`, `payload`, `hash`; die reichere Variante kompiliert nicht | LEGACY/UNVERIFIED |
| Wallet überträgt die signierte Transaktion ans Netzwerk | Wallet hat keinen Netzwerkcode; RPC nimmt keine Transaktion an | LEGACY/UNVERIFIED |
| Nodes prüfen Saldo, Nonce, Berechtigung | Nonce, Signatur, DID-Bindung, Replay: ja. **Saldo: nein** (kein Saldenmodell) | teilweise IMPLEMENTED |
| Nach Bestätigung werden Kontostände aktualisiert (Alice −100, Bob +100) | existiert nicht | LEGACY/UNVERIFIED |
| Blockzeit 360 Sekunden | Konstante `TARGET_BLOCK_TIME_SECS = 360` vorhanden, aber ungenutzt; Devnet finalisiert alle 2 s | NORMATIVE (Parameter) vs. Ist |
| Halving 360.000 Blöcke, 36 Zyklen, 360 Mio. ATC, 18 Dezimalstellen, Chain-ID 658467 | `economics.rs:16-22`, `tx.rs:11` — deckungsgleich | IMPLEMENTED (Konstanten) |
| Hybrid PoH/PoS/PoW | PoS-Runde (in-process, Devnet) + PoH-Tickkette vorhanden; **PoW nicht implementiert** | teilweise IMPLEMENTED, Rest RESEARCH |
| State Layer für Kontostände | nicht vorhanden | LEGACY/UNVERIFIED |
| ATC-VM führt Smart Contracts deterministisch aus | VM + Hülle + Admission vorhanden und getestet, aber nicht am Kettenpfad | IMPLEMENTED · TESTED, nicht integriert |
| Gebühren finanzieren Rewards | keine Gebührenerhebung im Code | RESEARCH |
| Staking bindet ATC, Wallet zeigt `available`/`locked` | `Balance{available, locked}`, `total()` vorhanden — aber ohne Kettenbindung | IMPLEMENTED (Modell), nicht angeschlossen |

---

## 7. Was für einen durchgehenden Pfad fehlt

Reihenfolge so, dass jede Stufe prüfbar bleibt; nichts davon ist heute implementiert.

1. **Ingress (B-1)** — genau ein Verb (`sendRawTransaction`) auf dem bestehenden RPC-Port,
   das eine `Transaction` + `public_key` + `signature` als `TxEnvelope` entgegennimmt und
   **ausschließlich** `Mempool::admit` ruft. Keine zweite Prüfroute, keine zweite Dekodierung.
2. **Mempool im Betrieb (B-2)** — der Daemon hält den Mempool, die Runde liest daraus
   (`take_all`), `run_round` bekommt echte `txs`.
3. **Selbsttragender Block (B-4-Voraussetzung)** — Block muss die Transaktionen oder einen
   `tx_root` mit den Transaktionen tragen; ein Block aus IDs ist für einen fremden Knoten nicht
   prüfbar. Das ist eine Änderung am Blockmodell und damit an ATC-HASH/Format, also eine
   Standardfrage, keine Nebenarbeit.
4. **Kontozustand (B-4)** — Salden, Nonces, Gebühren, Emission als **Bereiche der
   Systemzustandswurzel** (P1d-B2, AD-B1-03). `TxLedger` wird damit Teil des Zustands, nicht
   ein Nebenbuch.
5. **Atomarer Commit (P1d-C)** — Block, Kontozustand und Policy in einer Transaktion, mit den
   vier Vorbedingungen aus AD-B1 (Höhe, Best-Hash, State-Root, State-Version). Vorher ist kein
   Transaktionseffekt dauerhaft.
6. **Ausführung als Zustandsübergang (B-5)** — der Node ruft `System::execute` auf, bekommt
   `Receipt` + `after`-Zustand und übernimmt beides im selben Commit. Bis dahin bleiben
   Vertragsausführung und Kette zwei Welten.
7. **Persistenz (B-5/P1d-C)** — append-only Commit-Log als Autorität, Snapshots nur als
   Beschleuniger; heute existiert keine Datei, die einen Block überlebt.

Bis 4–6 stehen, ist die ehrliche Aussage über die A-TownChain: **ein signierbares
Transaktionsformat, ein geprüfter Mempool, eine Devnet-Kette mit leeren Blöcken, eine
getestete Vertragsausführung ohne Kettenbezug und kein dauerhafter Zustand.**

---

## 8. Nicht behauptet

- Kein `cargo test`-Lauf in diesem Schritt (reine Leseerhebung, kein Code geändert). Die
  Testaussagen oben stammen aus den Testdateien, nicht aus einem neuen Lauf.
- Kein `CI_VERIFIED`, kein `E2E_VERIFIED` für irgendeine Etappe.
- Keine Aussage über die 33 konsolidierten Repositories außerhalb dieses Monorepos; die
  Erhebung gilt für `a-townchain` unter dem genannten HEAD.
