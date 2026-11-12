# ATC-BC-002-CANDIDATE — ATCB-v2 (Pruef- und Ruecklaufsemantik)

**Status:** KANDIDAT · nicht normativ · **aendert ATC-BC-001 nicht**
**Datum:** 2026-11-09 · **Geltung nach** Ratifizierung in `atc-standards`

## 1. Anlass

`require` wird im Lowering still verworfen, weil ATC-BC-001 keinen Pruef- oder
Abbruchbefehl kennt. Ein Vertrag mit unerfuellter Bedingung laeuft auf PASS.
Semantische Vertragstreue zwischen Sprache und VM ist damit verletzt.

## 2. Verhaeltnis zu ATC-BC-001

- ATC-BC-001 bleibt unveraendert und gueltig. Kein stilles Umschreiben.
- v2 ist eine neue Generation mit eigener Versionskennung im Kopf.
- Ein Artefakt deklariert seine Generation. Der Verifier lehnt v2-Befehle in
  einem v1-Artefakt ab und v1-Artefakte nicht still auf v2 ab.
- Mischbetrieb: Eine Kette fuehrt je Vertrag genau eine Generation.

## 3. Neue Befehle

| Befehl | Stapel | Wirkung |
| --- | --- | --- |
| ASSERT | zieht Bedingung | Bedingung 0 -> Ruecklauf mit Fehlercode |
| REVERT | zieht Code | sofortiger Ruecklauf mit uebergebenem Code |
| REVERTIF | zieht Bedingung, Code | Ruecklauf bei Bedingung ungleich 0 (Abschnitt 15) |

Bewusst klein: keine Ausnahmen, keine Fehlerobjekte auf dem Stapel.
Ruecklauf ist ein Abbruch, kein Sprungziel.

## 4. Ruecklauf-Ebenen

- **Frame:** Aenderungen des laufenden Vertragsaufrufs werden zurueckgenommen.
- **Transaktion:** alle Aenderungen der Transaktion werden zurueckgenommen,
  einschliesslich abhaengiger Aufrufe. Diese Ebene ist die Sicherheitsgarantie.
- Regel: Ein fehlgeschlagener Transfer darf nie nur eine Seite belasten.

Umsetzung: Schreib-Set je Frame; Commit erst bei Erfolg des gesamten Aufrufs.
Speicherzugriffe, die nie geschrieben wurden, erzeugen keinen Eintrag.

## 5. Fehlercodes (deterministisch)

| Code | Bedeutung | Zustand |
| --- | --- | --- |
| E01 | Pruefung fehlgeschlagen | Ruecklauf |
| E02 | Ruecklauf angefordert | Ruecklauf |
| E03 | Gas erschoepft | Ruecklauf, Gas vollstaendig verbraucht |
| E04 | Stapelgrenze | Ruecklauf |
| E05 | Speichergrenze | Ruecklauf |
| E06 | Aufruftiefe ueberschritten (Grenze 64) | Ruecklauf |
| NEU-01 | Unbekannter Befehl | Abbruch, gilt als ungueltig. Kennung wird nach vollstaendiger Code-Inventur vergeben (siehe Abschnitt 17) |

Codes sind Teil des Ergebnisses und werden im Beleg mitgeschrieben.
Ein unbekannter Befehl ist kein Ruecklauf, sondern ein Abbruch: Das Programm ist
ungueltig, es gibt nichts zurueckzunehmen. Deshalb erhaelt er eine eigene
Kennung und nicht E06.

## 6. Gas auf Fehlerpfaden

- Kosten werden **vor** der Wirkung berechnet und belastet.
- Ein Ruecklauf erstattet keine Ausfuehrungszeit; verbrauchtes Gas bleibt
  verbraucht. Zwei Knoten mit gleichem Zustand berechnen gleiche Kosten.
- Speicher-Schreibkosten fallen je geschriebenem Eintrag an, auch wenn der
  Eintrag durch Ruecklauf verschwindet.

## 7. Verifier-Regeln (harte Grenze)

1. Nur bekannte Befehle der deklarierten Generation.
2. Jedes Sprungziel liegt auf einer Befehlsgrenze im Programm.
3. Kontrollfluss endet an HALT, REVERT oder Ruecklauf, nie durch Absturz.
4. Stapeltiefe ist auf jedem Pfad innerhalb der Grenze.
5. Kein Befehl nach einem unbedingten Abbruch ist erreichbar.
6. ASSERT und REVERTIF nur mit korrekter Stapelzahl.
Verstoesse = Artefakt ungueltig, keine Teilausfuehrung.

## 8. Lowering von `require`

`require(bedingung, code)` wird zu: Bedingung auswerten, ASSERT mit Code.
Kann eine Bedingung nicht als Bytecode dargestellt werden, ist das ein
Compiler-Fehler, kein stilles Weglassen. Diese Regel gilt fuer alle
sicherheitsrelevanten Konstrukte.

## 9. Testmatrix (Mindestumfang)

