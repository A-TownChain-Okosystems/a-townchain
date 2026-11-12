# P1d-B2 — B2-S1 bis B2-S5: was als Nachweis gilt, bevor P1d-C beginnt

> **Status:** `GATES OFFEN` (Owner-Bewertung P1d-B2, 12.11.2026). **Keine Implementierung.**
> **P1d-C bleibt gesperrt.** Dieses Dokument arbeitet die fünf Spezifikationsgates zu
> pruefbaren Eintrittsnachweisen aus. Es schliesst keines davon: kein Gate hat einen
> Nachweispfad, alle fuenf stehen auf `OPEN`.
> **Basis:** `82ae098d`. Dieser Schritt aendert keinen Code und behauptet deshalb keine
> neuen Ausfuehrungsergebnisse (§8).
> **Register:** `docs/decisions/exec-decision-register.yaml`, Block `chain_state`,
> Abschnitte `specification_gates` und `p1d_c_entry`. Der Registerstand unten sowie die
> Gate-Steckbriefe in §1 werden aus dieser Datei erzeugt und sind nicht von Hand zu
> aendern.

## Registerstand (erzeugt)

<!-- BEGIN CHAIN-STATE-GATES (erzeugt aus docs/decisions/exec-decision-register.yaml, nicht von Hand aendern) -->

**Spezifikationsgates P1d-B2** (Owner, 2026-11-12; Erhebung `82ae098d (P1d-B2-Gates, Ausarbeitung ohne Codeaenderung)`; Quelle: `docs/decisions/exec-decision-register.yaml`)

> Ein Gate ist geschlossen, wenn seine Abnahme belegt ist, nicht wenn sie beschlossen ist. `SPECIFIED` verlangt fertige Artefakte und einen gruenen Pruefer; `IMPLEMENTED` ist erst nach P1d-C vergebbar.

| ID | Gegenstand | entscheidet | Status | Eintrittstests |
|---|---|---|---|---|
| B2-S1 | Zustandsdatenmodell und kanonische Serialisierung | AD-B1-03 | OPEN | 5 |
| B2-S2 | Komposition der Zustandswurzel | AD-B1-03 | OPEN | 5 |
| B2-S3 | Commit-Log: Datensatzformat, Dauerhaftigkeit, ein Schreiber | AD-B1-02 | OPEN | 6 |
| B2-S4 | Recovery- und Integritaetssemantik | AD-B1-02 | OPEN | 6 |
| B2-S5 | Ausfuehrungsnachweis: Ergebnis gebunden an Parent-Hash und Parent-State-Root | AD-B1-01, AD-B1-03 | OPEN | 5 |

### B2-S1 Zustandsdatenmodell und kanonische Serialisierung

**Status:** `OPEN` | **entscheidet:** AD-B1-03 | **sperrt:** P1d-C

**Abnahme:** Gleicher logischer Zustand ergibt byte-identische Eingabe in die Wurzel; Testvektoren, die von zwei Implementierungen unabhaengig reproduziert werden

**Worum es geht:** Das Modell legt fest, WELCHE Daten ueberhaupt Zustand sind und WIE sie kanonisch zu Bytes werden: Bereichsliste, Schluesselfamilien mit fester Bytebreite, Werttypen, Sortierregel, Leerwert je Bereich und die ausdrueckliche Nichtaufnahme. Es ist die Eingabe der Wurzel und damit Konsensgut: eine Aenderung ist eine harte Abspaltung, keine Formatierung.

**Was festgelegt werden muss:**

- Vollstaendige Bereichsliste, auch fuer noch nicht implementierte Bereiche (Konten, Nonces, Vertragsspeicher, Deployment- und Berechtigungsregister, Gebuehrenzustand)
- Schluesselfamilien je Bereich mit fester Breite und Endianness; keine Varints auf dem Konsenspfad
- Werttypen je Bereich (u64-Betrag, u256-Slot, Bytes32) und ihre Kodierung
- Sortierregel: aufsteigend nach kanonischem Schluesselbyte, nicht nach Einfuegereihenfolge
- Kanonischer Leerwert je Bereich, definiert statt 'nicht vorhanden'
- Ausgeschlossene Daten mit Begruendung, insbesondere Zeitstempel im gehashten Zustand
- Bytebreiten ausgezaehlt und im Werkzeug als Pflichtpruefung verankert (Lehre vom 12.11.2026: 15 statt 16 Byte)

**Artefakte, ohne die das Gate nicht schliessbar ist:**

- `standards/atc/candidates/ATC-STATE-002-CANDIDATE.md, Abschnitt Zustandsmodell (Kennung ist ein Vorschlag; die Familienzuordnung entscheidet atc-standards)`
- `standards/atc/conformance/state-model-corpus.json, Vektoren logischer Zustand zu kanonischen Bytes`
- `tools/atc_state_model.py, unabhaengige zweite Implementierung mit --check gegen den Korpus`

