# ATCLang-Frontend — Reichweite gegen den echten Korpus

**Datum:** 08.11.2026
**Basis:** `a-townchain` main `583d5a5`
**Gegenstand:** vier Syntaxformen, die der Korpus schreibt und die eigene Spezifikation
teilweise verlangt, das Frontend aber nicht annahm.
**Entscheidungsregel:** Nur Formen umgesetzt, die in `specs/language/SPEC.md` stehen oder
die vorhandene Semantik unverändert weiterbenutzen. Nichts erfunden.

---

## 1. Was umgesetzt wurde

| Form | Rechtsgrundlage | Semantik |
|------|-----------------|----------|
| `#` bis Zeilenende als Kommentar | SPEC §2.6 (die Referenz kannte das immer) | Lexer-Trivia, wie `//` |
| `module NAME` und `module NAME { … }` | SPEC §3.1 `module_decl` | Programmkopf; der Block ist **reine Gruppierung**, seine Deklarationen bleiben auf Programmebene. Kein zweiter Namensraum, keine zweite Auflösung. |
| `import "std/crypto.atc" as Crypto`, `import "std/chain.atc"` | Korpusform, Schlüsselwort `import` steht im Token-Satz der Spezifikation | Dateiimport mit Alias; ohne Alias gilt der Dateistamm als Name (`crypto`) |
| `import ATC::OS::Kernel` | wie `use ATC::OS::Kernel` | wird als `UseDeclaration` abgelegt, identische Semantik |
| `use ATC::Net::{ TCPSocket, UDPSocket }` | Gruppenform von `use` | `UseGroupDeclaration`; jeder Name wird importiert und im Projektstadium geprüft (Wurzel-Namespace `ATC`, Symbol bekannt) |

Der `#`-Kommentar war kein Auslegungsstreit, sondern ein Fehler gegen die eigene
Spezifikation: der Lexer kannte nur `//`.

## 2. Was ausdrücklich unverändert bleibt

Die kanonische AST-JSON-Ausgabe **bestehender** Knoten ist byte-gleich. Die neuen Knoten
(`ModuleDeclaration`, `FileImportDeclaration`, `UseGroupDeclaration`) entstehen nur in
Dateien, die vorher überhaupt nicht parsten. Damit bleiben `abi_sha256` und die
Artefakt-Hashes der Vertragsquellen unberührt.

## 3. Messung am echten Korpus

Alle `.atc`-Dateien des Monorepos (ohne `.git`, ohne Verzeichnisse namens `.atc`, das sind
Governance-Metadaten), geprüft mit `atc check`:

| Stand | Dateien | angenommen | abgelehnt |
|-------|---------|-----------|-----------|
| main `583d5a5` | 1645 | 476 | 1169 |
| mit diesem Stand | 1645 | **880** | **765** |

**+404 Dateien** (24,6 % des Korpus) werden jetzt angenommen.

Vertragsquellen (`apps/atc-contracts`, `components/contracts`): 81 Dateien, **0 abgelehnt** —
vorher wie nachher. Die Änderung kostet keine Vertragsquelle.

## 4. Tests

`cargo test` im atclang-Workspace: **alle Suiten grün**, `atc-core` **132 Tests**
(vorher 115, also 17 neue). Neu abgedeckt:

- Lexer: `hash_kommentar_ist_trivia`, `hash_kommentar_schliesst_zeichen_aus`
  (`#` in einem String bleibt Stringinhalt), `modul_import_und_alias_sind_schluesselwoerter`
- Parser: `hash_kommentar_vor_contract`, `module_kopf_ohne_block`,
  `module_block_gruppiert_deklarationen`, `module_block_verlangt_schliessende_klammer`,
  `dateiimport_mit_alias`, `dateiimport_ohne_alias`, `import_pfadform_ist_semantisch_use`,
  `gruppenimport_mit_mehreren_namen`, `gruppenimport_mit_einem_namen_ohne_komma`,
  `gruppenimport_verlangt_schliessende_klammer`
