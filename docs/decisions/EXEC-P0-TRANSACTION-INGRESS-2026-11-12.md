# P0 — Transaktionseingang: Abgleich und Integrationsplan

**Stand:** 12.11.2026 · **Erhebungsbasis:** `a-townchain` `3bb54702` (Register-Commit nach
`159b7bf0`); Arbeitsbaum unverändert, kein Code geändert, keine Testsuite ausgeführt.
**Prüfmethode:** Quelltext gelesen, Zeilenangaben aus dieser Fassung; `cargo check` für
`modules/atc-blockchain/kernel` ausgeführt.

Dieses Dokument gleicht das vorgelegte Architektur- und Integrationsaudit gegen den Code ab und
spezifiziert daraus das Arbeitspaket P0 so, dass es prüfbar ist. `IMPLEMENTED` heißt nicht
`VERIFIED`; `CI_VERIFIED` wird hier nirgends vergeben.

---

## 1. Abgleich der neun Befunde

| ID | Befund des Audits | Ergebnis am Code | Beleg |
|---|---|---|---|
| TX-01 | Kein RPC-Submit-Pfad zum Mempool | **bestätigt** | `rpc.rs:169-186` (nur `chain_id`, `boot_hash`, `peers`, `ping`, `did_count`, `height`, `consensus`, `did_epoch`, `DID`); kein Verb nimmt eine Transaktion |
| TX-02 | Mempool nicht in den Node-Eingang integriert | **bestätigt, präzisiert**: `Mempool::admit` hat außerhalb der Tests **null** Aufrufer; der Daemon hält nur einen `TxLedger` | `mempool.rs:91`, `main.rs:64` |
| TX-03 | Konsens-Loop verarbeitet keine echten Transaktionen | **bestätigt** | `main.rs:85` `run_round(&mut kette_locked, &[], &mut ledger)` |
| TX-04 | ATC-VM nicht im Node-Ausführungspfad | **bestätigt, verschärft**: nicht nur `atc-vm` fehlt in `components/node/Cargo.toml` — **kein Crate im Workspace hängt überhaupt von `atc-system` ab**. `atc-vm` und `atc-exec-admission` sind ausschließlich Workspace-Mitglieder (`Cargo.toml:17,27,28`), keine Abhängigkeit | `components/node/Cargo.toml` (`atc-algorithm`, `atc_wallet`, `secp256k1`, `serde`, `serde_json`, `sha2`); `grep atc-system --include=Cargo.toml` → nur das eigene Manifest |
| TX-05 | Contract-Payload und Gas nicht konsensseitig abgerechnet | **bestätigt**: `TxType::Contract` hat keinen produktiven Konsumenten; `check_block` liest `gas_price`/`gas_limit` nie | `mempool.rs:193-244`, `wallet/tx.rs:25-37` |
| TX-06 | Keine dauerhafte Chain-/State-Persistenz | **bestätigt**: `Chain` hält `Vec<Block>`; `ContractState` hält `Vec<u64>`; die einzige Datei im Node ist die DID-Registry | `chain.rs:70`, `system/state.rs:16`, `did.rs:273` |
| TX-07 | Eigene lokale Chain in `atc-system` | **bestätigt**: `execute` erzeugt einen Block in `self.chain` | `system.rs:478` `self.chain.produce(&payload)`, `:488` `block_height: self.chain.height()` |
| TX-08 | Zustandswurzel nicht Teil der Blockvalidierung | **bestätigt**: der Node-Block hat kein `state_root`-Feld; `state_root_before` wird berechnet und nur ins Receipt geschrieben | `chain.rs:15-20`, `system.rs:396`, `:484` |
| TX-09 | Kein vollständiger Multi-Node-Vote-Austausch | **bestätigt**: das Crate sagt es selbst; Votes laufen in-process, Gossip trägt nur Blöcke | `consensus.rs:12-20`, `gossip.rs:1-11` |

Die Statusmatrix des Audits (Abschnitt 11) trifft damit zu. Drei Punkte sind zu **präzisieren**,
weil sie sonst als Integrationsdetail missverstanden werden:

1. **„Node kann die VM nicht aufrufen, weil `atc-vm` fehlt"** ist zu schwach. Die Kante fehlt
   vollständig: es gibt keinen Konsumenten von `atc-system` im gesamten Workspace, auch nicht in
   `atc-cli`. Die VM ist heute ein Bibliothekspfad ohne Aufrufer.