**Offene Owner-Punkte:**

- O-S1-1 Schluesselbreiten: Vorschlag Konto 8 Byte, Vertragsadresse und Speicherslot je 32 Byte, jede Breite mit eigener Begruendung
- O-S1-2 Zeitstempel: Vorschlag Ausschluss aus dem gehashten Zustand; Zeit bleibt Belegdatum ausserhalb der Wurzel
- O-S1-3 Konten und Nonces existieren im Code heute nicht (System kennt keine Salden); der Bereich wird trotzdem jetzt festgelegt, siehe B2-S2

**Eintrittstests:**

| ID | Pruefung | was sie belegt |
|---|---|---|
| B2-S1-T-01 | Zwei verschiedene Einspeisereihenfolgen desselben logischen Zustands ergeben byte-identische kanonische Bytes | Sortierregel statt Einfuegereihenfolge |
| B2-S1-T-02 | Leerer Bereich und nicht vorhandener Bereich ergeben den definierten Leerwert, ein belegter Bereich nicht | kanonischer Leerwert, keine Verwechslung von leer und belegt |
| B2-S1-T-03 | Rust-Implementierung und tools/atc_state_model.py bilden denselben Korpus auf dieselben Bytes ab | zwei unabhaengige Implementierungen, gleiches Ergebnis |
| B2-S1-T-04 | Ein Zeitstempelwechsel bei sonst gleichem Zustand aendert die kanonischen Bytes nicht | Nichtaufnahme nicht deterministischer Felder |
| B2-S1-T-05 | Das Werkzeug lehnt einen Vektor mit falscher Feldbreite ab, statt ihn zu akzeptieren | Breiten sind Pflichtpruefung, nicht Kommentar |

### B2-S2 Komposition der Zustandswurzel

**Status:** `OPEN` | **entscheidet:** AD-B1-03 | **sperrt:** P1d-C

**Abnahme:** Domaene, Bereichswurzeln, Reihenfolge und Leerwerte festgelegt; Aenderung eines aufgenommenen Bereichs aendert die Wurzel, Aenderung eines ausgeschlossenen Bereichs nicht

**Worum es geht:** Die Systemwurzel ist eine Komposition ueber Bereiche, nicht die Wurzel eines einzelnen Vertrags. Heute bildet state_root(code_hash) genau einen Bereich ab (components/system/src/state.rs, Domaene ATC-STATE-V1 plus code_hash plus Wortzahl plus Woerter). Diese Funktion behaelt ihre Bedeutung; die Systemwurzel ist eine zweite, eigene Domaene darueber.

**Was festgelegt werden muss:**

- Domaenenname der Systemwurzel mit ausgezaehlter Bytebreite, getrennt von ATC-STATE-V1, artifact_hash und code_hash
- Bereichs-IDs mit fester Breite und die Kompositionsreihenfolge (Vorschlag: aufsteigende Bereichs-ID)
- Bereichswurzel als Hash ueber Domaene, Bereichs-ID, Anzahl und sortierte Eintraege, mit Laengenpraefixen gegen Konkatenationskollisionen
- Kanonische Leerwertwurzel je Bereich, damit ein leerer Bereich deterministisch ist
- Ausdruecklicher Ausschluss des Chain-Kopfes (Hoehe, Best-Hash) aus der Wurzel: der Blockhash haengt an der Wurzel, die Wurzel darf nicht am Blockhash haengen; die Parent-Bindung bleibt Vorbedingung des Commits
- Feststellung, dass die Bereichsliste vollstaendig sein muss, auch fuer noch nicht implementierte Bereiche: ein spaeter ergaenzter Bereich aendert jede Wurzel und bricht jede bestehende Kette

**Artefakte, ohne die das Gate nicht schliessbar ist:**

- `standards/atc/candidates/ATC-STATE-002-CANDIDATE.md, Abschnitt Wurzelkomposition`
- `standards/atc/conformance/state-model-corpus.json, Abschnitt Bereichs- und Systemwurzeln`
- `tools/atc_state_model.py, Wurzelberechnung mit --check`

**Offene Owner-Punkte:**

- O-S2-1 Domaenenzeichenkette und Breite der Systemwurzel
- O-S2-2 Breite der Bereichs-ID und die Reihenfolge der Komposition
- O-S2-3 Ausschluss des Chain-Kopfes (Empfehlung, sonst Zirkel zwischen Blockhash und Wurzel)
- O-S2-4 Bindet die Bereichswurzel Vertragsspeicher weiterhin code_hash (heutiges Verhalten), oder wandert die Codebindung aus der Wurzel heraus; die zweite Variante aendert jeden bestehenden Wert

