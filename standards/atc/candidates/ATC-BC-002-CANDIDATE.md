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

## 10. Offene Entscheidungen

1. Fehlercodes: feste Liste (oben) oder offener Zahlenraum mit Registrierung?
2. Ruecklauf-Ebene als Vertragsattribut oder je Aufruf deklariert?
3. Schreib-Set je Frame oder globale Liste mit Frame-Marken?
4. Gas-Vorabberechnung: je Befehl konstant oder abhaengig von Operanden?
5. Migrationspolitik der Kette: Zwangsumstellung je Blockhoehe oder je Vertrag?
