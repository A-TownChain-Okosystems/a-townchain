---
document_id: ATC-DOC-AUDIT-031
title: "Systemintegration — Quelle, Artefakt, Freigabe, VM und Kette als ein Pfad"
version: 1.0.0
status: evidence
owner: A-TownChain-Okosystems
created: 2026-11-11
updated: 2026-11-11
standard: ATC-STD-MD-001
---

# Systemintegration 2026-11-11

**Scope:** neues Crate `components/system` plus eine Erweiterung in `components/vm`.
**Nicht in diesem Scope:** M-03 / `aurora-ai` (unveraendert), Konsens, Netzwerk, Kernel,
Abrechnung, CI.

## 1 — Was vorher fehlte

Die Integrationsmatrix vom 09.11.2026 hat vier Universen ohne Ausfuehrungskante
belegt: `atc-vm` hatte 0 Nutzer, `atc-core` erreichte kein Produkt, der Node fuehrte
0 Bytecode aus, es gab keinen Admission-Punkt. Zwischen Compiler und VM existierte
keine Abbildung: `atc-core` erzeugt `Instruction` (ei­gene Kodierung, PC-relative
Spruenge), `atc-vm` fuehrt `Op` in einem ATCB-Container aus (absolute Spruenge,
`u64`-Stapel, Speicherplaetze statt Rahmen).

## 2 — Was jetzt laeuft

Ein Pfad, fuenf Stufen, jede mit eigenem Beleg:

| Stufe | Crate | Ergebnis |
|---|---|---|
| 1 Quelle | `atc-core` | `CompiledProgram`, verifiziert (`verify before trust`) |
| 2 Abbildung | `atc-system` | `Translation` mit PC-Zuordnung, oder Ablehnung |
| 2 Artefakt | `atc-vm` | ATCB-v1-Container, SHA-256 ueber Kopf und Operationsstrom |
| 3 Freigabe | `atc-exec-admission` + `atc-system` | Entscheidung mit echter ECDSA-Signatur |
| 4 Ausfuehrung | `atc-vm` | Zustandsuebergang mit Gas, transaktional |
| 5 Buchung | `atc-node` | Block an die Kette, `Chain::verify` gruen |

Die Systemkomponente implementiert **weder eine zweite VM noch einen zweiten
Compiler**. Sie uebersetzt (das ist eine Abbildung, kein Interpreter), signiert,
prueft, reicht den Zustand durch und bucht.

## 3 — Die fuenf Gates

### A — gleiche Semantik in beiden Interpreten

`tests/differential.rs` vergleicht den Referenz-Interpreter (`atc_core::vm::execute_verified`
ueber ein verifiziertes ATCA-Artefakt) mit dem Kettenpfad. Jeder tragende Fall hat einen
**von Hand gerechneten** Erwartungswert; verglichen wird gegen beide Wege:

Arithmetik und Praezedenz (7, 19, 21, 14), Schleife (120, Nie-Lauf 0), alle sechs
Vergleiche in beiden Richtungen (10 Faelle), if/else (2, 2, 1), Zustandslesen (1),
eingebetteter Korpus mit sechs tragenden Faellen (14, 22, 16, 42, 1, 1).

**Befund, der den Vergleich erst ehrlich macht:** Ein Programm ohne eigenes `return`
(Top-Level-Form `let r = ...;`) liefert im Referenzweg die implizite Rueckgabe von
`__main__`, und die ist konstant 0. Ein Vergleich solcher Programme beweist nichts.
Der ausgelieferte Differential-Korpus (18 Dateien) ist genau so gebaut: 13 Dateien sind
uebersetzbar, 5 liegen ausserhalb des Subsets, und **keine** liefert ein echtes Ergebnis.
Der Korpus ist damit eine Breiten-, keine Werteprobe. Er wird als solcher gefahren und
als solcher ausgewiesen, nicht als Beweis verkauft.

### B — die Abbildung ist strikt

`tests/backend.rs`, 13 Faelle. Abgelehnt werden: `call` (keine Aufrufkonvention),
`Neg`, `Pop`, negative Konstanten (`u64`-Stapel), Sprung hinter das Programmende,
Ruecksprung vor den Programmanfang, leeres Programm, Programme ueber 65 536 Operationen.
Sprungziele werden absolut umgerechnet; die PC-Zuordnung zeigt immer auf den **Anfang**
einer mehrteiligen Erweiterung (`!=` wird zu `Eq, Push(1), Swap, Sub`, `<=` zu fuenf
Operationen). Ein Sprung an das Programmende wird auf einen echten Abschluss-`Halt`
gelegt, nicht auf ein Ziel ausserhalb.

### C — Signaturpruefung

