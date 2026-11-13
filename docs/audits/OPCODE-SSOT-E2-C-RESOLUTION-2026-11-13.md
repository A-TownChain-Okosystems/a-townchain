# OPCODE-SSOT — E2-C-Entscheidung: eine normative Quelle, explizite Generationen

> **Datum:** 13.11.2026 · **Art:** Architekturentscheidung (E2-C), noch keine Codeaenderung
> **Basis:** `docs/audits/OPCODE-SSOT-INVENTORY-2026-11-13.md` (E2-A/E2-B)
> **Auftrag:** Owner-Freigabe 13.11.2026 — eine normative Opcode-Quelle, explizite Generationen, keine impliziten Byte-Kompatibilitaeten
> **Status dieses Dokuments:** Entscheidung dokumentiert. Implementierung folgt in E2-C-1 bis E2-C-4, Nachweis in E2-E.

---

## 0. Entscheidung in einem Absatz

ATC-BC-001 ist und bleibt die **einzige normative Opcode-Quelle**. ATC-BC-002 existiert
ausschliesslich als **Generation 2** und niemals als Neubelegung der Generation 1. Die
Generation ist ein **explizites Feld im Artefakt**, wird nie aus Bytes geraten und trennt
die Tabellen vollstaendig: ein Code, der in der deklarierten Generation nicht definiert
ist, wird abgewiesen statt umgedeutet. Die heute im Monorepo versiegelte VM-Generation 1
ist keine BC-001 und wird deshalb **zurueckgezogen** und als Generation 2 neu versiegelt.
Es gibt genau eine Tabelle im Code, abgeleitet aus dem normativen Text; `.ops` bleibt
ausdruecklich ein Textformat ohne Byte-Autoritaet.

## 1. Grundsaetze (verbindlich fuer alle Folgearbeiten)

| Id | Grundsatz |
|---|---|
| G1 | Eine normative Tabelle: ATC-BC-001. Keine zweite, keine dritte. |
| G2 | Generationen sind explizit, versionsbehaftet und Teil der Integritaet. |
| G3 | Keine implizite Byte-Kompatibilitaet: gleiche Zahl in anderer Generation ist ein anderer Befehl. |
| G4 | Kein bestehender Opcode-Wert wird geaendert. Aenderungen geschehen nur durch eine neue Generation. |
| G5 | Fail-closed: unbekannt, falsche Generation, abgeschnitten, ueberzaehlig = Fehler mit Code und Offset. |
| G6 | Keine automatische Konvertierung zwischen Generationen. |
| G7 | Statusclaims nur mit Evidence; kein `CI_VERIFIED` ohne aktuellen CI-Lauf. |

## 2. R1 — ATC-BC-001 ist die normative SSOT

- Normativer Text: `atclang/specs/bytecode/SPEC.md` (v1.0.0-FROZEN), 19 Opcodes,
  big-endian, 10-Byte-Kopf, keine Checksum im Container.
- Der Compiler bildet diese Tabelle bereits exakt ab (`encode_canonical`,
  `decode_bytecode`). Das bleibt so; die dortige Tabelle ist eine **Ableitung**, keine
  zweite Quelle.
- **Eine** maschinenlesbare Ableitung wird zur Build- und Testzeit gegen den normativen
  Text geprueft (Drift-Test). Compiler und VM konsumieren ansonsten diese Ableitung.
  Vorschlag zur Umsetzung: Generator in `tooling/atc-engineering` plus Test, der bei
  Abweichung fehlschlaegt. Der Generator ersetzt keine Norm und aendert keinen Wert.
- Was BC-001 **nicht** ist: keine Aussage ueber Gas, Ausnahmen oder `require`-Semantik.
  Diese Luecke ist der Anlass von BC-002 und bleibt eine Kandidatenfrage.

## 3. R2 — ATC-BC-002 ausschliesslich als Generation 2

- BC-002 bleibt **Kandidat**, bis eine Ratifizierung in `atc-standards` erfolgt. Bis dahin
  ist es eine implementierte Generation ohne normative Wirkung.
- Die Basisbefehle von BC-002 (`0x01..0x10`) sind **nur** in Artefakten gueltig, die
  Generation 2 deklarieren. Die Selbstbezeichnung des Kandidatentexts „Generation 1" fuer
  diese Basisbefehle entfaellt als Bezeichnung; sie beschreibt keinen Zustand, den ein
  Artefakt annehmen darf.
- Die zusaetzlichen Befehle `0x11 Assert`, `0x12 Revert`, `0x13 RevertIf` existieren
  ausschliesslich in Generation 2.
- Eine Generation 1 mit BC-002-Belegung wird nicht mehr erzeugt (siehe R5).