- Projektstadium: `gruppenimport_bringt_jeden_namen_in_den_namensraum`,
  `gruppenimport_meldet_fremden_wurzel_namespace`, `dateiimport_alias_wird_importname`,
  `dateiimport_ohne_alias_nutzt_dateistamm`

Die Negativtests sind Absicht: eine geöffnete Klammer muss weiter ein Fehler sein, sonst
wird aus einer Erweiterung eine Aufweichung.

## 5. Was offen bleibt (Dialektfragen, keine Fehler)

> Nachgeführt in Abschnitt 7: `pub`, punktierte Modulnamen und Schlüsselwort-Felder sind
> inzwischen umgesetzt, die Tabelle unten zeigt den Stand vor dem zweiten Schritt.

Nach diesem Stand sind die häufigsten Ablehnungen keine Implementierungslücken mehr, sondern
Formen, die **keine Spezifikation definiert**. Sie brauchen eine Entscheidung, nicht Code:

| Form | Dateien (aktiv) | Was zu entscheiden wäre |
|------|-----------------|-------------------------|
| `pub fn …` | 106 | Sichtbarkeitsmarker ohne Wirkung — oder gar nicht einführen |
| `module A.B[AD-49] {` | 24 | punktierte Modulnamen und `[AD-xx]`-Tags |
| `state:` als Feldname | 23 | Schlüsselwörter als Namen (mechanisch nachziehbar in `expect_name`) |
| `struct X:` mit eingerücktem Rumpf | 20 | Einrückungssyntax (SPEC §3.2 kennt `INDENT`/`DEDENT`, der Lexer hat sie nicht) |

## 6. Bekannte Auffälligkeiten der Basis (nicht von dieser Änderung)

`cargo clippy -D warnings` und `cargo fmt --check` sind auf `583d5a5` **nicht** sauber:
zwei vorbestehende Clippy-Funde (`project.rs` `push_hint`, `semantics.rs` verschachteltes
`if let`) und Formatabweichungen in zahlreichen Dateien des Monorepos. Die hier geänderten
Zeilen sind formatiert; fremde Dateien wurden nicht angefasst, um den Diff lesbar zu halten.

Nicht Teil dieses Schritts: `ai/atclang/examples/{event_bus,kernel,shivamon}.atc` scheitern
weiter (Slice `data[:120]`, `state:` als Feldname, `Rarity::COMMON` im Ausdruck). Kein Test
parst `examples/`, deshalb ist das bisher niemandem aufgefallen.

---

## 7. Zweiter Schritt: `pub`, punktierte Module, Schlüsselwort-Felder

**Datum:** 08.11.2026, aufbauend auf `66cd13b`

| Form | Dateien | Entscheidung |
|------|---------|--------------|
| `pub fn` / `pub struct` / `pub const` | 106 | Angenommen und **verworfen**. ATCLang kennt keine Sichtbarkeitsebenen. Das AST-Feld `is_pub` ist im Compiler fest auf `false` verdrahtet und bleibt es: würde der Marker es kippen, verschöben sich die ABI-Hashes jeder Datei, die `pub` schreibt. |
| `module ATC.OS.Kernel` | 24 | Punkt ist Namensbestandteil, der Name wird als `ATC.OS.Kernel` geführt. |
| `module x[AD-49] { … }` | (in den 24) | Der Klammerzusatz ist ein Governance-Verweis, kein Sprachbestandteil: angenommen und verworfen. Ein fehlendes `]` bleibt ein Fehler. |
| `struct S { state: u64 }`, `x.state`, `S { state: 1 }` | 23 | Feldnamen dürfen Schlüsselwörter sein. An diesen Stellen kann nur ein Name stehen, das Schlüsselwort hat dort keine zweite Bedeutung. |

**Neuer Stand:** 1645 Dateien, **937 angenommen**, 708 abgelehnt (+57 gegenüber `66cd13b`,
+461 gegenüber `583d5a5`).