1. Bedingung wahr -> Ausfuehrung laeuft, Gas wie spezifiziert
2. Bedingung falsch -> Ruecklauf, Rueckgabewert E01
3. Falsche Bedingung beim Transfer -> beide Salden unveraendert
4. Ruecklauf im inneren Aufruf -> nur Frame betroffen
5. Ruecklauf in der aeusseren Transaktion -> alles zurueckgenommen
6. Gas erschoepft mitten im Schreiben -> kein Teilzustand
7. v2-Befehl in v1-Artefakt -> Verifier lehnt ab
8. Sprung in die Mitte eines Befehls -> Verifier lehnt ab
9. Unerreichbarer Code nach HALT -> Verifier meldet, Ausfuehrung nie
10. Zwei Laeufe identisch -> gleiche Kosten, gleiche Codes, gleiche Events
11. Abbruch an jeder Schreibposition (Fault-Injection) -> konsistenter Zustand
12. Sprachseitig: jede akzeptierte Bedingung erscheint im Bytecode

## 10. Entscheidungsstand

Die zuvor offenen Punkte (Fehlercode-Raum, Ruecklauf-Ebene, Schreib-Set-Form,
Gebuehrenberechnung, Migrationspolitik) sind am 2026-11-09 vom Owner entschieden
und in Abschnitt 11 (Entscheidungen) sowie 11 (Legacy) und 12 (Nachweise)
verbindlich festgehalten. Es bleiben keine offenen Semantikfragen.

## 11. Entscheidungen (Owner, 2026-11-09)

### A. Aktivierung und Migration

- Aktivierung ab einer festen Blockhoehe `atcb_v2_activation_height` (Kettenparameter).
- Ab Aktivierung: neue Deployments ausschliesslich v2. v2-Artefakt vor der Hoehe
  wird abgelehnt, v1-Artefakt nach der Hoehe ebenfalls.
- Bestehende v1-Vertraege behalten ihre Generation und Semantik unveraendert.
- Keine automatische Umwandlung v1 -> v2. Migration nur ueber einen expliziten,
  autorisierten Vorgang mit nachvollziehbarem Zustandsuebergang.
- Die Generation ist je Vertrag unveraenderlich; ein Wechsel ist ein neuer Vertrag.

### B. Schreib-Set und atomarer Abschluss

Das Schreib-Set erfasst: Storage-Schreibvorgaenge, Guthabenaenderungen,
Ereignisse, Wirkungen verschachtelter Aufrufe und Rueckgabewerte.
Uebernahme in den Transaktionszustand erst nach erfolgreichem Abschluss des
gesamten Aufrufs. Ereignisse werden je Rahmen gepuffert und bei Ruecklauf
verworfen, nie halb ausgeliefert.

### C. Gas

- Gas wird vor der Wirkung belastet; bei Ruecklauf keine Erstattung.
- Unzureichendes Gas: Ruecklauf mit E03, kein Teilzustand, kein Teilereignis.
- Verifier-Kosten sind deterministisch, fallen bei Deployment an und werden
  nicht aus dem Ausfuehrungsbudget gedeckt.
- Verschachtelte Aufrufe erhalten ein Teilbudget; nicht verbrauchtes Budget
  faellt an den Aufrufer zurueck, die Summe ueberschreitet nie das Startbudget.
- Maximale Aufruftiefe 64; Ueberschreitung ist Ruecklauf mit E06.

### D. REVERTIF-Semantik (eindeutig)

Stapellage vor Ausfuehrung: `[..., Bedingung, Code]`, Code oben.
Die Bedingung entscheidet, ob zurueckgelaufen wird; der Code liefert den Grund.
Die verbindliche Wahrheitstabelle steht in Abschnitt 15.
REVERTIF springt nie. Es gibt keinen bedingten Kontrollfluss und kein Ziel.

### E. Verifier (erweitert)

Zusaetzlich zu den sechs Regeln aus Abschnitt 7:
- Statische Typen je Stapelstelle (I64, ADDR, BYTES, BOOL); Typfehler = ungueltig.
- Maximale Aufruftiefe 64 und maximale Stapeltiefe 4096.
- Speichergrenzen: 65536 Speicherplaetze, 65536 Operationen je Artefakt.
- Gas-Gueltigkeit: jeder erreichbare Pfad endet in HALT oder Ruecklauf.
- Kein erreichbarer Befehl nach Ruecklauf; Ruecklauf ist kein Sprungziel.

## 12. Legacy-Vertraege (v1)

Kompatibilitaet ist keine Sicherheitsfreigabe. Bis zur Bewertung gilt:
- v1-Vertraege werden inventarisiert (ID, Deployer, Artefakt-Hash, Hoehe).
- Vertraege mit sicherheitsrelevanten Bedingungen werden gekennzeichnet.
- Gekennzeichnete Vertraege werden geprueft; unsichere werden stillgelegt
  (Quarantaene) statt weiter ausgefuehrt.