`tests/signature.rs`, 11 Faelle. Signiert wird mit secp256k1-ECDSA ueber den SHA-256 der
kanonischen Policy-Bytes (`ATC-EXEC-ADM-v1\0`, Laengenpraefixe). Abgelehnt werden:
fehlende Signatur, Nullsignatur, zu kurze Signatur, DER-kodierte Signatur, fremder
Schluessel, geanderte Policy-Bytes, fehlende Domain-Trennung, nicht kanonische
Policy-Bytes, abweichender beobachteter Hash, fremde Komponente, zu viele Rechte.

**Getrennt gehalten:** eine gueltige Signatur beweist die Kontrolle ueber einen
Schluessel, nicht die Erlaubnis. Ein eigener Test signiert korrekt und wird trotzdem
abgelehnt, weil der Signierer in der Authority nicht autorisiert ist.

### D — Gas 0 blockiert wirklich

`tests/e2e.rs`. Geprueft wird nicht der Fehler, sondern die Folgen: Zustand byte-gleich,
Kettenhoehe unveraendert, kein Block. Zusaetzlich wird das **minimale** Budget durch
Aufsteigen gesucht und dann nachgewiesen, dass `minimal - 1` nichts ausfuehrt und
`minimal` durchlaeuft. Es gibt genau einen Ausfuehrungseinstieg im System; einen
alternativen Pfad gibt es nicht.

### E — Zustandsintegration ohne Seiteneffekte

- Generation kommt ausschliesslich aus dem Artefakt. Neuer Einstieg
  `Vm::from_artifact_with_context(artifact, caller, storage)` hat **keinen**
  Generationsparameter; ein Test haelt fest, dass sie aus dem Artefakt kommt und der
  Zustand trotzdem gesetzt wird.
- Zwei frische Systeme mit gleicher Quelle, gleichem Zustand und gleichem Gas erzeugen
  **dieselbe Zustandswurzel und denselben Blockhash** (Determinismus).
- Zustand wird durchgereicht: `state_root_after` des ersten Laufs ist
  `state_root_before` des zweiten.
- Die Zustandswurzel bindet Code und Zustand: zwei verschiedene Vertraege mit gleichem
  Ergebnis haben verschiedene Wurzeln.
- Fehler sind atomar: bei Ablehnung bleibt der Aufruferzustand unveraendert, es entsteht
  kein Block, und jede Entscheidung schreibt genau einen Audit-Eintrag (auch die
  Ablehnung).

## 3a — Evidenzstufen (was ein gruener Test wirklich sagt)

Vier Aussagen, die nicht verwechselt werden duerfen. Jede wird hier getrennt gefuehrt:

| Stufe | Aussage | Beleg in diesem Paket |
|---|---|---|
| E1 | Ausfuehrung erfolgreich | `atc-system demo` endet mit Exit 0, Kette verifiziert |
| E2 | Ergebnis reproduzierbar | zwei frische Systeme erzeugen gleiche Zustandswurzel und gleichen Blockhash |
| E3 | Rechenregel tatsaechlich geprueft | Faelle mit **von Hand gerechnetem** Erwartungswert in beiden Wegen (7, 19, 21, 14, 120, 16, 22, 42; alle sechs Vergleiche in beiden Richtungen) |
| E4 | Semantik beider Interpreter gleich | `tests/differential.rs` gegen den Referenz-Interpreter, inklusive der beiden bekannten Abweichungen |

**Wichtig fuer den Korpus:** Ein Programm ohne eigenes `return` liefert im Referenzweg die
implizite Rueckgabe von `__main__`, und die ist konstant 0. Ein Vergleich solcher
Programme belegt E1 und E2, **nicht** E3 und **nicht** E4. Der ausgelieferte Korpus
(18 Dateien, 13 uebersetzbar, 5 ausserhalb des Subsets) ist genau so gebaut und liefert
durchgehend 0: er ist eine Breitenprobe. Deshalb tragen die Tests mit
Erwartungswert und der eingebettete Korpus (sechs Faelle) die Aussagen E3 und E4, und der
Datei-Korpus steht daneben als Deckungsnachweis ohne eigene Beweiskraft.

**Nicht durch lokale Tests belegbar:** `CI_VERIFIED` (kein Actions-Lauf) und
`E2E_VERIFIED` ueber mehrere Validatoren (es gibt einen Prozess). Beide bleiben offen.

## 3b — Ganzzahl-Semantik

Die Divergenz bei Ueberlauf und Vorzeichen ist in Form von Tests festgehalten und
maschinenlesbar markiert. Der Entwurf der verbindlichen Fassung (Bitbreite, Signedness,
Ueberlauf, Division und Rest, Fehlerverhalten an Grenzwerten, Testmatrix) liegt als
[`docs/decisions/VM-INTEGER-SEMANTICS-DRAFT.md`](../decisions/VM-INTEGER-SEMANTICS-DRAFT.md).
Er entscheidet nichts und aendert keinen Code; die Reihenfolge fuer die Umsetzung steht
darin.