`atc-core` hat jetzt **141 Tests** (9 neue, u. a. `pub_ist_bedeutungslos_und_landet_nicht_in_der_ast`,
`modulname_darf_punktiert_sein`, `modul_tag_ohne_klammer_ist_ein_fehler`,
`schluesselwort_als_strukt_feld`).

**Invarianten unverändert nachgewiesen:** `apps/atc-contracts` 50/33/0/18,
`components/contracts` 46/29/0/18, Registry-Gate PASS 16/16 (prüft `source_sha256` **und**
`abi_sha256`), EXEC-CHAIN 20/20.

### Was jetzt noch abgelehnt wird (340 aktive Dateien, ohne `/archive/`)

Die häufigsten Ursachen sind erneut Formen ohne Spezifikation, keine Lücken:

| Anzahl | Form | Was zu entscheiden wäre |
|--------|------|-------------------------|
| 42 | `event Name { … }` u. ä. auf Top-Level | `event` ist im Lexer kein Schlüsselwort, sondern ein Bezeichner |
| 24 | führender Punkt / Fortsetzungszeilen | Fortsetzungssyntax über Zeilen |
| 20 | `struct X:` mit eingerücktem Rumpf | Einrückungssyntax (SPEC §3.2 nennt `INDENT`/`DEDENT`, der Lexer hat sie nicht und kennt keine Spaltenpositionen) |
| 19 | Feldtyp mit `[` (`data: u8[120]`) | Array-Längen-Notation |
| 14 | `enum X:` eingerückt | wie oben |
| 11 | `a..b` / doppelter Punkt | Bereichsoperator |
| 9 | Felder ohne Komma, nur durch Zeilenumbruch getrennt | Zeilen als Trenner |

Die eingerückten Rümpfe (34 Dateien zusammen) brauchen zuerst ein Positionsmodell im Lexer:
ohne Spaltennummer je Token kann der Parser eine Blockausdehnung nicht bestimmen. Das ist ein
eigener Schritt, keine Zeile im Parser.

---

## 8. Dritter Schritt: `event` auf Programmebene, Typlängen, Zeilentrenner

**Datum:** 08.11.2026, aufbauend auf `e9fce31`

| Form | Gewinn | Entscheidung |
|------|--------|--------------|
| `event Transfer { from: Address }` ausserhalb eines Contracts | 42 Dateien | `event` bleibt **Bezeichner**, kein Schlüsselwort: sonst wäre `let event = 1;` nicht mehr erlaubt. Zusätzlich die Blockform mit `{}` neben der bisher einzigen Klammerform `event X(a: T)`. |
| Feldtyp mit Länge `data: u8[120]` | 19 Dateien | Die Länge ist Teil des Typnamens (`u8[120]`), nicht des Feldnamens. Fehlendes `]` bleibt ein Fehler. |
| Struct-Felder nur durch Zeilenumbruch getrennt | 9 Dateien | Angenommen, wenn auf den Namen direkt `:` folgt. Ein Name mit Doppelpunkt kann an dieser Stelle nichts anderes sein als das nächste Feld. |

**Neuer Stand:** 1645 Dateien, **1095 angenommen**, 550 abgelehnt.
Gegenüber `e9fce31` (937) sind das **+158**, gegenüber `583d5a5` (476) **+619**.
Ohne `/archive/` bleiben **270** aktive Ablehnungen (vorher 340).

`atc-core` hat **146 Tests** (5 neue: `event_auf_programmebene`,
`event_bleibt_als_variablenname_erlaubt`, `feldtyp_mit_fester_laenge`,
`typlaenge_ohne_klammer_ist_ein_fehler`, `strukt_felder_ohne_komma`).
Clippy `-D warnings` weiter sauber.

**Invarianten unverändert:** `apps/atc-contracts` 50/33/0/18, `components/contracts`
46/29/0/18, Registry-Gate PASS 16/16, EXEC-CHAIN 20/20.

### Verbleibende aktive Ablehnungen (270)