- Migration nach v2 nur mit ausdruecklicher Autorisierung.

## 13. BC2-Nachweisliste

| ID | Anforderung | Nachweis |
| --- | --- | --- |
| BC2-001 | Versionserkennung und Kopfpruefung | Test: v2 im v1-Kopf abgelehnt |
| BC2-002 | ASSERT/REVERT/REVERTIF | Test: drei Wege, je ein Fehlercode |
| BC2-003 | Schreib-Set und atomarer Abschluss | Test: Abbruch laesst Zustand gleich |
| BC2-004 | Verschachtelte Aufrufe | Test: innerer Ruecklauf, aeusserer Erfolg |
| BC2-005 | Gas deterministisch | Test: gleicher Pfad, gleiches Gas |
| BC2-006 | Stapel- und Kontrollflusspruefung | Test: ungueltige Sprungziele abgelehnt |
| BC2-007 | Legacy-Kompatibilitaet | Test: v1-Artefakt behaelt Semantik |
| BC2-008 | Aktivierung und Migration | Test: vor/nach Hoehe, kein Auto-Umzug |
| BC2-009 | Differential und Negativ | Test: zwei Laeufe byte-gleich |

## 14. Vier Konsistenzpruefungen vor der Implementierung (Owner, 2026-11-09)

### 14.1 Aktivierung und Konsens

`atcb_v2_activation_height` ist Kettenparameter und fuer alle Validatoren
identisch; abweichender Wert = Konsensbruch, nicht Konfigurationsfrage.
Prueffaelle (Nachweis BC2-008):
- Block hoehe-1: v2-Artefakt wird abgelehnt.
- Aktivierungsblock: v2-Deployment zulaessig, v1-Deployment abgelehnt.
- Block hoehe+1: v1-Vertraege laufen unveraendert weiter.
- Reorganisation unter die Hoehe: v2-Effekte werden mit dem Block verworfen.
- Wiederherstellung aus Snapshot ueber die Hoehe hinweg: gleiche Entscheidung.

### 14.2 Verschachteltes Rollback

Fehlschlag im inneren Aufruf rollt zuerst den eigenen Frame zurueck.
Der Fehler wird als Rueckgabewert an den Aufrufer gereicht und ist dort
behandelbar. Wird er nicht behandelt, rollt die gesamte Transaktion zurueck.
Kein Zustand eines fehlgeschlagenen Frames darf nach aussen sichtbar sein,
auch nicht teilweise. Unbehandelter Fehler ist kein Erfolg mit Beigabe.

### 14.3 Gas und Fehlercodes

E06 ist fest reserviert fuer Aufruftiefe 64 und unveraenderlich.

**Hinweis 12.11.2026 (keine Aenderung dieser Entscheidung, R-901):** Der
SCE-Kandidat schlaegt in REQ-SCE-040 die Aufruftiefe 8 vor. Dieser Wert wuerde die
Owner-Entscheidung von 64 nur durch eine ausdrueckliche Aenderung ersetzen; die
Aufloesung ist offen. Bis dahin gilt hier unveraendert 64, und der SCE-Vorschlag
ist als Vorschlag zu lesen. Siehe `docs/decisions/EXEC-DECISION-TEMPLATE-2026-11-12.md`, E-1.
Gas wird weder erstattet noch doppelt verbucht, auch nicht durch
Fehlerbehandlung: einmal belastet bleibt belastet. Rueckgabe nicht
verbrauchten Teilbudgets ist keine Erstattung, sondern Rueckfluss an den
Aufrufer, und die Gesamtsumme ueberschreitet nie das Startbudget.

### 14.4 REVERTIF und Stapelsicherheit

Verifier und Interpreter setzen die Reihenfolge identisch um:
`[..., Bedingung, Code]`, Code oben, erst Code, dann Bedingung.
Abgelehnt werden: Stapelunterlauf, ungueltige Fehlercodes ausserhalb des
reservierten Raums, nicht erreichbare Kontrollflussbereiche nach HALT oder
Ruecklauf. Eine Abweichung zwischen Verifier und Interpreter ist ein Fehler,
kein Auslegungsspielraum.

## 15. REVERTIF — verbindliche Wahrheitstabelle

Befund aus der Code-Pruefung (09.11.): Die erste Umsetzung lief bei Bedingung
gleich 0 zurueck. Damit war REVERTIF ein ASSERT mit Fehlercode, also redundant,
und der Name traf das Verhalten nicht.

Entscheidung: Die Bedingung entscheidet, der Code ist der Grund.
Stapellage `[..., Bedingung, Code]`, Code oben, erst Code, dann Bedingung.

| Bedingung | Code | Verhalten |
| --- | --- | --- |
| 0 | 0 | weiterlaufen, Code verfaellt |
| 0 | 0x0100..0x01FF | weiterlaufen, Code verfaellt |
| ungleich 0 | 0 | Ruecklauf, allgemeiner Grund (E02) |
| ungleich 0 | 0x0100..0x01FF | Ruecklauf mit angegebenem Grund |

