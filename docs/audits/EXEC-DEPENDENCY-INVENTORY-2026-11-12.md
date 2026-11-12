# Abhaengigkeitsinventur und SSOT-Abgleich — Smart-Contract-Ausfuehrung

**Status:** Inventur und Abgleich. Trifft **keine** Entscheidung.
**Datum:** 2026-11-12 · **Basis:** `a-townchain` main `995948b5`
**Bezug:** `docs/audits/EXEC-OPEN-POINTS-2026-11-12.md` (Befunde R-901 bis R-907)

Diese Inventur beantwortet nicht, welcher Wert richtig ist, sondern **wo** die
sieben Befunde im Code und in den Dokumenten haengen und **was sich aendert**,
wenn einer davon aufgeloest wird. Die Risikobezeichnungen R-901 bis R-907 sind
die des Reviews vom 12.11.2026.

## 1. Geprueft Quellen

| Datei | Blob | Rolle |
|---|---|---|
| `components/vm/src/artifact.rs` | `9a1c3bf913e2` | Container, Magic, Opcode-Kodierung |
| `components/vm/src/vm.rs` | `df3848e190ae` | Instruktionssatz, Grenzen, Gaskosten |
| `components/system/src/system.rs` | `8cf3594dc3dc` | Deployment, Freigabe, Ausfuehrung, Beleg |
| `components/system/src/state.rs` | `8cf3594dc3dc` | Zustandswurzel |
| `components/exec-admission/src/lib.rs` | `8cf3594dc3dc` | `code_hash` als Freigabeschluessel |
| `components/node/src/did.rs`, `components/wallet/src/keys.rs` | `8cf3594dc3dc` | Kontoadressen, keine Vertragsadressen |
| `docs/os-docs/atclang/vm/atcvm.py`, `.../compiler/compiler.py` | `ded4c7fa6ba9`, `a8f4210c8569` | zweite VM, `CALL_EXT` |
| `standards/atc/candidates/ATC-BC-002-CANDIDATE.md` | `e2697e7ba565` | Containerregeln, Fehlercodes, Tiefe 64 |
| `standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md` | `8a02b11` | Anforderungen der Ausfuehrung |

Die Blobs von `state.rs`, `system.rs`, `exec-admission` und den Wallet-/Node-Dateien
stammen aus demselben Commit-Baum; einzelne Zeilennummern wurden am Arbeitsbaum
geprueft, der zum Zeitpunkt der Inventur mit `main` uebereinstimmte.

## 2. R-903 — Artefaktidentitaet: der gesamte `code_hash`-Pfad

Der Befund ist groesser als ein Name. `code_hash` ist heute **der Container-Hash**
und wird an sieben Stellen als Identitaet verwendet.

| Stelle | Was dort passiert | Folge einer Umstellung auf `stream_hash` |
|---|---|---|
| `system.rs:223` | `let code_hash = artifact.hash();` beim Deployment | anderer Wert, also neue Vertragsidentitaet |
| `system.rs:228` | Selbstpruefung: neu geparstes Artefakt muss denselben Hash ergeben | bleibt gueltig, prueft aber kuenftig den falschen Gegenstand |
| `system.rs:97`, `233`, `248` | `Deployment.code_hash` gespeichert und weitergegeben | jedes gespeicherte Deployment traegt den alten Wert |
| `system.rs:268`, `322` | `state.state_root(&code_hash)` vor und nach der Ausfuehrung | **alle Zustandswurzeln aendern sich** |
| `state.rs:12`, `43-52` | Zustandswurzel = SHA-256 ueber `ATC-STATE-V1\0`, Code-Hash, Wortzahl, Woerter | Domaine bleibt, Eingabe aendert sich |
| `system.rs:276`, `282-283` | `observed_hash = artifact.hash()` geht als `observed_hash` in die Freigabe | Freigabe vergleicht kuenftig Stream gegen Container |
| `exec-admission/src/lib.rs:434` | `observed_hash != policy.code_hash` = Ablehnung | jede registrierte Policy passt nicht mehr |
| `exec-admission/src/lib.rs:188-204` | Registry ist ausdruecklich "SSOT fuer code_hash zu Policy" | alle Schluessel werden ungueltig |
| `exec-admission/src/lib.rs:368-373` | Mindestversion und Widerruf haengen an `code_hash` | Widerrufs- und Versionssperren verlieren ihr Ziel |
| `system.rs:126`, `335` | `Receipt.code_hash` | Belege tragen einen anderen Wert |
| `system.rs:327`, `main.rs:111` | Anzeige des Code-Hash | nur Ausgabe |

