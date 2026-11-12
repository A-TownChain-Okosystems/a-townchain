# Architekturentscheidungen D1 bis D7 (Ausarbeitung zur Owner-Entscheidung)

**Datum:** 2026-11-12
**Stand:** `a-townchain` main `43b194c86`
**Grundlage:** `docs/audits/ATCLANG-VM-CHAIN-INTEGRATION-2026-11-11.md` (Befunde IV-01 bis IV-13),
`docs/audits/CONTRACT-EXECUTION-INVENTORY-2026-11-12.md` (Inventar, Konflikte K-01 bis K-12)
**Status:** Die sieben Entscheidungen sind am 12.11.2026 vom Owner beantwortet
worden: vier freigegeben (D1, D3, D5, D7), drei bedingt freigegeben (D2, D4, D6).
Alle sieben stehen auf **DECIDED**. Numerische Gaspreise, Transaktions- und
Blockgaslimit sowie die Implementierungs- und CI-Gates sind ausdruecklich **nicht**
freigegeben; P2 bleibt gesperrt. Die Abschnitte "Empfehlung" bleiben als
Begruendung stehen, sind aber nicht mehr die Vorlage, sondern die Herleitung der
getroffenen Entscheidung.

<!-- BEGIN DECISION-REGISTER (erzeugt aus docs/decisions/exec-decision-register.yaml, nicht von Hand aendern) -->

**Entscheidungsregister** (Owner, 2026-11-12; Quelle: `docs/decisions/exec-decision-register.yaml`)

| ID | Entscheidung (Titel) | Owner-Entscheidung | Ratifikation | Spezifikation | spezifiziert in | Umsetzung | Gate |
|---|---|---|---|---|---|---|---|
| D1 | Bytecode-Kopf: eigene VM-Huelle, ATC-BC-001 unveraendert | freigegeben | nicht ratifiziert (Kandidat) | SPECIFIED | Kandidat §5.1, §18.1-18.3 und §19.1-19.5 (nicht ratifiziert) | OPEN | SCE-G1 |
| D2 | Aufrufkonvention: ABI-Selektor, Calldata, Rueckgabepuffer | bedingt freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.3, §5.10 (nicht ratifiziert) | OPEN | SCE-G3 |
| D3 | Storage: Frame-lokal und persistente S1-Slots | freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.4 (nicht ratifiziert) | OPEN | SCE-G4 |
| D4 | Gas: eine Tabelle, Transaktionsbudget, Gebuehrenabbuchung | bedingt freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.6, §5.10 (nicht ratifiziert) | OPEN | SCE-G6 |
| D5 | Blockausfuehrung: deterministische Validator-Ausfuehrung | freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.7, §5.10 (nicht ratifiziert) | OPEN | SCE-G7 |
| D6 | Identitaet: 32-Byte-Identitaetswort | bedingt freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.5, §5.10 (nicht ratifiziert) | OPEN | SCE-G5 |
| D7 | Frontend: ATCLang kanonisch, .ops nur Testwerkzeug | freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.2 (nicht ratifiziert) | OPEN | SCE-G2 |

Zustandsleiter: **DECIDED** (Entscheidung getroffen) → **SPECIFIED** (vollstaendig normativ spezifiziert) → **IMPLEMENTED** (im bestehenden System nachgewiesen). Kein Zustand ersetzt den naechsten automatisch; die Gates bleiben bis zum jeweiligen Nachweis offen.

Drei Ebenen: **Owner-Entscheidung** ist intern verbindlich, auch wenn der zugehoerige Standard nicht ratifiziert ist. **Ratifikation** sagt, ob der Standard in Kraft ist; "nicht ratifiziert (Kandidat)" bindet niemanden. Ein Wert im Kandidaten bleibt **Vorschlag**, solange der Owner ihn nicht ausdruecklich beschliesst.