| Anzahl | Form |
|--------|------|
| 34 | `struct X:` / `enum X:` mit eingerücktem Rumpf (braucht Spaltenpositionen im Lexer) |
| 24 | Fortsetzungszeilen / führender Punkt |
| 11 | `a..b` |
| 8 | `type X = …` (kein Schlüsselwort im Lexer) |
| Rest | Einzelfälle: Slices `data[:120]`, `Fn` als Typ, Ausdrücke wie `Rarity::COMMON`, `if` ohne Klammern |

---

## 9. Vierter Schritt: Spaltenmodell, eingerückte Rümpfe, `func`

**Datum:** 08.11.2026, aufbauend auf `140003d`

### Spaltenmodell im Lexer

Bis hierher trug ein Token keine Position; damit war eine eingerückte Blockausdehnung
nicht bestimmbar. Jetzt:

- `Pos { line, col }`, 1-basiert, und `tokenize_pos(src) -> (Vec<Token>, Vec<Pos>)`.
- `tokenize` bleibt unverändert und delegiert an `scan`; jeder der 53 Ablagepunkte merkt
  sich den Zeichenindex über einen `push`-Helfer, daraus wird die Zeile/Spalte über eine
  Tabelle gerechnet. `Token` selbst ist unberührt, also auch jeder Vergleich im Parser.
- Das `Eof`-Token bekommt die Endposition des Textes.

### Eingerückte Rümpfe (`struct X:` / `enum X:`)

`at_indented_line(base)` beantwortet genau die DEDENT-Frage aus SPEC §3.2: ein Feld
gehört zum Rumpf, solange seine Spalte größer ist als die Spalte der Deklaration.
Beim Rücksprung endet der Block, das nächste Token gehört wieder dem umgebenden
Kontext. Ohne Positionen antwortet die Funktion `false` und der Parser **rät nicht**,
sondern lehnt ab (eigener Test).

`enum X:` versteht zusätzlich `VARIANT = wert` und Varianten mit `(…)`.

### `func` als zweite Schreibweise für `fn`

32 Korpusdateien schreiben `func`. Es ist dieselbe Deklaration, kein eigener Knoten.

### Der Korpuszähler bleibt bei 1095 — und das ist der Befund

1645 Dateien, weiterhin **1095 angenommen**. Die eingerückten Rümpfe verschwinden aus
der Fehlerliste, aber die betroffenen Dateien werden dadurch nicht angenommen: sie
tragen mehrere unbelegte Formen **gleichzeitig**. Die Dateizahl ist damit kein
Fortschrittsmaß mehr, die Blockergruppen sind es:

| Anzahl | Erstrfehler der aktiven Ablehnungen |
|--------|--------------------------------------|
| 32 | `func` (in diesem Schritt behoben, greift erst, wenn der nächste Blocker fällt) |
| 24 | Fortsetzungszeilen / führender Punkt |
| 11 | `a..b` / doppelter Punkt |
| 5 | `type X = …` |
| Rest | Einzelfälle |

Aktive Ablehnungen ohne `/archive/`: 270 (unverändert). Der nächste sinnvolle Schritt ist
deshalb **eine frühere Fehlersammlung** statt eines Abbruchs beim ersten Fund: erst wenn
alle Blocker einer Datei bekannt sind, lässt sich sagen, wie viele Formen wirklich fehlen.

### Stand

`atc-core`: **151 Tests**, Clippy `-D warnings` ohne Befund, `lexer.rs` fmt-sauber,
`parser.rs` mit den 4 vorbestehenden Diffs (meine Zeilen sauber und formatiert).
Invarianten: `apps/atc-contracts` 50/33/0/18, `components/contracts` 46/29/0/18,
Registry-Gate PASS 16/16, EXEC-CHAIN 20/20.

---

## 10. Fünfter Schritt: Fehlersammlung statt Erstfehler-Abbruch

**Datum:** 08.11.2026, aufbauend auf `6dccb27`

### Was gebaut wurde

