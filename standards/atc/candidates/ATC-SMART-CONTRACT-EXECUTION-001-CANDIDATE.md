# ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE

**Status:** KANDIDAT · nicht normativ · keine Registry-Bindung
**Datum:** 2026-11-09 · **Geltung nach** Ratifizierung in `atc-standards`

## 1. Zweck

Verbindlicher Lebenszyklus fuer Smart Contracts: Source -> Compiler -> IR ->
Bytecode -> Deployment -> Validation -> Execution -> State Transition ->
Consensus -> Finality. Der Standard ordnet Zustaendigkeiten zu und benennt
die Pruefpunkte zwischen den vier Systemen.

## 2. Nicht-Ziele

- Keine zweite Sprache neben ATCLang.
- Keine zweite VM neben der ATC-VM.
- Keine Konsensregeln in der VM.
- Keine Aenderung eingefrorener Formate ohne Aenderungsverfahren.

## 3. Verantwortungsmatrix

| System | Darf verantworten | Darf nicht |
| --- | --- | --- |
| ATCLang | Syntax, Typen, IR, Bytecode-Erzeugung | Konsens, Persistenz, Netzwerk |
| ATC Smart-Contract | Vertragsregeln, Storage-Schema, ABI | Eigene VM, eigene Execution |
| ATC-VM | Determinismus, Gas, Speichergrenzen | Zeit, Zufall, Netzwerk, Signaturen |
| A-TownChain | Transaktion, Konsens, Zustaende | Vertragslogik interpretieren |

## 4. Lifecycle mit Pruefpunkten

| Code | Stufe | Eingang | Ausgang | Zustaendig |
| --- | --- | --- | --- | --- |
| SCX-001 | Source | Vertragsquelle | geparste Einheit | ATCLang |
| SCX-002 | Compiler | Einheit | typgeprueftes IR | ATCLang |
| SCX-003 | Bytecode | IR | Artefakt mit Magie ATCB + Version | ATCLang |
| SCX-004 | Verifikation | Artefakt | gueltig oder Ablehnung | ATC-VM Verifier |
| SCX-005 | Deployment | signierte Transaktion | Contract ID + Storage-Wurzel | A-TownChain |
| SCX-006 | Aufruf | Transaktion + Kontext | Ausfuehrungsauftrag | A-TownChain |
| SCX-007 | Ausfuehrung | Bytecode + Zustand | Ergebnis, Gas, Events | ATC-VM |
| SCX-008 | Zustandswechsel | Ergebnis | neuer Zustand oder Abbruch | A-TownChain |
| SCX-009 | Konsens | Vorschlag | bestaetigter Block | A-TownChain |
| SCX-010 | Finalitaet | Block | dauerhafter Zustand | A-TownChain |

Regel: Keine Stufe darf die Pruefung einer spaeteren Stufe ersetzen.
Eine gueltige Compilierung ist kein Ausfuehrungsrecht.

## 5. Blocker: `require` ist heute wirkungslos

Belegt am 08.11.2026: Ein Vertrag mit `require(x == 0)` bei x=7 erzeugt acht
Operationen ohne Pruefinstruktion und laeuft auf PASS (Exit 0).

Ursache: Der eingefrorene Opcode-Satz ATC-BC-001 kennt keinen Assert- oder
Revert-Op. `require` wird beim Lowering still verworfen. Damit ist jede
Vertragsinvariante, die nur in einem `require` steht, ungeschuetzt.

Konsequenz fuer diesen Standard: SCX-003 bis SCX-008 sind nicht erfuellbar,
solange Bedingungen nicht als Bytecode existieren. Ein Smart-Contract-Standard
darf `require` deshalb erst normieren, wenn ATCB-v2 entschieden ist.

Optionen (Entscheidung Owner):
- A) ATCB-v2 fuehrt ASSERT und REVERT ein, versioniert, mit Verifier-Gate.
- B) `require` wird auf JumpIfNotZero plus Abbruchpfad abgebildet, ohne neues
  Op; Nachteil: kein Revert-Grund, kein sauberer Speicher-Rollback.
Empfehlung: A, weil ohne Revert-Semantik atomare Transfers nicht sicher sind.

## 6. Determinismus (SCX-007)

Zwei Validatoren mit gleichem Ausgangszustand und gleicher Transaktion
erzeugen identisches Ergebnis, identisches Gas, identische Events und
identisches Fehlerverhalten. Verboten in der Ausfuehrung: Systemzeit,
Zufall, Netzwerk, Dateisystem, Umgebungsvariablen, Iterationsreihenfolge
nicht deterministischer Strukturen.

## 7. Kanonische Kodierung und ABI

- Jede signierte Nutzlast traegt Domain-Praefix und Laengenfelder.
- ABI ist versioniert; Aufrufe nennen Version und Funktionsauswahl explizit.
- Die Blockchain uebernimmt keine vom Client behaupteten Ergebnisse.
- Gas wird von der VM berechnet, nie vom Aufrufer geliefert.

## 8. Evidence je Stufe

Jeder Pruefpunkt braucht Test plus Nachweis, sonst bleibt er unter der
Statusleiter. Reihenfolge: IMPLEMENTED, TESTED, CI_VERIFIED, E2E_VERIFIED,
VERIFIED. Nicht nachgewiesen bleibt BLOCKED.

## 9. Offene Entscheidungen vor Ratifizierung

1. ATCB-v2: ASSERT/REVERT einfuehren (Empfehlung) oder Abbildung ohne neues Op.
2. Revert-Semantik: Speicher-Rollback je Rahmen oder je Transaktion.
3. Storage-Modell: Maps als VM-Primitive oder Contract-seitig ueber Slot-Rechnung.
4. ABI-Kodierung: eigene kanonische Form oder etabliertes Format.
5. Gas-Modell: feste Op-Preise oder Tabelle mit Versionsbindung.
