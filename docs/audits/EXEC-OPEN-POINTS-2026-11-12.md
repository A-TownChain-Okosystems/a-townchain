# Offene Punkte P1a — Pruefung gegen ATC-BC-002 und die vorhandenen Vektoren

**Status:** Pruefbericht. Trifft **keine** Entscheidung und aendert keine Anforderung.
**Datum:** 2026-11-12 · **Basis:** `a-townchain` main `78596f79`
**Fortsetzung:** `docs/audits/EXEC-DEPENDENCY-INVENTORY-2026-11-12.md` (wo die
Befunde im Code haengen) und `docs/decisions/EXEC-DECISION-TEMPLATE-2026-11-12.md`
(Blocker und Entscheidungen getrennt)

Dieser Bericht beantwortet eine einzige Frage: Was sagen die heute gueltigen oder
im Entwurf befindlichen Regeln von ATC-BC-002 und die vorhandenen Testvektoren zu
den vier offenen Freigabepunkten, und was fehlt fuer eine Entscheidung. Er fuehrt
bewusst keine Alternative ein, die nicht schon in den Quellen steht.

## 1. Geprueft Quellen

| Quelle | Blob | Rolle |
|---|---|---|
| `standards/atc/candidates/ATC-BC-002-CANDIDATE.md` | `e2697e7ba565` | Kandidat, nicht normativ; Pruef-, Ruecklauf- und Containerregeln |
| `standards/atc/ATC-93-ATCLANG_VM_BYTECODE.md` | `697267a16c26` | aeltere VM-Dokumentation, 105 Opcodes, Status "IMPLEMENTED" |
| `components/vm/src/artifact.rs` | `9a1c3bf913e2` | Container in Code, Magic, Opcode-Zuordnung |
| `components/vm/src/vm.rs` | `df3848e190ae` | Instruktionssatz, Grenzen, Gastabelle |
| `components/system/src/system.rs` | `8cf3594dc3dc` | Aufruf der VM, `caller` Typ |
| `ai/atclang/specs/conformance/atcb-vm-vectors.json` | `9913903cb048` | vorhandene Konformanzvektoren |
| `docs/os-docs/atclang/vm/atcvm.py`, `.../compiler/compiler.py` | `ded4c7fa6ba9`, `a8f4210c8569` | zweite VM in Python, 105 Opcodes, `CALL_EXT` |
| `standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md` | `1ca0b80` | v1.5.0-draft, die vier offenen Punkte |
| `standards/atc/conformance/identity-value-vectors.json` | `ff9ac11` | T-31 bis T-39 |

## 2. Punkt 1 — Magic der Huelle

**Was gilt heute.** Zwei Stellen benutzen `ATCB` fuer den **Container**:
ATC-BC-002 Abschnitt 16 (Zeile 244: `magic | 4 | ATCB`) und der Code
(`artifact.rs:22`, `pub const MAGIC = b"ATCB"`). SCE-D1 sagt dagegen: `ATCB` ist
der Instruktionsstrom, die Huelle heisst `SCE-ARTIFACT` und braucht ein eigenes
Magic (Vorschlag `ATCE`).

**Zwei Befunde, die daraus folgen und die in der Entscheidung nicht sichtbar sind:**

1. **Der Hash wird zweimal definiert, mit verschiedenem Umfang.** ATC-BC-002
   Abschnitt 16, Zeile 248: `Hash = SHA-256 ueber alle vorhergehenden Bytes`, also
   ueber Magic, Containerformat, Generation, Anzahl und Strom. REQ-SCE-041
   definiert dagegen `code_hash_32` als `SHA-256 ueber den kanonischen
   Instruktionsstrom (nicht ueber die Huelle)` und verwendet genau diesen Wert in
   der Adressableitung. Beide Werte sind legitim, aber sie sind **nicht gleich**.
   Wird der Container-Hash versehentlich als `code_hash` verwendet, entstehen
   andere Vertragsadressen. Es fehlen zwei getrennte Namen (etwa `stream_hash`
   und `artifact_hash`) und ein Vektor, der beide Werte fuer denselben Vertrag
   festhaelt.
