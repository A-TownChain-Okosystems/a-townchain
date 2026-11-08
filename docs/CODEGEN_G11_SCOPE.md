# Vertrags-Codegen (G11) — Ausgangslage, Schnitte, offene Entscheidungen

Status: freigegeben (Owner 08.11.2026), Umsetzung gestartet — Schnitt S1
Stand: 2026-11-08
Bezug: ATCLang-Roadmap G11, ATC-STD-SC-002/019 (Contract Registry), Gate A8/A9

Dieses Dokument beschreibt, was Contract-Codegen heute blockiert, in welcher
Reihenfolge es umsetzbar ist und welche Entscheidungen dafür beim Owner liegen.
Es behauptet **keinen** Fortschritt am Codegen selbst.

## 1. Geprüfte Ausgangslage

Aus dem Code (`ai/atclang/crates/atc-core`), nicht aus älteren Statusaussagen:

- **Instruktionssatz** (`src/bytecode.rs`): `ConstI64`, `LoadLocal`, `StoreLocal`,
  `Add/Sub/Mul/Div/Neg`, `Eq/Ne/Lt/Gt/Le/Ge`, `Call{function,argc}`, `Return`,
  `Pop`, `Jump`, `JumpIfFalse`. Also ausschließlich 64-Bit-Ganzzahlen, lokale
  Variablen und Kontrollfluss.
- **Fehlt vollständig**: Zustandsslots, Storage-Zugriff, Strings/Bytes, Adressen,
  Hash-/Krypto-Host-Aufrufe, Event-Ausgabe.
- **Lowering** (`src/lower.rs`): Vertragslogik wird abgelehnt —
  `Emit`, `Require`, `Assign` liefern `"Vertragsstatement ausserhalb des
  ausfuehrbaren Subsets"`; Feld-/Index-/Array-Ausdrücke ebenso. `Contract`,
  `State`, `Event` gelten als reine Metadaten.
- **Folge**: Ein Vertrag kompiliert heute zu einem Programm mit **null**
  Funktionen. Gate A8 prüft daher Parser- und Typprüfung, nicht Ausführbarkeit.
- **Vorhanden und nutzbar**: Parser (AST), Semantik Stufe 1, Typprüfung Stufe 2,
  projektweite Auflösung Stufe 3, kanonische ABI (Stufe 4) mit `abi_sha256`,
  Registry-Bindung über Gate A9.

## 2. Was Codegen zusätzlich braucht

| Lücke | Nötig für | Größe |
| --- | --- | --- |
| L1 Zustandsslots (`LoadState`/`StoreState`) | `self.<feld>` lesen/schreiben | klein |
| L2 `Require`/`Emit` als Instruktionen | Guards, Events | klein |
| L3 Typen jenseits `i64` (Adresse, u256, Bytes, string) | Signaturen der ABI | groß |
| L4 Host-Aufrufe (`sha256`, Signaturprüfung, `msg_sender`, `now`) | Krypto, Umgebung | mittel |
| L5 Vertragsdispatch (`<Vertrag>::<fn>` adressierbar) | Aufruf von außen | klein |
| L6 Storage-Persistenz | Zustand über Transaktionen | außerhalb von atc-core |
| L7 Deployment-Artefakt (Bytecode + ABI + Hash) | Registry/Node | mittel |

## 3. Vorgeschlagene Schnitte

Jeder Schnitt ist für sich prüfbar; keiner setzt den nächsten voraus.

- **S1 — Zustand im Speicher.** `LoadState(u16)`/`StoreState(u16)` auf einem
  Slot-Vektor, Slot-Zuordnung aus der ABI (sortiert, also stabil). Akzeptanz:
  ein Vertrag mit zwei Feldern liest, schreibt und liest zurück; ABI-Slots und
  Bytecodeslots stimmen überein; Verifier lehnt Slot ≥ Feldzahl ab.
- **S2 — Guards und Events.** `Require` → Sprung auf Fehlerpfad,
  `Emit` → Instruktion mit Event-Index und Argumenten. Akzeptanz: fehlschlagendes
  `require` beendet ohne Zustandsänderung; Event-Argumente in Deklarationsreihenfolge.
- **S3 — Dispatch.** Jede Vertragsfunktion als `<Vertrag>::<fn>` im
  Funktionstab, `init` als Einstieg. Akzeptanz: `atc compile` liefert für die
  16 registrierten Quellen Funktionen > 0; Aufruf über Index läuft in der VM.
- **S4 — Typen.** Adresse/Hash/Bytes als opake 32-Byte-Werte, `u256` als Paar
  oder BigInt, `string` nur lesbar. Akzeptanz: Signaturen der ABI sind im
  Bytecode abbildbar, Rundlauf über `encode`/`verify` grün.
- **S5 — Host-Schnittstelle.** Versionierte Import-Tabelle (Name → Index),
  fail-closed bei unbekannter Funktion. Akzeptanz: `sha256` im Vertrag liefert
  denselben Hash wie die Referenzimplementierung.