**Eintrittstests:**

| ID | Pruefung | was sie belegt |
|---|---|---|
| B2-S2-T-01 | Aenderung in einem aufgenommenen Bereich aendert die Systemwurzel | der Bereich ist wirklich Teil der Wurzel |
| B2-S2-T-02 | Aenderung eines ausgeschlossenen Bereichs (Cache, Debug, Audit-Metadaten) aendert die Systemwurzel nicht | Nichtaufnahme ist wirksam, nicht nur dokumentiert |
| B2-S2-T-03 | Ein leerer Bereich ergibt die festgelegte Leerwertwurzel, unabhaengig von der Anzahl der Bereichseintraege in der Komposition | Leerwert ist kanonisch und nicht implementierungsabhaengig |
| B2-S2-T-04 | Vertauschte Reihenfolge der Bereichswurzeln ergibt eine andere Systemwurzel | die Komposition bindet die Reihenfolge oder sortiert nachweislich selbst |
| B2-S2-T-05 | Fuer dieselben Woerter und denselben code_hash ist die Bereichswurzel Vertragsspeicher byte-gleich zum heutigen state_root(code_hash) | keine stille Neudefinition des bestehenden Werts |

### B2-S3 Commit-Log: Datensatzformat, Dauerhaftigkeit, ein Schreiber

**Status:** `OPEN` | **entscheidet:** AD-B1-02 | **sperrt:** P1d-C

**Abnahme:** Feldliste je Datensatz, Dauerhaftigkeitsregel und Schreibhoheit festgelegt; abgeschnittener Schwanz wird verworfen, nicht akzeptiert; Luecke in state_version entwertet den Schwanz

**Worum es geht:** Das Log ist die einzige Autoritaet. Format und Dauerhaftigkeit sind deshalb selbst Konsensgut: ein Datensatz, der auf einem Knoten als committet gilt und auf einem anderen nicht, erzeugt zwei Ketten.

**Was festgelegt werden muss:**

- Feldliste je Datensatz: state_version, parent_state_version, parent_height, parent_hash, state_root_after, block_hash, Art (deploy oder exec), Payload-Verweis (Hash und Laenge), Pruefsumme
- Rahmung: Laengenkopf, Rumpf, Pruefsumme; ein Datensatz ist vollstaendig, wenn alle drei vorhanden sind
- Dauerhaftigkeit, ausdruecklich getrennt vom vollstaendigen Schreiben: write(2) ohne sync ist nicht committet; Commit-Punkt ist das erfolgreiche fsync/fdatasync des Datensatzes einschliesslich Laengenkopf
- Keine Bestaetigung an den Aufrufer vor erfolgreichem sync; ein sync-Fehler ist COMMIT_REJECTED und hinterlaesst keinen wirksamen Zustand
- Genau ein Schreiber: die Kompositionswurzel haelt die Schreibhoheit, Anhaengen ist exklusiv, im Betrieb wird nicht gekuerzt und nicht ueberschrieben
- state_version luecken- und sprunglos, monoton, unabhaengig von der Blockhoehe
- Obergrenze je Datensatz mit eigener Begruendung, nicht aus bestehenden VM-Grenzen abgeleitet

**Artefakte, ohne die das Gate nicht schliessbar ist:**

- `docs/spec/ATC-COMMIT-LOG-FORMAT.md, Datensatzformat, Feldbreiten, Dauerhaftigkeitsregel, Schreibhoheit`
- `tools/check_commit_log.py, Pruefer gegen Log-Fixturen (vollstaendig, abgeschnitten, Pruefsummenfehler, Luecke)`
- `standards/atc/conformance/commit-log-fixtures.json, die zugehoerigen Fixturen`

**Offene Owner-Punkte:**

- O-S3-1 Dauerhaftigkeit je Datensatz oder Sammelsync: Vorschlag je Datensatz, weil ein Sammelsync zwei logische Commits zu einem Dauerhaftigkeitspunkt verschmilzt
- O-S3-2 Pruefsummenverfahren und Breite (Vorschlag SHA-256 vollstaendig ueber den Rumpf mit eigener Domaene)
- O-S3-3 Breite des Laengenkopfes und Wert der Obergrenze je Datensatz
- O-S3-4 Speichermedium (offener Punkt B2-O4), Semantik bleibt davon unabhaengig

**Eintrittstests:**

