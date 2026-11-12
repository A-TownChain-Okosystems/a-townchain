# Entscheidungsvorlage — drei Blocker, vier Entscheidungen

**Status:** Vorlage. Enthaelt **keine** Entscheidung, keine Empfehlung und keine
Freigabe.
**Datum:** 2026-11-12 · **Basis:** `a-townchain` main `995948b5`
**Bezug:** `docs/audits/EXEC-OPEN-POINTS-2026-11-12.md`,
`docs/audits/EXEC-DEPENDENCY-INVENTORY-2026-11-12.md`

## 1. Wie diese Vorlage zu lesen ist

Die sieben Befunde des Reviews zerfallen in zwei Arten, die nicht in einem
Beschluss zusammengefasst werden duerfen:

- **Blocker** sind Fragen der gemeinsamen Ausfuehrungsarchitektur. Solange sie
  offen sind, ist jede Grenzwert- oder Typentscheidung eine Entscheidung auf
  unsicherem Grund.
- **Entscheidungen** sind Fragen, die der Owner beantworten muss, weil sie
  entweder bereits von ihm beantwortet wurden oder eine Norm setzen.

Jeder Eintrag fuehrt den Status in **drei getrennten Ebenen**, damit eine
Owner-Entscheidung nicht mit einer ratifizierten Norm verwechselt wird:

| Ebene | Bedeutung |
|---|---|
| Owner-Entscheidung | vom Owner getroffen, intern verbindlich, auch wenn der Standard nicht ratifiziert ist |
| Standard | ratifiziert oder Kandidat; ein Kandidat bindet niemanden |
| Vorschlag | im Kandidaten enthalten, nicht beschlossen |

**Grundregel dieser Vorlage:** Kein Blocker wird durch das Eintragen einer Zahl
in ein Dokument geschlossen.

## 2. Blocker

### B-1 Aufrufarchitektur — R-902 und R-907

**Frage.** Wie ruft ein Vertrag einen anderen auf, und in welcher Form traegt der
Aufruf seine Identitaet?

**Stand.** Der Rust-Instruktionssatz hat 19 Befehle ohne Aufruf
(`components/vm/src/vm.rs:14-35`); ATC-BC-002 Abschnitt 16 ebenso
(Zeile 250, `0x01` bis `0x13`). Ein Aufruf existiert nur in der Python-VM
(`docs/os-docs/atclang/compiler/compiler.py:201`) und in ATC-93 (Opcode 40).
`caller` ist `u64` (`components/system/src/system.rs:266`) und wird im Demo als
`1` uebergeben (`main.rs:79`).

**Was ein Aufruf mitbringen muss**, unabhaengig von der Auslegung: neuer Frame,
Aufruferidentitaet, Speichergrenzen, Gasbudget, Rueckgabepuffer, Rollback des
Frames. Diese sechs Punkte sind heute **null** von sechs implementiert.

**Warum das ein Blocker ist.** `CALL` ist keine Erweiterung von `Caller`. Ein
32-Byte-Wert passt nicht in einen Stapel aus `u64`; ein Aufruf braucht einen
Frame, den die VM nicht kennt; und beides beruehrt den eingefrorenen
Instruktionssatz ATC-BC-001. Damit ist es eine Generationenfrage, keine
Detailkorrektur.

**Vorgelagert und ungeklaert (R-906).** Ob `CALL_EXT` aus der Python-VM
uebernommen werden darf, haengt daran, ob diese VM autoritativ, Referenz oder
Altlast ist. Diese Einordnung ist nirgends dokumentiert. Eine Uebernahme vor der
Klaerung waere eine dritte Wahrheit.

**Austrittskriterium.** Ein Instruktionssatz mit genau einer Aufrufdefinition,
Frame-Modell, Rollback-Bezug und Worttyp, plus die Feststellung, welche
Implementierung autoritativ ist.

### B-2 Artefaktidentitaet — R-903

**Frage.** Welches Magic traegt der Container, und welcher Hash ist welche
Identitaet?

