# Generische Struct-Deklarationen — Spezifikations-Kandidat

**Status:** KANDIDAT, **nicht normativ.** Keine Implementierung ohne Ratifizierung.
**Owner-Entscheidung:** 08.11.2026, „ARCHITEKTURZIEL: AUFNEHMEN".
**Verhältnis zur Spezifikation:** `SPEC.md` §3.3 definiert heute
`struct ::= "struct" IDENT block_field+` — ohne Typparameter. Dieses Dokument füllt
genau diese Lücke. Es ändert die bestehende Spezifikation nicht, sondern schlägt die
fehlende Produktion vor. Nach Ratifizierung ist eine SPEC-Revision mit Versions-Bump
fällig (Normativitäts-Regel, `SPEC.md` Kopf).

---

## 1. Ausgangslage (belegt, nicht vermutet)

| Beleg | Aussage |
|---|---|
| `SPEC.md` §2.3 | Die Keywords **`where`** und **`with`** existieren bereits, ausdrücklich gruppiert als „Generics/Traits" |
| `SPEC.md` §6 | Die Fehlerklasse **`InvalidGenericError`** existiert |
| `SPEC.md` §2.4 | Generische Typ-**Anwendung** ist Typ-Inventar: `Map`, `List`, `Set`, `Array`, `Vec`, `Tuple`, `Option`, `Result`, `Either` |
| `semantics/SPEC.md` §9.1 | `parse_type` hatte einen „generischen Zweig (`<...>`)" — Typ-**Argumente** werden also geparst |
| `SPEC.md` §3.3 | Die Struct-**Deklaration** kennt **keine** Typparameter |

Die Sprache kann also `Map<String, Address>` als Feldtyp schreiben, aber nicht
erklären, woher `K` und `V` kommen. Das Feld `Map<K,V>` im Korpus funktioniert heute
nur, weil der Parser `K` und `V` als undurchsichtige Typnamen durchreicht. Genau
deshalb ist die Deklarationsseite keine Erweiterung, sondern die fehlende Hälfte
einer schon vorhandenen Funktion.

---

## 2. Syntax (nur Syntax, keine Bedeutung)

```text
struct        ::= "struct" IDENT type_params? struct_body
type_params   ::= "<" type_param ("," type_param)* ">"
type_param    ::= IDENT
struct_body   ::= "{" block_field+ "}"          # Klammerform
               |  ":" NEWLINE INDENT (feldzeile)+ DEDENT   # eingerückte Form
```

- Ein Typparameter: `struct Query<T> { value: T }`
- Mehrere, in Deklarationsreihenfolge: `struct Map<K, V> { key: K, value: V }`
- Keine Vorgabewerte, keine Varianz-Annotation, keine Constraints an dieser Stelle
  (siehe §6).

**Eindeutigkeit ohne Rückverfolgung:** An Deklarationsposition ist `<` nach dem
Strukturnamen immer der Beginn der Typparameterliste; eine andere Verwendung von `<`
ist dort nicht grammatisch. Der Parser braucht deshalb keinen Backtracking-Schritt.

---

## 3. AST

```text
StructDeclaration {
  name:        String,
  type_params: [String],      // neu, Deklarationsreihenfolge
  fields:      [FieldDeclaration]
}
```

**ABI-Invariante (verbindlich):** `type_params` erscheint im kanonischen Encoding
**nur, wenn die Liste nicht leer ist**. Für jede Struct ohne Typparameter muss das
Encoding byte-gleich zum Stand vor dieser Änderung bleiben, sonst verschieben sich
ABI-Hashes bestehender Verträge. Nachweis: Registry-Gate (heute 16/16) plus ein
eigener Regressionstest, der Encoding und Hash eines Structs vor/nach der Änderung
vergleicht. Die genaue Stelle (JSON-Ausgabe und `encode_canonical`) ist vor der
Implementierung zu prüfen — der Test ist die Bedingung, nicht die Behauptung.

---

## 4. Typsemantik

1. **Sichtbarkeit.** Ein Typparameter ist ausschließlich im eigenen Struct-Rumpf
   sichtbar, in Feldtypen und in der Feldreihenfolge unabhängig von der Position.
2. **Eindeutigkeit.** Zwei Typparameter mit gleichem Namen sind ein Fehler
   (`DuplicateTypeParam`), nicht „der letzte gewinnt".