| ID | Pruefung | was sie belegt |
|---|---|---|
| B2-S3-T-01 | Ein Datensatz wurde vollstaendig geschrieben, aber vor dem sync abgebrochen: er gilt nicht als committet | vollstaendiges Schreiben ist nicht Dauerhaftigkeit |
| B2-S3-T-02 | Nach erfolgreichem sync gilt der Datensatz genau einmal als committet | Commit-Punkt ist der sync, nicht der Rueckgabewert |
| B2-S3-T-03 | Ein zweiter Schreiber kann nicht anhaengen, solange die Schreibhoheit beim ersten liegt | ein Schreiber, keine konkurrierenden Anhaenge |
| B2-S3-T-04 | Eine Luecke in state_version entwertet den Schwanz ab der Luecke | Sprunglosigkeit ist Pruefkriterium |
| B2-S3-T-05 | Ein erneutes Anhaengen derselben state_version wird abgewiesen | kein doppelter Uebergang durch Wiederholung |
| B2-S3-T-06 | Ein Datensatz ueber der Obergrenze wird vor dem Schreiben abgewiesen | Grenze wirkt vor der Allokation, nicht danach |

### B2-S4 Recovery- und Integritaetssemantik

**Status:** `OPEN` | **entscheidet:** AD-B1-02 | **sperrt:** P1d-C

**Abnahme:** Neustart stellt (hoehe, best_hash, state_root, state_version) allein aus dem Log her; Snapshotverlust ohne Wirkung; Snapshot-Log-Widerspruch wird erkannt und zugunsten des Logs entschieden

**Worum es geht:** Recovery ist die Umkehrung des Commits. Sie muss zwei Faelle strikt trennen, die heute beide 'kaputt' waeren: einen abgeschnittenen Schwanz, der verworfen werden darf, und einen vollstaendigen, aber ungueltigen Datensatz, der einen Integritaetsfehler ausloest und niemals stillschweigend uebersprungen wird.

**Was festgelegt werden muss:**

- Definition vollstaendig: Laengenkopf, Rumpf und Pruefsumme vorhanden; nur ein unvollstaendiger Datensatz ist ein Schwanz
- Unvollstaendiger Schwanz: wird verworfen, das Log wird auf den letzten vollstaendigen Datensatz gekuerzt, der Vorgang wird mit verworfener state_version protokolliert; kein Fehler, kein Zustandswechsel
- Vollstaendiger, aber ungueltiger Datensatz (Pruefsumme, Parent-Bindung, state_version-Luecke, Wurzelwiderspruch): INTEGRITAETSFEHLER, Abbruch, kein Weiterlauf, kein Ueberspringen, keine automatische Reparatur
- Der im Betrieb gueltige Satz bleibt gueltig: ein als Schwanz erkannter Datensatz darf nie als gueltig uebernommen werden, ein vollstaendiger nie als Schwanz verworfen werden
- Snapshot ist abgeleitet: gebunden an (state_version, state_root), bei Widerspruch zum Log verworfen und aus dem validierten Log neu gebaut, Loeschung ohne Informationsverlust
- Keine Heilung durch Nachrechnen beim Start: Wiederausfuehrung waere eine Konsensentscheidung, kein Recovery
- Fehleridentitaet als Code, nicht als Meldungstext

**Artefakte, ohne die das Gate nicht schliessbar ist:**

- `docs/spec/ATC-COMMIT-LOG-FORMAT.md, Abschnitte Recovery und Integritaet`
- `tools/check_commit_log.py mit den Faellen Schwanz, Pruefsummenfehler, Parent-Bruch, Luecke`
- `standards/atc/conformance/commit-log-fixtures.json, je Fall eine Fixtur mit Sollergebnis`

**Offene Owner-Punkte:**

- O-S4-1 Verhalten bei Integritaetsfehler: Vorschlag Startabbruch mit benanntem Code; Wiederherstellung aus Snapshot plus Nachrechnen wird verworfen, weil sie eine Konsensentscheidung waere
- O-S4-2 Darf ein abgeschnittener Schwanz beim Start automatisch verworfen werden (Vorschlag ja), und muss die verworfene state_version sichtbar protokolliert werden (Vorschlag ja)

**Eintrittstests:**

| ID | Pruefung | was sie belegt |
|---|---|---|
| B2-S4-T-01 | Abgeschnittener Schwanz: Neustart ergibt den Stand des letzten vollstaendigen Datensatzes, ohne Fehler, mit protokollierter verworfener Version | Schwanz verwerfen statt akzeptieren |
| B2-S4-T-02 | Vollstaendiger Datensatz mit falscher Pruefsumme: Integritaetsfehler, kein Zustand, kein Weiterlauf | ungueltig wird nicht als Schwanz behandelt |
| B2-S4-T-03 | Gebrochene Parent-Bindung: Integritaetsfehler statt Ueberspringen | Kettenglied wird geprueft, nicht geglaubt |
| B2-S4-T-04 | Luecke in state_version: Integritaetsfehler, nicht Schwanz | die Grenze zwischen Schwanz und Fehler ist entschieden |
| B2-S4-T-05 | Snapshot widerspricht dem Log: Log gewinnt, Snapshot wird verworfen und neu gebaut, Ergebnis identisch | Snapshot ist Beschleuniger, keine zweite Autoritaet |
| B2-S4-T-06 | Snapshot geloescht: vollstaendige Wiederherstellung allein aus dem Log, gleiches Tupel aus Hoehe, Best-Hash, Wurzel und state_version | das Log traegt die Wiederherstellung allein |