**Stand.** `ATCB` ist gleichzeitig Container-Magic (`artifact.rs:22`,
ATC-BC-002 Zeile 243) und laut SCE-D1 der Name des Stroms. Dazu zwei Hashes mit
verschiedenem Umfang: ATC-BC-002 Zeile 248 ueber den ganzen Container,
REQ-SCE-041 ueber den Strom allein. Die Inventur zeigt elf Stellen, die den
Container-Hash als Identitaet verwenden, darunter Zustandswurzel
(`state.rs:43-52`) und Freigabeschluessel (`exec-admission/src/lib.rs:188-204`,
`434`).

**Warum das ein Blocker ist.** Wird der Container-Hash still durch den
Strom-Hash ersetzt, aendern sich Zustandswurzeln, Freigabe- und
Widerrufsschluessel und die Vertragsidentitaet gleichzeitig. Eine Umbenennung
ohne diese drei Folgen ist keine Korrektur.

**Austrittskriterium.** Zwei benannte Hashes (`stream_hash`, `artifact_hash`),
genau eine Datei, die das Containerformat besitzt, ein abgeschlossenes Inventar
aller Verwendungen, und ein Vektor, der beide Werte fuer denselben Vertrag
festhaelt.

**Besonderheit.** Dieser Blocker ist unabhaengig von B-1 und heute entscheidbar,
weil kein Artefakt-Hash als Goldwert gespeichert ist und `System::execute` noch
nicht aus dem Knoten aufgerufen wird.

### B-3 Verfahren — R-905

**Frage.** Wie wird die Zirkularitaet aufgeloest: Grenzwerte brauchen T-21 bis
T-24, die Tests brauchen den Aufrufmechanismus, der Aufrufmechanismus ist P2,
und P2 ist bis P1a gesperrt?

**Stand.** Der Plan verlangt die Bestaetigung der Grenzwerte vor dem
P1a-Abschluss und sperrt P2 bis dahin. Beides zusammen ist nicht erfuellbar.

**Drei Wege, keiner gewaehlt** (aus dem Bericht, Abschnitt 6.1):
(a) Grenzwerte bleiben vorlaeufig, mit Pruefpunkt nach dem Aufrufmechanismus;
(b) P2 wird eng begrenzt nur fuer den Aufrufmechanismus geoeffnet;
(c) Grenzwerte werden ohne Tests bestaetigt, was der eigenen Regel widerspricht.

**Austrittskriterium.** Ein beschlossener Weg, damit P1a entweder abschliessbar
wird oder ausdruecklich als dauerhaft offen gefuehrt wird.

## 3. Entscheidungen

### E-1 Aufruftiefe — R-901

| Ebene | Status |
|---|---|
| Owner-Entscheidung 64 | getroffen am 2026-11-09, intern verbindlich |
| ATC-BC-002 als Standard | Kandidat, **nicht ratifiziert** |
| SCE-Wert 8 | Vorschlag |
| Konfliktaufloesung | offen |

**Optionen.**
A) 64 bleibt. SCE-REQ-040 wird auf 64 geaendert, `E06` bleibt ausschliesslich
Aufruftiefe, der Test auf Tiefe 9 entfaellt.
B) 8 ersetzt 64. Dann ist das eine ausdrueckliche Aenderung der
Owner-Entscheidung vom 09.11., `E06` braucht eine neue Belegung, und der Test auf
Tiefe 9 wird zum Pflichtfall.
Jede weitere Zahl waere eine dritte Festlegung und ist hier nicht vorgesehen.

**Auswirkungen.**

| Bereich | A | B |
|---|---|---|
| Instruktionen | keine | keine |
| Hashes | keine | keine |
| Gas | keine | keine |
| Fehlercodes | `E06` unveraendert | `E06` neu zu belegen |
| Bestand | kein Bestand betroffen | kein Bestand betroffen |

**Fehlende Evidenz.** Messung des Speichers je Frame und daraus die Frage, ob
Tiefe mal Frame-Groesse im Speicherbudget liegt. Ohne Frame gibt es keine
Messung, siehe B-1.

**Entscheidungsreife.** Der Wert selbst ist heute entscheidbar, seine
Verifikation nicht.

### E-2 Grenzwerte Calldata und Rueckgabe — Calldata 8 192 Byte, Rueckgabe 4 096 Byte

| Ebene | Status |
|---|---|
| Owner-Entscheidung | keine |
| ATC-BC-002 | kein Gegenstueck, weil kein Aufruf existiert |
| SCE | Vorschlag (REQ-SCE-039, REQ-SCE-040) |