`parse_program_collecting(src)` im `atc-core`: derselbe Parser, aber nach einem Fehler
wird bis zur nächsten Deklaration gesprungen und weitergemacht. Anker sind die Token,
die eine Deklaration beginnen können (`let/const/fn/use/import/struct/enum/contract/
state/module/pub/@`, `event`, `type`); eine schließende `}` beendet den Sprung, damit ein
Fehler im Rumpf nicht in den nächsten Block läuft. Gesammelt wird auf Programmebene und
im Contract-Körper.

Der Normalbetrieb ist **nicht** betroffen: `collect` ist standardmäßig `false` und der
Fehler verlässt den Parser wie bisher (drei Tests halten das fest, unter anderem
`normalbetrieb_bricht_beim_ersten_fehler_ab`).

CLI: `atc check --all <datei|verzeichnis>` gibt alle Blocker aus.

### Der Befund: die Zahlen sind jetzt lesbar, und sie sind unangenehm

| | |
|---|---|
| aktive Dateien (ohne `/archive/`, `/legacy/`) | 888 |
| davon mit Blockern | 270 |
| Blocker roh | **1997** |
| verschiedene Blocker je Datei | **882** |
| Durchschnitt | **7,4 Blocker je fehlerhafter Datei** |

Verteilung: 8 Dateien mit 1 Blocker, 93 mit 5, 11 mit 10, 5 mit 20, 2 mit 46.

Damit ist die frühere Annahme widerlegt, die 270 Dateien hätten „einen" fehlenden Dialekt.
Sie tragen im Schnitt sieben, einzelne bis zu 46. Der stille Effekt: jede behobene Form
hat genau eine Datei um **einen** Blocker weitergebracht, nicht eine Datei mehr
angenommen. Das erklärt, warum der Korpuszähler seit `e9fce31` bei 1095 steht.

**Wichtige Einschränkung:** die häufigste Meldung — „Deklaration am Top-Level erwartet"
in 262 von 270 Dateien — ist ganz überwiegend **Folgefehler**. Nach dem Sprung landet der
Parser mitten in einer Anweisung, die der Ausdrucksteil nicht kennt. Für die Auswertung
zählt deshalb die Liste ohne diese Meldung (rund 620 echte Formen), nicht die Rohzahl.

### Die echten Blockerarten (verschieden je Datei gezählt)

| Dateien | Form |
|---------|------|
| 19 | `Feldname nach '.'` |
| 12 | `Funktionsname erwartet` |
| 10 | `Typ erwartet` |
| 9 | `Deklaration im contract erwartet` |
| 7 | `',' oder '}' nach Enum-Variante` |
| 7 | `Funktionsdeklaration register ist außerhalb des unterstützten Parsers` |
| 6 | `'{' nach if-Bedingung` |
| 6 | `Feldname im Event` |
| 5 | `']' nach Index` |

Neu sichtbar geworden ist unter anderem **`struct Query<T> {`**: generische Typparameter
an der Struct-Deklaration kennt der Parser nicht, obwohl `Map<K, V>` als Feldtyp längst
geht. Das war vorher hinter anderen Fehlern verborgen.

### Stand

`atc-core`: **154 Tests**, Clippy `-D warnings` ohne Befund.
Invarianten unverändert: `apps/atc-contracts` 50/33/0/18, `components/contracts`
46/29/0/18, Registry-Gate PASS 16/16, EXEC-CHAIN 20/20, Korpus 1095/1645.

---

## 11. Untersuchung der 19 Dateien mit `Feldname nach '.'`

**Datum:** 08.11.2026, Stand `40ebb3c`, aktiv ohne `/archive/` und `/legacy/`.

### Ergebnis: eine einzige fehlende Konstruktion, nicht 19 Fehler

Die 19 Dateien sind **12 verschiedene Inhalte** (Spiegel unter `docs/os-docs/`,
`docs/os-docs/wiki/kai-os/` und `modules/`), und alle scheitern an derselben Form:
dem halboffenen Bereich `..`.

