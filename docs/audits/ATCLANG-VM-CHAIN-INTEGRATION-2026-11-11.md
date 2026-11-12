# ATCLang, ATC-VM und A-TownChain: Integration-Audit

**Datum:** 2026-11-11
**Gebundener Stand:** `a-townchain` main `09fee2a3962ce21e465a985724e57471ce112179`
**Gegenstand:** Die sechs Fragen aus der Architekturvorlage (Abschnitt 10), beantwortet
am laufenden Code des Monorepos, nicht an der Dokumentation.
**Nicht Gegenstand:** `aurora-ai`, Konsensguete, Netzwerk, Kernel, Produkt-Apps.

## 0. Kurzfassung

Die drei Komponenten sind **einmal** verbunden, und zwar in dem Crate
`components/system`. Dort existiert die Kante Quelle, Kompilat, Artefakt,
Freigabe, VM, Block, mit 46 lokalen Tests und einem lauffaehigen Beispiel.

Alles, was in der Vorlage als Muss beschrieben ist, ist damit aber nur zur Haelfte
erfuellt:

1. **Der Knoten fuehrt nichts aus.** `components/node` hat keine Abhaengigkeit auf
   `atc-vm`; `TxType::Contract` hat null Konsumenten; die Block-Payload ist eine
   Liste von Transaktions-IDs. Ausfuehrung ist damit **kein Bestandteil der
   Blockgueltigkeit**, und der Satz "jeder Validator muss dasselbe Ergebnis
   berechnen" ist derzeit nicht pruefbar, weil kein Validator etwas berechnet.
2. **Ein Vertrag "laeuft", aber es laeuft nichts.** Der kanonische Token-Vertrag
   `ATC-8300` wird zu **2 Instruktionen** uebersetzt. Ergebnis `0`, Zustand
   unveraendert. Alle Funktionen (`init`, `balance_of`, `transfer`) sind
   unerreichbar, weil es keine Aufrufkonvention und keine Calldata gibt.
3. **Zwei Formate heissen `ATCB` v1** und meinen verschiedene Bytes. Compiler und
   VM lesen den jeweils anderen Stand fail-closed ab, aber der Name ist doppelt
   belegt und keine Instruktionsnummer bedeutet auf beiden Seiten dasselbe.
4. **Gas existiert dreimal unverbunden**, Gebuehren werden nirgends abgebucht, und
   die Identitaet der Kette (DID) trifft die Identitaet der VM (u64) nirgends.

Kurz: die Architektur aus Abschnitt 1 der Vorlage ist als **Pfad** implementiert,
als **Semantik** noch nicht. Was fehlt, sind die fuenf ABIs aus Abschnitt 6 der
Vorlage, und zwar als Vertraege, nicht als Konventionen.

## 1. Die sechs Fragen

### Frage 1: Wo kompiliert ATCLang tatsaechlich den Bytecode?

**Antwort:** in `ai/atclang/crates/atc-core`, Stufen Lexer, Parser, Lowering,
Verifier. Das Ergebnis ist ein `CompiledProgram` mit `Instruction`-Vektoren je
Funktion.

- Instruktionssatz und kanonischer Encoder:
  `ai/atclang/crates/atc-core/src/bytecode.rs:21` (`enum Instruction`),
  `bytecode.rs:187` (Kopf `ATCB`, `u16` BE Version, `u32` BE Anzahl).
  Enthalten sind `ConstI64`, `LoadLocal`, `StoreLocal`, Arithmetik, sechs
  Vergleiche, `Call{function,argc}`, `Return`, `Pop`, `Jump(i16)`, `JumpIfFalse(i16)`.
- Verifikation: `bytecode.rs:235` (Worklist-Fixpoint, Stackhoehen an Join-Stellen,
  Sprungziele, Division durch Konstante 0).
