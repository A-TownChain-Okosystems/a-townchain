# Implementierungsplan: Smart-Contract-Ausfuehrung

**Datum:** 2026-11-12
**Stand:** `a-townchain` main `43b194c86`
**Bezug:** `ATC-SMART-CONTRACT-EXECUTION-001` (Entwurf),
`docs/decisions/AD-EXEC-D1-D7.md`, `docs/audits/CONTRACT-EXECUTION-INVENTORY-2026-11-12.md`

**Grundregel:** Kein Code wird geschrieben, bevor die zugehoerige Entscheidung
gefallen ist. Jede Phase hat ein Eingangskriterium (Entscheidung), ein
Ausgangskriterium (Gate) und einen Nachweis. Eine Phase ohne Nachweis gilt als
nicht abgeschlossen, unabhaengig davon, wie viel Code entstanden ist.

## Uebersicht

| Phase | Inhalt | Eingang | Gate |
|---|---|---|---|
| P0 | Inventar, Konflikte, Vertragsentwurf | Audit vom 11.11.2026 | liegt vor |
| P1 | Entscheidungen D1 bis D7 | P0 | erledigt 12.11.2026 (alle DECIDED) |
| P1a | Auflagen der bedingten Freigaben spezifizieren (D2, D4, D6) | P1 | in Arbeit: REQ-SCE-039..050 |
| P1b | Messung und Zahlen (Gaskosten je Instruktionsklasse, Limits) | P1a | offen |
| P2 | Format und Huelle | D1, D7 | SCE-G1, G2, G8 |
| P3 | Aufruf Stufe 1 und Identitaet | D2 (Stufe 1), D6 | SCE-G3 (teil), G5 |
| P4 | Speicher und Zustand | D3 | SCE-G4 |
| P5 | Gas und Gebuehren | D4 | SCE-G6 |
| P6 | Ausfuehrung im Block | D5 | SCE-G7, G10 |
| P7 | Call zwischen Vertraegen, Ereignisse | D2 Stufe 2 nach BC-002 | SCE-G3 (voll) |
| P8 | E2E und CI | P2 bis P7 | G9, CI_VERIFIED, E2E_VERIFIED |

## P0 — Inventar und Konflikte (abgeschlossen)

**Ergebnis.** 20 Implementierungsstellen erfasst, sieben Domaenen mit
konkurrierenden Wahrheiten, 12 Konflikte K-01 bis K-12, jeder mit der
Entscheidung, die ihn aufloest.

**Nachweis.** `docs/audits/CONTRACT-EXECUTION-INVENTORY-2026-11-12.md`.

## P1 — Entscheidungen (Owner) — erledigt am 12.11.2026

**Ergebnis.** Vier Entscheidungen freigegeben (D1, D3, D5, D7), drei bedingt
freigegeben (D2, D4, D6). Alle sieben stehen auf DECIDED. Register:
`docs/decisions/exec-decision-register.yaml`, Tabelle identisch in
`AD-EXEC-D1-D7.md` und im Kandidaten (`tools/decision_register.py --check`).

**Nicht freigegeben und damit nicht Teil von P1:** numerische Gaspreise,
Transaktionsgaslimit, Blockgaslimit, Implementierungs- und CI-Gates.

**P2 ist nicht freigegeben.** Die Sperre gilt, bis P1a und die Konsistenz des
Registers erfuellt sind; P1b (Zahlen) blockiert die Gasphase, nicht den Beginn von P2.

## P1a — Auflagen spezifizieren und schliessen

**Arbeit.** Die Bedingungen der bedingten Freigaben in pruefbare Anforderungen
ueberfuehren und die vier Befunde der Owner-Pruefung vom 12.11.2026 als Korrektur
in die bestehenden Dokumente einarbeiten, nicht als neue Entscheidung daneben
stellen.

### Abschlusskriterien (verbindlich vor dem Schliessen von P1a)