3. **Abschattung.** Ein Typparameter darf einen äußeren Typnamen verdecken
   (`struct Map<String, V>`); die Abschattung ist lokal und endet mit dem Rumpf.
4. **Bindung.** Im Rumpf ist ein Typparameter ein vollwertiger Typ: als Feldtyp, als
   Argument eines anderen generischen Typs (`List<T>`) und verschachtelt
   (`Map<String, List<T>>`) mit unbegrenzter Tiefe.
5. **Typarität.** `Query` mit einem Parameter verlangt genau ein Typargument in jeder
   Verwendung. Zu wenige oder zu viele Argumente sind Fehler
   (`TypeArgumentCount`), nicht stillschweigend `Any`.
6. **Fehlende Bindung.** Wird ein generischer Typ ohne Typargumente verwendet, ist das
   ein Fehler (`MissingTypeArgument`) — außer in dem Kontext, in dem der Parameter
   selbst gebunden wird (Feldtyp innerhalb des generischen Structs).
7. **Nicht deklarierter Parameter.** Ein Feldtyp `T` in einem Struct ohne `<T>` ist
   `UnknownType` und damit derselbe Fehler wie jeder andere unbekannte Typ.
8. **Keine strukturelle Gleichheit ohne Argumente.** `Query` und `Query<u64>` sind
   verschiedene Typen; die Typprüfung vergleicht Name **und** Argumentliste.

**Nicht** Teil dieses Kandidaten: Constraints (`where`), Varianz, generische
Funktionen, generische Enums, generische Contracts, Rekursion über Typparameter.

---

## 5. Testmatrix (Bedingung für „implementiert")

| # | Fall | Erwartung |
|---|------|-----------|
| 1 | Einfacher Typparameter `struct Query<T> { value: T }` | angenommen, `type_params = ["T"]` |
| 2 | Mehrere Typparameter `struct Map<K, V>` | angenommen, Reihenfolge `["K","V"]` |
| 3 | Generischer Feldtyp `value: T` | als Typ `T` im AST, nicht als unbekannter Name |
| 4 | Verschachtelt `Map<String, List<T>>` | angenommen, Tiefe 2 |
| 5 | Doppelter Parameter `struct S<T, T>` | Fehler `DuplicateTypeParam` |
| 6 | Nicht deklarierter Parameter `struct S { x: T }` | Fehler `UnknownType` |
| 7 | Falsche Argumentzahl `Query<u64, u8>` bei einem Parameter | Fehler `TypeArgumentCount` |
| 8 | **Regression** bisher gültige Structs | Encoding und ABI-Hash byte-gleich |
| 9 | Eingerückte Form `struct Query<T>:` + Felder | angenommen (nutzt das Spaltenmodell) |

Fall 8 ist der wichtigste: er entscheidet, ob die Änderung ABI-neutral ist.

---

## 6. Ausdrücklich separat zu spezifizieren

| Thema | Warum getrennt | Vorschlag |
|---|---|---|
| **Constraints** (`where`, `with`) | Die Keywords existieren, die Semantik nicht. Ohne Definition wären Constraint-Prüfungen Erfindung. | Eigenes Dokument; `where` als Klausel nach den Typparametern |
| **Monomorphisierung** | Entscheidet über Bytecode, nicht über Syntax. Berührt Determinismus und ABI. | Eigenes Dokument im `specs/bytecode`-Umfeld |
| **Codegenerierung** | Erst nach semantischer Definition sinnvoll (Owner-Leitentscheidung) | Nach §4 und nach §6/Constraints |

---

## 7. Umsetzungsreihenfolge

1. Syntax und AST, mit bedingter Emission (§3) — Parser kann Typparameter lesen.
2. Negativtests 5, 6, 7 auf Parser-Ebene, soweit dort entscheidbar.
3. Typprüfung und Typarität (§4) — erst damit ist der Fall „generisch" auch geprüft.
4. Registrierung der Fälle 1–9 als Tests, Fall 8 als Hash-Vergleich.
5. Danach, separat: Constraints, Monomorphisierung, Codegen.

Bis Schritt 4 abgeschlossen ist, bleibt `struct Query<T>` ein **anerkannter
Architektur-Kandidat** und keine zugesagte Sprachfunktion.