- Verpackung mit eigenem Hash: `ai/atclang/crates/atc-core/src/artifact.rs:10`
  (Magic `ATCA`), `artifact.rs:104` (kanonische Nutzlast), `artifact.rs:143`
  (`encode` = Nutzlast plus SHA-256).
- Aufrufbar ueber die CLI: `ai/atclang/crates/atc-core/src/main.rs:94` (`run`),
  `main.rs:133` (`compile`), `main.rs:147` (`check`).

**Wichtig:** Das Kompilat ist **kein** ATCB-Artefakt der VM. Der Compiler
erzeugt `ATCA` (Programm plus Metadaten) beziehungsweise einen ATCB-Kopf nach
ATC-BC-001 (Instruktionsstrom allein). Die Uebersetzung auf die Operationen der VM
liegt **nicht** im Compiler.

### Frage 2: Wo wird dieser Bytecode in der ATC-VM geladen?

**Antwort:** in `components/vm/src/artifact.rs`. `Artifact::parse` liest den
Container, prueft Magic, Containerformat, Generation, angehaengten SHA-256 und die
Operationsgrenze; erst danach entsteht ein `Artifact`, aus dem
`Vm::from_artifact` beziehungsweise `from_artifact_with_context` einen
ausfuehrbaren Zustand baut.

- Kopfformat: `components/vm/src/artifact.rs:8-15` (Kommentar), `artifact.rs:22`
  (`MAGIC`), `artifact.rs:24` (`CONTAINER_VERSION`), `artifact.rs:28`
  (`GENERATION_V1`), `artifact.rs:302` (`parse`).
- Operationssatz: `components/vm/src/vm.rs:14` (`enum Op`). 16 Befehle fuer v1,
  drei zusaetzliche fuer v2 (`Assert`, `Revert`, `RevertIf`).
- Opcode-Belegung: `components/vm/src/artifact.rs:206-221`.

**Befund:** Dieser Loader laedt **kein** Kompilat des Compilers. Er laedt nur, was
das Backend in `components/system` vorher erzeugt hat. Der Compiler selbst hat
keinen Leser fuer ATCB, die VM keinen Leser fuer ATCA. Die Kante ist die
Uebersetzung, nicht ein gemeinsames Format.

### Frage 3: Wo ruft der Blockchain-Node die VM auf?

**Antwort:** nirgends. Es gibt keine Stelle im Knoten, die eine VM aufruft.

- Abhaengigkeiten des Knotens: `components/node/Cargo.toml:9-10` (`atc-algorithm`,
  `atc_wallet`). Kein `atc-vm`, kein `atc-system`.
- `TxType::Contract` (`components/wallet/src/tx.rs:21`) hat im gesamten
  Monorepo **null Konsumenten** (Suche ueber `*.rs`, Treffer nur in Dokumentation).
- Der Mempool prueft Signatur, DID-Bindung, Nonce, Ketten-ID und Duplikat
  (`components/node/src/mempool.rs:91`), **nicht** `tx.payload` und **nicht**
  `tx.gas_price`/`tx.gas_limit`. Ein Contract-Transaktionsobjekt wird
  angenommen und danach nie ausgewertet.
- Bloecke tragen eine Zeichenkette: `components/node/src/chain.rs:17`
  (`payload: String`), erzeugt aus Transaktions-IDs
  (`chain.rs:251` `encode_tx_ids`), angehaengt in `chain.rs:157`
  (`produce_tx_block`).
- Der einzige Ort, der VM und Kette zusammenbringt, ist
  `components/system/src/system.rs:301` (Zustand in die VM),
  `system.rs:321` (Zustand aus der VM) und `system.rs:325-330`
  (Payload `exec:<code8>:<wurzel>`, dann `chain.produce`).
  Das ist ein Bibliothekspfad, kein Knotenverhalten: `System` haelt seine eigene
  `Chain` im Speicher (`system.rs:163`), die ueber RPC nie erreichbar ist.