| Anzahl Zeilen | Form | Beispiel |
|---|---|---|
| 160 | `for … in a..b` | `for i in 0..cids.len() {` |
| 13 | Index-Bereich | `to_u8(seq[0..2])`, `self.history[start..]` |
| 75 | kein Bereich | `"…"`-Strings und Kommentare mit `...` (kein Befund) |

Insgesamt enthalten **92 aktive Dateien** `..`, die meisten scheitern vorher an einem
anderen Blocker. Die 19 sind also nur die Spitze.

### Der entscheidende Punkt: `..` ist bereits spezifiziert

Die Sprachspezifikation kennt die Konstruktion vollständig, der Rust-Kern nicht:

| Stelle | Aussage |
|--------|---------|
| `language/SPEC.md` §2.1 (Token-Inventar) | **`DOTDOT`** und **`DOTDOTEQ`** sind Token der Sprache |
| `language/SPEC.md` §3.4, Stufe 7 (`parse_primary`) | Produktion **`Range`** |
| `language/SPEC.md` §3.4, Grundformen | „**Ranges `..`/`..=`, Slices und Ternaries existieren als AST-Knoten (RangeExpr, SliceExpr, TernaryExpr)**" |
| `specs/semantics/SPEC.md` §9.3 | `SliceExpr`-Codegen fehlt (bekannter Compiler-Backlog) |

Damit ist das **keine Architekturentscheidung, sondern eine Konformitätslücke**: die
Konstruktion ist normativ vorhanden, der kanonische Rust-Kern hat weder das Token noch
den AST-Knoten. Das ist der billigste Block der ganzen Liste, weil nichts entworfen
werden muss.

### ABI-Sicherheit geprüft

`apps/atc-contracts` und `components/contracts`: **0 Zeilen** mit `..`. Neue AST-Knoten
können deshalb keinen bestehenden Vertrags-Hash verändern — vorausgesetzt, das
kanonische Encoding bestehender Knoten bleibt unberührt.

### Einordnung

`for i in 0..n` ist eine `RangeExpr` an der Iterator-Position, `seq[0..2]` eine
`SliceExpr` an der Index-Position. Beides gehört in Lexer und Parser (Konformität),
der Codegen von `SliceExpr` bleibt der bereits dokumentierte Backlog und ist **nicht**
Teil dieser Lücke.

Nächster Schritt bleibt die Reihenfolge des Owners: Spezifikation der generischen
Structs (siehe `specs/language/GENERIC-STRUCTS-CANDIDATE.md`), danach Implementierung.

---

## 12. Sechster Schritt: `..` spezifikationskonform (P0/P1)

**Datum:** 08.11.2026, aufbauend auf `4e84d04`. Owner-Freigabe: P0 `..`-Konformität, P1 Range-/Slice-Regression. Generics bleiben Kandidat.

### Was die Untersuchung vorher ergab

`..` ist kein Entwurf, sondern **spezifiziert und nicht implementiert**: Token `DOTDOT`/`DOTDOTEQ` (SPEC §2.1), Produktion `Range` (§3.4 Stufe 7), AST-Knoten `RangeExpr`/`SliceExpr` (§3.4, §4). Zusätzlich fehlte die **`for`-Schleife vollständig** — SPEC §3.2 definiert `for_stmt`, der Rust-Kern hatte weder Token noch Statement.

### Implementiert (fünf Schichten)

| Schicht | Änderung |
|---|---|
| **Lexer** | `..` → `Token::DotDot`, `..=` → `Token::DotDotEq`. Eigene Token, **kein** parser-seitiger Sonderfall für zwei Punkte. `Token::Dot` bleibt Feldzugriff. |
| **AST** | `Expr::Range { start, end, inclusive }`, `Expr::Slice { base, start, end, inclusive }`, `Stmt::For { var, iter, body }`; JSON mit `RangeExpr`, `SliceExpr`, `ForStatement`. |
| **Parser** | Bereichsstufe, `Slice` im Index, `for IDENT in <bereich> block`. `for`/`in` bleiben **Bezeichner** (wie `event`), damit Quellen mit diesen Namen nicht brechen. |
| **Typprüfung** | Neue Regel für **jeden** Funktionskörper: ein Bereich ist nur als `for`-Iterator und als Index zulässig, als Wert Fehler. `for` mit offenem Ende ist Fehler. |
| **Lowering** | `for` als gezählte Schleife aus vorhandenen Opcodes (`ConstI64`/`LoadLocal`/`StoreLocal`/`Lt`/`Le`/`Add`/`Jump`/`JumpIfFalse`) — **kein neuer Opcode**, damit Bytecode-Spezifikation und Determinismus unberührt bleiben. |