Abgrenzung: ASSERT bricht bei falscher Bedingung ab und kennt keinen Code.
REVERTIF bricht bei wahrer Bedingung ab und nennt einen Grund. Zwei
verschiedene Waechter, keine doppelte Instruktion.

Code-Raum: 0 = allgemein, 0x0100..=0x01FF = reserviert, alles andere unzulaessig
(0x00FF, 0x0200 und hoeher werden von Verifier UND Interpreter abgelehnt,
auch wenn ein Programmpfad den Verifier umgeht).

## 16. Kanonisches Artefakt (Umsetzung P0-2)

Die Generation ist Teil des Artefakts und Teil seines Hashes. Ein Aufrufer kann
sie nicht setzen. Aufbau, alles Little Endian:

| Feld | Bytes | Inhalt |
| --- | --- | --- |
| magic | 4 | `ATCB` |
| Containerformat | 1 | 0x01 |
| Generation | 1 | 1 = ATCB-v1, 2 = ATCB-v2 |
| Operationszahl | 4 | u32 |
| Operationsstrom | n | siehe Tabelle |
| Hash | 32 | SHA-256 ueber alle vorhergehenden Bytes |

Befehlscodes: 0x01 Push(u64), 0x02 Add, 0x03 Sub, 0x04 Mul, 0x05 Div, 0x06 Dup,
0x07 Swap, 0x08 Eq, 0x09 Lt, 0x0A Load(u32), 0x0B Store(u32), 0x0C Caller,
0x0D Jump(u32), 0x0E JumpIfZero(u32), 0x0F JumpIfNotZero(u32), 0x10 Halt,
0x11 Assert, 0x12 Revert(u32), 0x13 RevertIf (v2 ab 0x11).

Regeln beim Lesen:
1. Nur Generation 1 und 2. Jeder andere Wert wird abgelehnt.
2. v2-Befehle in einem v1-Artefakt werden beim Lesen abgelehnt, nicht erst bei
   der Ausfuehrung.
3. Unbekannter Befehlscode, zu kurze Eingabe, falsche Kennung und ueberzaehlige
   Bytes werden abgelehnt.
4. Der Hash deckt Generation und Befehlscodes ab. Ein geaendertes
   Generationsbyte ergibt einen anderen Hash und damit ein ungueltiges Artefakt.
5. Es gibt keine Konvertierung zwischen Generationen.

Offen (BC2-010, nicht Teil von P0-2): Signaturbindung des Artefakts. Der Hash
ist die Grundlage; die Signatur ueber den Hash folgt mit ATC-SEC-EXEC-001.

## 17. Fehlercode-Zuordnung (normativ, 2026-11-09)

Verbindliche Entscheidung des Owners. Diese Zuordnung ist normative Grundlage
fuer die spaetere Umsetzung. Sie ist **kein Nachweis** einer implementierten
Fehlerklassifikation: Im VM-Crate existiert derzeit keine numerische Kennung,
und es wurde in diesem Schritt kein Code geaendert.

| Kennung | Bedeutung | Verhalten | Stand |
| --- | --- | --- | --- |
| E01 | Pruefung fehlgeschlagen | Ruecklauf | bestehend |
| E02 | Ruecklauf angefordert | Ruecklauf | bestehend |
| E03 | Gasbudget reicht nicht aus | Ruecklauf, Gas vollstaendig verbraucht | bestehend, bestaetigt |
| E04 | Stapelgrenze ueberschritten | Ruecklauf | bestehend, bestaetigt, ausschliesslich Stapelgrenze |
| E05 | Speichergrenze ueberschritten | Ruecklauf | bestehend |
| E06 | Maximale Aufruftiefe ueberschritten (Grenze 64) | Ruecklauf | bestehend, ausschliesslich Aufruftiefe |
| NEU-01 | Unbekannter Befehl | Abbruch, ungueltiges Programm | Kennung noch nicht vergeben |

**Korrigierte Fehlzuordnungen:** Abschnitt 11 C ordnete unzureichendes Gas der
Kennung E04 zu. Das war falsch, weil E04 die Stapelgrenze bezeichnet; richtig ist
E03. Die fruehere Bedeutung von E06 als "unbekannter Befehl" war ebenfalls
falsch, weil E06 unveraenderlich fuer die Aufruftiefe reserviert ist.

**Nicht umnummeriert:** Keine bereits vergebene Kennung wurde verschoben. E01,
E02, E03, E04 und E05 behalten ihre Bedeutung; E06 behaelt seine Reservierung,
nur die zweite, widerspruechliche Zuordnung entfaellt.

**Offen (NEU-01):** Fuer den unbekannten Befehl wird eine eigene Kennung erst
nach vollstaendiger Inventur aller vergebenen Codes festgelegt. Solange das
nicht geschehen ist, darf keine Nummer vergeben und keine Kennung im Code
verwendet werden.