**Folge:** Die Vorlage verlangt, dass die Blockchain das Ausfuehrungsergebnis
prueft und uebernimmt. Heute uebernimmt sie es nicht, weil sie es nicht kennt.

### Frage 4: Wie werden State Reads und State Writes uebertragen?

**Antwort:** als **vollstaendige Kopie** eines `Vec<u64>`. Es gibt keine Schluessel,
keine Adressen, kein Lese-/Schreib-Set und keine Beweise.

- Uebergabe hinein: `components/system/src/system.rs:301`
  (`Vm::from_artifact_with_context(&artifact, caller, state.words().to_vec())`).
- Uebergabe heraus: `system.rs:321` (`ContractState::from_words(vm.state().to_vec())`).
- Wurzel: `components/system/src/state.rs:43` (SHA-256 ueber Domain, Code-Hash,
  Wortzahl und Woerter in Little Endian).
- Lesezugriff/Schreibzugriff in der VM: `components/vm/src/vm.rs:371` und
  `vm.rs:374`, jeweils mit Grenze `MAX_STORAGE_SLOTS = 65_536`
  (`vm.rs:10`), Wachstum bis zur Grenze, Ruecknahme bei Fehler ueber eine
  Sicherungskopie (`vm.rs:319-328`).

**Befund:** Das Speichermodell der Sprache (`state`-Felder, `Map<K,V>`) ist im
Kettenpfad nicht vorhanden. `StorageLayout` aus Stufe S1
(`ai/atclang/crates/atc-core/src/storage.rs`) ist ausschliesslich ueber die CLI
`atc storage` erreichbar (`main.rs:317`); kein VM- oder System-Pfad nutzt es.
Deshalb gibt es auch keine Slot-Nummern, die ein Vertrag mit der Kette teilen
koennte, und keine Wurzel, die man einem Pruefer vorlegen koennte, ohne den ganzen
Zustand mitzuschicken.

### Frage 5: Wie werden Gas und Transaktionsgebuehren synchronisiert?

**Antwort:** gar nicht. Es gibt drei Gasbegriffe ohne Verbindung.

| Ort | Feld | Wer liest es |
|---|---|---|
| Transaktion | `gas_price`, `gas_limit` (`components/wallet/src/tx.rs:31-32`) | niemand im Knoten |
| System | `DEFAULT_GAS = 50_000` (`components/system/src/system.rs:37`) | `execute`-Aufrufer, per Parameter ueberschreibbar |
| VM | `GAS_TABLE_V1`, 19 Befehle, Kennung `GAS_TABLE_V1` (`components/vm/src/vm.rs:71-80`) | `execute_state_transition_with_gas` (`vm.rs:201`) |

- Verbrauch wird intern gemessen, aber bei Erfolg **nicht** zurueckgegeben
  (Kommentar `vm.rs:68-71`; bekannte Luecke GAS-AUD-005).
  Das Receipt traegt deshalb nur das Budget.
- Eine Abbuchung gibt es nirgends: der Knoten hat kein Konto, keinen Saldo und
  keine Gebuehrenlogik (`components/node/src/chain.rs:4-6` sagt das ausdruecklich),
  das System bucht nichts ab, der Wallet-Saldo ist ein lokales Modell
  (`components/wallet/src/balance.rs`).

**Folge:** Ein Angreifer kann die Gebuehrenachse heute nicht umgehen, weil sie
nicht existiert. Genauso wenig kann ein Validator Gebuehren pruefen.

### Frage 6: Fuehren alle Validatoren dieselbe Ausfuehrungslogik aus?

**Antwort:** Es gibt keine Ausfuehrung, die ein Validator ausfuehrt, also auch
keine Abweichung. Der Konsens stimmt ueber einen Hash ab.

- Proposal-Signatur: secp256k1/ECDSA, RFC 6979, Low-S, Domain
  `ATC-PROPOSE-V1`, Pruefung gegen den Pubkey im Set
  (`components/node/src/validator.rs:1-20`, `validator.rs:136`).