Offen und nicht Teil dieser Freigabe: Numerische Gaspreise (Messung je Instruktionsklasse und Vertragsbenchmark); Endgueltiges Transaktionsgaslimit; Endgueltiges Blockgaslimit; Implementierungs- und CI-Gates; Konflikt Aufruftiefe: Owner-Entscheidung 64 vom 09.11.2026 in ATC-BC-002 (Kandidat, nicht ratifiziert) gegen SCE-Vorschlag 8 (R-901); Fehlende CALL-Instruktion und Identitaetstyp auf dem u64-Stapel (R-902, R-907); Autoritaet der beiden Instruktionssaetze (R-906).

**Stufe-1-Ratifikation** (Owner, 2026-11-12; Quelle: `docs/decisions/exec-decision-register.yaml`)

| ID | Gegenstand | Option | ratifiziert | Basis | Wirkung |
|---|---|---|---|---|---|
| S1-1 | Drei getrennte Identitaeten: artifact_hash, code_hash, state_root | A mit Domaenenvariante 1b | 2026-11-12 | `a63e41a93` | zweite Hashidentitaet wird berechnet, nicht nur benannt; neue Containerfassung ist Teil derselben Entscheidung; ATC-STATE-V2 unabhaengig von der Variante |
| S1-2 | Policy-Bindung: welche Identitaet Freigabe, Widerruf, Mindestversion bindet | C, asymmetrisch | 2026-11-12 | `a63e41a93` | Freigabe und Mindestversion am code_hash, Widerruf getrennt fuer Code und Artefakt; getypte Schluesselraeume, zwei Abfragen, nur als vollstaendige Einheit (S1-T-38) |
| S1-3 | Erstattung: Grundsatz | A, ausschliesslich Gasrueckgabe | 2026-11-12 | `a63e41a93` | gas_refund in Gaseinheiten, kein Geldbetrag; ungenutztes Budget verfaellt, auch nach Einfuehrung einer Kontobelastung |
| S1-4 | Testschichtung L1/L2/L3 | B, nur L1 zaehlt fuer P1a | 2026-11-12 | `a63e41a93` | L1 muss nicht ausfuehrbar sein, um zu zaehlen; L2 und L3 bleiben bis B-1 gesperrt; L1 ist heute EMPTY, P1a bleibt offen |

Durch die Stufe-1-Ratifikation entfallen: B-2 Artefaktidentitaet im Identitaetsteil (R-903); Container-Magic am 12.11.2026 entschieden: ATCE fuer die Huelle; B-3 Verfahren (R-905): Schichtung entschieden, L2 und L3 bleiben bis B-1 gesperrt.

Eine Stufe-1-Ratifikation entscheidet die Option, nicht den Standard: der Kandidat bleibt Kandidat, P1a bleibt offen und P2 bleibt gesperrt.

Nicht freigegeben sind weiterhin Implementierung und Release.


<!-- END DECISION-REGISTER -->

### Auflagen, die aus der Freigabe folgen

| Entscheidung | Auflage | Umgesetzt in |
|---|---|---|
| D2 | Selektor, Wortgroesse, Encoding, Groessen, Adressableitung, atomare Rueckabwicklung | REQ-SCE-039 bis REQ-SCE-041, REQ-SCE-043 |
| D4 | fuenf getrennte Gasfelder, Abrechnung unabhaengig vom Rollback, kein kostenloser Verbrauch | REQ-SCE-045 bis REQ-SCE-047 |
| D6 | vier Nachweise zur Identitaet, keine Vertrauensluecke Wallet/Kette/VM | REQ-SCE-048, Gate SCE-G5 |
| D5 | deterministische Reihenfolge, identische VM-Version, reproduzierbare Receipts | REQ-SCE-042, REQ-SCE-049, REQ-SCE-050 |
| alle | VM-Versionierung, State-Rollback, Replay-Sicherheit | REQ-SCE-042 bis REQ-SCE-044 |

### Was die Freigabe nicht bedeutet