**Abgrenzung zu Ruecklauf-Codes:** Vertrags-Ruecklaufcodes bleiben im Raum 0 und
0x0100-0x01FF und sind von diesen Kennungen getrennt. Ruecklauf ist ein
behandelbares Ergebnis, ein Abbruch wie NEU-01 nicht.

**Praeisierung NEU-01 (Ladephase, nicht Laufzeit):** Ein unbekannter Befehl kann
waehrend der Ausfuehrung nicht auftreten. Die Instruktion ist ein typisierter
Wert; unbekannte Bytes werden beim Lesen des Artefakts abgelehnt, bevor ein
Programm entsteht. NEU-01 bezeichnet deshalb die Ablehnung bei Annahme und
Ladung, nicht einen Laufzeitfehler, und gehoert nicht in die Ruecklauf-Kennungen
der Ausfuehrung.

## 18. Gastabelle GAS_TABLE_V1 (normativ, 2026-11-09)

Verbindliche Kosten der Generationen 1 und 2. Die Tabelle ist unveraenderlich;
jede Aenderung erzeugt eine neue Kennung (GAS_TABLE_V2 und so fort).

| Opcode | Kosten | Anmerkung |
| --- | --- | --- |
| Push | 1 | |
| Dup | 1 | |
| Swap | 1 | |
| Caller | 1 | |
| Add | 2 | |
| Sub | 2 | |
| Mul | 2 | |
| Div | 2 | Division durch 0 ist Laufzeitfehler, Belastung bleibt |
| Eq | 2 | |
| Lt | 2 | |
| Load | 3 | |
| Store | 3 | flach, unabhaengig vom Speicherwachstum |
| Jump | 2 | |
| JumpIfZero | 2 | |
| JumpIfNotZero | 2 | |
| Halt | 0 | ausdruecklich kostenlos, kein impliziter Wert |
| Assert | 2 | ab v2 |
| Revert | 1 | ab v2, codeunabhaengig |
| RevertIf | 3 | ab v2 |

**G-01 Vollstaendigkeit:** Alle 19 Varianten haben genau einen Kostenwert. Ein
neuer Befehl ohne Eintrag ist unzulaessig; die Tabelle ist erschoepfend.

**G-02 Versionierung:** Die Kennung ist `GAS_TABLE_V1`. Sie ist unveraenderlich
und wird bei jeder Kostenentscheidung fortgeschrieben, nie ueberschrieben.

**G-03 Abrechnung:** Gas wird vor der Wirkung der jeweiligen Instruktion
belastet. Eine Instruktion, deren Kosten nicht gedeckt sind, wirkt nicht.

**G-04 Fehlerverhalten:** Unzureichendes Gas ergibt E03. Die Zustandsänderung
wird zurueckgenommen; das Budget gilt als verbraucht. Die derzeitige
Implementierung meldet `required` und `remaining`, nullt das Budget aber nicht.
Beobachtbar ist der Unterschied nicht, weil das Budget lokal ist; festgelegt ist
der Verbrauch.

**G-05 Keine impliziten Kosten:** `Halt` ist mit 0 und `Store` mit 3 ausdruecklich
bepreist. Speicherwachstum ist in dieser Fassung **nicht** bepreist; eine
Bepreisung ist eine neue Tabellenversion (GAS_TABLE_V2) und keine stille
Ergaenzung.

**G-06 Regression:** Jede der 19 Varianten erhaelt einen eigenen Kostentest
sowie einen Test des Belastungszeitpunkts. Die Testsuite ist Teil des Nachweises,
nicht die Tabelle allein.

**G-07 Aenderungsschutz:** Eine Kostenänderung erfordert eine neue
Tabellenkennung, einen neuen Nachweis und eine Festlegung, ab welcher Blockhoehe
sie gilt. Bestehende Ausfuehrungsergebnisse duerfen nicht stillschweigend
abweichen.

**Nicht Teil dieser Tabelle:** Die Schrittobergrenze fuer budgetfreie Ausfuehrung.
Sie ist ein eigener Sicherheitsblocker (SB-01 in
`docs/audits/VM-SECURITY-BLOCKERS.md`) und wird nicht durch die Gastabelle
geloest. Ein Gasbudget begrenzt nur, wo eines uebergeben wird.

**BC2-011 (entschieden 2026-11-09):** Die Gastabellenkennung ist ein
**Protokollparameter der Kettenkonfiguration** und gehoert nicht in das
Artefakt. Damit bleiben Artefaktformat und Hashbindung unveraendert; die
Abrechnung ergibt sich aus dem verbindlichen Protokollzustand, nicht aus dem
Bytecode.

Verbindliche Regeln:
- Validatoren verwenden fuer denselben Block dieselbe Tabellenkennung. Zwei
  Validatoren mit verschiedenen Tabellen sind keine gueltige Konfiguration.