- **S6 — Artefakt.** Bytecode + ABI + beide Hashes in einer Datei, Registry
  bindet den Bytecode-Hash zusätzlich zum ABI-Hash. Akzeptanz: Gate A10 prüft
  Bytecode-Drift.

Außerhalb von `atc-core` und **nicht** Teil von G11: Storage-Backend im Node,
Transaktions-/Gas-Modell, Deployment-Ausführung.

## 4. Risiken

- **R1 Semantik-Festschreibung.** Sobald Bytecode erzeugt wird, sind
  Instruktionssatz und Typabbildung schwer änderbar. Deshalb S1–S3 mit
  Versionierung im Artefakt, nicht erst S6.
- **R2 Zwei Wahrheiten.** ABI-Hash (Schnittstelle) und Bytecode-Hash
  (Umsetzung) können auseinanderlaufen; beide müssen registriert sein, sonst
  entsteht ein stiller Drift wie vor Gate A9.
- **R3 Aufweichen der Prüfkette.** Ein „lauffähiger" Vertrag ohne
  Host-Funktionen und Storage wäre ein Fehlschluss aus grünen Tests; S5/S6
  dürfen nicht als erledigt gelten, solange L6 offen ist.

## 5. Entscheidungen (Owner, 08.11.2026)

Alle vier Fragen sind entschieden und damit verbindlich:

1. **Speicherlayout:** ABI-definierte Zustandsslots fester Breite, Feldnamen nur
   Metadaten.
2. **u256:** native VM-Arithmetik, keine Delegation der Konsens-Arithmetik an
   eine Node-Bibliothek.
3. **Reihenfolge:** S1–S3 zuerst; die Basistypen für S1–S3 werden vorab
   definiert, damit S4 keine versteckte Voraussetzung wird.
4. **Artefakt:** kanonisches Binärformat mit Header (Magic, Format-, VM-, ABI-,
   Layout-Version, Instruktionszahl, Code-Länge, Prüfsumme); JSON+Base64 nur zur
   Inspektion.

Zusätzlich verbindlich: Gates G11-A (deterministisches Compiler-Ergebnis),
G11-B (deterministische Ausführung), G11-C (keine unkontrollierten Host-Aufrufe),
G11-D (ABI-/Storage-/Artefakt-Kompatibilität). Ein grüner Parser- oder
Typechecker-Test gilt nicht mehr als Vertragskompilierung.

## 6. Stand S1 (2026-11-08)

Umgesetzt: `atc_core::storage::StorageLayout` — determinstische Slot-Zuordnung
aus der ABI, 32 Byte je Slot, `offset = index * 32`, Reihenfolge `(Vertrag, Feld)`
lexikographisch statt Deklarationsreihenfolge, Kollisionsschutz je
`Vertrag::Feld`, eigene `layout_version`, kanonische Textform und `layout_hash`.
Dynamische Typen (`string`, `List`, `Map`) werden als `Dynamic` geführt und sind
in S1 nicht adressierbar; unbekannte Typen als `Opaque`, nichts wird geraten.
`atc storage <ziel>` zeigt je Quelle `layout_hash`, Slotzahl, Speicherbedarf und
`s1=yes|no`.

Bestand: 34 Quellen mit Zustand, 1 Quelle vollständig S1-adressierbar
(`examples/ATCAvatarNFT.atc`); der Rest nutzt `string`/`Map` und wartet auf S4.

**Blocker fuer die naechsten S1-Teile:** `ATC-BC-001` ist als v1.0.0-FROZEN
eingefroren (Freeze-Anker `ee26531`). `LoadState`/`StoreState` lassen sich daher
nicht stillschweigend in den bestehenden Instruktionssatz aufnehmen. Der
Instruktionssatz braucht eine neue Formatversion (ATCB v2) mit eigenem
kanonischem Encoder und eigener Version im Header — als SCR gegen `ATC-BC-001`
zu fuehren, nicht als stille Erweiterung.

## 7. Offene Entscheidungen (weiterhin)

1. **Zustandsmodell:** Slots fester Größe aus der ABI (einfach, deterministisch)
   oder Key-Value über Feldnamen (flexibler, aber Hash-/Reihenfolgeabhängig)?
2. **`u256`/Beträge:** eigenes Paar-Arithmetik-Subset im VM oder Host-Aufruf an
   die Node-Bibliothek?
3. **Reihenfolge:** S1–S3 (Zustand, Guards, Dispatch) zuerst, damit Verträge
   überhaupt lauffähig werden — oder S4 (Typen) vorziehen, damit Signaturen
   vollständig sind?
4. **Artefaktformat:** ein Binärformat mit Header oder JSON+Base64, dafür sofort
   im Registry-Gate prüfbar?

Weiter offen: Bytecode-Formatversion fuer State-Instruktionen (ATCB v2 per SCR).

Der Status bleibt bis zum Nachweis eines ausfuehrbaren Vertrags:
`NOT PRODUCTION_READY`.