### Zwei Auslegungen, offen dokumentiert

1. **Präzedenz:** Die SPEC nennt `Range` unter `parse_primary`. Für `0..cids.len()` muss der Bereich aber schwächer binden als der Postfix-Zugriff, sonst wird das Ende abgeschnitten. Die Stufe liegt deshalb direkt über `||` (wie in Rust). Die **Knotenform** folgt der SPEC; die Einordnung der Stufe ist eine Auslegung.
2. **Feldform:** SPEC und `registry.json` legen die **Namen** der Knoten fest, nicht ihre Felder. Gewählt wurde die Form analog zu `IndexAccess` (`base`, `start`, `end`, `inclusive`). Auch das ist eine Auslegung, keine Spec-Aussage.

### Qualitätsmatrix (Owner-Anforderung) — Nachweis je Punkt

| Anforderung | Nachweis |
|---|---|
| Lexer erkennt die Form | `lexer::tests::bereichs_token_sind_unterscheidbar` |
| Parser erzeugt den richtigen AST | `for_ueber_bereich`, `inklusiver_bereich`, `index_bereich_ist_slice`, `offenes_bereichsende_parst` |
| Typprüfung akzeptiert gültige Ausdrücke | `for_bereich_wird_akzeptiert`, `slice_index_wird_akzeptiert` |
| Typprüfung verwirft ungültige Ausdrücke | `bereich_als_wert_wird_abgelehnt`, `for_mit_offenem_ende_wird_abgelehnt` |
| Compiler verarbeitet die Form | `for_schleife_zaehlt_und_endet` (4), `for_inklusiv_erreicht_das_ende` (40), `for_ueber_leeren_bereich_laeuft_nie` (7), `for_schleife_ist_verschachtelbar` (22) — über die echte VM |
| ABI-/Hash-Verträge unverändert | Registry-Gate PASS 16/16, `apps/atc-contracts` 50/33/0/18, `components/contracts` 46/29/0/18 |
| Tests auf aktuellem `main`-HEAD | frischer Klon, 170 Tests grün |

Negativtests Parser: `for_ohne_in_ist_ein_fehler`, `for_ohne_bereich_ist_ein_fehler`.

### Ausdrücklich NICHT implementiert

**`SliceExpr`-Codegen.** Ein Slice braucht einen eigenen Opcode; das ist eine Bytecode-Änderung und damit ein eigener Entscheid (die SPEC nennt fehlendes `SliceExpr`-Codegen bereits als Backlog, `semantics/SPEC.md` §9.3). Das Lowering meldet das klar: „Slice-Ausdruck: Codegen ist nicht implementiert (eigener Opcode noetig)". Test: `slice_ohne_codegen_meldet_sich_klar`.

### Nebenbefund, wichtig für die künftige Messung

Ein **Top-Level-`fn` mit nicht parsebarem Rumpf** wird still zur `NativeFunction` (Tokens konserviert). Deshalb war ein Negativtest im freien Funktionskontext grün, obwohl der Code falsch ist; der Test läuft jetzt im Contract-Kontext, wo es keinen Fallback gibt. Für „Parser-Akzeptanz" heißt das: top-level zählt nur eingeschränkt als Nachweis.

### Stand

Korpus: **1113 von 1645** angenommen (vorher 1095, +18). `atc-core`: **170 Tests**, Clippy `-D warnings` ohne Befund, alle geänderten Dateien fmt-sauber. EXEC-CHAIN 20/20, Gate PASS 16/16.