**Schlussfolgerung der Inventur.** Ein Umbenennen korrigiert nichts. Die drei
Wirkungen sind: (a) neue Zustandswurzeln, (b) ungueltige Freigabe- und
Widerrufsschluessel, (c) neue Vertragsidentitaet. Solange die Vertragsadresse
nach REQ-SCE-041 aus dem Strom-Hash gebildet werden soll, braucht es beide Werte
gleichzeitig, nicht einen statt des anderen.

Zusaetzlich: eine Vertragsadressableitung existiert im Code **nicht**. Es gibt nur
`address_from_pubkey` (`components/wallet/src/keys.rs:10`) fuer Konten und die DID
daraus (`components/node/src/did.rs:121-123`). REQ-SCE-041 ist damit vollstaendig
unimplementiert.

## 3. R-903 — Magic

| Stelle | Inhalt | Wirkung einer Umbenennung |
|---|---|---|
| `artifact.rs:22` | `pub const MAGIC = b"ATCB"` | Schreib- und Lesepfad |
| `artifact.rs:277` | Kodierung schreibt das Magic | jedes erzeugte Artefakt |
| `artifact.rs:303-306` | Laengen- und Magic-Pruefung beim Lesen | Fehlerpfad |
| `artifact.rs:452` | Test `zu_kurz_und_falsche_magic_abgelehnt` | Test muss mit |
| `ATC-BC-002:243-248` | Tabelle: `magic ATCB`, Hash ueber alle vorhergehenden Bytes | Formatbeschreibung |
| SCE-D1, SCX-003 | `ATCB` = Strom, Huelle eigenes Magic | Zieldefinition |

Weil das Magic in den Hash eingeht, aendert jede Umbenennung **jeden**
Artefakt-Hash. Heute ist das billig: es ist kein Artefakt-Hash als Goldwert
gespeichert, und der einzige vorhandene Vektor (`atcb-vm-vectors.json`) verlangt
einen Hash, nennt aber keinen Wert. Sobald ein Vektor oder eine Freigabe einen
Hash festhaelt, wird dieselbe Aenderung teuer.

## 4. R-902 und R-907 — Aufruf und Aufrufer

| Stelle | Stand | Bedeutung |
|---|---|---|
| `vm.rs:14-35` | 19 Befehle, **kein** Aufruf | Vertrag-zu-Vertrag nicht ausfuehrbar |
| `vm.rs:391` | `Op::Caller` pusht `self.caller` | Aufrufer ist ein Wert, kein Frame |
| `system.rs:266` | `caller: u64` als Parameter von `execute` | Aufrufer ist eine Zahl |
| `system.rs:301` | `Vm::from_artifact_with_context(..., caller, ...)` | Durchreichen ohne Identitaet |
| `main.rs:79`, `system.rs:369` | Aufruf mit `caller = 1` | kein Bezug zu einer Adresse |
| `artifact.rs:143`, `217` | `Caller` als Opcode `0x0C` | Kodierung vorhanden, Semantik nicht |
| `ATC-BC-002:250` | Befehlsliste `0x01` bis `0x13`, kein Call | auch der Kandidat kennt keinen Aufruf |
| `docs/os-docs/atclang/compiler/compiler.py:201` | `emit(OP.CALL_EXT, ...)` | nur die Python-VM kennt einen externen Aufruf |
| ATC-93:118, `233` | `CALL_EXT` = 40, `CALLER` = 104, 105 Opcodes | dritte Beschreibung, andere Nummerierung |

**Konsequenz.** `CALL` ist keine Erweiterung von `caller`: ein Aufruf braucht
Frame, Identitaet, Speichergrenzen, Gasbudget, Rueckgabepuffer und Rollback. Der
Stapel traegt `u64`, also ist auch der Identitaetstyp davon abhaengig. R-902 und
R-907 sind damit **eine** Baustelle mit zwei Eintraegen.

## 5. R-901 — Aufruftiefe

| Stelle | Aussage |
|---|---|
| `ATC-BC-002:50` | `E06` = Aufruftiefe ueberschritten, Grenze 64 |
| `ATC-BC-002:133` | maximale Aufruftiefe 64, Ueberschreitung ist Ruecklauf mit `E06` |
| `ATC-BC-002:146` | Aufruftiefe 64 und Stapeltiefe 4096 im Verifier |
| `ATC-BC-002:197` | `E06` fest reserviert, unveraenderlich |
| `ATC-BC-002:282` | Fehlercodetabelle, `E06` ausschliesslich Aufruftiefe |
| SCE-REQ-040 | Aufruftiefe 8 |
| Code | **kein** Tiefenzaehler, kein Aufruf, kein `E06` |