### B2-S5 Ausfuehrungsnachweis: Ergebnis gebunden an Parent-Hash und Parent-State-Root

**Status:** `OPEN` | **entscheidet:** AD-B1-01, AD-B1-03 | **sperrt:** P1d-C

**Abnahme:** Ein Ergebnis, das auf einem anderen Parent gerechnet wurde, wird von der Kompositionswurzel allein abgewiesen, ohne Nachrechnen der Ausfuehrung

**Worum es geht:** Zwischen Ausfuehrung und Commit liegt ein Ergebnis. Es muss die Ausgangsversion so tragen, dass die Kompositionswurzel ohne Wiederausfuehrung entscheiden kann, ob es noch gilt. Heute wird state_root_before berechnet (components/system/src/system.rs:396) und nur in den Beleg geschrieben, nie verglichen.

**Was festgelegt werden muss:**

- Ergebnisfelder: parent_height, parent_hash, parent_state_root, state_version_before, state_root_after, code_hash, Payload-Verweis (Hash und Laenge), Gaskosten
- Pruefung allein anhand dieser Felder gegen den autoritativen Stand; vier Vergleiche, kein Nachrechnen, keine Heilung
- state_root_before wird verglichen, nicht protokolliert: die heute nur belegte Groesse wird Pruefgroesse
- Ablehnungsgruende als geschlossene Aufzaehlung (veraltete Hoehe, fremder Parent-Hash, fremde Ausgangswurzel, veraltete state_version, Dauerhaftigkeitsfehler, Integritaetsfehler)
- Ein abgewiesenes Ergebnis hinterlaesst weder Log-Datensatz noch Zustands- oder Policy-Aenderung

**Artefakte, ohne die das Gate nicht schliessbar ist:**

- `docs/spec/ATC-COMMIT-LOG-FORMAT.md, Abschnitt Ergebnisfelder und Vorbedingung`
- `Rust-Tests in P1d-C, die die vier Vergleiche und die Ablehnungsgruende einzeln belegen`

**Offene Owner-Punkte:**

- O-S5-1 Traegt das Ergebnis die Payload selbst oder nur ihren Verweis (Vorschlag Verweis, Transport bleibt Sache des Blocks)
- O-S5-2 Wortlaut und Umfang der Ablehnungsgruende (Vorschlag sechs Werte, wie oben)

**Eintrittstests:**

| ID | Pruefung | was sie belegt |
|---|---|---|
| B2-S5-T-01 | Ergebnis mit fremdem parent_hash wird abgewiesen, ohne dass die VM erneut laeuft | die Kompositionswurzel entscheidet ohne Nachrechnen |
| B2-S5-T-02 | Ergebnis mit fremdem parent_state_root wird abgewiesen, obwohl die Ausfuehrung inhaltlich korrekt war | Bindung an die Ausgangswurzel, nicht an das Ergebnis allein |
| B2-S5-T-03 | Nach einer Abweisung sind Log, Zustand und Policy unveraendert | kein Teilerfolg, kein Aufraeumbedarf |
| B2-S5-T-04 | Jeder Ablehnungsgrund ist ein eigener, stabiler Code | Fehleridentitaet ist unterscheidbar, nicht ein Sammeltext |
| B2-S5-T-05 | Der Vergleich von state_root_before ist als Regressionstest abgesichert: wird der Vergleich entfernt, schlaegt der Test fehl | die Pruefung existiert wirklich und ist nicht nur protokolliert |

## P1d-C-Abnahmetests (Eintrittsbedingung, nicht Teil von C)