- Quorum: zwei Drittel der Voting-Power, aufgerundet
  (`components/node/src/consensus.rs:116`), Finalisierung in
  `consensus.rs:262`.
- Blockimport: `chain.rs:177` prueft Hoehe und Transaktionen, dann
  `produce(encode_tx_ids(...))`. Kein erneutes Ausfuehren, kein Vergleich von
  Zustandswurzeln, weil der Block keine enthaelt.
- Die Determinismus-Eigenschaft ist auf dem Pfad, den es gibt, nachgewiesen:
  zwei frische Systeme mit gleicher Eingabe erzeugen gleiche Wurzel und gleichen
  Blockhash (`components/system/tests/e2e.rs:94`).

**Folge:** Der Satz "andernfalls koennten Validatoren unterschiedliche Zustaende
berechnen" beschreibt derzeit keine Gefahr, weil der Zustand nicht in die
Blockgueltigkeit eingeht. Sobald Vertragsausfuehrung Teil der Blockgueltigkeit wird,
ist die Frage sofort scharf, und dann fehlt die Zustandswurzel im Block.

### Der zweite Ausfuehrungspfad: EXEC-GATE

Neben atclang gibt es einen aelteren, im Betrieb benutzten Weg:
`components/contracts/exec_chain/*.atc` wird durch `assemble.py` in ein Textformat
`.ops` uebersetzt und von `atc-vm-runner --ops` auf der ATVM ausgefuehrt
(`components/vm/src/main.rs:1-12`, `components/vm/tests/exec_chain_e2e.rs:1-12`).
Der `.ops`-Parser liegt in `components/vm/src/ops.rs:31`; 12 Tests in
`exec_chain_e2e.rs` decken ihn ab.

Bedeutung fuer die Fragen 1 bis 3: Die VM wird real benutzt, aber von einer
zweiten, aelteren Sprachfront. Der Compiler aus Frage 1 ist daran nicht beteiligt.
Damit gibt es drei Traeger fuer denselben Vertragsbegriff: `ATCA`
(atclang-Kompilat), `ATCB` (VM-Container) und `.ops` (Assembler-Text). Nur der
letzte Weg ist heute durchgehend in Benutzung, und er ist textuell.

## 2. Befunde