Belegt sind ausserdem: Stapel 4096 (`vm.rs:9`, BC-002:146) und Programm 65 536
(`vm.rs:7`, BC-002:146). Diese beiden stimmen ueberein und sind nicht strittig.

## 6. R-904 — Gas

| Stelle | Stand |
|---|---|
| `vm.rs:71-100` | `GAS_TABLE_V1`, 19 Befehlscodes, eigene Kennung |
| `vm.rs:102` | Kosten je Befehl, erschoepfend |
| `system.rs:37` | `DEFAULT_GAS = 50_000` |
| `system.rs:131` | Beleg traegt `gas_limit`, **kein** `gas_used` |
| `system.rs:125-135` | kein Feld fuer Gebuehr, Erstattung oder Rueckfluss |
| `wallet/src/tx.rs:31-32` | `gas_price`, `gas_limit` in der Transaktion, vom Knoten nie gelesen |
| `ATC-BC-002:127` | "bei Ruecklauf keine Erstattung" |
| `ATC-BC-002:200` | Rueckgabe des Teilbudgets an den Aufrufer = Rueckfluss, keine Erstattung |
| SCE-REQ-046, -047, -055 | fuenf Felder, Erstattungsformel als Vorschlag |

Drei Ebenen sind zu trennen: verbrauchtes Gas (bleibt verbraucht), nicht
verbrauchtes Unterbudget eines Frames (Gas-Rueckfluss an den Aufrufer, in
BC-002:200 entschieden), nicht verbrauchtes Transaktionsbudget (Geld oder
Verfall, in BC-002 nicht geregelt). Die SCE-Formel
`refund_amount = gas_refund * gas_price` ist eine Aussage der dritten Ebene und
damit eine Gebuehrenentscheidung, nicht eine Folge des Rueckflusses.

## 7. Compiler- und Backendkette

`system.rs:217-221`: `compile::compile(source)` nach
`backend::translate(&compiled)` nach `Artifact::new(GENERATION_V1, ops)`. Diese
Kette ist der einzige Pfad, der Artefakte erzeugt. Sie kennt **keinen** Selektor,
keine Calldata, keinen Rueckgabepuffer und keine Vertragsadresse. R-902, R-907 und
REQ-SCE-041 setzen alle in dieser Kette an.

## 8. Wo die Wirkung heute nicht ankommt

`System::execute` wird ausschliesslich aus dem Demo-Binary
(`components/system/src/main.rs:79`) und einem Test (`system.rs:369`) aufgerufen.
`components/node` fuehrt keine Abhaengigkeit auf `atc-system`, `atc-vm` oder
`code_hash`. Die Folgen aus Abschnitt 2 sind damit **latent**: sie treffen heute
keinen laufenden Konsens, keinen Bestand und keine Freigabe. Das ist der Grund,
warum die Umbenennung jetzt billig ist, und derselbe Grund, warum sie nach der
Verdrahtung teuer wird.

## 9. Matrix der Befunde

| ID | Befund | Betroffene Stellen (Abschnitt) | Status |
|---|---|---|---|
| R-901 | Aufruftiefe 8 gegen 64 | 5 | Konflikt zwischen Owner-Entscheidung und Vorschlag, offen |
| R-902 | keine `CALL`-Instruktion | 4 | fehlende Instruktion, offen |
| R-903 | Container-Magic und Hashumfang | 2, 3 | zwei Definitionen, offen |
| R-904 | Gas-Rueckfluss gegen Geldabrechnung | 6 | Begriffs- und Gebuehrenfrage, offen |
| R-905 | Zirkularitaet P1a und P2 | siehe Bericht Abschnitt 6.1 | Verfahrensfrage, offen |
| R-906 | zwei VM-Instruktionssaetze | 4 (`atcvm.py`, ATC-93) | Autoritaet unklar, offen |
| R-907 | Identitaetstyp auf dem Stapel | 4 | haengt an R-902, offen |

## 10. Was diese Inventur nicht feststellt

- Ob die Python-VM unter `docs/os-docs` autoritativ, Referenz oder Altlast ist.
- Ob ausserhalb dieses Repos gespeicherte Artefakt-Hashes oder Freigaben
  existieren, die an einem Umbenennen haengen.
- Welche der drei Instruktionsbeschreibungen die kuenftige Grundlage ist.

Alle drei sind Fragen an den Owner, keine Feststellungen dieses Berichts.
