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
| REVERTIF | zieht Bedingung, Code | Ruecklauf nur bei 0 |

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
| E06 | Unbekannter Befehl | Abbruch, gilt als ungueltig |

Codes sind Teil des Ergebnisses und werden im Beleg mitgeschrieben.

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
- Unzureichendes Gas: Ruecklauf mit E04, kein Teilzustand, kein Teilereignis.
- Verifier-Kosten sind deterministisch, fallen bei Deployment an und werden
  nicht aus dem Ausfuehrungsbudget gedeckt.
- Verschachtelte Aufrufe erhalten ein Teilbudget; nicht verbrauchtes Budget
  faellt an den Aufrufer zurueck, die Summe ueberschreitet nie das Startbudget.
- Maximale Aufruftiefe 64; Ueberschreitung ist Ruecklauf mit E06.

### D. REVERTIF-Semantik (eindeutig)

Stapellage vor Ausfuehrung: `[..., Bedingung, Code]`, Code oben.
1. Code wird gezogen, dann Bedingung.
2. Bedingung ungleich 0: Ausfuehrung laeuft weiter, Code verfaellt.
3. Bedingung gleich 0: sofortiger Ruecklauf mit dem gezogenen Code.
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