| ID | Befund | Schwere | Ort |
|---|---|---|---|
| IV-01 | Der Knoten ruft die VM nicht auf. Ausfuehrung ist kein Bestandteil der Blockgueltigkeit; `TxType::Contract` ohne Konsumenten. | blockierend fuer L1 | `components/node/Cargo.toml:9-10`, `chain.rs:17` |
| IV-02 | Die einzige Kante liegt in `components/system`, nicht im Compiler und nicht im Knoten. Wer `atclang` direkt nutzt (CLI, IDE, SDK), erhaelt `ATCA` und kann nicht ausfuehren. | hoch | `components/system/src/backend.rs`, `ai/atclang/crates/atc-core/src/artifact.rs:10` |
| IV-03 | Zwei Koepfe namens `ATCB` v1: Compiler `u16` BE Version und `u32` BE Anzahl; VM Container-Byte `0x01`, Generation, `u32` LE Anzahl, SHA-256-Tail. Gleiche Magic, verschiedene Bytes. | hoch | `ai/atclang/.../bytecode.rs:187` gegen `components/vm/src/artifact.rs:8-15` |
| IV-04 | Keine Aufrufkonvention: `Call` wird abgelehnt, keine Calldata, kein Dispatch, kein Rueckgabepfad ausser einem `u64`. `ATC-8300` laeuft als 2 Instruktionen mit Ergebnis 0. | blockierend fuer Smart Contracts | `components/system/src/backend.rs:156` |
| IV-05 | Lokale Variablen sind Speicherplaetze: `LoadLocal` wird zu `Op::Load`, `StoreLocal` zu `Op::Store`, und `Load`/`Store` sind persistenter Vertragszustand. Jede Lokale ist damit ein Zustandsfeld. | hoch | `backend.rs:115-116`, `components/vm/src/vm.rs:371-389` |
| IV-06 | Zustand ist `Vec<u64>` ohne Schluessel, Adressraum, Lese-/Schreib-Set oder Beweis. `StorageLayout` (S1) ist an keinen Laufzeitpfad angebunden. | hoch | `components/system/src/state.rs:12-53`, `main.rs:317` |
| IV-07 | Gas ist dreifach unverbunden (Tx-Felder ungenutzt, Systembudget, VM-Tabelle). Keine Abbuchung, kein Saldo, Verbrauch nicht im Receipt. | hoch | `tx.rs:31-32`, `system.rs:37`, `vm.rs:71-80` |
| IV-08 | Identitaetsdomaenen treffen sich nicht: `caller` ist `u64`, die Kette identifiziert per DID-Zeichenkette (`tx.rs:29`). Eine Berechtigungspruefung im Vertrag ist nicht ausdrueckbar. | hoch | `system.rs:266`, `vm.rs:58` |
| IV-09 | Die Freigabe ist im Kettenpfad nicht verankert: nur `atc-system` prueft die Policy-Signatur, der Knoten akzeptiert jeden Payload. Der Devnet-Schluessel ist die Konstante `[0x42; 32]`, also oeffentlich bekannt. | mittel (Devnet), hoch (Mainnet) | `system.rs:31`, `system.rs:163` |
| IV-10 | Konsens prueft keine Ausfuehrung, Blockimport rechnet nicht nach. Keine Zustandswurzel im Block. | hoch | `consensus.rs:116`, `chain.rs:177-200` |
| IV-11 | Offene Semantikdivergenz: `i64` geprueft gegen `u64` umlaufend, `Lt` vorzeichenbehaftet gegen vorzeichenlos. Maschinenlesbar in `DIVERGENZEN`. | mittel, bewusst offen | `components/system/src/backend.rs:32` |
| IV-12 | Keine Ereignisse: `emit` wird im Lowering verworfen, das Receipt hat kein Ereignisfeld. Fuer Token-Standards fehlt die Spur. | mittel | Anlass von `standards/atc/candidates/ATC-BC-002-CANDIDATE.md` Abschnitt 1 |

| IV-13 | Drei Vertragsfronten statt einer: `atclang` (ATCA), VM-Container (ATCB) und EXEC-GATE (`.ops`). Nur der `.ops`-Weg ist heute durchgehend benutzt, und er umgeht den Compiler. | hoch | `components/contracts/exec_chain`, `components/vm/src/ops.rs:31` |

Was **traegt** und nicht in Frage gestellt wird:

- Der Pfad selbst ist fail-closed an jeder Stufe: manipulierte Artefaktbytes,
  fremder Schluessel, Widerruf, Ablauf, Gas 0 und ein Gas weniger als noetig
  erzeugen jeweils einen Abbruch ohne Zustandsaenderung
  (`components/system/tests/e2e.rs:36-215`, `signature.rs`).
- Zustandsuebergabe ist transaktional: bei VM-Fehler wird der Zustand
  zurueckgenommen (`vm.rs:319-328`), `state` wird nur bei Erfolg fortgeschrieben
  (`system.rs:332`).
- Determinismus auf dem bestehenden Pfad: gleiche Eingabe, gleiche Wurzel,
  gleicher Blockhash (`e2e.rs:94`).

## 3. Messungen am gebundenen Stand

Lokaler Lauf mit `cargo test -p atc-core -p atc-node -p atc-system -p atc-vm`
auf `09fee2a3`:

| Crate | Tests | Ergebnis |
|---|---:|---|
| `atc-core` (Compiler) | 197 | 0 rot |
| `atc-node` (Kette) | 89 | 0 rot |
| `atc-system` (Integration) | 46 | 0 rot |
| `atc-vm` (VM) | 132 | 0 rot |
| **Summe** | **464** | **0 rot** |

