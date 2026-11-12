# P1d-B2 — Chain- und Zustandsautorität: die vier Architekturentscheidungen

> **Status:** `ENTSCHEIDUNGEN GETROFFEN` (Owner, 12.11.2026). **Keine Implementierung.**
> **P1d-C ist nicht freigegeben.** Die Entscheidungen sind getroffen, aber nicht
> spezifiziert: Datenmodell, Zustandswurzel-Komposition, Commit-Log-Format und
> Recovery-Semantik müssen erst feststehen (§7). Ein getroffener Beschluss ist kein
> Nachweis.
> **Basis:** `d97b476` (P1d-B1-Inventur). Dieser Schritt ändert keinen Code und
> behauptet deshalb keine neuen Ausführungsergebnisse (§10).
> **Register:** `docs/decisions/exec-decision-register.yaml`, Block `chain_state`
> (AD-B1-01 bis AD-B1-04, Spezifikationsgates B2-S1 bis B2-S5, offene Punkte B2-O1
> bis B2-O4). Der folgende Block wird aus dieser Datei erzeugt.

## Registerstand (erzeugt)

<!-- BEGIN CHAIN-STATE-DECISIONS (erzeugt aus docs/decisions/exec-decision-register.yaml, nicht von Hand aendern) -->

**Registerstand P1d-B2** (Owner, 2026-11-12; Basis `d97b476 (P1d-B1 Inventur, keine Codeaenderung)`; Quelle: `docs/decisions/exec-decision-register.yaml`)

**ATC-ATOMIC-STATE-TRANSITION — ATC Atomic State Transition Rule**

> Kein Systemzustand darf dauerhaft wirksam werden, bevor der zugehoerige autoritative Commit erfolgreich abgeschlossen wurde. Jeder Commit ist an eine eindeutig identifizierte Ausgangsversion gebunden. Chain, konsensrelevanter Zustand und ausfuehrungsrelevante Policy repraesentieren nach einem erfolgreichen Commit dieselbe bestaetigte Version.

Damit sind fuenf Vorgaenge getrennt: Ausfuehrung, Vorbereitung, Validierung, dauerhafte Uebernahme, Wiederherstellung.

| ID | Gegenstand | Entscheidung | Owner-Entscheidung | Status | Phase |
|---|---|---|---|---|---|
| AD-B1-01 | Eigentuemer der Chain | Kompositionswurzel | freigegeben | DECIDED | P1d-D |
| AD-B1-02 | Persistenzmodell des Zustands | Append-only Commit-Log als einzige Autoritaet, Snapshot je Hoehe als abgeleiteter Beschleuniger | freigegeben | DECIDED | P1d-C |
| AD-B1-03 | Inhalt der Zustandswurzel | Vollstaendiger konsensrelevanter Zustand; Leerwert kanonisch; Komposition statt Einzelwurzel | freigegeben | DECIDED | P1d-C |
| AD-B1-04 | Policy-Registrierung ist Teil des Deployment-Commits | Atomarer Deployment-Commit; vorgelagerte Registrierung nur unverbindlich, idempotent und ohne wirksame Berechtigung | freigegeben | DECIDED | P1d-C |

**Commit-Vorbedingung** (Owner-Vorgabe, keine fuenfte Entscheidung, sondern die Bedingung, ohne die AD-B1-01 bis AD-B1-04 nicht pruefbar waeren): `ExpectedParentHeight`, `ExpectedParentHash`, `ExpectedStateRoot`, `ExpectedStateVersion`.

> Der Commit wird nur ausgefuehrt, wenn alle vier Werte noch dem autoritativen Ausgangszustand entsprechen. Bei Abweichung gilt COMMIT_REJECTED mit STATE_UNCHANGED, CHAIN_UNCHANGED und POLICY_UNCHANGED.

**Spezifikationsgates** (muessen vor P1d-C geschlossen sein; ein Gate ist geschlossen, wenn seine Abnahme belegt ist, nicht wenn sie beschlossen ist)