- Eine Aenderung der Gastabelle ist eine Protokollaktivierung, keine lokale
  Einstellung. Sie erfolgt explizit, nie implizit.
- Die VM uebernimmt die Kennung ausschliesslich aus dem verbindlichen
  Ausfuehrungskontext. Ein beliebiger Aufrufer darf sie nicht als Parameter
  setzen; ein Aufruferparameter waere eine Umgehung der Abrechnung.

**Bindung an den Protokollzustand:** Die Auswahl der Gastabelle wird an die
Blockhoehe beziehungsweise den verbindlichen Protokollzustand gebunden, nach
demselben Aktivierungsplan wie eine Versionsumstellung (Abschnitt 11 A). Bei
gleicher Blockhoehe muss jeder Validator dieselbe Tabelle anwenden. Ohne diese
Bindung koennten Validatoren identischen Bytecode mit unterschiedlichen
Kostenmodellen abrechnen und sich uneinig werden, obwohl derselbe Code laeuft.

**Umsetzungsstand:** Im Crate existiert genau eine Tabelle als benannte
Konstante `GAS_TABLE_ID` mit `GAS_TABLE_V1`. Es gibt heute keinen Auswahlpunkt,
an dem ein Aufrufer eine andere Tabelle vorgeben koennte. Die Anbindung an
`ChainContext` und die Kettenintegration sind **nicht** nachgewiesen und bleiben
offen; sie sind Voraussetzung fuer eine zweite Tabellenversion.

## 19. Ausfuehrungsgrenzen SB-01 (zur Pruefung vorgelegt, 2026-11-09)

**Status: normativ ausgearbeitet, Implementierung NICHT freigegeben.** Dieser
Abschnitt wird vor jeder Codeaenderung geprueft. Er entscheidet D1 und D2 nach
Owner-Vorgabe und schlaegt den Grenzwert mit Begruendung vor.

**Fundstelle dieses Dokuments:** Repository `A-TownChain-Okosystems/a-townchain`,
Pfad `standards/atc/candidates/ATC-BC-002-CANDIDATE.md`. Es liegt **nicht** im
Repository `a-townchain-ecosystem` und nicht in `atc-standards`; dort ist der
Commit nicht aufloesbar.

### 19.1 Zwei getrennte Schutzaufgaben

| Mechanismus | Aufgabe | Reichweite |
| --- | --- | --- |
| Gasbudget | normative Ressourcenabrechnung | konsensrelevante Ausfuehrung ohne gueltiges Budget ist unzulaessig |
| Schrittobergrenze | harte Grenze gegen Endlosschleifen und fehlerhafte Gasannahmen | gilt unabhaengig davon, ob Gas abgerechnet wird |

Die Schrittobergrenze ist **kein Ersatz** fuer das Pflichtbudget und das
Pflichtbudget **kein Ersatz** fuer die Schrittobergrenze. Gas begrenzt nur, wo
ein Budget existiert; die Schrittobergrenze begrenzt immer.

### 19.2 D1 — Regeln

- Jede Ausfuehrung besitzt eine deterministische Schrittobergrenze, auch `run()`
  ohne Gasbudget, `run_with_gas()` und `execute_state_transition_with_gas()`.
- Die Grenze ist protokollweit identisch. Sie ist kein Aufruferparameter und
  nicht lokal konfigurierbar.
- Ein lokaler Timeout oder eine CPU-Zeitbegrenzung ist kein Ersatz. Zeit ist
  nicht deterministisch und damit nicht konsensfaehig.
- Die Pruefung erfolgt **vor** der Wirkung der jeweiligen Instruktion, in der
  Reihenfolge aus Abschnitt 19.4.
- **HALT zaehlt als Schritt.** Die Schrittzahl umfasst jede ausgefuehrte
  Instruktion einschliesslich HALT. HALT kostet 0 Gas, wird aber mitgezaehlt.
- Ein Dev-Modus darf keine unbegrenzte Ausfuehrung anbieten. Hoechstens eine
  explizit gekennzeichnete, protokollfest definierte Grenze.
- Konsensrelevante Ausfuehrung verlangt zusaetzlich ein gueltiges Gasbudget.

### 19.3 D2 — Kennung

| Kennung | Bedeutung | Verhalten |
| --- | --- | --- |
| E03 | OutOfGas | Ruecklauf, Gas vollstaendig verbraucht |
| E04 | StackLimitExceeded | Ruecklauf |
| E06 | CallDepthExceeded | Ruecklauf |
| NEU-01 | Unbekannter Opcode beim Artefaktladen | Ladefehler, kein Laufzeitfehler |
| NEU-02 (Platzhalter, **nicht vergeben**) | ExecutionStepLimitExceeded | Abbruch mit Rollback |

Regeln: eigene Kennung, getrennt von E03 und E06. Die numerische Kennung wird
erst nach vollstaendiger Inventur aller bestehenden Codes vergeben, ohne
Umnummerierung und ohne Wiederverwendung. Das Erreichen der Schrittobergrenze
darf **nicht** als E03 gemeldet werden, auch wenn gleichzeitig Gas fehlt.