2. **`atc-algorithm` als SSOT** ist belegt, aber mit **zwei Bezugsarten**: `components/node` und
   `components/mining` hängen per `path = "../algorithm"`, `apps/atc-mining` hängt per
   `git … rev = bf2d0fa2` auf das eigenständige Repository, `apps/atc-node` per
   `rev = a9f81887`. Zwei Mining-Kopien mit unterschiedlichen Pins sind ein Drift-Risiko, keine
   zweite Wahrheit — solange `apps/*` nicht gebaut werden.
3. **`atc-vm` als „zweiter VM-Kern"**: im Produktpfad existiert genau eine VM
   (`components/vm`). Ein zweiter Kern liegt ausschließlich im **Archiv**
   (`docs/os-docs/docs/archive/monorepo-full/...`), wird nicht gebaut und ist LEGACY, keine
   konkurrierende Wahrheit.

---

## 2. Zwei Befunde, die im Audit fehlen

**F-1 · Das einzige Crate mit dem vollständigen Blockmodell kompiliert nicht.**
`modules/atc-blockchain/kernel/src/blockchain.rs` definiert
`Block { id, height, parent_hash, proposer_did, timestamp, poh_hash, transactions, tx_root, state_root, gas_used, total_fees, signature }`
— also genau die Felder, die in der Architekturbeschreibung als vorhanden gelten. Das Manifest
setzt `[lib] path = "src/blockchain.rs"`, der Code referenziert `crate::mempool::{MemoryPool,
StateDb, TxValidator}` und `ConsensusEngine`, die dort nicht existieren:

```
$ cd modules/atc-blockchain/kernel && cargo check --offline
error: could not compile `atc-blockchain` (lib) due to 17 previous errors
```

`ecosystem/integration` (ebenfalls `UNVERIFIED_BUILD`) hängt an diesem Crate. Wer das Blockmodell
aus dem Code ableiten will, landet entweder beim reduzierten Node-Block oder bei einem nicht
baubaren Crate. Das ist eine SSOT-Verletzung, nicht nur eine Lücke.

**F-2 · Zwei Crates heißen `atc-node`.**
`components/node/Cargo.toml` und `apps/atc-node/Cargo.toml:2` tragen denselben Namen; `apps/atc-node`
ist kein Workspace-Mitglied und seit P1d-A ohne `sync_pull`. Der Name ist damit eine Falle für
jeden, der Crate-Namen statt Pfaden liest.

---

## 3. P0 in einem Satz

Eine signierte kanonische Transaktion erreicht **nachweislich** den Mempool des laufenden Prozesses,
wird dort nach den bestehenden Regeln geprüft, und eine Runde nimmt sie in einen Block auf — ohne
zweite Prüfroute, ohne zweite Kodierung, ohne Zustandsbehauptung.

**Nicht Ziel von P0:** VM-Aufruf, Gebührenabbuchung, Kontostände, Persistenz, Multi-Node-Votes,
Finalitätsnachweis gegen einen Reorg.

---

## 4. Entwurf

### 4.1 Dateien und Funktionen

| Datei | Änderung |
|---|---|
| `components/node/src/rpc.rs` | neue Methode `submit_transaction` in `answer_json`; neuer Zustand `Arc<Mutex<Mempool>>` neben `consensus`; Ablehnungscodes (4.3) |
| `components/node/src/main.rs` | `Mempool::with_policy(capacity, IdentityPolicy::permissive())` anlegen, per `with_mempool` an `DevnetRpc` geben, **denselben** Mempool im Konsens-Thread verwenden, `take_all()` je Runde an `run_round` übergeben |
| `components/node/src/mempool.rs` | unverändert, außer: `capacity` muss aus einer benannten Konstante kommen (4.4) |
| `components/node/src/consensus.rs` | unverändert (`run_round` nimmt `&[TxEnvelope]` bereits) |
| `components/wallet/src/tx.rs` | **keine** Formatänderung; optional ein `to_wire()/from_wire()`-Paar als einzige Kodierung (4.2) |
| `components/node/tests/ingress.rs` | neu: T-01…T-12 |