2. **Ein Umbenennen des Magic aendert jeden Artefakt-Hash**, weil der Hash das
   Magic einschliesst. Heute kostet das nur einen Testlauf: es ist **kein**
   Artefakt-Hash als Goldwert gespeichert. Der einzige vorhandene Vektor
   (`atcb-vm-vectors.json`) verlangt "deterministic-artifact-hash", nennt aber
   keinen Wert. Betroffen waeren die Konstante `artifact.rs:22`, die Lesepruefung
   (`artifact.rs:306`) und der Test `zu_kurz_und_falsche_magic_abgelehnt`
   (`artifact.rs:452`).

**Dritter Befund: der Container-Hash wird bereits als `code_hash` verwendet.**
`components/system/src/system.rs:223` setzt `let code_hash = artifact.hash();`,
also den Hash der ganzen Huelle. Derselbe Wert geht in die Zustandswurzel
(`state.rs:43-52`), in den Freigabeantrag (`system.rs:276`, verglichen in
`exec-admission/src/lib.rs:434`) und in die Registry, die sich ausdruecklich als
"SSOT fuer `code_hash` zu Policy" bezeichnet (`exec-admission/src/lib.rs:188-204`).
REQ-SCE-041 meint mit `code_hash` dagegen den Strom-Hash. Es sind elf Stellen;
die vollstaendige Liste steht in der Abhaengigkeitsinventur, Abschnitt 2. Eine
Umbenennung korrigiert daran nichts: es braucht zwei benannte Hashes mit
getrennten Verwendungen.

**Was fuer die Entscheidung fehlt.** Nicht die Namenswahl selbst, sondern die
Zuordnung, **wer** das Containerformat besitzt: ATC-BC-002 Abschnitt 16 oder
SCE-ARTIFACT. Solange beide es beschreiben, bleibt es bei zwei Wahrheiten, auch
wenn das Magic umbenannt wird.

## 3. Punkt 2 — die drei Grenzwerte

**Aufruftiefe: 8 gegen 64, und die 64 ist eine Owner-Entscheidung, aber keine
ratifizierte Norm.** ATC-BC-002 nennt die Aufruftiefe an vier Stellen als 64
(Zeilen 50, 133, 146, 282) und reserviert `E06` ausdruecklich und unveraenderlich
dafuer (Zeile 197). REQ-SCE-040 schlaegt Tiefe **8** vor.

Die Ebenen sind dabei streng zu trennen, sonst wird aus einem internen Beschluss
eine scheinbare Norm:

| Ebene | Status |
|---|---|
| Owner-Entscheidung 64 (09.11.2026) | intern getroffen und verbindlich |
| ATC-BC-002 als Standard | Kandidat, **nicht ratifiziert** |
| SCE-Wert 8 | Vorschlag |
| Konfliktaufloesung | offen |

Daraus folgt: die 64 darf nicht als bereits ratifizierte Norm behandelt werden,
und eine Abweichung von ihr ist nicht stillschweigend moeglich, sondern nur als
ausdrueckliche Aenderung der Owner-Entscheidung. Zusaetzlich kollidiert die
SCE-Formulierung "Aufruftiefe 9 wird abgelehnt, Statuscode eindeutig" mit der
Rolle von `E06`.

**Calldata und Rueckgabepuffer: es gibt nichts, wogegen man sie pruefen koennte.**
Der Rust-Instruktionssatz hat 19 Befehle ohne einen einzigen Aufruf
(`vm.rs:14-35`), ATC-BC-002 Abschnitt 16 ebenso (`0x01` bis `0x13`, kein Call).
Calldata, Rueckgabepuffer und Aufruftiefe existieren in dieser VM nicht. Folgen:

- Die Kantentests T-21 bis T-24 sind heute **nicht ausfuehrbar**. Sie brauchen
  einen Aufruf, den es nicht gibt.
- Die Werte 8 192 und 4 096 Byte haben in keiner Quelle eine Entsprechung; sie
  stehen nur im SCE-Kandidaten.