### 19.4 Verhalten beim Erreichen der Grenze

- Die Instruktion, welche die Grenze ueberschreiten wuerde, wird nicht
  ausgefuehrt.
- Die Ausfuehrung endet deterministisch mit ExecutionStepLimitExceeded.
- Bereits erfolgte Zustandsänderungen werden zurueckgenommen.
- Es entstehen keine teilweise bestaetigten Zustandsänderungen.
- Das Verhalten ist fuer gaslose und gasgebundene Ausfuehrung identisch.
**Verbindliche Reihenfolge je Instruktion** (vier Schritte, in dieser Ordnung):

1. Pruefen, ob noch ein Ausfuehrungsschritt verfuegbar ist.
2. Schrittzaehler nach dem Zaehlmodell fortschreiben.
3. Gasbelastung pruefen und durchfuehren.
4. Instruktion ausfuehren.

**Fehlerprioritaet bei gleichzeitiger Erschoepfung:** Es gewinnt die
Schrittobergrenze. Das ist eine bewusst gewaehlte Prioritaet, kein Zufall der
Implementierung: Der Befund darf nicht vom Kostensatz des jeweiligen Befehls
abhaengen, und die Schrittgrenze ist die Grenze, die unabhaengig von jeder
Gasannahme gilt. Gas bleibt in diesem Fall unbelastet, weil die Instruktion nicht
wirkte. Diese Prioritaet ist zu testen (Abschnitt 19.7).

### 19.5 Grenzwert — Vorschlag 1 000 000 Schritte

**Begruendung, drei unabhaengige Argumente:**

1. **Kompatibilitaet (harte Untergrenze).** Repo-weite Inventur (nicht nur
   `components/vm`): alle Aufrufe von `run_with_gas` und
   `execute_state_transition_with_gas` liegen in `components/vm`, es gibt keinen
   Aufrufer ausserhalb. Groesstes Budget: **100 000** (zwei Aufrufe in
   `tests/atcb_v2.rs`); die uebrigen liegen bei 0, 1, 2, 3, 4, 5, 10 und 100.

   Die Formel gilt unter drei ausdruecklichen Voraussetzungen:
   (i) jede ausgefuehrte Instruktion ausser HALT kostet mindestens 1 Gas;
   (ii) HALT kostet 0 Gas und wird hoechstens einmal ausgefuehrt;
   (iii) die Schrittzahl umfasst auch HALT.

   Daraus folgt `Schritte <= Gasverbrauch + 1`. Bei einem Hoechstbudget von
   100 000 ist die Obergrenze **100 001 Schritte**; 1 000 000 veraendert damit
   keine bestehende gasgebundene Ausfuehrung.

   **Grenze der Aussage:** Die Inventur deckt dieses Repository. Das VM-Crate hat
   heute ausserhalb seiner eigenen Tests keine Nutzer, also gibt es auch keine
   groesseren Budgets aus Vertraegen. Sobald die Kette die VM produktiv aufruft,
   ist die Inventur zu wiederholen; der Wert ist dann gegen die tatsaechlichen
   Budgets erneut zu pruefen.
2. **Kosten einer einzelnen Ausfuehrung (Messung).** Gemessen in einem
   Release-Build auf dieser Maschine: 4,1 bis 5,0 x 10^8 Schritte pro Sekunde,
   ermittelt mit Gas als Schrittzaehler (Gas geteilt durch Kostensatz), weil es
   keinen Schrittzaehler gibt. 1 000 000 Schritte kosten damit rund 2 bis 2,5 ms.
   Bei einem hundertfach langsameren Ziel (Debug-Build, schwache Hardware,
   unguenstige Verzweigung) bleiben es 200 bis 250 ms.

   **Was die Messung nicht belegt:** Sie gilt fuer diese Maschine und diesen
   Build. Sie ist kein Nachweis der Sicherheit unter unguenstigen
   Validatorbedingungen. Sie belegt **nicht**, dass 1 000 000 Schritte fuer einen
   Angreifer unrentabel sind. Diese Aussage wird hiermit zurueckgenommen: Ein
   Angreifer kann viele begrenzte Ausfuehrungen hintereinander ausloesen; die
   Grenze pro Ausfuehrung sagt darueber nichts. Was eine solche Aussage stuetzen
   wuerde, steht in Abschnitt 19.8.
3. **Verhaeltnis zur Programmobergrenze.** Ein Artefakt fasst hoechstens 65 536
   Befehle. 1 000 000 Schritte erlauben rund 15 vollstaendige Durchlaeufe eines
   maximalen Programms. Wer mehr braucht, braucht echtes Gas und ist in der
   Abrechnung sichtbar, nicht im Sicherheitsnetz.