`Mempool::admit` bleibt die **einzige** Prüfstelle für eingehende Transaktionen. Der RPC ruft
nichts anderes; `TxLedger::check_block` prüft bei der Blockbildung erneut, wie heute.

### 4.2 Transportform — die offene Formatfrage

`Transaction` hat heute **keine** kanonische Wire-Form: `signing_bytes()` (`tx.rs:50`) ist der
Digest-Input ohne Signatur, nicht die Transportkodierung. Wer jetzt Felder als JSON überträgt und
im Node wieder zusammensetzt, baut eine zweite Kodierung neben der Signierform — genau das, was
§5 der Konsensregeln verbietet.

Vorschlag (zu entscheiden, nicht entschieden): **eine** Domäne `ATC-TX-WIRE-V1`, Aufbau
`Domäne ‖ chain_id ‖ tx_type ‖ len(sender_did) ‖ sender_did ‖ len(recipient) ‖ recipient ‖
amount(16) ‖ gas_price ‖ gas_limit ‖ nonce ‖ timestamp ‖ len(payload) ‖ payload ‖ poh_hash(32) ‖
public_key(33) ‖ signature(64)`, übertragen als Hex oder Base64 in **einem** JSON-Feld. Der Node
dekodiert genau einmal, prüft Feldlängen gegen die Domäne und ruft `admit`. Damit ist die
Transportform eine Ableitung der Signierform, keine zweite Wahrheit.

Zwei Details, die sonst später brechen:
- `amount` ist `u128`. In JSON als **Dezimal-String** übertragen, nie als JSON-Zahl (sonst
  Präzisionsverlust ab 2^53 in jedem JS-Client).
- Die Wire-Form braucht einen **eigenen** Größengrenzwert mit Begründung. `MAX_RPC_REQUEST_BYTES
  = 8192` (`rpc.rs:9`) ist eine Transportschranke für Statusabfragen und darf nicht stillschweigend
  als Transaktionsgrenze gelten (Regel: Grenzwerte nicht ableiten). Bis der Wert aus P1b kommt:
  eigener Wert, eigener Grenzwerttest an der Kante.

### 4.3 Fehlerabbildung

| MempoolError | JSON-RPC | Bedeutung |
|---|---|---|
| `InvalidSignature`, `SenderNotBound`, `KeyMismatch` | `-32001` | Signatur/DID-Bindung ungültig |
| `InvalidChainId` | `-32002` | falsche Chain-ID |
| `NonceGap`, `NonceTooLow` | `-32003` | Nonce passt nicht (erwartet wird der Wert mitgegeben) |
| `DuplicateTransaction` | `-32004` | ID bereits gesehen |
| `Full` | `-32005` | Mempool voll |
| `UnknownDid` | `-32006` | strikter Modus |
| kaputte Wire-Form | `-32602` | `invalid params`, ohne Feldinhalte zu spiegeln |

Kein Fehlerfall darf den Mempool verändern; `seen_ids` und `expected_nonce` bleiben unberührt.
Das ist T-09.

### 4.4 Betriebliche Folge, die P0 ehrlich benennen muss

- Ohne Persistenz (P1d-C) verliert ein Neustart **Mempool und Kette**. Das ist für Devnet
  zulässig, muss aber beim Start ausgegeben werden, nicht nur in der Doku stehen.
- Der Devnet-Cap von 64 Blöcken (`chain.rs:103`) wird mit echten Transaktionen **praktisch**
  erreichbar. Das ist B2-O2, nicht P0 — aber P0 darf den Cap nicht stillschweigend erhöhen.
- Für einen Multi-Node-Betrieb muss die Transaktion den **Proposer** erreichen. Gossip trägt heute
  nur Blöcke (`gossip.rs:1-11`). P0 ist daher korrekt als Single-Prozess-Devnet abgegrenzt;
  Tx-Weiterleitung ist ein eigener Schritt und darf nicht als „erledigt" mitlaufen.

---

## 5. Abnahmekriterien

