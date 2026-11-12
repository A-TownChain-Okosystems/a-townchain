# Architekturentscheidungen D1 bis D7 (Ausarbeitung zur Owner-Entscheidung)

**Datum:** 2026-11-12
**Stand:** `a-townchain` main `43b194c86`
**Grundlage:** `docs/audits/ATCLANG-VM-CHAIN-INTEGRATION-2026-11-11.md` (Befunde IV-01 bis IV-13),
`docs/audits/CONTRACT-EXECUTION-INVENTORY-2026-11-12.md` (Inventar, Konflikte K-01 bis K-12)
**Status:** Vorschlag. Keine dieser Entscheidungen ist beschlossen. Jede Zeile
"Empfehlung" ist eine begruendete Vorlage, keine Feststellung.

**Reihenfolge (technische Abhaengigkeit, nicht Beschluss):**
D1 (Format) → D7 (Frontend) → D2 (Aufruf) → D3 (Speicher) → D6 (Identitaet) → D4 (Gas) → D5 (Block).

---

## D1 — Welches Bytecodeformat ist das normative?

**Ist-Stand.** `ATC-BC-001` ist `SPEC-FROZEN` und normativ seit 06.10.2026
(`ai/atclang/specs/bytecode/SPEC.md`, SCR-0071); Referenzimplementierung ist
`atc-core/src/bytecode.rs`. Die VM implementiert einen anderen Container
(`components/vm/src/artifact.rs:8-15`) mit anderer Opcode-Belegung
(`0x01` Push gegen `0x01` ConstI64, `0x10` Halt gegen `0x10` Add) und nennt ihn
ebenfalls `ATCB` v1. Es liegt damit eine **Normabweichung** vor, nicht nur eine
Namensdopplung (Konflikt K-01, K-02).

**Optionen**

| Option | Inhalt | Kosten | Risiko |
|---|---|---|---|
| A | Instruktionsstrom bleibt `ATC-BC-001`. Der VM-Container wird zu einer eigenstaendigen Huelle mit eigenem Magic (Vorschlag `ATCE`), die Generation, Stromlaenge, Layout-Version und SHA-256 traegt. Opcodes der VM werden auf die BC-001-Nummerierung umgestellt. | Umbau des VM-Loaders und des Backends, alle Devnet-Artefakte neu erzeugen | klein: rund 40 betroffene Tests, keine Aussenzusage bricht, weil niemand Artefakte archiviert |
| B | `ATC-BC-002` wird Nachfolger als v2 mit eigener Opcode-Belegung; BC-001 bleibt fuer atclang v0.1 bestehen. | zwei Generationen dauerhaft pflegen, Compiler braucht zwei Emitter | hoch: zwei Wahrheiten, genau der Zustand, der heute schon schadet |
| C | Beide Formate im Loader akzeptieren (Bridge) | sehr klein | sehr hoch: erzeugt das doppelte Format dauerhaft |

**Empfehlung: Option A.**
Begruendung: BC-001 ist bereits normativ, hat einen Verifier mit Worklist-Analyse
und Tests, und die VM-Seite ist mit 19 Befehlen die kleinere Baustelle. A erfuellt
die Vorgabe "genau ein normatives Bytecodeformat" ohne den eingefrorenen Standard
anzutasten. Die Huelle wird damit zu dem, was sie technisch ist: Transport und
Integritaet, nicht Sprache. BC-002 bleibt danach frei fuer das, was es wirklich
hinzufuegt (`Assert`, `Revert`, Ruecklauf-Ebenen), ohne mit der Huelle vermischt zu
werden.

**Folgen bei A.** `components/system/src/backend.rs` wird vom Opcode-Uebersetzer
zum Semantikpruefer (Abbildung wird identisch, Divergenzen bleiben als Pruefung).
Die VM verliert `u64`-umlaufende Arithmetik nicht automatisch; das ist eine eigene
Entscheidung im Befehlssatz (siehe D4/K-12). `ai/atclang/specs/bytecode/ATCB-1.md`
verweist kuenftig auf die Huelle `ATCE`. Kein zweiter Compiler, keine zweite VM.