| ID | Abnahme | setzt voraus | Status |
|---|---|---|---|
| C-T-01 | Veralteter state_root wird abgewiesen | B2-S2, B2-S5 | OPEN |
| C-T-02 | Falscher Ausgangs-Blockhash wird abgewiesen | B2-S5 | OPEN |
| C-T-03 | Zwei parallele Aufrufer mit unterschiedlichem Zustand koennen nicht beide committen | B2-S3, B2-S5 | OPEN |
| C-T-04 | Fehler nach der VM-Ausfuehrung veraendert keinen dauerhaften Zustand | B2-S3 | OPEN |
| C-T-05 | Fehler waehrend des Block-Commits veraendert keinen dauerhaften Zustand | B2-S3, B2-S4 | OPEN |
| C-T-06 | Neustart nach einem Commit stellt einen konsistenten Chain- und Zustandszustand her | B2-S4 | OPEN |
| C-T-07 | Wiederholte Ausfuehrung derselben Transaktion erzeugt keinen doppelten Zustandsuebergang | B2-S3 | OPEN |
| C-T-08 | state_root_before wird gegen den autoritativen Ausgangszustand geprueft, nicht nur protokolliert | B2-S5 | OPEN |

Die acht Abnahmen stehen hier, weil ein Abnahmetest, der erst beim Umbau entsteht, nicht mehr als Eintrittsbedingung wirken kann. `C-T-08` ist die Verschaerfung aus der Owner-Bewertung vom 12.11.2026: `state_root_before` wird in C Pruefgroesse und bleibt nicht Belegfeld.

<!-- END CHAIN-STATE-GATES -->

## 1 Was ein Gate ist und was es nicht ist

Die vier Entscheidungen aus P1d-B2 (`AD-B1-01` bis `AD-B1-04`) sind **getroffen**. Ein
getroffener Beschluss ist aber kein Nachweis, und ein Nachweis ist keine Umsetzung. Die
Statusleiter des Registers trennt genau das:

| Stufe | Bedeutung | wer sie vergibt |
|---|---|---|
| `DECIDED` | Architekturentscheidung getroffen, Alternative benannt und verworfen | Owner |
| `SPECIFIED` | normativ festgelegt, Artefakte vorhanden, Pruefer gruen | Nachweis, nicht Beschluss |
| `IMPLEMENTED` | im bestehenden System nachgewiesen, Tests vorhanden | P1d-C |
| `OPEN` | nichts davon | Ausgangszustand |

Zwei Regeln machen die Leiter wirksam, und beide sind im Werkzeug verankert, nicht nur
im Text:

1. **Ein Gate ist geschlossen, wenn seine Abnahme belegt ist.** `tools/decision_register.py`
   weist `SPECIFIED` und `IMPLEMENTED` ohne Feld `evidence` als Fehler ab. Ein Gate kann
   also nicht durch Bearbeiten eines Statusworts geschlossen werden.
2. **Die P1d-C-Abnahmetests stehen im Register, nicht in C.** Ein Abnahmetest, der erst
   beim Umbau entsteht, kann nicht mehr als Eintrittsbedingung wirken. Die acht Abnahmen
   `C-T-01` bis `C-T-08` sind deshalb schon jetzt Teil der Eintrittspruefung.

## 2 Die zwei Verschaerfungen aus der Owner-Bewertung

Beide betreffen Saetze, die in P1d-B2 zu weich formuliert waren, und beide sind jetzt
Pflichtinhalt eines Gates.

### 2.1 Vollstaendiges Schreiben ist nicht Dauerhaftigkeit (B2-S3)

P1d-B2 hat den Commit-Punkt als "vollstaendiges Anhaengen des Log-Datensatzes"
beschrieben. Das ist zu wenig: ein vollstaendig geschriebener Datensatz liegt danach im
Seitencache und ist nach einem Stromausfall verloren. Die Regel wird deshalb getrennt
gefuehrt:

| Zustand | Bedeutung | Folge |
|---|---|---|
| geschrieben | Bytes sind an das Medium uebergeben | **nicht** committet, keine Bestaetigung an den Aufrufer |
| dauerhaft | `fsync`/`fdatasync` fuer Datensatz **und** Laengenkopf erfolgreich | committet, genau einmal |
| sync gescheitert | Rueckgabe mit Fehler | `COMMIT_REJECTED`, kein wirksamer Zustand, Datensatz bleibt Schwanz |

Daraus folgen drei Aussagen, die sonst offen blieben: der Commit-Punkt ist der
erfolgreiche `sync`, nicht der Rueckgabewert der Schreibfunktion; eine Bestaetigung vor
dem `sync` ist unzulaessig; und ein gescheiterter `sync` ist kein halber Erfolg, sondern
ein abzuraeumender Schwanz (B2-S4).

### 2.2 Die Grenze zwischen Schwanz und Integritaetsfehler (B2-S4)

P1d-B2 hat "ein abgeschnittener Schwanz ist kein Commit, er wird verworfen" festgehalten.
Ohne die Gegenrichtung ist die Regel gefaehrlich: sie wuerde auch einen **vollstaendigen,
aber ungueltigen** Datensatz entsorgbar machen. Recovery trennt deshalb zwei Faelle, und
zwar an einer definierten Grenze:

| Fall | Erkennung | Verhalten |
|---|---|---|
| unvollstaendiger Schwanz | Laengenkopf, Rumpf oder Pruefsumme fehlen | verwerfen, Log auf den letzten vollstaendigen Datensatz kuerzen, verworfene `state_version` protokollieren, kein Fehler |
| vollstaendiger, aber ungueltiger Datensatz | Pruefsumme falsch, Parent-Bindung gebrochen, Luecke in `state_version`, Wurzelwiderspruch | `INTEGRITAETSFEHLER`, Abbruch, kein Weiterlauf, kein Ueberspringen, keine automatische Reparatur |

Die Grenze ist damit selbst Pruefgegenstand: **vollstaendig** heisst Laengenkopf, Rumpf
und Pruefsumme vorhanden. Nur was unvollstaendig ist, darf ein Schwanz sein. Ein als
Schwanz erkannter Datensatz darf nie als gueltig uebernommen werden, ein vollstaendiger
nie als Schwanz verworfen werden.

Ausgeschlossen wird dabei ausdruecklich die naheliegende Reparatur: beim Start
nachzurechnen und den Zustand neu zu erzeugen. Wiederausfuehrung ist eine
Konsensentscheidung, kein Recovery. Ein Knoten, der sich selbst heilt, heilt sich
moeglicherweise in eine andere Kette.

## 3 Warum die Bereichsliste in B2-S2 jetzt vollstaendig sein muss

Der naheliegende Weg waere, heute nur den Bereich aufzunehmen, der im Code existiert
(Vertragsspeicher), und weitere Bereiche spaeter zu ergaenzen. Das ist nicht zulaessig,
und der Grund ist strukturell:

Heute bildet `state_root(code_hash)` genau einen Bereich ab: Domaene `ATC-STATE-V1` mit
abschliessendem Nullbyte, `code_hash`, Wortzahl als `u32` und die Woerter als `u64`
(`components/system/src/state.rs`, Blob `0eb6a9ea`). Sobald die Systemwurzel ueber
mehrere Bereiche komponiert wird, aendert **jeder** zusaetzliche Bereich die Wurzel
jedes Blocks. Ein spaeter ergaenzter Bereich ist damit keine Erweiterung, sondern eine
harte Abspaltung: jede bestehende Kette wird ungueltig, jeder gespeicherte Zustand passt
nicht mehr zu seinem Block.

Deshalb legt B2-S2 die Bereichsliste **vollstaendig** fest, einschliesslich der Bereiche,
die im Code noch nicht existieren (Konten, Nonces, Deployment- und
Berechtigungsregister, Gebuehrenzustand), jeweils mit kanonischem Leerwert. Solange ein
Bereich nicht implementiert ist, ist er leer; das ist ein definierter Zustand und keine
Luecke.

Zwei Folgen, die zur Festlegung gehoeren:

* **Der Chain-Kopf gehoert nicht in die Wurzel.** Der Blockhash haengt an der Wurzel; die
  Wurzel darf nicht am Blockhash haengen. Die Parent-Bindung bleibt deshalb
  Commit-Vorbedingung (vier erwartete Werte) und wird nicht Teil der Wurzel. Diese
  Trennung ist keine Geschmacksfrage, sie verhindert einen Zirkel.
* **`state_root(code_hash)` behaelt seine Bedeutung.** Die bestehende Funktion wird nicht
  zur Systemwurzel umgedeutet, sondern zur Bereichswurzel des Vertragsspeichers. Dieselbe
  Funktion mit zwei Bedeutungen waere eine zweite Wahrheit (Architektur-Grundregel). Ob
  die Codebindung in der Bereichswurzel bleibt, ist Owner-Punkt `O-S2-4`: die Alternative
  aendert jeden bestehenden Wert.

## 4 `state_root_before`: heute berechnet, nicht geprueft

Der Owner verlangt, dass `state_root_before` in C tatsaechlich gegen den autoritativen
Ausgangszustand geprueft wird. Der Befund am Code (Blob `42dab555`) stuetzt das:

* `system.rs:396` berechnet `let state_root_before = state.state_root(&deployment.code_hash);`
* der Wert erscheint im `Receipt` (`state_root_before`), wird aber **nirgends verglichen**
* `system.rs:470` berechnet `state_root_after`, `:478` produziert den Block mit einer
  Payload aus `code_hash` und `state_root_after`, `:480` setzt `*state = after`

Eine Groesse, die berechnet und protokolliert, aber nicht geprueft wird, sieht in jedem
Beleg wie eine Pruefung aus. Deshalb ist sie als `C-T-08` **aus dem Beleg in die
Eintrittsbedingung** gewandert, mit einem Regressionstest, der fehlschlaegt, wenn der
Vergleich entfernt wird (B2-S5-T-05). Ein Test, der nur den Erfolgsfall abdeckt, wuerde
das Entfernen der Pruefung nicht bemerken.