| Test | Erwartung |
|---|---|
| T-01 | gültige Transaktion → `admit` Ok, ID im Mempool, Antwort enthält die Tx-ID |
| T-02 | dieselbe Transaktion zweimal → zweite Antwort `-32004`, Mempool unverändert (Länge 1) |
| T-03 | manipulierte Signatur → `-32001`, keine Aufnahme |
| T-04 | `sender_did` ≠ Adresse des Pubkeys → `-32001` (`SenderNotBound`) |
| T-05 | falsche `chain_id` → `-32002` |
| T-06 | Nonce-Sprung und Nonce zu klein → `-32003`, jeweils mit erwartetem Wert |
| T-07 | kaputte Wire-Form (falsche Längen, abgeschnitten, Hex-Fehler) → `-32602`, keine Panik, kein Teilzustand |
| T-08 | Wire-Grenzwert: Länge exakt am Limit akzeptiert, ein Byte darüber abgelehnt |
| T-09 | jeder Fehlerfall: `expected_nonce` und `seen_ids` bytegleich zu vorher |
| T-10 | Runde mit einer gültigen Tx → Block mit dieser Tx-ID im Payload, Höhe +1, Ledger-Nonce fortgeschrieben |
| T-11 | Runde mit einer abgelehnten Tx → Block bleibt leer oder die Runde scheitert; Kette unverändert |
| T-12 | Determinismus: gleiche Tx-Folge in zwei Prozessen → identische Blockhashes |

Zusätzlich als Strukturwächter: ein Quelltext-Test, der belegt, dass `Mempool::admit` **nur** aus
`rpc.rs` (und Tests) aufgerufen wird — analog zu den bestehenden Wächtern aus P1d-A/B.

`P0` gilt als `IMPLEMENTED`, wenn T-01…T-12 grün sind; `TESTED` mit dem Testprotokoll; `CI_VERIFIED`
erst mit einem Lauf auf dem Default-Branch-HEAD (Run, Job, Step, Log).

---

## 6. Reihenfolge und Abhängigkeiten

1. **Formatentscheidung** (Wire-Form, Größe, `amount`-Kodierung) — ohne sie kein P0-Code.
2. **P0** wie oben (Single-Prozess, keine Zustandsbehauptung).
3. **P1d-C** (atomarer Commit, Persistenz) — P0 darf nicht vorgreifen; ein „Commit" ohne Log wäre
   eine zweite Wahrheit über Dauerhaftigkeit.
4. **P1** (Tx-Weiterleitung im Gossip, echte Multi-Node-Votes), dann **P2** (VM als
   Zustandsübergang im Commit), dann **P3/P4** (Gas, Kontostände als Wurzelbereiche).

Die Reihenfolge des Audits (P0…P6) ist damit bestätigt, mit einer Verschiebung: **P5 (Persistenz)
ist bei uns P1d-C und muss vor P2 liegen**, weil ein VM-Effekt ohne Commit-Punkt nicht dauerhaft
werden kann. P2 vor P1d-C würde einen zweiten, unbestätigten Ausführungspfad erzeugen.

---

## 7. Zur Frage nach dem wöchentlichen Audit

Ein wöchentlicher Lauf gegen die HEADs ist sinnvoll, aber nur mit drei Bedingungen, sonst erzeugt
er Berichte ohne Aussage:

1. **HEAD pinnen und Blob-SHAs mitschreiben.** Ein Befund gilt für einen Commit, nicht für „main".
   Ändert sich der Blob, ist der Befund neu zu erheben (bisher mehrfach genau daran gescheitert).
2. **Befunde gegen Tests prüfen, nicht gegen Prosa.** Jede Zeile der Statusmatrix braucht eine
   Datei:Zeile oder einen Testnamen.
3. **Nur Abweichungen melden.** Ein Wochenbericht, der 60 unveränderte Zeilen wiederholt, wird
   nicht gelesen.

In dieser Form ist der Lauf als geplanter Agenten-Task umsetzbar (wöchentlich, Vergleich gegen
den zuletzt gespeicherten Stand, Meldung nur bei Änderung).

---

## 8. Nicht behauptet

- Kein Testlauf, kein `CI_VERIFIED`, kein `E2E_VERIFIED`.
- Keine Aussage über Repositories außerhalb dieses Monorepos (insbesondere nicht über den
  `a-townchain-ecosystem`-Stand `50075cc7`).
- Die Wire-Form in 4.2 ist ein Vorschlag, keine Entscheidung; §5-Konformität ist zu prüfen, bevor
  Code entsteht.