| ID | Spezifikation | sperrt | Abnahme |
|---|---|---|---|
| B2-S1 | Zustandsdatenmodell und kanonische Serialisierung | P1d-C | Gleicher logischer Zustand ergibt byte-identische Eingabe in die Wurzel; Testvektoren, die von zwei Implementierungen unabhaengig reproduziert werden |
| B2-S2 | Komposition der Zustandswurzel | P1d-C | Domaene, Bereichswurzeln, Reihenfolge und Leerwerte festgelegt; Aenderung eines aufgenommenen Bereichs aendert die Wurzel, Aenderung eines ausgeschlossenen Bereichs nicht |
| B2-S3 | Commit-Log: Datensatzformat, Dauerhaftigkeit, ein Schreiber | P1d-C | Feldliste je Datensatz, Dauerhaftigkeitsregel und Schreibhoheit festgelegt; abgeschnittener Schwanz wird verworfen, nicht akzeptiert; Luecke in state_version entwertet den Schwanz |
| B2-S4 | Recovery- und Integritaetssemantik | P1d-C | Neustart stellt (hoehe, best_hash, state_root, state_version) allein aus dem Log her; Snapshotverlust ohne Wirkung; Snapshot-Log-Widerspruch wird erkannt und zugunsten des Logs entschieden |
| B2-S5 | Ausfuehrungsnachweis: Ergebnis gebunden an Parent-Hash und Parent-State-Root | P1d-C | Ein Ergebnis, das auf einem anderen Parent gerechnet wurde, wird von der Kompositionswurzel allein abgewiesen, ohne Nachrechnen der Ausfuehrung |

**Offen und hier nicht entschieden:**

- B2-O1 Authority-Pfade (Widerruf, Obergrenzen, Signiererautorisierung) schreiben heute ohne Block; ob sie derselben Regel unterliegen, ist nicht entschieden
- B2-O2 Dauerhafte Kette gegen Devnet-Grenze: produce lehnt ab 64 Bloecken ab; der Wert ist eine Entwicklungsgrenze und braucht fuer eine persistente Kette eine eigene Begruendung
- B2-O3 Format des Deployment-Blocks: nach D0-1 transportiert der Payload den ATCE-Container; Festlegung erfolgt mit D0-1/P1a, nicht hier
- B2-O4 Speichermedium des Commit-Logs (Datei, KV, Ledger): Semantikfrage entschieden, Technologie offen

Die vier Entscheidungen sind **getroffen**, nicht umgesetzt: sie aendern keinen Code, vergeben kein `CI_VERIFIED` und oeffnen P1d-C erst, wenn die Spezifikationsgates belegt sind.

<!-- END CHAIN-STATE-DECISIONS -->

## 0 Phasenordnung ab 12.11.2026 (löst die Zuordnung der B1-Inventur ab)