**Gate:** SCE-G1 (ein Format, ein Verifier, eine Huelle) mit dem Nachweis, dass
dasselbe Artefakt von Verifier und VM gelesen wird.

---

## D7 — Welches Frontend ist das kanonische?

**Ist-Stand.** Zwei Wege: ATCLang (`atc-core`, ATCA) und EXEC-GATE
(`components/contracts/exec_chain`, `.atc` mit eigener Syntax, `assemble.py` zu
`.ops`, ausgefuehrt von `atc-vm-runner --ops`). Nur der zweite Weg ist heute
durchgehend benutzt, und er umgeht den Compiler (Befund IV-13, Konflikt K-10).

**Optionen**

| Option | Inhalt |
|---|---|
| A | ATCLang ist kanonisch. `.ops` bleibt ausschliesslich Testfixture (Vektoren, Differential-Corpus) und wird im Produktionspfad abgeschaltet. |
| B | EXEC-GATE bleibt kanonisch, ATCLang wird Generator dafuer |
| C | Beide bleiben, mit Gleichstellung |

**Empfehlung: Option A.**
Begruendung: Die Sprache ist das Produkt; ein Assembler-Text als Produktionspfad
hat keine Typpruefung, keine ABI und keine Registry-Anbindung (SC-019). B kehrt die
Richtung um und macht jede Sprachfunktion zu einer Assembler-Konvention. C ist der
heutige Zustand mit zwei Semantiken.

**Folgen bei A.** `atc-vm-runner --ops` bekommt einen ausdruecklichen
Entwicklungsmodus und wird aus dem Kettenpfad entfernt. Die 16 Registry-Vertraege
werden schrittweise auf ATCLang portiert; bis zur Paritaet laufen beide, danach
wird der `.ops`-Produktionspfad stillgelegt und im Register als Legacy gefuehrt.

**Gate:** SCE-G2 (ein Frontend) mit dem Nachweis, dass jeder Vertrag im
Produktionspfad aus ATCLang entsteht.

---

## D2 — Wie wird ein Vertrag aufgerufen?

**Ist-Stand.** Es gibt keinen Aufruf. `Instruction::Call` wird abgelehnt
(`components/system/src/backend.rs:156`), es gibt keine Calldata, keinen
Selektor, keinen Rueckgabepuffer und keine Vertragsadresse. `ATC-8300` wird zu
2 Instruktionen und liefert `0` (Befund IV-04, Konflikt K-03, K-12).

**Optionen**

| Option | Inhalt | Bewertung |
|---|---|---|
| A | Vollstaendiger Aufrufvertrag: Selektor, kanonische Calldata, Rueckgabepuffer, Value separat, deterministische Vertragsadresse, Call-Tiefe | noetig fuer jeden Smart Contract |
| B | Nur Einstiegsfunktion ohne Argumente (Status quo) | kein Vertrag im Sinne der Vorlage |
| C | Nachrichten-ABI mit beliebigem Nachrichtenraum | groesser als noetig, schwerer zu verifizieren |

**Empfehlung: Option A, in zwei Stufen.**
Stufe 1 (ohne neuen Opcode): ein Aufruf traegt Selektor und Calldata, die VM
fuehrt genau eine Funktion aus und liefert einen Rueckgabepuffer. Stufe 2: der
`Call`-Befehl zwischen Vertraegen, **erst nach** Ratifizierung von ATC-BC-002,
weil ein Aufruf ohne `Revert`-Semantik bei Teilfehlern halbe Zustaende
hinterlaesst. Diese Reihenfolge ist Teil der Empfehlung, nicht Zierde.

**Eckpunkte, die der Vertrag festlegen muss**

- Selektor: erste 4 Byte von `SHA-256(kanonische Signatur)`, Signaturquelle ist
  `ai/atclang/crates/atc-core/src/abi.rs` (Feld `signature`, `sha256` je Vertrag).