## 4 — Messwerte (11.11.2026, lokaler Baum)

- `cargo test --workspace`: **662 bestanden, 0 fehlgeschlagen**.
- `cargo test -p atc-system`: **46** Tests in vier Suiten (13 Backend, 11 Differential,
  11 E2E, 11 Signatur), 0 fehlgeschlagen.
- `cargo test -p atc-vm`: **132** Tests in fuenf Suiten.
- `cargo clippy -p atc-system --all-targets --no-deps -- -D warnings`: 0.
- `rustfmt --check`: 0.
- Beispiel-Durchlauf `atc-system demo`: 19 ATCLang-Instruktionen → 20 ATCB-Operationen,
  138-Byte-Artefakt, Code-Hash `5f2c38c8…`, Ergebnis 10, Blockhoehe 2, `Chain::verify` ok.

## 5 — Offene Punkte (bewusst nicht versteckt)

1. **Ueberlauf und Vorzeichen.** `atc-core` rechnet `i64` geprueft, `atc-vm` `u64`
   umlaufend. Ein Ueberlauf, den die Referenz ablehnt, laeuft in der VM weiter; negative
   Zwischenwerte kehren den Vergleich um. Beide Faelle sind als Test dokumentiert und
   maschinenlesbar in `DIVERGENZEN`. Die Loesung ist ein VM-Befehl mit Ueberlauf-Abbruch
   beziehungsweise ein Vorzeichen-Vergleich, also eine Entscheidung am Vertrag, keine
   Eigenschaft des Backends.
2. **Keine Aufrufkonvention.** `call` wird abgelehnt, nur die Einstiegsfunktion wird
   uebersetzt. Weitere Funktionen sind unerreichbar, nicht stumm weggelassen.
3. **Speichermodell.** Lokale Variablen werden auf Speicherplaetze abgebildet. Ein
   Vertrag, der `let x = 0;` schreibt, ueberschreibt damit den Zustand; persistent lesbar
   ist nur, was vor dem ersten Schreiben gelesen wird (`let s = s + 1;`). Das ist die
   im Lebenszyklus-Standard offene Entscheidung, nicht deren Beantwortung.
4. **Kein Nonce.** Die Freigabe ist an Domain, Code-Hash und Ablauf gebunden, aber es gibt
   keinen Aufrufkontext mit Nonce. Eine Ausfuehrung ist innerhalb der Gueltigkeit
   wiederholbar (das ist fuer Zustandsuebergaenge gewollt); ein Replay-Schutz fuer
   Transaktionen existiert hier nicht.
5. **Schluesselwahl.** Umgesetzt ist secp256k1, weil `atc-wallet` (WAL-SIGN-001) das so
   macht. Die Entscheidung Ed25519 gegen secp256k1 in ATC-SEC-EXEC-001 ist offen.
6. **Keine Ereignisse, kein Gasverbrauch im Beleg.** Die VM meldet verbrauchtes Gas bei
   Erfolg nicht zurueck (bekannter Befund GAS-AUD-005); das Receipt traegt deshalb nur
   das Budget, nicht den Verbrauch.
7. **Kein Widerruf ausserhalb des Prozesses.** Widerruf, Obergrenzen und Ablauf leben im
   Speicher des Systems; es gibt keine persistente oder verkettete Authority-Liste.
8. **CI.** Kein Nachweis. `git push` scheitert weiter an der unbetaetigten
   Committer-Mail; Uebertragung erfolgt ueber die API. Damit gibt es fuer diesen Stand
   **keine** CI-Evidenz, unabhaengig davon, wie gruen der lokale Lauf ist.

## 6 — Status

| Bereich | Status |
|---|---|
| Integrationspfad | IMPLEMENTED |
| Lokale Tests (46 + 662) | TESTED |
| Gate A Differential | NACHGEWIESEN, mit benannter Einschraenkung (Korpus) |
| Gate B Striktheit | NACHGEWIESEN |
| Gate C Signatur | NACHGEWIESEN, Nonce offen |
| Gate D Gas 0 | NACHGEWIESEN |
| Gate E Zustand | NACHGEWIESEN |
| Divergenz Ueberlauf/Vorzeichen | OFFEN (VM-Vertrag) |
| Aufrufkonvention | OFFEN |
| Speichermodell | OFFEN (Lebenszyklus-Standard) |
| CI_VERIFIED | OFFEN |
| E2E_VERIFIED | OFFEN (ein Prozess, keine zwei Validatoren) |
| VERIFIED | NICHT ERREICHT |