Eine getroffene Entscheidung ist noch keine Spezifikation und keine Umsetzung.
Der Kandidat traegt die Auflagen jetzt als Anforderungen; bis zur Ratifizierung
bleibt er Kandidat, und bis zum Nachweis bleibt jedes Gate offen. Der Zustand
"IMPLEMENTED" ist fuer keine der sieben Entscheidungen erreicht.

**Reihenfolge (technische Abhaengigkeit, nicht Beschluss):**
D1 (Format) → D7 (Frontend) → D2 (Aufruf) → D3 (Speicher) → D6 (Identitaet) → D4 (Gas) → D5 (Block).

---

## D1 — Welches Bytecodeformat ist das normative?

**DECIDED — freigegeben (12.11.2026).** Eigene VM-Huelle, ATC-BC-001 unveraendert; Magic, eigene Versionskennung, festgelegte Byte-Reihenfolge, eindeutige Laengenangabe, kanonische Serialisierung, keine Aenderung am Transaktionsformat.

**Nachtrag 12.11.2026 (Befund 1):** Die uebernommene Lifecycle-Zeile SCX-003
sprach noch von "Artefakt mit Magie ATCB + Version" und widersprach damit dieser
Entscheidung. SCX-003 beschreibt jetzt den Instruktionsstrom nach ATC-BC-001 plus
die getrennte Huelle SCE-ARTIFACT; `ATCB` bleibt ausschliesslich der Strom. Das
Magic der Huelle ist `ATCE` (bestätigt am 12.11.2026, Fassung `0x02`), `ATCB` bleibt der Strom.

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
| A | Instruktionsstrom bleibt `ATC-BC-001`. Der VM-Container wird zu einer eigenstaendigen Huelle mit eigenem Magic (`ATCE`), die Generation, Stromlaenge, Layout-Version und SHA-256 traegt. Opcodes der VM werden auf die BC-001-Nummerierung umgestellt. | Umbau des VM-Loaders und des Backends, alle Devnet-Artefakte neu erzeugen | klein: rund 40 betroffene Tests, keine Aussenzusage bricht, weil niemand Artefakte archiviert |
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

**DECIDED — freigegeben (12.11.2026).** ATCLang ist das einzige kanonische Frontend; `.ops` nur fuer interne Tests und Testfixtures, niemals als produktiver Bytecode-Eingang.

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

**DECIDED — bedingt freigegeben (12.11.2026).** Auflagen: Selektor-Hashverfahren und Breite, Wortgroesse und Calldata-Encoding, maximale Rueckgabepuffergroesse, deterministische Vertragsadressableitung, atomare Rueckabwicklung fehlgeschlagener Transaktionen. Vertrag-zu-Vertrag-Aufrufe bleiben bis BC-002 gesperrt. Spezifiziert in REQ-SCE-039 bis REQ-SCE-041 sowie REQ-SCE-043.

**Nachtrag 12.11.2026 (Spezifikationsluecke A):** Die Adressableitung nannte 16
Byte Domain und 76 Byte Gesamtlaenge. `ATC-CONTRACT-V1` ist 15 Byte lang, die
Eingabe ist 75 Byte. Die Laengen sind jetzt bytegenau festgelegt, mit
Pflichtpruefung `len(domain) == 15` und `len(input) == 75`; ein zusaetzliches Byte
waere eine eigene Formatentscheidung und ist nicht enthalten. Ohne diese Korrektur
haetten zwei Implementierungen verschiedene Vertragsadressen berechnet.

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

**DECIDED — freigegeben (12.11.2026).** Lokale im Execution Frame, Persistenz ausschliesslich in S1-Slots, `layout_hash` je Vertragsversion pruefbar, Maps erst nach normierter Schluesselcodierung, Hashing und Storage-Proofs.

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

**DECIDED — bedingt freigegeben (12.11.2026).** Nachweise noetig: DID und Kontoidentitaet nutzen dieselbe Ableitung; keine Identitaetseinschleusung ueber Transaktionsfelder; Signaturpruefung authentifiziert; interne Aufrufe bilden den unmittelbaren Aufrufer ab; keine Vertrauensluecke Wallet/Kette/VM. Siehe REQ-SCE-048 und Gate SCE-G5.