- Calldata: kanonische Folge von 32-Byte-Woertern, Laenge explizit im Aufruf.
- Rueckgabe: Rueckgabepuffer mit Statuscode, Laenge, Daten; kein stilles
  Abschneiden.
- Vertragsadresse: deterministisch aus Deployer, Nonce und Code-Hash, mit eigener
  Domain-Trennung, Format wie `address_from_pubkey` (`ATC:` plus 20 Byte).
- Value: eigener Wert, nicht Teil der Calldata.
- Grenzen: Call-Tiefe, Calldata-Groesse, Rueckgabegroesse, Zaehler je Frame.

**Folgen bei A.** Die VM bekommt einen Aufrufrahmen (Frame mit Codezeiger,
Lokalen, Rueckgabepuffer), `receipt` traegt Status, Rueckgabedaten und Adresse.
`caller: u64` entfaellt (siehe D6). `TxType::Contract` wird endlich benutzt.

**Gate:** SCE-G3 (Aufrufvertrag) mit Pflichtfaellen: gueltiger Aufruf, unbekannter
Selektor, falsche Calldata-Laenge, Rueckgabe zu gross, Aufruf an nicht
existierenden Vertrag.

---

## D3 — Wie wird Zustand gehalten?

**Ist-Stand.** Drei Modelle ohne Verbindung: Rahmen-lokale im Compiler,
persistente Woerter in der VM (`Op::Load`/`Op::Store`, `components/vm/src/vm.rs:371-389`),
ABI-Slots aus S1 (`atc-core/src/storage.rs`, 32 Byte, `LAYOUT_VERSION = 1`,
`layout_hash`), die nur die CLI erreicht (Befund IV-05, IV-06, Konflikt K-04).

**Optionen**

| Option | Inhalt | Bewertung |
|---|---|---|
| A | Lokale leben in einem begrenzten Frame-Speicher. Persistent sind ausschliesslich S1-Slots, adressiert ueber den ABI-Layout-Hash. | deckt die G11-Freigabe und die SC-Anforderungen |
| B | Alles bleibt persistent (Status quo) | jede Schleifenvariable schreibt Kettenzustand |
| C | Nur opake Blobs je Vertrag | keine Feld-Semantik, keine Migration, keine ABI |

**Empfehlung: Option A.** Sie ist bereits durch die G11-Freigabe vom 08.11.2026
gedeckt (Slots fester Breite, Feldnamen nur Metadaten, versionierte Layouts,
`layout_hash`). Neu ist nur die Anbindung an die Laufzeit: eigene Befehle fuer
Lokale (`LocalLoad`, `LocalStore`) getrennt von Zustandsbefehlen
(`SLoad`, `SStore`), und der Layout-Hash wird im Artefaktkopf verlangt, damit ein
Vertrag nicht gegen ein fremdes Layout laeuft.

**Offen und ausdruecklich nicht empfohlen zu erfinden:** `Map<K,V>`. Ein
Schluessel-Slot entsteht deterministisch aus Layout-Hash, Slot-Index und
kanonischem Schluessel, ist aber ein eigenes Befehlspaar und gehoert in eine
Generation nach BC-001. Bis dahin gilt ehrlich: Maps sind auf der Kette nicht
unterstuetzt, und der Compiler muss sie im Kettenziel ablehnen, statt sie still
abzuflachen.

**Folgen bei A.** `ContractState` wird von `Vec<u64>` zu einer geordneten
Slot-Tabelle mit Layout-Bezug; `state_root` wird aus (Vertragsadresse, Layout-Hash,
Slot, Wert) gebildet. Das ist eine Aenderung an `state.rs`, nicht an der VM allein.

**Gate:** SCE-G4 (Speichertrennung) mit Pflichtfaellen: Lokale ueberschreibt keinen
Slot, gleicher Feldname in zwei Vertraegen kollidiert nicht, Layout-Wechsel
bricht die Ausfuehrung ab.

---

## D6 — Wie heisst der Aufrufer?