- Stapel 4 096 und Programm 65 536 sind dagegen belegt (`vm.rs:7,9`,
  ATC-BC-002 Zeile 146) und stimmen ueberein.

## 4. Punkt 3 — Erstattung

ATC-BC-002 sagt an drei Stellen etwas zum Verbrauch, aber nichts ueber Geld:

| Stelle | Aussage |
|---|---|
| Zeile 127 | "Gas wird vor der Wirkung belastet; bei Ruecklauf keine Erstattung." |
| Zeile 133 | Unzureichendes Gas: Ruecklauf mit `E03`, Gas vollstaendig verbraucht |
| Zeile 200 | "Rueckgabe nicht verbrauchten Teilbudgets ist keine Erstattung, sondern Rueckfluss an den Aufrufer" |
| Zeile 197 | "einmal belastet bleibt belastet" |

**Befund.** Verbrauchtes Gas bleibt verbraucht: darin stimmen ATC-BC-002 und die
SCE-Klassen ueberein, `E03` entspricht der Erschoepfung mit vollem Budget. Der
offene Teil ist ein **Begriffsproblem**: ATC-BC-002 unterscheidet ausdruecklich
"Erstattung" (Geld, wird verneint) von "Rueckfluss" (Gas, an den Aufrufer). Das
SCE-Feld heisst `gas_refund` und wird im Text "Erstattung" genannt. Wird der
Rueckfluss des ungenutzten Transaktionsbudgets an den Absender als "Erstattung"
bezeichnet, widerspricht das dem Wortlaut von Zeile 127, ohne dass sich an der
Zahl etwas aendert.

Drei Ebenen sind auseinanderzuhalten, und nur die dritte ist Geld:

| Ebene | Regel |
|---|---|
| verbrauchtes Gas | bleibt verbraucht (BC-002 Zeile 127, 197) |
| nicht verbrauchtes Unterbudget eines Frames | fliesst in Gas-Einheiten an den Aufrufer zurueck (BC-002 Zeile 200) |
| nicht verbrauchtes Transaktionsbudget | monetar gegenueber dem Absender: **nicht geregelt** |

Die SCE-Formel `refund_amount = gas_refund * gas_price` macht aus dem
Gas-Rueckfluss eine Geldabrechnung und entscheidet damit die dritte Ebene, nicht
die zweite. Ein Geldmodell (Konto, Abbuchung, Betrag) existiert in ATC-BC-002
**nicht**; es steht ausschliesslich in D4 und im SCE-Kandidaten. Die
Erstattungsregel ist damit keine Frage an ATC-BC-002, sondern eine Frage des noch
fehlenden Kontomodells.

## 5. Punkt 4 — `caller` bei Vertrag-zu-Vertrag

**Befund: der Aufruf selbst fehlt, nicht nur seine Semantik.**

| Ebene | Aufruf zwischen Vertraegen | `caller` |
|---|---|---|
| ATC-BC-002 Abschnitt 16 | kein Befehl | `0x0C Caller`, ohne Typangabe |
| Rust-VM | kein Befehl (19 Ops) | `Op::Caller` pusht `u64` (`vm.rs:391`, `system.rs:266`) |
| Python-VM in `docs/os-docs` | `CALL_EXT` vorhanden (`compiler.py:201`) | `CALLER` als Opcode 104 (ATC-93 Zeile 233) |

Der Unterschied ist grundsaetzlich: `Caller` liefert eine vorhandene
Aufruferinformation, `CALL` wuerde einen neuen Ausfuehrungsframe erzeugen. Ein
Frame braucht Identitaet, Speichergrenzen, Gasbudget, Rueckgabepuffer und
Rollback-Semantik. Der Abstand zum Zielbild ist damit groesser als ein Typwechsel:

| Bestandteil | heute | SCE-Ziel |
|---|---|---|
| `caller` | `u64` | 32-Byte-`IdentityValue` |
| Stapelwort | `u64` | 32 Byte |
| Aufrufbefehl | fehlt | nicht spezifiziert |
| Frame-Verwaltung | keine | erforderlich |
| Vertrag-zu-Vertrag | nicht implementiert | haengt an einer neuen Instruktion |