## 4. R3 — Eindeutige Generationserkennung

| Id | Regel |
|---|---|
| R3.1 | Das Artefakt deklariert seine Generation in einem eigenen Feld. Es gibt keinen Default und keinen Rueckfall auf „1". |
| R3.2 | Fehlendes, unbekanntes oder widerspruechliches Generationsfeld ist ein Fehler, bevor ein Opcode gedeutet wird. |
| R3.3 | Pro Generation gilt eine **geschlossene** Tabelle. Ein Code, der in der deklarierten Generation nicht definiert ist, wird abgewiesen; er wird nicht in der anderen Generation nachgeschlagen. Damit entfaellt die heutige Sonderregel, dass `0x01..0x10` in beiden Generationen gelten. |
| R3.4 | Endianness ist Eigenschaft der Generation (BC-001 big, BC-002 little) und wird nie aus dem Inhalt abgeleitet. |
| R3.5 | Die Generation geht in die Integritaetspruefung ein (bei BC-002 heute schon ueber den Hash des Kopfes). |
| R3.6 | Ein BC-001-Container (viertes Byte `0x00`) und ein BC-002-Container (viertes Byte `0x01`) duerfen nie ineinander geparst werden. Die heutige Trennung bleibt, wird aber als Regel dokumentiert und getestet statt als Zufall der Kopfgeometrie. |
| R3.7 | Ablehnungscodes bleiben stabil: `E-010` unbekannter Opcode, `E-011` Opcode gehoert nicht zur deklarierten Generation, jeweils mit Offset. |

## 5. R4 — Stilllegung des Legacy-Encoders

Betroffen: `Instruction::encode` und `Bytecode::encode` in `atclang/crates/atc-core/src/bytecode.rs`.

| Schritt | Inhalt | Nachweis |
|---|---|---|
| R4.1 | Beide Funktionen werden als `deprecated` markiert und als ausschliesslich test-nah gekennzeichnet, solange sie existieren. | Doc-Kommentar plus Test, der keinen Produktionsaufrufer zulaesst |
| R4.2 | Aufruferpruefung: heute ausschliesslich Tests innerhalb von `bytecode.rs`. Bleibt das so, werden beide Funktionen entfernt. | Quelltext-Scan als Test, kein Treffer ausserhalb `#[cfg(test)]` |
| R4.3 | Muss ein Legacy-Encoder aus Kompatibilitaetsgruenden bleiben, wird er an die kanonische Tabelle gebunden (`Return 0x21`, `Jump 0x30`, `JumpIfFalse 0x31`, `Pop 0x40`), mit Test gegen die SSOT-Ableitung. | Gleichheitstest Encoder/SSOT |
| R4.4 | Ein Rueckfall auf die verschobene Tabelle (`Return 0x30`, `Pop 0x31`, `Jump 0x40`, `JumpIfFalse 0x41`) ist ein Testfehler. | Negativtest mit diesen vier Bytes |

Der Legacy-Encoder ist heute nicht Teil des Produktionspfads (`Artifact::from_program`
nutzt `encode_canonical`). Die Stilllegung aendert deshalb kein Verhalten, nur die Flaeche.

## 6. R5 — Migration und Kompatibilitaet

Ausgangslage: `components/system/src/system.rs:254` versiegelt jedes Vertragsartefakt mit
`GENERATION_V1`, also mit der BC-002-Belegung unter dem Namen „Generation 1". Diese
Kombination ist der eigentliche Defekt der Architektur: ein Artefakt behauptet eine
Generation, die normativ anders belegt ist.

| Id | Regel |
|---|---|
| R5.1 | Nach der Umstellung bedeutet Generation 1 ausschliesslich ATC-BC-001. Die bisherige Belegung existiert dann nicht mehr unter diesem Namen. |
| R5.2 | Bestehende Artefakte der alten Generation 1 werden **nicht** umgedeutet. Sie werden als Generation 2 neu versiegelt: gleicher Operationsstrom, neues Generationsbyte, neuer Hash. Der Vorgang ist mechanisch, aber explizit und protokolliert. |
| R5.3 | Es gibt keine stille Konvertierung. Ein altes Artefakt, das als Generation 1 eingereicht wird, scheitert fail-closed und nennt Code und Offset. |
| R5.4 | Persistierte Artefakte: im Repo existieren keine `.atcb`-Dateien; Artefaktbytes entstehen beim Deploy und werden ueber Envelope und Hash an den Block gebunden. Betroffen sind daher nur Dev-/Testketten, die neu aufgebaut werden. Vor der Umstellung ist zu pruefen, ob ausserhalb des Repos Artefakte liegen; das ist ein Gate, keine Annahme. |
| R5.5 | Die Uebersetzer-Bruecke (`backend.rs`) bleibt der Ort, an dem ATCLang-`Instruction` auf VM-`Op` abgebildet wird. Sie arbeitet weiter auf Namen, nicht auf Bytes. Die dort dokumentierten Semantikdivergenzen (Ueberlauf, Vorzeichen von `Lt`) bleiben offen und werden nicht stillschweigend als geloest gefuehrt. |
| R5.6 | `.ops` wird in allen Dokumenten als **Textformat** (Assembler-Eingabe, Fixtures) gefuehrt, nie als Byte-ABI und nie als Opcode-Quelle. |
| R5.7 | Die Doppelimplementierung `atc-vm` (Repo) gegen `components/vm` (Monorepo) braucht eine Owner-Entscheidung: eine Quelle, die andere als Archiv. Bis dahin darf keine der beiden als Referenz fuer Opcode-Werte zitiert werden. |