`atc-system demo`: 19 ATCLang-Instruktionen zu 20 ATCB-Operationen, 138-Byte-Artefakt,
Code-Hash `5f2c38c8...`, Ergebnis `10`, Blockhoehe 1, `Chain::verify` ok, Wiederholung
ohne weitere Zustandsaenderung.

Sweep ueber die sechs Vertraege in `ai/atclang/examples`:

| Vertrag | Ergebnis | Befund |
|---|---|---|
| `atc8300.atc` | PASS, Ergebnis 0 | 2 Instruktionen, Zustand unveraendert. Token-Standard laeuft leer. |
| `atcos_main.atc` | PASS, Ergebnis 0 | 2 Instruktionen, Zustand unveraendert. |
| `main.atc` | PASS, Ergebnis 0 | 14 Instruktionen, Zustand aendert sich (`7241ca80...` zu `1b1f0934...`), obwohl das Programm nur rechnet. Beleg fuer IV-05. |
| `event_bus.atc` | FAIL, Stufe compile | Parser: `data : data[:120]` |
| `kernel.atc` | FAIL, Stufe compile | Parser: `for` ueber `pids` |
| `shivamon.atc` | FAIL, Stufe compile | Parser: `Rarity::COMMON` |

Damit ist die Aussage aus der Vorlage bestaetigt: eine dokumentierte
Architekturverbindung ist kein Nachweis einer funktionierenden Integration. Drei
von sechs Vertraegen erreichen ueberhaupt die Kette, und keiner von diesen dreien
fuehrt seine Fachlogik aus.

## 4. Die fuenf ABIs aus Abschnitt 6 der Vorlage

| ABI | Zustand | Begruendung |
|---|---|---|
| Bytecode ABI | **fehlt** | Zwei Koepfe mit gleichem Namen (IV-03), zwei Operationssaetze ohne gemeinsame Nummerierung. |
| Execution Context ABI | **teilweise** | `ChainContext` traegt Chain-ID, Netz-ID, Genesis-ID, Protokoll- und VM-Version (`components/vm/src/context.rs:6-13`). Es fehlen Sender-Identitaet (IV-08), Vertragsadresse, Blockhoehe/-zeit, Calldata und Wert. |
| State Access ABI | **fehlt** | Ganze Vektor-Kopie statt Schluesselzugriff (IV-06), keine Zugriffslisten, keine Beweise. |
| Gas ABI | **fehlt** | Tabelle vorhanden, aber kein Vertrag zwischen Tx, Block und VM (IV-07), Verbrauch nicht im Ergebnis. |
| Execution Result ABI | **teilweise** | `Receipt` traegt Code-Hash, Wurzel vorher/nachher, einen `u64`, Blockhoehe und Hash. Es fehlen Ereignisse, Fehlercode, Gasverbrauch und strukturierte Rueckgabedaten. |

## 5. Entscheidungen, die vor der Implementierung fallen muessen

1. **D1 Kopfwechsel.** Welcher Kopf ist `ATCB` v1: ATC-BC-001 (FROZEN, `u16` BE)
   oder der Container mit Generation und LE aus ATC-BC-002 (Kandidat)? Eine der
   beiden Kennungen muss umbenannt oder neu versioniert werden, sonst bleibt der
   Name doppelt belegt. Ohne diese Entscheidung ist jeder Loader eine Vermutung.
2. **D2 Aufrufkonvention.** Calldata, Funktionsselektor, Rueckgabedaten,
   Vertragsadresse und `Call`-Semantik, oder die ausdrueckliche Festlegung, dass
   L1 nur zustandslose Einzelprogramme ausfuehrt.
3. **D3 Speichermodell.** Trennung von Rahmen-lokalen und persistenten Slots.
   `StorageLayout` (S1) muss an die VM angebunden werden, sonst bleibt jeder
   Vertrag eine Ansammlung von Speicherplaetzen ohne Namen.