| Punkt | Abschlusskriterium | Stand | Beleg |
|---|---|---|---|
| D1 | SCX-003 und SCE-ARTIFACT verwenden dieselbe Formatdefinition; `ATCB` ist nicht das Magic des VM-Containers | erledigt | Kandidat §5, Klarstellung zu SCX-003 |
| D1 | Magic der Huelle bestaetigt (`ATCE` oder anderer Name) | **offen** | Vorschlag im Kandidaten |
| D2 | 8 192 Byte Calldata, 4 096 Byte Rueckgabe, Tiefe 8 ausdruecklich bestaetigen oder ersetzen | **offen** | Begruendung in REQ-SCE-040, Grenzwerttests T-21 bis T-24 |
| D2 | Adressableitung inklusive Bytebreiten, Serialisierung und Code-Hash eindeutig; Kollisionsbehandlung geregelt | erledigt | REQ-SCE-041, Tabelle 76 Byte, T-27 |
| D2 | Kanonisierung der Signatur eindeutig, inklusive Parametertypen und verschachtelter Typen | erledigt | REQ-SCE-039, T-25, T-26 |
| D4 | Fehlerklassen und Erstattungsregel unterscheiden | Klassen erledigt, Regel **offen** | REQ-SCE-047, REQ-SCE-054, T-28, T-29 |
| D4 | Einheiten von `gas_refund` und Geldbetrag getrennt | erledigt | REQ-SCE-055 |
| D6 | `origin`, `caller` und Vertragsidentitaet getrennt definiert | erledigt | REQ-SCE-051 bis REQ-SCE-053 |
| D6 | `caller`-Semantik bei Vertrag-zu-Vertrag mit ATC-BC-002 abgestimmt | **offen** (haengt an BC-002) | REQ-SCE-052 |
| D5 | Validator-Orchestrierung und VM-Ausfuehrungsverantwortung getrennt | erledigt | Matrix §4, REQ-SCE-056 |
| Alle | Register-Generator und Konsistenzpruefung erfolgreich | erledigt | `tools/decision_register.py --check` PASS |

### Zwei Regeln fuer die Grenzwerte

1. **Keine Ableitung aus bestehenden Grenzen.** Dass ein Wert zu
   `MAX_STACK_ITEMS` oder zu einem Testbudget passt, ist kein Argument. Jeder
   Grenzwert braucht eine eigene Begruendung (Speicher je Frame, Belegroesse,
   Kostenverhaeltnis) und einen Grenzwerttest, der genau an der Kante scheitert.
2. **Ein Wert ohne Test ist ein Vorschlag.** 8 192 / 4 096 Byte und Tiefe 8 bleiben
   als Vorschlag gekennzeichnet, bis T-21 bis T-24 gruen sind.

**Ausgang.** Alle Zeilen der Tabelle auf "erledigt" oder mit benanntem Eigentuemer
und Termin. Erst dann ist P2 freigegeben.

## P1b — Messung und Zahlen

**Arbeit.** Instruktionskosten je Klasse messen, nicht schaetzen. Dazu ein
Messaufbau, der dieselbe Messung zweimal ausfuehrt und die Abweichung ausweist, und
realistische Vertragsbenchmarks statt eines einzelnen Durchsatzwerts.

**Ausgang.** Eine Tabelle mit gemessenen Kosten, ein Transaktionsgaslimit und ein
Blockgaslimit, jedes mit Messprotokoll. Der Referenzwert 100 000 Gas
(`ATC-BC-002-CANDIDATE` §19.5) bleibt Vergleichswert, nicht Konsenslimit; 50 000
(`DEFAULT_GAS`) bleibt Implementierungswert.

**Risiko.** Ohne Messung ist jede Zahl eine Setzung. Deshalb ist P1b keine
Feinheit, sondern die Grundlage der Abrechnung.

## P2 — Ein Format, eine Huelle, ein Frontend

**Arbeit.**
1. Huelle nach SCE-ARTIFACT definieren (eigenes Magic, Generation, Laenge,
   Layout-Hash, SHA-256).
2. VM-Loader auf den Strom nach ATC-BC-001 umstellen, Opcode-Belegung angleichen.
3. Backend in `components/system` vom Opcode-Uebersetzer zum Semantikpruefer
   zurueckbauen; verbleibende Ablehnungen bleiben Ablehnungen.