**Verworfene Alternativen:** 100 000 (zu nah an der heutigen Obergrenze, kein
Sicherheitsabstand); 10 000 000 (rund 24 ms im Release, ohne Gewinn, weil
legitime Ausfuehrung ohnehin durch Gas begrenzt ist); 65 536 (Programmlaenge ist
keine Schrittzahl, Schleifen vervielfachen sie); kein Wert, nur Pflichtbudget
(laesst `run()` unbegrenzt und damit SB-01 offen).

**Was der Wert nicht ist:** keine Laufzeitgarantie. Die Schrittzahl ist
deterministisch und konsensfaehig; die Laufzeit haengt von der Maschine ab und
ist kein Konsensgegenstand. Die Messung begruendet nur, dass die Grenze
operativ tragbar ist.

**Vorbehalt zur Messmethode:** Der Durchsatz wurde mit Gas als Zaehler
bestimmt, nicht mit einem Schrittzaehler, den es noch nicht gibt. Fuer die drei
verwendeten Programme mit einheitlichen Kostensaetzen ist die Umrechnung exakt;
fuer gemischte Programme ist sie eine Abschaetzung.

### 19.6 Auswirkung auf bestehende Vertraege

Keine. Gueltige Programme, die heute terminieren, terminieren weiter, weil die
Grenze oberhalb jedes im Repo verwendeten Budgets liegt und fuer gasgebundene
Ausfuehrung ohnehin `Schritte <= Gas + 1` gilt. Ein Programm, das heute innerhalb
von 100 000 Gas durchlaeuft, kann die Grenze nicht erreichen. Ausdruecklich
ausgenommen und als Risiko benannt: Programme, die heute endlos laufen
(`Jump(0)` ohne Budget) werden kuenftig abgewiesen. Das ist die Absicht und der
einzige Verhaltensunterschied.

### 19.7 Testmatrix (verbindlich fuer die spaetere Umsetzung)

| Fall | Pfad | Erwartung |
| --- | --- | --- |
| `Jump(0)` ohne Budget | `run()` | begrenzter Abbruch mit ExecutionStepLimitExceeded |
| `Jump(0)` mit Budget | `run_with_gas()` | Abbruch durch die zuerst erreichte Grenze |
| genau erlaubte Schrittzahl | beide | Erfolg, wenn das Programm regulaer endet |
| ein Schritt darueber | beide | Abbruch vor der Wirkung der naechsten Instruktion |
| Fehler nach `Store` | beide | vollstaendiger Rollback, kein Teilzustand |
| gleiche Grenze auf allen Einstiegen | `run`, `run_with_gas`, Kettenpfad | identische Schrittzaehlung |
| zwei Laeufe, gleiches Programm | alle | identisches Ergebnis und identische Schrittzahl |
| Grenze nicht als OutOfGas gemeldet | beide | eigene Kennung, auch bei gleichzeitig fehlendem Gas |
| gleichzeitige Erschoepfung beider Grenzen | beide | Schrittgrenze gewinnt (Fehlerprioritaet 19.4), Gas bleibt unbelastet |
| HALT als letzter Schritt | beide | HALT wird mitgezaehlt, ein Programm mit genau `S` Instruktionen inklusive HALT laeuft durch |

Zusaetzlich: Der Grenzwert wird als benannte Konstante gefuehrt (analog
`GAS_TABLE_ID`), damit eine Aenderung sichtbar und nur per
Protokollaktivierung moeglich ist.

### 19.8 Abgrenzung: Ausfuehrungslimit und aggregiertes Blocklimit

Die Schrittobergrenze aus 19.2 ist ein **Limit pro Ausfuehrung**. Sie begrenzt
nicht die Rechenlast eines Blocks. Ein Block mit vielen gueltigen Transaktionen
kann die Grenze je Transaktion beliebig oft ausschoepfen; die Summe ist durch
diese Norm nicht gedeckt.

**Nicht durch SB-01 gedeckt und ausdruecklich offen (SB-02):**
- Blockgaslimit beziehungsweise Blockschrittlimit als Obergrenze je Block.
- Wechselwirkung mit Transaktionsrate und Mempool-Annahme.
- Admission-Control: welche Transaktionen werden ueberhaupt angenommen.
- Angreiferkosten als Produkt aus Grenze, Rate, Gebuehren und Validatorleistung.

**Was eine Aussage ueber Angreiferrentabilitaet braucht:** eine Rechnung ueber
die genannten Groessen mit gemessenen Werten je Validator-Klasse, nicht eine
Messung einer einzelnen Ausfuehrung. Bis dahin gilt: SB-01 verhindert
Endlosschleifen und begrenzt eine einzelne Ausfuehrung; es ist kein
Doors-Schutz gegen Lastvielfache.

**Konsequenz fuer den Grenzwert:** Der Vorschlag 1 000 000 bleibt davon
unberuehrt, weil er nur die Einzelausfuehrung betrifft. Das Blocklimit ist eine
eigene Entscheidung mit eigener Norm und wird nicht durch die Wahl dieser Zahl
mitentschieden.