## 7. R6 — Verbindliche Akzeptanztests (E2-D)

Jeder Test ist so formuliert, dass er bei einer zweiten Tabelle oder einem stillen
Bedeutungswechsel fehlschlaegt.

| Id | Test | Erwartung |
|---|---|---|
| T1 | Alle 19 BC-001-Opcodes kodieren und dekodieren | Bytegleichheit Eingabe/Ausgabe, BE-Operanden |
| T2 | Alle 19 BC-002-Opcodes in Generation 2 | Bytegleichheit, LE-Operanden |
| T3 | Die sieben gemeinsamen Bytes (`0x01`, `0x02`, `0x03`, `0x10`, `0x11`, `0x12`, `0x13`) in falscher Generation | Fehler `E-011`, kein Ausfuehren |
| T4 | Unbekannter Opcode je Generation | Fehler `E-010` mit Offset |
| T5 | BC-001-Container gegen BC-002-Parser und umgekehrt | Fehler in der Containerpruefung, keine Deutung |
| T6 | Trailing Bytes und abgeschnittene Operanden | Fehler, kein Teilausfuehren |
| T7 | Hash-Tail bei BC-002 manipulieren | Integritaetsfehler vor jeder Ausfuehrung |
| T8 | Generation im Kopf fehlt oder ist unbekannt | Fehler, kein Default auf 1 |
| T9 | Legacy-Encoder gegen SSOT-Ableitung | entweder entfernt oder identisch zur kanonischen Tabelle |
| T10 | Negativtest Legacy-Belegung (`Return 0x30`, `Pop 0x31`, `Jump 0x40`, `JumpIfFalse 0x41`) | darf nicht mehr produzierbar sein |
| T11 | Determinismus | zwei Laeufe erzeugen identische Bytes und identischen Hash |
| T12 | Drift-Test Tabelle gegen normativen Text | Abweichung = Testfehler |
| T13 | Live-Pfad versiegelt nach der Umstellung Generation 2, nie „Generation 1 mit BC-002-Bytes" | Artefaktkopf und Tabelle stimmen ueberein |
| T14 | Bestehende Tests der Pakete bleiben gruen | keine Regression |

## 8. Umsetzungsreihenfolge (E2-C-1 bis E2-C-4)

| Schritt | Inhalt | Grenze |
|---|---|---|
| E2-C-1 | SSOT-Ableitung erzeugen und Drift-Test (T12) | keine Bytewerte aendern |
| E2-C-2 | Geschlossene Tabellen je Generation, Generationsfeld verbindlich, `E-011` fuer alle Codes (R3.1–R3.7), T3/T4/T8 | keine Werte aendern, nur Zulaessigkeit |
| E2-C-3 | Legacy-Encoder stillegen (R4), T9/T10 | kein Verhaltenswechsel im Produktionspfad |
| E2-C-4 | Live-Pfad auf Generation 2 umstellen (R5.1–R5.4), T13 | keine stille Konvertierung |
| E2-E | Remote-HEAD, Blob-SHAs, Testlaeufe und -artefakte dokumentieren | ohne CI kein `CI_VERIFIED` |

## 9. Was ausdruecklich nicht entschieden ist

- Ratifizierung von ATC-BC-002. Bis dahin bleibt die Generation 2 implementiert, aber
  nicht normativ.
- Ob BC-002 langfristig die Gas- und Fehlercode-Modellierung uebernimmt.
- Aufloesung der Doppelimplementierung `atc-vm` gegen `components/vm` (R5.7) — das ist
  eine Owner-Entscheidung, keine Messung.
- Semantikdivergenzen Ueberlauf und `Lt`-Vorzeichen. Sie bleiben als offen markiert.