4. `.ops` aus dem Produktionspfad nehmen, im Register als Testwerkzeug fuehren.
5. Bestehende Devnet-Artefakte neu erzeugen, Code-Hashes neu binden.

**Nachweis.** Ein Artefakt, gelesen von Verifier und VM mit identischem Strom;
Gate SCE-G1, G2, G8. Rundlauf-Test als Pflichtfall T-01 und T-03.

**Was nicht passiert.** Keine Aenderung an ATC-BC-001, keine zweite VM, kein
Parallelbetrieb beider Formate.

**Eingang jetzt erfuellt.** D1 und D7 sind entschieden (freigegeben). P2 darf
beginnen, sobald P1a abgeschlossen ist und das Register konsistent ist.

## P3 — Aufruf Stufe 1 und Identitaet

**Arbeit.**
1. `CallEnvelope` (SCE-CALL) implementieren: Adresse, Selektor, Calldata, Wert,
   Gasbudget, Aufruferidentitaet.
2. Selektor aus dem Vertrags-ABI (`abi.rs`), Calldata als kanonische Wortfolge.
3. Rueckgabepuffer mit Status und Laenge, Fehlerfaelle als eigene Statuscodes.
4. Vertragsadresse deterministisch ableiten (Deployer, Nonce, Code-Hash, eigene
   Domain).
5. `caller` von `u64` auf das 32-Byte-Identitaetswort umstellen, inklusive
   Receipt-Feld.

**Nachweis.** T-04, T-05, T-06, T-12, T-13; Gate SCE-G3 (ohne Call), G5.

**Bewusste Grenze.** Noch kein Vertrag-zu-Vertrag-Aufruf. Ohne Ruecklauf waere
das eine halbe Zustandsaenderung.

## P4 — Speicher und Zustand

**Arbeit.**
1. Lokale in einen Frame-Speicher legen, eigene Befehle fuer Lokale.
2. Persistente Slots nach S1, Adressierung ueber Layout-Hash aus dem Artefakt.
3. `ContractState` von `Vec<u64>` auf eine Slot-Tabelle mit Layout-Bezug
   umstellen, `state_root` entsprechend.
4. Maps im Kettenziel ausdruecklich ablehnen, solange kein Schluesselbefehl
   existiert.

**Nachweis.** T-07, T-08, T-19; Gate SCE-G4.

**Nebenwirkung, die benannt sein muss.** `state.rs` und der Beleg aendern sich;
alte Entwicklungszustaende sind nicht mehr lesbar. Fuer Devnet akzeptabel, fuer
Testnet braucht es einen Migrationshinweis.

## P5 — Gas und Gebuehren

**Arbeit.**
1. `GAS_TABLE_V2` als einzige Tabelle, zusammengefuehrt aus `GAS_TABLE_V1` und der
   ATC-14-Formel fuer Deployment.
2. Budget aus `tx.gas_limit`, nicht aus `DEFAULT_GAS`; Mempool prueft das Feld.
3. Verbrauch im Beleg, auch bei Erfolg (schliesst GAS-AUD-005).
4. Kontofuehrung und Abbuchung; Fehlschlag verbraucht das ganze Budget.
5. Ruecklauf bei erschoepftem Budget ohne Teilwirkung.

**Nachweis.** T-09, T-10, T-11, T-17; Gate SCE-G6.

**Offen und blockierend.** Ohne Kontofuehrung ist REQ-SCE-025 nicht erfuellbar.
Das ist eigene Arbeit, kein Nebenprodukt.

## P6 — Ausfuehrung im Block

**Arbeit.**
1. Blockstruktur erweitern: Belege, Zustandswurzel vorher/nachher,
   Aufnahmeverweis, Gasabrechnung je Transaktion.
2. `atc-node` bekommt die VM-Kante; `TxType::Contract` wird ausgewertet.
3. Nachrechnen beim Import, Abbruch bei Abweichung, kein stilles Weiterreichen.
4. Aufnahme nach ATC-SEC-EXEC-001 im Kettenpfad verankern, Devnet-Schluessel
   ausdruecklich als Devnet kennzeichnen.