4. **D4 Gas und Gebuehren.** `tx.gas_limit` ueber Systembudget in die VM,
   Verbrauch im Receipt, Abbuchung gegen ein Konto. Vorher ist "Gebuehren" ein
   Feld ohne Wirkung.
5. **D5 Konsens und Ausfuehrung.** Entweder Zustandswurzel in den Block und
   Nachrechnen beim Import, oder die ausdrueckliche Erklaerung, dass Devnet-Bloecke
   keine Ausfuehrungsfinalitaet tragen.
6. **D6 Identitaet.** `caller` als `u64` gegen DID: entweder eine kanonische,
   kollisionsfreie Abbildung, oder `caller` wird ein ABI-Wert und die VM bekommt
   einen Identity-Host-Aufruf.

7. **D7 Ein Frontend.** Welcher Weg ist der kanonische Vertragspfad: `atclang`
   oder EXEC-GATE (`.ops`)? Zwei Frontends nebeneinander bedeuten zwei Semantiken,
   zwei Testkoerper und zwei Wahrheiten fuer denselben Vertrag.

## 6. Status nach Evidenzstufe

| Bereich | Stufe | Beleg |
|---|---|---|
| Compiler (Lexer bis Verifier, ATCA) | IMPLEMENTED / TESTED lokal | 197 Tests am gebundenen Stand |
| VM (ATCB-Container, Gas, Ruecknahme) | IMPLEMENTED / TESTED lokal | 132 Tests |
| Kette (Block, Mempool, Proposal-Signatur) | IMPLEMENTED / TESTED lokal | 89 Tests |
| Integrationspfad Quelle zu Block (in `atc-system`) | IMPLEMENTED / TESTED lokal | 46 Tests, Demo-Lauf |
| Fachlogik von Vertraegen auf der Kette | NICHT VORHANDEN | IV-04, IV-05 |
| Ausfuehrung als Blockbestandteil | NICHT VORHANDEN | IV-01, IV-10 |
| Gebuehren | NICHT VORHANDEN | IV-07 |
| Ereignisse | NICHT VORHANDEN | IV-12 |
| CI_VERIFIED | OFFEN | keine Actions-Evidenz am Stand |
| E2E_VERIFIED (mehrere Validatoren) | OFFEN | ein Prozess, ein Knoten |

## 7. Was dieser Audit nicht behauptet

- Keine Aussage ueber Konsensguete, Finalitaet oder Netzwerkverhalten.
- Keine Aussage ueber `aurora-ai`, Kernel oder Produkt-Apps.
- Keine CI-Evidenz. Alle Testzahlen sind lokale Laeufe am genannten Stand.
- Keine Aussage darueber, dass der Pfad produktionsreif ist. Er ist es nicht.

## 8. Fortsetzung (12.11.2026)

Dieser Audit beschreibt den Ist-Stand. Die Arbeit daraus liegt in vier Dokumenten:

- [`CONTRACT-EXECUTION-INVENTORY-2026-11-12.md`](CONTRACT-EXECUTION-INVENTORY-2026-11-12.md):
  Implementierungsstellen, konkurrierende Wahrheiten und die Konfliktmatrix K-01 bis K-12.
- [`../decisions/AD-EXEC-D1-D7.md`](../decisions/AD-EXEC-D1-D7.md): die sieben
  Entscheidungen ausgearbeitet, mit Optionen, Empfehlung, Folgen und Gates.
- [`../../standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md`](../../standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md):
  der Integrationsvertrag als Kandidat, 38 Anforderungen, 10 Gates.
- [`../EXECUTION-INTEGRATION-PLAN.md`](../EXECUTION-INTEGRATION-PLAN.md): Phasen P0 bis P8
  mit Eingangs- und Ausgangskriterien.

Kein Punkt dieses Audits ist damit behoben. Er ist geordnet.
