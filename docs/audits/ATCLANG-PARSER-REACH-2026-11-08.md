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