**Nachweis.** T-14, T-15, T-18, T-20; Gate SCE-G7 und G10. Zwei Validatoren,
zwei Prozesse, gleiche Wurzel.

**Bruch.** Die bestehende Kette ist nach dieser Phase nicht mehr kompatibel. Das
ist der Punkt, an dem ein Netz neu gestartet wird, und er muss vorher angekuendigt
sein.

## P7 — Call und Ereignisse (nach BC-002)

**Arbeit.** `Call` mit eigenem Frame, Ruecklauf je Frame und je Transaktion,
Ereignisse als Teil des Belegs. Voraussetzung ist die Ratifizierung von
ATC-BC-002 (Assert, Revert, Ruecklauf-Ebenen, Fehlercodes).

**Nachweis.** Pflichtfaelle fuer Tiefe, Zyklen, Teilfehler; Gate SCE-G3 vollstaendig.

## P8 — E2E und CI

**Arbeit.** Zwei Knoten, getrennte Prozesse, gleicher Block, gleiche Wurzel; CI
am gebundenen Commit; Register und Doku nachziehen.

**Nachweis.** `CI_VERIFIED` und `E2E_VERIFIED` fuer denselben SHA. Die 464 lokalen
Tests vom 11.11.2026 bleiben lokale Evidenz und werden nicht hochgestuft.

## Risiken

| Risiko | Wirkung | Umgang |
|---|---|---|
| K-11: ATC-14 fordert Wasm, die Kette fuehrt eine Stack-Maschine | Standardkonflikt, der bei einer Pruefung auffaellt | SCR auf ATC-14, nicht stillschweigend umdeuten |
| Fehlende Kontofuehrung | P5 nicht abschliessbar | eigene Phase vor P5, ehrlich im Plan |
| CI blockiert (E-Mail, workflow-Scope) | CI_VERIFIED nicht erreichbar | Owner-Aktion, solange offen bleibt der Status offen |
| 16 Registry-Vertraege auf `.ops` | Migration noetig vor P2-Abschluss | Portierung mit Paritaetsvergleich, danach Legacy |
| Devnet-Neustart in P6 | Datenverlust in Devnet | vorher ankuendigen, Register und Belege neu erzeugen |
| Reihenfolge wird uebersprungen | zwei Wahrheiten kehren zurueck | Gate je Phase, kein Phasenbeginn ohne Vorgaenger |

## Definition of Done fuer ATC-SMART-CONTRACT-EXECUTION-001

Der Vertrag ist erfuellt, wenn alle zehn Gates mit Nachweis vorliegen und
zusaetzlich gilt:

1. Ein Vertrag aus ATCLang wird deployt, aufgerufen und aendert Zustand, ohne dass
   jemand von Hand Bytes schreibt.
2. Zwei unabhaengige Validatoren berechnen dieselbe Zustandswurzel und lehnen
   einen absichtlich abweichenden Block ab.
3. Ein Aufruf, der scheitert, hinterlaesst weder Zustand noch Gebuehrenerstattung.
4. Kein Pfad im Produktionsbetrieb kommt ohne Typpruefung, ABI und Freigabe aus.
5. Die Dokumentation sagt dasselbe wie der Code, und jede Aussage traegt ihre
   Evidenzstufe.

Bis dahin gilt der Status aus dem Audit vom 11.11.2026: der Pfad ist da, die
Semantik fehlt, und `VERIFIED` ist nicht erreicht.

## Stand 12.11.2026

| Frage | Antwort |
|---|---|
| Ist P1 erledigt? | Ja: sieben Entscheidungen, alle DECIDED, Register fuehrend. |
| Ist P2 freigegeben? | Nein. Wartet auf P1a (Auflagen als Anforderungen) und den Registerabgleich. |
| Was ist der naechste konkrete Schritt? | P1a abschliessen: Vorschlagswerte bestaetigen, Nachweise zu REQ-SCE-048 benennen, dann P2 beginnen. |
| Was bleibt bis zuletzt offen? | Zahlen aus P1b, CI- und E2E-Evidenz. Kein Gate gilt ohne Nachweis. |