**Auswirkungen.** Instruktionen: der Aufruf muss Puffergroessen kennen (B-1).
Hashes: keine. Gas: Kosten je Byte, sobald ein Aufruf existiert. Kompatibilitaet:
keine, weil nichts existiert.

**Fehlende Evidenz.** Kantentests T-21 bis T-24, also B-1. **Entscheidungsreife:
nein**, ohne Aufruf nicht pruefbar.

### E-3 Erstattung und Rueckfluss — R-904

| Ebene | Status |
|---|---|
| ATC-BC-002 | Rueckfluss des Teilbudgets an den Aufrufer entschieden (Zeile 200); Erstattung an den Absender verneint (Zeile 127) |
| Geldmodell | in ATC-BC-002 nicht vorhanden |
| SCE-Formeln | Vorschlag, nicht beschlossen |

**Optionen.**
A) Nur Rueckfluss: ungenutztes Transaktionsbudget verfaellt, `gas_refund` zaehlt
ausschliesslich Gas, das ein Frame an seinen Aufrufer zurueckgibt. Belegfelder
ohne Geldbetrag.
B) Rueckfluss plus Geldrueckzahlung: zusaetzlich `refund_amount`, wie im
SCE-Vorschlag. Das verlangt ein Konto, eine Abbuchung und eine ausdrueckliche
Aufhebung des Wortlauts "keine Erstattung".

**Auswirkungen.** Gas: Belegfelder. Konten: existieren heute nicht (eigener
Task). Instruktionen und Hashes: keine.

**Fehlende Evidenz.** Kontofuehrung, Messung der Kosten, T-28 mit Zahlen.
**Entscheidungsreife.** Grundsatzentscheidung heute moeglich, Bemessung nicht.

### E-4 `caller`-Semantik — abhaengig von B-1

| Ebene | Status |
|---|---|
| ATC-BC-002 | Opcode `0x0C Caller`, ohne Typangabe |
| Code | `u64` (`system.rs:266`) |
| SCE | 32-Byte-`IdentityValue` (REQ-SCE-052, -057 bis -059) |

**Optionen.**
A) `caller` wird ein 32-Byte-`IdentityValue` nach dem kanonischen Layout.
B) `caller` bleibt `u64`; Berechtigungen im Vertrag sind dann nur ueber eine
Tabelle ausserhalb der Identitaet moeglich.
C) `caller` bleibt `u64`, Berechtigung nur ausserhalb des Vertrags (die in D6
verworfene Variante).

**Auswirkungen.** Instruktionen: A braucht einen Worttyp auf dem Stapel, also
eine Generationenfrage. Hashes: keine. Gas: A aendert die Stapelkosten. Bestand:
A bricht v1-Artefakte, die `Caller` verwenden.

**Fehlende Evidenz.** Aufrufdefinition (B-1), Testvektoren fuer die
Frame-Identitaet. **Entscheidungsreife: nein.**

## 4. Reihenfolge

```
Stufe 1 (heute entscheidbar)
  B-2 Magic und Hash-Namen      unabhaengig
  E-1 Aufruftiefe (Wert)        unabhaengig, Verifikation spaeter
  E-3 Erstattung (Grundsatz)    unabhaengig, Bemessung spaeter
  B-3 Verfahren                 Voraussetzung fuer den P1a-Abschluss

Stufe 2 (nach B-1)
  B-1 Aufrufarchitektur, inklusive R-906 (Autoritaet der Instruktionssaetze)
  E-4 caller-Semantik
  E-2 Grenzwerte Calldata und Rueckgabe
  T-21 bis T-24, danach P1a-Abschluss
```

## 5. Freigabestand (unveraendert)

| Gegenstand | Stand |
|---|---|
| P1a | offen |
| P2 | gesperrt |
| SCE v1.5.0-draft | Entwurfsgrundlage |
| ATC-BC-002 | Kandidat, nicht ratifiziert |
| Produktionsimplementierung | nicht freigegeben |

## 6. Keine Entscheidung getroffen

Diese Vorlage stellt Optionen, Auswirkungen und fehlende Evidenz zusammen. Sie
waehlt nichts aus. Solange kein Blocker aufgeloest und keine Entscheidung
getroffen ist, darf P1a nicht als abschliessbar und P2 nicht als freigegeben
dargestellt werden.