| Phase | Ergebnis | Freigabebedingung | vorher geführt als |
|---|---|---|---|
| **B1** | Ownership-Inventur | abgeschlossen (`d97b476`) | B1 (Teil 1) |
| **B2** | Architekturentscheidungen (dieses Dokument) | vier Entscheidungen dokumentiert | B2 („State-Root-Vorbedingung") |
| **C** | Persistenz und atomarer Commit | kein partieller Commit | B2 (Teil 2) + C |
| **D** | Runtime-Komposition | eine Chain-Autorität | D |
| **E** | `import_tx_block` | kein alternativer Sicherheitsweg | E |
| **F** | E2E-Sicherheitsaudit | Gesamtsystem nachgewiesen | F |

Die Verschiebung ist beabsichtigt und wird hier festgehalten, damit die Phasen nicht
nachträglich ihre Bedeutung wechseln:

* **B2 ist jetzt eine reine Entscheidungsphase.** In der B1-Inventur war B2 noch als
  „State-Root-Vorbedingung" mit zwei Abnahmetests geführt. Eine Vorbedingung ist
  jedoch ohne Commit nicht prüfbar; die beiden Tests stehen deshalb ab jetzt in **C**
  (§9). Die Sache selbst ist nicht verschwunden, sie ist Vorbedingung des Commits.
* **„Eine Chain-Autorität" wandert von B1 nach D.** B1 hat die Doppelung nur
  nachgewiesen; das Herauslösen der Kette aus `System` ist ein Umbau an der
  Komposition und gehört damit zu D.
* **B2 und C bleiben getrennt.** Erst Datenmodell, Zustandswurzel, Commit-Protokoll
  und Recovery-Semantik, dann Implementierung.

## 1 Die übergeordnete Regel: ATC Atomic State Transition Rule

> Kein Systemzustand darf dauerhaft wirksam werden, bevor der zugehörige autoritative
> Commit erfolgreich abgeschlossen wurde. Jeder Commit ist an eine eindeutig
> identifizierte Ausgangsversion gebunden. Chain, konsensrelevanter Zustand und
> ausführungsrelevante Policy repräsentieren nach einem erfolgreichen Commit dieselbe
> bestätigte Version.

Die Regel trennt fünf Vorgänge, die heute im selben Funktionsaufruf liegen:
**Ausführung** (`System::execute`), **Vorbereitung** (Ergebnis gebunden an die
Ausgangsversion), **Validierung** (vier erwartete Werte, §6), **dauerhafte Übernahme**
(ein Commit-Punkt) und **Wiederherstellung** (Log → Zustand, §3).

Drei Folgerungen, die aus der Regel unmittelbar gelten und nicht zur Disposition
stehen:

1. **Kein Teilerfolg.** Ein Block ohne Zustand und ein Zustand ohne Block sind beide
   unzulässig, unabhängig davon, in welcher Reihenfolge geschrieben wird.
2. **Kein „nachträgliches Geradeziehen".** Ein späterer Vorgang darf einen früheren
   nicht reparieren; ein fehlgeschlagener Commit hinterlässt nichts, was aufgeräumt
   werden müsste.
3. **Keine Wirksamkeit vor dem Commit.** Das gilt ausdrücklich auch für
   Berechtigungen (§5), nicht nur für Daten.

## 2 AD-B1-01 — Eigentümer der Chain: die Kompositionswurzel

**Entscheidung.** Genau **eine** Chain-Instanz. Sie gehört der Kompositionswurzel
(Runtime), nicht `System`. `System` erzeugt Ausführungsergebnisse und liest die Kette;
es schreibt sie nicht.

**Was heute dagegen steht** (belegt an `d97b476`):

* `System` hält die Kette als privates Feld (`components/system/src/system.rs:184`) und
  produziert Blöcke direkt in der Ausführung (`self.chain.produce`, `:478`).
* `DevnetNode` hält eine **zweite** Kette (`components/system/src/node_import.rs:69`,
  aufgebaut in `start`, `:77`). Beide Bestände leben im selben Prozess.
* `Chain::produce` verändert den Bestand sofort (`components/node/src/chain.rs:117`).
  Ein Aufrufer, der danach scheitert, hat bereits einen Block erzeugt.

**Abgrenzung, die zum Beschluss gehört.** Eine **gemeinsame Referenz auf dieselbe
Instanz** ist die Lösung. Zwei Objekte, die regelmäßig miteinander abgeglichen werden,
sind **keine** Lösung: sie sind eine zweite Wahrheit mit zusätzlichem Aufwand und
driften genau dann, wenn der Abgleich ausfällt. Deshalb ist „Sync zwischen zwei Ketten"
verworfen, nicht nur „zwei Ketten".

**Folgen.**

1. `System` verliert `chain`; die Blockproduktion verlässt `execute`.
2. `DevnetNode.chain` entfällt als Dublette, weil es die einzige Kette **ist**.
3. Der Zugriff auf die Kette wird zu einer kontrollierten Schnittstelle der
   Kompositionswurzel: lesen erlaubt, anhängen nur über den Commit (§6).

**Abnahme (Phase D).** Kein Pfad außerhalb der Kompositionswurzel kann den Bestand
verändern; das ist strukturell zu prüfen (privates Feld, ein Schreibweg), nicht durch
Konvention.

## 3 AD-B1-02 — Persistenzmodell: Log ist Autorität, Snapshot ist Beschleuniger

**Entscheidung.** Ein **append-only Commit-Log** ist die maßgebliche Wahrheit. Ein
**Snapshot je bestätigter Höhe** ist ein abgeleiteter Wiederherstellungsbeschleuniger
und **niemals** eine zweite Autorität.

**Was heute fehlt.** Es gibt keinen Zustandsspeicher. `ContractState` lebt im Aufrufer,
`*state = after` (`system.rs:480`) ist die einzige Übernahme; die Kette hält ihre
Blöcke in einem `Vec` im Arbeitsspeicher. Damit ist „Neustart stellt einen konsistenten
Zustand her" heute nicht definierbar, weil es kein dauerhaftes Ziel gibt.

**Regeln, die aus der Entscheidung folgen.**

| Regel | Inhalt |
|---|---|
| Autorität | Nur das Log entscheidet über (Höhe, Best-Hash, Zustandswurzel, Zustandsversion) |
| Snapshot | jederzeit löschbar, ohne Informationsverlust; bei Widerspruch gewinnt das Log, der Snapshot wird verworfen und neu gebaut |
| Reihenfolge | Ein Snapshot wird **nach** dem Log-Datensatz geschrieben; ein Fehler dabei ist kein Commit-Fehler |
| Wiederanlauf | Das Log allein muss den Zustand rekonstruieren; der Snapshot darf nur abkürzen |
| Ein Schreiber | Ein Commit-Log hat genau einen Schreiber; Nebenläufigkeit wird über die Ausgangsversion (§6) aufgelöst, nicht über gleichzeitige Schreibzugriffe |

**Verworfen:** nur Snapshots ohne Log (kein Wiederanlaufnachweis, keine Reihenfolge);
dauerhaft nur Log ohne Snapshot als Endzustand (als erster Schritt zulässig, aber ohne
Wiederanlaufbeschleunigung).

**Folge für die Kette selbst.** Mit diesem Beschluss wird auch der Blockbestand
persistiert, weil er aus demselben Log stammt. Das ist der Punkt, an dem die
Doppelung aus AD-B1-01 praktisch verschwindet: es gibt dann eine Kette, ein Log und
einen Commit-Punkt.

## 4 AD-B1-03 — Inhalt der Zustandswurzel: vollständiger konsensrelevanter Zustand

**Entscheidung.** Die Zustandswurzel deckt den **vollständigen konsensrelevanten
Zustand**. Aufgenommen wird, was deterministische Ausführung und die Gültigkeit
künftiger Blöcke beeinflusst; ausgeschlossen wird, was nur beobachtet.

| Datenbereich | Aufnahme | Stand heute (belegt am Code, `d97b476`) |
|---|---|---|
| Kontostände | ja | **nicht vorhanden**: kein Konto- oder Bestandszustand im System- oder Knotenpfad (Grep nach `balance`/`account`: nur `components/interop`-Lockbox und Mining-Nonce, beide außerhalb des Konsenspfads) |
| Nonces / Replay-Schutz | ja | **vorhanden, aber nur im Speicher**: `TxLedger.expected_nonce` (`components/node/src/mempool.rs:152`, Prüfung `:193-220`); nicht persistiert, in keiner Wurzel |
| Contract Storage | ja | **vorhanden**: `ContractState.words` (`components/system/src/state.rs:19`), Wurzel über Domäne, `code_hash`, Wortzahl und Wörter (`:43`) |
| Deployment- und Berechtigungsregister | ja, sofern ausführungsrelevant | **vorhanden, ohne Wurzel**: `Registry` (`system.rs:185`), Freigabeentscheidung in `execute` |
| Konsensrelevante Registry | ja | **nicht vorhanden** als Zustand; die Genesis bindet die Kette über `boot_hash`, ist aber Identität, kein Zustand |
| Transaktionsrelevante Gebührenzustände | ja | **nicht vorhanden**: Gas ist heute Aufrufparameter (`gas_limit`), kein Zustand |
| Temporäre Caches | nein | — |
| Reine Debug-Informationen | nein | — |
| Nicht konsensrelevante Audit-Metadaten | nein | `Authority.log()` (`system.rs:186`) ist Protokoll, kein Konsenszustand |

**Vier Zusatzregeln, ohne die die Entscheidung nicht prüfbar wäre.**

1. **Kanonischer Leerwert.** Jeder aufgenommene Bereich braucht einen definierten
   Leerwert. Fehlt er, hashen zwei Knoten denselben logischen Zustand verschieden
   („nicht vorhanden" gegen „leer").
2. **Komposition statt Einzelwurzel.** Heute ist die Wurzel die Wurzel **eines**
   Vertrags: `state_root(&code_hash)` (`state.rs:43`) mischt den Code-Hash in den
   Wurzelwert. Diese Funktion ist **nicht** als Systemwurzel wiederverwendbar; eine
   Systemwurzel ist eine Komposition über Bereiche (je Vertrag eine Bereichswurzel).
   Dieselbe Funktion mit zwei Bedeutungen wäre eine zweite Wahrheit.
3. **Eigene Domäne.** Die Systemwurzel erhält eine eigene Domänentrennung, getrennt von
   `ATC-STATE-V1` und von `artifact_hash`/`code_hash` (S1-1: drei Identitäten, keine
   vierte, die eine der drei ersetzt).
4. **Aktivierungsregel.** Ein Datenbereich, der neu in den Konsenspfad kommt, **muss**
   in die Wurzel aufgenommen werden. Prüfbar ist das als Test: eine Änderung des
   Bereichs ändert die Wurzel, eine Änderung außerhalb nicht (B2-S2).

**Was die Wurzel heute behauptet, ohne es zu binden.** Der Block-Payload enthält die
Zustandswurzel als Text (`system.rs:473-476`: `exec:<8 Byte code_hash>:<state_root>`).
Damit behauptet der Block eine Wurzel, gegen die nichts geprüft wird. Nach C gehört die
Wurzel in den Commit-Datensatz; der Payload bleibt Transportmittel.

**Nicht entschieden.** Die genaue Datenstruktur und die kanonische Serialisierung
werden **separat** spezifiziert (B2-S1, B2-S2). Ich lege sie hier bewusst nicht fest:
ohne Spezifikation wäre jede Implementierung ein Vorschlag, der sich später als
inkompatibel erweist.

## 5 AD-B1-04 — Policy-Registrierung ist Teil des Deployment-Commits

**Entscheidung.** Die verbindliche Policy-Registrierung ist Teil **derselben logischen
Transaktion** wie der Deployment-Commit. Eine vorgelagerte Registrierung ist nur
zulässig, wenn sie ausdrücklich **unverbindlich** ist, idempotent bleibt und bei
fehlgeschlagenem Deployment **keine wirksame Berechtigung** hinterlässt.

**Zwei Fragen, die getrennt bleiben müssen.**

| Frage | Inhalt | Wirkung |
|---|---|---|
| Autorisierungsentscheidung | darf dieser Deployment-Vorgang stattfinden? | Prüfung, kein Dauerzustand |
| Policy-Registrierung | ab wann gilt die Entscheidung dauerhaft? | Zustand, commit-pflichtig |

Die heutige Implementierung beantwortet nur die erste Frage dauerhaft: `deploy`
registriert (`system.rs:295`) und im Container-Pfad (`:357`), **ohne Block**. Ein
Deployment, das nie ausgeführt wird, hinterlässt damit eine wirksame Policy. Das ist
derselbe Fehler wie ein Zustand ohne Block, nur ohne Chain-Bezug.

**Folgen.**

1. Ein Deployment wird ein **Chain-Ereignis**: Block, Zustand und Policy in einem
   Commit. Der Payload transportiert nach D0-1 den `ATCE`-Container; kein eigenes
   Deployment-Payloadformat (B2-O3).
2. **Idempotenz ist Voraussetzung, nicht Beiwerk.** `Registry::register` vergleicht
   heute byteweise und ist bei Gleichheit idempotent
   (`components/exec-admission/src/lib.rs:253-273`, S1-2-F1). Diese Eigenschaft macht
   die vorgelagerte Registrierung überhaupt vertretbar.
3. **Audit-Semantik.** Ein Audit-Eintrag über einen fehlgeschlagenen Versuch darf
   bestehen bleiben, muss aber als **Versuch ohne Commit** erkennbar sein und darf
   keinen erfolgreichen Commit suggerieren. Ein Audit-Eintrag ist niemals der Nachweis
   einer Berechtigung; die Zulassung entscheidet sich am Register, nicht am Protokoll.

**Verworfen:** die vorgelagerte, sofort wirksame Registrierung (heutiger Zustand) — sie
erzeugt eine Berechtigung ohne Block und verletzt die Regel aus §1 Punkt 3.

## 6 Die Commit-Vorbedingung (Owner-Vorgabe, hier als Vertrag festgehalten)

Keine fünfte Entscheidung, sondern die Bedingung, ohne die AD-B1-01 bis AD-B1-04 nicht
prüfbar wären.

```text
Leseversion (von der Kompositionswurzel, einmal, vor der Ausführung):
    ExpectedParentHeight
    ExpectedParentHash
    ExpectedStateRoot
    ExpectedStateVersion

Ausführung auf isoliertem Zustand (Kopie, kein Schreibzugriff auf das Original)
Ergebnis trägt die gelesene Ausgangsversion mit sich

Commit-Versuch:
    erneut lesen; alle vier Werte vergleichen
    ungleich  -> COMMIT_REJECTED, STATE_UNCHANGED, CHAIN_UNCHANGED, POLICY_UNCHANGED
    gleich    -> ein Commit-Punkt: Log-Datensatz (Block + Wurzel + Version)
```

**Bindung des Ergebnisses.** `System` darf keinen beliebigen `after`-Zustand liefern.
Das Ergebnis muss an Parent-Hash und Parent-State-Root **gebunden** sein, so wie es sie
gelesen hat. Die Kompositionswurzel prüft diese Bindung und entscheidet allein über den
Commit; sie rechnet die Ausführung nicht nach (B2-S5).

**Was `state_version` ist.** Ein monoton wachsender Zähler der committeten
Zustandsübergänge. Er steht im Log-Datensatz und muss **sprung- und lückenlos** sein;
eine Lücke entwertet den Log-Schwanz. `state_version` ist nicht dasselbe wie die Höhe:
Höhe zählt Blöcke, `state_version` zählt Zustandsübergänge. Solange jeder Block genau
einen Übergang committet, sind sie gleich — die Trennung ist trotzdem nötig, damit die
beiden Begriffe nicht stillschweigend zusammenfallen.

**Was der Commit-Punkt ist.** Das vollständige Anhängen des Log-Datensatzes. Ein
teilweise geschriebener Datensatz ist **kein** Commit; er wird beim Wiederanlauf
verworfen, nicht repariert (B2-S3 legt fest, woran „vollständig" erkennbar ist).

**Warum vier Werte und nicht zwei.** Höhe und Hash allein lassen zwei Aufrufer zu, die
auf demselben Block mit verschiedenem Zustand rechnen. `state_root` allein lässt einen
Aufrufer zu, der auf einem älteren Block mit zufällig gleicher Wurzel rechnet.
Die Version allein sagt nichts über den Inhalt. Erst die vier Werte zusammen binden
Ausführung, Zustand und Kettenposition.

## 7 Was vor P1d-C feststehen muss (Spezifikationsgates)

| ID | Spezifikation | Abnahme |
|---|---|---|
| B2-S1 | Zustandsdatenmodell und kanonische Serialisierung | gleicher logischer Zustand ergibt byte-identische Eingabe; Testvektoren, die von zwei Implementierungen unabhängig reproduziert werden |
| B2-S2 | Komposition der Zustandswurzel | Domäne, Bereichswurzeln, Reihenfolge, Leerwerte festgelegt; Änderung eines aufgenommenen Bereichs ändert die Wurzel, eine Änderung außerhalb nicht |
| B2-S3 | Commit-Log: Datensatzformat, Dauerhaftigkeit, ein Schreiber | Feldliste, Dauerhaftigkeitsregel, Schreibhoheit; abgeschnittener Schwanz wird verworfen, nicht akzeptiert; Lücke in `state_version` entwertet den Schwanz |
| B2-S4 | Recovery- und Integritätssemantik | Neustart stellt (Höhe, Best-Hash, Wurzel, Version) allein aus dem Log her; Snapshotverlust ohne Wirkung; Snapshot-Log-Widerspruch wird erkannt und zugunsten des Logs entschieden |
| B2-S5 | Ausführungsnachweis: Ergebnis gebunden an Parent-Hash und Parent-State-Root | ein auf einem anderen Parent gerechnetes Ergebnis wird von der Kompositionswurzel allein abgewiesen, ohne Nachrechnen |

Ein Gate ist geschlossen, wenn seine Abnahme **belegt** ist, nicht wenn sie beschlossen
ist. Bis dahin bleibt P1d-C gesperrt, auch wenn alle vier Entscheidungen vorliegen.

## 8 Was ich hier nicht entschieden habe

| ID | Offener Punkt | Warum nicht hier |
|---|---|---|
| B2-O1 | Authority-Pfade (Widerruf, Obergrenzen, Signiererautorisierung) schreiben heute ohne Block | eigener Zuständigkeitsbereich; die Regel aus §1 würde gelten, aber Umfang und Reihenfolge sind nicht beschlossen |
| B2-O2 | Dauerhafte Kette gegen die Devnet-Grenze: `produce` lehnt ab 64 Blöcken ab (`chain.rs:103`) | der Wert ist eine Entwicklungsgrenze; ein Grenzwert für eine persistente Kette braucht eine eigene Begründung und einen Grenzwerttest, er darf nicht aus der bestehenden Grenze abgeleitet werden |
| B2-O3 | Format des Deployment-Blocks | gehört zu D0-1/P1a: der Payload transportiert den `ATCE`-Container, kein zweites Payloadformat |
| B2-O4 | Speichermedium des Commit-Logs (Datei, KV, Ledger) | Semantik ist entschieden, Technologie ist eine Implementierungsfrage in C |

## 9 Abnahmetests, neu zugeordnet

| Test | Phase | Anmerkung |
|---|---|---|
| veralteter `state_root` wird abgewiesen | C | aus B2 verschoben: braucht einen Commit, den es noch nicht gibt |
| falscher Ausgangs-Blockhash wird abgewiesen | C | dito |
| veraltete `state_version` wird abgewiesen | C | neu aus §6 |
| zwei parallele Aufrufer mit verschiedenem Zustand: höchstens einer committet | C | |
| Fehler **nach** der VM-Ausführung verändert keinen dauerhaften Zustand | C | |
| Fehler **während** des Commits verändert keinen dauerhaften Zustand | C | umfasst den abgeschnittenen Log-Schwanz |
| Neustart nach Commit stellt konsistenten Chain-/State-Zustand her | C | setzt Log + Snapshot voraus (AD-B1-02) |
| wiederholte Ausführung erzeugt keinen doppelten Zustandsübergang | C | |
| Deployment ohne erfolgreichen Commit hinterlässt keine wirksame Policy | C | AD-B1-04 |
| wiederholtes identisches Deployment bleibt idempotent | C | AD-B1-04, `Registry::register` |
| Änderung eines aufgenommenen Bereichs ändert die Wurzel, außerhalb nicht | C | B2-S2 |
| Snapshotverlust ohne Wirkung; Snapshot-Log-Widerspruch zugunsten des Logs | C | AD-B1-02 |
| eine Chain-Instanz, keine Synchronisation zweier Bestände | D | AD-B1-01 |
| `import_tx_block` ohne Port-Bypass | E | |

## 10 Nachweis

```
pytest tools/tests                    47 passed          (vorher 40; +7 für den chain_state-Block)
decision_register.py --check          PASS (D1..D7 + D-0 + S1-2-F1; chain_state AD-B1-01..04)
check_build_coverage                  GATE PASS, 77 Manifeste, 7 ohne Build-Nachweis
cargo test --workspace                nicht erneut ausgeführt: dieser Schritt ändert
                                      keine .rs-Datei; letzter gemessener Stand bei
                                      Basis d97b476 ist 748 grün, 0 rot
```

Die Zeile zu `cargo test` ist bewusst keine Messung dieses Schritts. Ein
Dokumentationsschritt, der Testzahlen mitschreibt, ohne sie zu messen, behauptet
Evidenz, die er nicht hat.

Kein `CI_VERIFIED`, kein `E2E_VERIFIED`. Beide bleiben offen, bis ein Lauf am
Default-Branch-HEAD vorliegt.