**Nachtrag 12.11.2026 (Befund 3):** Die vier Nachweise reichen nicht, solange die
Identitaeten nicht getrennt sind. `origin` ist der Urheber der Transaktion und ueber
den Aufrufstapel unveraenderlich; `caller` ist der unmittelbar Aufrufende des
aktuellen Frames und bei Vertrag-zu-Vertrag die Vertragsadresse, nicht die Wallet;
die Vertragsadresse ist ein technischer Aufrufer mit eigenem Typkennzeichen. Eine
Implementierung, die `caller` dauerhaft auf `origin` stehen laesst, hebelt jede
Berechtigungspruefung aus. REQ-SCE-051 bis REQ-SCE-053; die `caller`-Semantik wird
mit ATC-BC-002 abgestimmt, bevor Vertrag-zu-Vertrag freigegeben wird.

**Nachtrag 12.11.2026 (Spezifikationsluecke B, ersetzt die Formulierung
"20-Byte-Adresse, links mit Nullen" in Option A):** Das Typkennzeichen war
gefordert, aber nicht kodiert. Es gibt jetzt genau ein kanonisches 32-Byte-Layout: Bytes 0..3
Kennzeichen (`0x00000001` Wallet, `0x00000002` Vertrag), Bytes 4..11 reserviert und
zwingend null, Bytes 12..31 die 20 Adressbytes. Ungueltig sind Kennzeichen null,
unbekannte Kennzeichen, nicht-null reservierte Bytes, Nulladresse und Nullwort;
unbekannte Kennzeichen werden niemals als Wallet gelesen. Das Kennzeichen setzt der
Frame, nicht der Aufrufer. Identitaeten sind Bytes, keine Zahlen; Arithmetik auf
ihnen ist unzulaessig. Eine Implementierung fuer Wallet, VM und Knoten (REQ-SCE-057
bis 059, T-31 bis T-39).

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
| A | `caller` wird ein 32-Byte-Identitaetswort nach dem kanonischen Layout (Kennzeichen Bytes 0..3, reserviert 4..11 zwingend null, Adresse 12..31; REQ-SCE-057), `u64` entfaellt | nutzt die bestehende Ableitung, keine neue Wahrheit |
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

**DECIDED — bedingt freigegeben (12.11.2026).** Fuenf getrennte Felder `gas_limit`, `gas_used`, `gas_price`, `fee_charged`, `gas_refund`; Abrechnung unabhaengig vom Rollback; ein fehlgeschlagener Vertrag verbraucht Rechenzeit nicht kostenlos; keine Gebuehrenumgehung ueber Backend oder Loader. Zahlen bleiben offen (P1b). Siehe REQ-SCE-045 bis REQ-SCE-047.

**Nachtrag 12.11.2026 (Befunde 4 und C, ersetzt die frueher hier stehende
Dreiteilung):** "Fehlgeschlagen" ist nicht gleich "Budget weg". Es gelten vier
Klassen:

| Klasse | Verbrauch | Erstattung |
|---|---|---|
| Gaserschoepfung | volles `gas_limit` | keine |
| Revert oder logischer Vertragsfehler | nur `gas_used` | ungenutztes Budget nach Erstattungsregel |
| Ablehnung vor der Ausfuehrung | kein Gas, keine Zustandsaenderung | keine Buchung |
| Interner Fehler der Ausfuehrungsinfrastruktur | kein Nutzeranteil | keine Belastung |