**Ist-Stand.** Die VM kennt `caller: u64` (`components/system/src/system.rs:266`,
`components/vm/src/vm.rs:58`). Die Kette kennt `ATC:`-Adressen, und die DID ist
dieselbe Ableitung: `address_from_pubkey` = `ATC:` plus 20-Byte-Prefix von
SHA-256 (SEC-P0-005, `components/wallet/src/keys.rs:10`), und
`DidRegistry::register` setzt die DID genau auf diesen Wert
(`components/node/src/did.rs:121`). Die Identitaet der Kette ist also bereits
einheitlich; nur die VM faellt heraus (Konflikt K-06).

**Optionen**

| Option | Inhalt | Bewertung |
|---|---|---|
| A | `caller` wird ein 32-Byte-Identitaetswort (20-Byte-Adresse, links mit Nullen), `u64` entfaellt | nutzt die bestehende Ableitung, keine neue Wahrheit |
| B | Tabelle `u64` zu DID im Zustand | zusaetzlicher Zustand, Synchronisationsproblem, kollidiert mit Rotaion |
| C | `u64` bleibt, Berechtigung nur ausserhalb des Vertrags | Berechtigung im Vertrag unmoeglich |

**Empfehlung: Option A.** Die Ableitung existiert, ist normativ (SEC-P0-005) und
wird von Wallet und DID-Registry bereits geteilt. Ein Identitaetswort kostet
nichts ausser einem Opcode, der 32 Byte liefert.

**Folgen bei A.** Der Backend- und VM-Testkoerper aendert sich, das Receipt traegt
die Aufruferadresse, und Berechtigungspruefungen im Vertrag werden ausdrueckbar
(`caller == owner`).

**Gate:** SCE-G5 (Identitaet) mit Pflichtfaellen: Adresse aus Pubkey, Rotation
aendert die aktuelle Adresse nicht, Aufruf ohne gueltige Bindung wird abgelehnt.

---

## D4 — Wie werden Gas und Gebuehren gerechnet?

**Ist-Stand.** Vier Quellen ohne Verbindung: `GAS_TABLE_V1` mit 19 Befehlen
(`components/vm/src/vm.rs:71-80`), `DEFAULT_GAS = 50_000`
(`components/system/src/system.rs:37`), `tx.gas_price`/`tx.gas_limit`
(`components/wallet/src/tx.rs:31-32`, vom Knoten nie gelesen) und die
ATC-14-Tabelle (`deploy 50000 + 10/byte`, `call`, `create 32000`). Verbrauch wird
bei Erfolg nicht zurueckgegeben (GAS-AUD-005), eine Abbuchung existiert nirgends
(Befund IV-07, Konflikt K-05, K-11).

**Optionen**

| Option | Inhalt | Bewertung |
|---|---|---|
| A | Ein Gastvertrag: eine versionierte Kostentabelle, Budget gleich `tx.gas_limit`, Verbrauch im Receipt, Gebuehr gleich Verbrauch mal `gas_price`, Abbuchung gegen ein Konto, Fehlschlag verbraucht das ganze Budget | erfuellt die Vorgaben der Vorlage |
| B | Zwei Budgets (heute) | keine Abrechnung, kein Schutz |
| C | Kostenloser Betrieb ohne Gas, nur Ops-Limit | einfacher, aber widerspricht ATC-14 und SC-016 |

**Empfehlung: Option A.** Eine Tabelle, eine Kennung (`GAS_TABLE_V2`), eine
Reihenfolge: Kosten vor der Wirkung belasten, Fehlschlag verbraucht alles,
Erfolg meldet den Verbrauch. Die Zahlen selbst sind eine Owner-Entscheidung; die
ATC-14-Formel fuer Deployment und `GAS_TABLE_V1` fuer Befehle sind die beiden
vorhandenen Quellen, die zu **einer** Tabelle zusammengefuehrt werden muessen.

**Folgen bei A.** `tx.gas_limit` wird vom Mempool geprueft und begrenzt die
Ausfuehrung, `receipt` traegt `gas_used`, `System::execute` bucht gegen ein Konto,
und die Kette braucht eine Kontofuehrung, die es heute nicht gibt.