## 5 Reihenfolge: welche Gates zuerst schliessbar sind

Die Gates sind nicht gleich weit von der Schliessung entfernt, und die Reihenfolge ist
nicht beliebig:

| Schritt | Gate | warum diese Position |
|---|---|---|
| 1 | B2-S1 | Datenmodell ist Eingabe fuer alles Weitere; ohne feste Bytes gibt es keine Wurzel |
| 2 | B2-S2 | Komposition braucht das Modell; danach sind Testvektoren fuer Wurzeln moeglich |
| 3 | B2-S3 | Log-Format braucht die Wurzel, weil jeder Datensatz eine `state_root_after` traegt |
| 4 | B2-S4 | Recovery prueft genau das Format und die Pruefsumme aus B2-S3 |
| 5 | B2-S5 | Ergebnisfelder brauchen Wurzel und `state_version`; das Ergebnis ist die Schnittstelle zum Commit |

B2-S1 und B2-S2 sind reine Spezifikations- und Werkzeugarbeit ohne Laufzeitaenderung und
damit ohne Konsensrisiko. B2-S3 und B2-S4 bringen Format und Dauerhaftigkeit, bleiben aber
ebenfalls ohne Laufzeitpfad, solange nur Fixturen und ein Pruefwerkzeug entstehen. B2-S5
ist die einzige Stelle, die Felder des laufenden Systems beruehrt; sie bleibt dennoch
Spezifikation, weil die Pruefung erst in C implementiert wird.

Kein Gate verlangt eine Aenderung an `components/system` oder `components/node`. Wer eines
der Gates mit einem Umbau "miterledigen" will, vermischt Entscheidung, Spezifikation und
Umsetzung; genau diese Vermischung soll die Phasenordnung verhindern.

## 6 Der Standards-Strang bleibt getrennt

Die beiden Standards-Branches sind inhaltlich fertig und **PR-READY, nicht PR-OPEN**:

| Branch | Repository | Inhalt | Blocker |
|---|---|---|---|
| `std/family-400-option-b` | `atc-standards` | Familie 400 nach Option B: 400 bis 499 bleibt SECURITY, 51 Ausnahmen nach §3, Familienregister, Views | PR-Anlage an unverifizierter E-Mail |
| `reserve/atc-std-410` | `atc-standards` | `ATC-STD-410` als reservierte, nicht normative Zeile | dieselbe Owner-Aktion |

Sie blockieren **P1d-C nicht**. Sie bedingen die normative Freigabe von `ATC-STD-410` und
damit `ADM-012`; das ist ein eigener Governance-Strang. Die Vermischung waere hier
besonders teuer: P1d-C wuerde auf einen Merge warten, der mit Zustandspersistenz nichts zu
tun hat.

## 7 Was dieses Dokument nicht ist

* **Keine Umsetzung.** Kein Gate ist geschlossen, kein Statuswort wurde veraendert.
* **Kein Standard.** Die vorgeschlagene Kennung `ATC-STATE-002` ist ein Vorschlag im
  Candidate-Ordner; ob und in welcher Familie sie registriert wird, entscheidet
  `atc-standards`, nicht dieses Dokument.
* **Kein `CI_VERIFIED`.** Es gibt keinen Lauf auf dem Default-Branch-HEAD fuer diese
  Aenderung, weil sie keinen ausfuehrbaren Code enthaelt.
* **Keine Grenzwerte.** `O-S3-3` (Obergrenze je Datensatz) ist bewusst offen: ein Wert
  wird begruendet und an der Kante getestet, nicht aus bestehenden VM-Grenzen abgeleitet.

## 8 Evidenzbindung

Angegeben sind Blobs zum Erhebungszeitpunkt `82ae098d`. Ein Blob-SHA bindet den Nachweis,
der Kopf-Commit ist nur der Zeitpunkt der Erhebung. Aendert sich ein Blob, sind die
betroffenen Aussagen neu zu pruefen.

| Gegenstand | Blob |
|---|---|
| `components/system/src/state.rs` (heutige Zustandswurzel, Domaene `ATC-STATE-V1`) | `0eb6a9ea` |
| `components/system/src/system.rs` (`state_root_before`, `produce`, `*state = after`) | `42dab555` |
| `components/node/src/chain.rs` (`produce`, 64-Block-Grenze) | `e5bc956a` |
| `docs/decisions/exec-decision-register.yaml` (Register, Gates) | `7e864050` |
| `docs/decisions/AD-P1D-B2-FOUR-DECISIONS-2026-11-12.md` (die vier Entscheidungen) | `adfefbca` |