Die vierte Klasse ist weder Revert noch Erschoepfung; wuerde sie als eines von
beiden abgerechnet, erzeugten identische Transaktionen je nach Zustand der
Infrastruktur unterschiedliche Gebuehren. Sie erzeugt keinen Beleg und keinen
Konsensstatus, sondern einen lokalen Knotenfehler (ATC-NODE-INFRA-001); Einzelheiten
in REQ-SCE-060. `gas_refund` zaehlt Gaseinheiten, der Geldbetrag traegt einen eigenen
Namen (REQ-SCE-055). Die Erstattungsformeln (`gas_refund = gas_limit - gas_used`,
`refund_amount = gas_refund * gas_price`, `fee_charged = gas_used * gas_price`)
stehen als Vorschlag im Kandidaten und sind nicht beschlossen. Pflichtfaelle:
T-28, T-29, T-40.

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
| A | Ein Gastvertrag: eine versionierte Kostentabelle, Budget gleich `tx.gas_limit`, Verbrauch im Receipt, Gebuehr gleich Verbrauch mal `gas_price`, Abbuchung gegen ein Konto, Fehlschlag verbraucht die geleistete Rechenzeit (Erschoepfung: volles Budget) | erfuellt die Vorgaben der Vorlage |
| B | Zwei Budgets (heute) | keine Abrechnung, kein Schutz |
| C | Kostenloser Betrieb ohne Gas, nur Ops-Limit | einfacher, aber widerspricht ATC-14 und SC-016 |

**Empfehlung: Option A.** Eine Tabelle, eine Kennung (`GAS_TABLE_V2`), eine
Reihenfolge: Kosten vor der Wirkung belasten, ein Fehlschlag belastet die
geleistete Rechenzeit (bei Erschoepfung das ganze Budget), Erfolg meldet den
Verbrauch. Die frueher hier stehende Formulierung "Fehlschlag verbraucht alles" ist
durch die Klassifizierung in vier Fehlerklassen ueberholt. Die Zahlen selbst sind eine Owner-Entscheidung; die
ATC-14-Formel fuer Deployment und `GAS_TABLE_V1` fuer Befehle sind die beiden
vorhandenen Quellen, die zu **einer** Tabelle zusammengefuehrt werden muessen.

**Folgen bei A.** `tx.gas_limit` wird vom Mempool geprueft und begrenzt die
Ausfuehrung, `receipt` traegt `gas_used`, `System::execute` bucht gegen ein Konto,
und die Kette braucht eine Kontofuehrung, die es heute nicht gibt.

**Gate:** SCE-G6 (Gasvertrag) mit Pflichtfaellen: Budget 0, Budget exakt
ausreichend, Budget ein Gas zu klein, Erschoepfung verbraucht das Budget, Revert belastet nur den Verbrauch, Verbrauch
identisch bei zwei Laeufen.

---

## D5 — Ist Ausfuehrung Teil der Blockgueltigkeit?

**DECIDED — freigegeben (12.11.2026).** Ausfuehrung wird Bestandteil der Block-State-Transition, mit deterministischer Reihenfolge, identischer VM-Version, identischer Gasberechnung, atomaren Zustandsaenderungen, reproduzierbaren Receipts, verbindlicher State-Root-Berechnung und unabhaengigem Nachrechnen. Ein Hash ohne Ausfuehrungsnachweis reicht nicht.

**Nachtrag 12.11.2026 (Befund 2):** "Der Validator rechnet nach" ist kein
Widerspruch zur Verantwortungsmatrix, aber die Matrix war zu grob. Der Knoten
bildet Auftrag, Kontext und Reihenfolge, ruft die normierte VM auf und vergleicht
Ergebnis, Verbrauch und Zustandswurzel. Die Interpretation der Vertragslogik liegt
ausschliesslich in der VM. Der Validator ist als eigene Zeile gefuehrt; REQ-SCE-056
verbietet eine zweite Auslegung im Knoten, im Backend oder im Loader.

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

## Stand nach der Freigabe (12.11.2026)