**Gate:** SCE-G6 (Gasvertrag) mit Pflichtfaellen: Budget 0, Budget exakt
ausreichend, Budget ein Gas zu klein, Fehlschlag verbraucht alles, Verbrauch
identisch bei zwei Laeufen.

---

## D5 — Ist Ausfuehrung Teil der Blockgueltigkeit?

**Ist-Stand.** Nein. `components/node` hat keine VM-Abhaengigkeit
(`Cargo.toml:9-10`), `TxType::Contract` hat null Konsumenten, die Block-Payload
ist eine Transaktions-ID-Liste (`components/node/src/chain.rs:17`, `:251`), der
Konsens stimmt ueber Hashes ab (`consensus.rs:116`), der Import rechnet nicht nach
(`chain.rs:177-200`). Die Zustandswurzel existiert nur im Bibliothekspfad
(Befund IV-01, IV-10, Konflikt K-07, K-08, K-09).

**Optionen**

| Option | Inhalt | Bewertung |
|---|---|---|
| A | Ausfuehrung ist Blockbestandteil: Block traegt Belege und Zustandswurzel, jeder Validator rechnet nach, Abweichung macht den Block ungueltig | erfuellt die Determinismusforderung |
| B | Nur der Proposer rechnet, andere pruefen einen Beweis | braucht ZKP, heute nicht vorhanden, bleibt Research |
| C | Devnet ohne Ausfuehrungsfinalitaet, ausdruecklich erklaert | ehrlicher Zwischenstand, kein Zielzustand |

**Empfehlung: Option A als Ziel, Option C ausdruecklich als Zwischenstand.**
Der Zwischenstand muss benannt sein, sonst liest sich ein gruener Test wie eine
Zusage. Reihenfolge: (1) Belege in den Block (Code-Hash, Status, `gas_used`),
(2) Zustandswurzel ueber Konten und Slots, (3) Nachrechnen beim Import mit
Abbruch bei Abweichung, (4) Aufnahme (Admission) im Kettenpfad verankern, (5)
Gasbudget aus der Transaktion.

**Folgen bei A.** Der Knoten bekommt die VM-Kante, der Block eine neue Struktur
(kein reiner Zeichenketten-Payload mehr), das Netzformat aendert sich, und die
bestehende Kette wird ungueltig (Devnet, akzeptabel, aber ausdruecklich benannt).

**Gate:** SCE-G7 (Ausfuehrung im Konsens) mit zwei unabhaengigen Validatoren, die
denselben Block annehmen und dieselbe Zustandswurzel berechnen, plus einem
Negativfall mit absichtlich abweichender Ausfuehrung.

---

## Was diese Ausarbeitung nicht entscheidet

- Keine Zahlen fuer Gaspreise, Gebuehren oder Limits.
- Keine endgueltige Benennung der Huelle aus D1 (Vorschlag `ATCE`, nicht Beschluss).
- Keine Aussage, ob `ATC-BC-002` jemals ratifiziert wird; D2 Stufe 2 haengt daran.
- Keine Aussage ueber Wasm: ATC-14 und ATC-21 fordern eine Wasm-Sandbox, die Kette
  fuehrt eine eigene Stack-Maschine. Das ist ein eigener Standardkonflikt (K-11)
  und braucht eine SCR auf ATC-14, nicht eine stillschweigende Umdeutung.

## Abhaengigkeitsgraph

```
D1 (Format) ──┬─→ D7 (Frontend) ─→ D2 (Aufruf, Stufe 1) ─→ D3 (Speicher) ─→ D4 (Gas)
              │                              │
              └─→ BC-002-Ratifizierung ──────┴─→ D2 (Stufe 2: Call)
D6 (Identitaet) ─────────────────────────────→ D2, D3, D5
D5 (Block) ← D2, D3, D4, D6, Aufnahme (SEC-EXEC-001)
```

Kein Schritt beginnt vor der Entscheidung seines Vorgaengers. Das ist die
Reihenfolge, nach der der Plan in `docs/EXECUTION-INTEGRATION-PLAN.md` gebaut ist.