Damit haengt REQ-SCE-052 nicht an der Ratifizierung von ATC-BC-002 allein: es
fehlt eine Instruktion. Und `CALL_EXT` aus der Python-VM darf nicht uebernommen
werden, solange deren Autoritaet nicht geklaert ist (R-906); eine Uebernahme
waere eine dritte Wahrheit. ATC-BC-002 v2 kennt sie nicht, also waere sie eine
Erweiterung von v2 oder eine dritte Generation. Zusaetzlich:

- **Werttyp.** Der Stapel traegt `u64`. Ein 32-Byte-`IdentityValue` passt nicht
  hinein. `Caller` kann heute nur eine Zahl pushen. Eine 32-Byte-Identitaet
  braucht entweder einen Worttyp auf dem Stapel oder einen anderen Weg in den
  Speicher. Beides beruehrt den eingefrorenen Instruktionssatz.
- **Drei Instruktionssaetze.** ATC-93 (105 Opcodes), ATC-BC-002 (19 Opcodes) und
  der Rust-Code (19 Varianten) beschreiben nicht denselben Satz. Nur ATC-93 und
  die Python-VM kennen einen externen Aufruf.

## 6. Zwei strukturelle Befunde

**6.1 Zirkularitaet zwischen P1a und P2.** Die Grenzwerte sollen erst nach den
Kantentests T-21 bis T-24 bestaetigt werden. Diese Tests brauchen einen
Aufrufmechanismus. Der Aufrufmechanismus ist Implementierungsarbeit und liegt in
P2. P2 ist bis zum Abschluss von P1a gesperrt. Ohne Aufloesung ist P1a nicht
abschliessbar. Drei Wege sind denkbar, keiner ist entschieden:
(a) Grenzwerte bleiben vorlaeufig, mit einem Pruefpunkt nach dem Aufrufmechanismus,
(b) P2 wird als eng begrenzte Ausnahme nur fuer den Aufrufmechanismus geoeffnet,
(c) Grenzwerte werden ohne Tests bestaetigt, was der eigenen Regel widerspricht.

**6.2 Zwei VMs.** Unter `docs/os-docs/atclang/vm/atcvm.py` liegt eine zweite VM in
Python mit 105 Opcodes, und ATC-93 fuehrt sie als "IMPLEMENTED". Ob sie
Dokumentations- oder Produktionscharakter hat, ist an keiner Stelle gekennzeichnet.
Fuer die vier offenen Punkte ist das relevant, weil ein Teil der Antworten nur
dort existiert (externer Aufruf, `CALLER` als Adresse).

## 7. Was je Punkt fuer eine Entscheidung fehlt

| Punkt | Entscheidbar, sobald | Fehlende Evidenz |
|---|---|---|
| Magic | Zuordnung des Containerformats geklaert | Entscheidung, welche Datei das Format besitzt; Vektor mit `stream_hash` und `artifact_hash` |
| Grenzwerte | Aufruftiefe 8 gegen 64 aufgeloest; Aufruf spezifiziert | Kantentests T-21 bis T-24, Messung Speicher je Frame |
| Erstattung | Kontomodell benannt | Festlegung, ob ungenutztes Budget zurueckfliesst oder verfaellt; T-28 mit Zahlen |
| `caller` | Call-Instruktion spezifiziert | Generation der Instruktion, Worttyp auf dem Stapel, Testvektoren |

## 8. Keine Entscheidung getroffen

Dieser Bericht stellt fest, was die Quellen sagen und was fehlt. Er bestaetigt
kein Magic, keinen Grenzwert, keine Erstattungsregel und keine Aufrufsemantik. Die
Vorschlaege im SCE-Kandidaten bleiben Vorschlaege; die Regeln in ATC-BC-002 bleiben
Kandidatenrecht; P1a bleibt offen, P2 bleibt gesperrt.