| Frage | Antwort |
|---|---|
| Sind D1 bis D7 entschieden? | Ja, alle sieben: DECIDED. Vier freigegeben, drei bedingt. |
| Sind sie spezifiziert? | Die Auflagen stehen als Anforderungen im Kandidaten (REQ-SCE-039 bis REQ-SCE-050). Ratifiziert ist nichts. |
| Sind sie umgesetzt? | Nein. Kein Gate ist erfuellt, CI_VERIFIED und E2E_VERIFIED sind offen. |
| Was fehlt bis P2? | Die drei Zusatzanforderungen als Anforderungen (erledigt), die Konsistenz des Registers in beiden Dokumenten (erledigt, `tools/decision_register.py --check`), und die Zahlen aus P1b. |
| Was ist ausdruecklich offen? | Numerische Gaspreise, Transaktionsgaslimit, Blockgaslimit, Implementierungs- und CI-Gates. |

## Grundregel fuer alle sieben Entscheidungen

**Eine Wahrheit je Funktionalitaet (SCE-P1).** Jede Komponente besitzt ihre eigene
Verantwortung, aber keine Komponente darf eine zweite Wahrheit ueber dieselbe
Funktionalitaet etablieren: eine Sprache, eine VM, ein Instruktionsstrom, eine
Huelle, eine Gastabelle je Generation, eine Zustandswurzel. Die Arbeitsteilung
bleibt: ATCLang erzeugt, ATC-VM verifiziert und fuehrt aus, A-TownChain verarbeitet
und bestaetigt, Validatoren rechnen deterministisch nach, der Konsens bestaetigt
ausschliesslich einen nachvollziehbaren Zustand.

**Pruefstand der offenen Auflagen:** `docs/audits/EXEC-OPEN-POINTS-2026-11-12.md`
(fuer D1, D2, D4 und D6 gegen ATC-BC-002 und die vorhandenen Vektoren, ohne
Entscheidung), die Abhaengigkeitsinventur
`docs/audits/EXEC-DEPENDENCY-INVENTORY-2026-11-12.md` (betroffene Codestellen) und
die Vorlage `docs/decisions/EXEC-DECISION-TEMPLATE-2026-11-12.md` (drei Blocker,
vier Entscheidungen).

**Drei Ebenen, die nicht vermischt werden duerfen:** eine Owner-Entscheidung ist
intern verbindlich, auch wenn der zugehoerige Standard nur Kandidat ist und nicht
ratifiziert wurde; ein Wert im Kandidaten ist ein Vorschlag. Die Ratsche gilt
auch fuer ATC-BC-002: dessen Aufruftiefe 64 ist eine Owner-Entscheidung vom
09.11.2026, der Standard selbst ist Kandidat.

## Nachtrag 12.11.2026: Magic und Integritaetsdigest

Der Owner hat zwei Widersprueche in der Spezifikation gefunden und entschieden.

**Magic.** Mein Vorschlag in REQ-SCE-067, `ATCB` als Byte-Konstante des Containers
beizubehalten und die Kollision mit `ATC-BC-001` als Namensgleichheit zu behandeln,
war falsch. Er widersprach derselben Spezifikation: Abschnitt 5 legt fest, dass
`ATCB` ausschliesslich den eingefrorenen Strom bezeichnet, dass die Huelle ein
eigenes Magic erhaelt und dass ein Container mit gleichem Magic und anderer Semantik
untersagt ist. Genau dieser Doppelkopf ist Audit-Befund IV-03. Festlegung: Huelle
`ATCE`, Strom `ATCB`, Containerfassung `0x02`.

**Integritaetsdigest.** Der Tail war in §18.3 als SHA-256 ohne Domaene beschrieben,
REQ-SCE-062 definiert `artifact_hash` dagegen mit Domaenenpraefix. Zwei Definitionen
derselben Sache. Festlegung: der Tail **ist** `artifact_hash`. Kein Zirkel, weil der
Tail nicht in die Eingabe eingeht.

**Folge fuer die Vektoren.** Neu berechnet: GV-1, GV-3, GV-5, GV-7 und NV-2 aus dem
Magic; zusaetzlich GV-9, weil es GV-1 als Eingabe enthaelt. Unveraendert: GV-2, GV-4,
GV-6, GV-8, NV-1. Nachgerechnet mit `tools/check_stage1_vectors.py`.
