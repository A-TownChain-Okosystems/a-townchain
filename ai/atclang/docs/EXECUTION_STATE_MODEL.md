---
document_id: ATC-DOC-LANG-EXEC-STATE
title: ATCLang Execution State Model
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
updated: 2026-11-12
standard: ATC-STD-MD-001
---

# ATCLang Execution State Model

Verbindliche Regel seit dem Audit vom 12.11.2026 (Befund F1, Schritt E1):

> **Compiler Success ist kein ausfuehrbarer Vertrag.**

Ein erfolgreicher Parse- oder Typcheck ist **kein** Nachweis, dass eine Quelle
ausfuehrbar ist. Vor E1 verwarf `lower_program` jeden Vertragsrumpf, erzeugte ein
Ersatzprogramm (`ConstI64(0)` / `Return`) und meldete Erfolg: `atc run` gab `0` aus,
`System::deploy` nahm den Vertrag an, die Kette fuehrte `Push(0); Halt; Halt` aus.
Die Logik war verloren, der Status sagte "OK".

## Zustaende und ihr Nachweis

Jeder Zustand braucht einen konkreten, reproduzierbaren Nachweis. Kein Zustand darf
aus einem spaeteren abgeleitet oder aus einem frueheren angenommen werden.

| Zustand | Bedeutung | Nachweis |
|---|---|---|
| `PARSED` | Quelle ist syntaktisch lesbar | `atc compile <datei>` erzeugt AST-JSON |
| `TYPE_CHECKED` | Typen und Referenzen stimmen | Semantik-/Typstufe ohne Fehler |
| `SEMANTICALLY_VALIDATED` | Projektregeln (Basen, Duplikate, Importe) erfuellt | `atc check <verzeichnis>` ohne Fehler |
| `BYTECODE_GENERATED` | Lowering hat ein Kompilat erzeugt | `lower_program` liefert `Ok(CompiledProgram)` |
| `BYTECODE_VERIFIED` | Artefakt-Hash und Verifizierer stimmen | `Artifact::verify()` erfolgreich |
| `DEPLOYABLE` | Deploy-Gate nimmt das Artefakt an | `System::deploy` ohne Ablehnung |
| `EXECUTABLE` | Entry-Point vorhanden, Ausfuehrung liefert ein Ergebnis | `atc run` mit definiertem Resultat |

## Was E1 im Monorepo durchsetzt

- `lower::executable_subset_gate(prog)` ist das **eine** Gate fuer das ausfuehrbare
  Subset. Es laeuft vor dem Lowering und wird von `lower_program`, `atc run` und
  `atc check <datei>` verwendet. Die Meldung nennt Vertrag, Basen, Funktionen,
  Zustandsfelder, Events und `require`-Guards.
- `lower::require_entry_point(prog)` verlangt fuer `atc run` eine Funktion `main`.
  Eine leere oder rein deklarative Quelle ist nicht ausfuehrbar; vor E1 endete
  `atc run` dafuer mit Exit 0 und gab `0` aus.
- Es gibt kein Ersatzprogramm als Erfolgspfad. Der Fehler bleibt bis zum Aufrufer
  erhalten (Exit 1, keine Ausgabe, kein Artefakt).
- `atc check <datei>` meldet fuer eine Vertragsquelle nicht mehr "validiert",
  sondern den Gate-Fehler. Der Projektmodus `atc check <verzeichnis>` bleibt die
  Strukturpruefung des Korpus (siehe `docs/SMART_CONTRACTS.md`, Gate A8).

## Abgrenzung

Vertragsunterstuetzung bleibt bei **null**. Die Ablehnung ist eine
Sicherheitsmassnahme, keine Fehlerbehebung: der Compiler behauptet nicht laenger,
einen Vertrag zu unterstuetzen, dessen Inhalt er verwirft. Die Umsetzung von
Vertraegen (Opcode-SSOT ATC-BC-001, VM-Decoder, End-to-End-Deploy) ist eigenen
Schritten vorbehalten.
