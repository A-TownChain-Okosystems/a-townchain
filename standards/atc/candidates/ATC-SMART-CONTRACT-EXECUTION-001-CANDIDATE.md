# ATC-SMART-CONTRACT-EXECUTION-001 — Integrationsvertrag Compiler, VM und Blockchain

> **Standard-ID:** ATC-SMART-CONTRACT-EXECUTION-001
> **Kategorie:** Execution / Smart Contracts
> **Tier:** 0 (Kandidat fuer foundational)
> **Status:** KANDIDAT, **nicht normativ.** Keine Implementierungspflicht, keine
> Ratifizierung, keine Registry-Bindung vor der Owner-Freigabe nach ATC-STD-000 §9.
> **Version:** 1.0.0-draft
> **Datum:** 2026-11-12
> **Autor:** Michael Wroblewski (Owner-Vorgabe), Entwurf durch Aurora
> **Grundlage:** `docs/audits/ATCLANG-VM-CHAIN-INTEGRATION-2026-11-11.md`,
> `docs/audits/CONTRACT-EXECUTION-INVENTORY-2026-11-12.md`,
> `docs/decisions/AD-EXEC-D1-D7.md`
> **Abhaengigkeiten:** ATC-BC-001 (`SPEC-FROZEN`), ATC-BC-002 (Kandidat),
> ATC-SEC-EXEC-001 (Kandidat), ATC-STD-600 (approved), ATC-STD-SC-001..020,
> ATC-14 (Determinismus), ATC-STD-000 (Verfassung, §9, §30)

---

## 1. Zweck

Dieser Standard ist der **eine** verbindliche Vertrag zwischen drei Komponenten,
die heute jeweils fuer sich funktionieren und zusammen nicht:

- **ATCLang** beschreibt, was ausgefuehrt werden soll.
- **ATC-VM** fuehrt deterministisch aus.
- **A-TownChain** macht die Ausfuehrung zur gueltigen Transaktion.

Er legt nicht die Interna der drei Komponenten fest. Er legt fest, **was zwischen
ihnen gilt**: Format, Aufruf, Speicher, Gas, Identitaet, Ergebnis und die Frage,
wer die Ausfuehrung prueft. Ziel ist ein Zustand, in dem ein Vertrag, der in
ATCLang geschrieben ist, auf jedem Validator dasselbe Ergebnis und dieselbe
Zustandswurzel erzeugt, und in dem ein abweichendes Ergebnis den Block ungueltig
macht.

**Leitsatz:** Spezifikation, Implementierung und Version sind getrennte Dinge.
Dieser Vertrag ist die Spezifikation. Ob eine Implementierung ihn erfuellt, sagt
die Evidenzstufe in Abschnitt 10, nicht die Existenz dieses Dokuments.

## 2. Geltungsbereich

**Im Geltungsbereich:** jede Stelle, an der ein Vertrag entsteht, geladen,
ausgefuehrt, abgerechnet oder als Zustand bestaetigt wird, im A-TownChain-L1 und
im Devnet.

**Nicht im Geltungsbereich:** Sprache und Typsystem (ATCLang-Spec), Aufnahme- und
Lizenzrecht im Detail (ATC-SEC-EXEC-001, ATC-LIC), Oekonomie der Gebuehrenhoehe,
Konsensverfahren selbst (ATC-STD-202, `components/algorithm`), Kernel und
Produkt-Apps.

## 3. Begriffe

| Begriff | Definition |
|---|---|
| **Instruktionsstrom** | die kanonische Befehlsfolge nach ATC-BC-001 |
| **Artefakt** | Huelle um einen Instruktionsstrom mit Generation, Laenge, Layout-Bezug und Integritaetsnachweis |
| **Vertragsadresse** | deterministisch abgeleitete Kennung eines deployten Vertrags |
| **Aufruf** | Adresse, Selektor, Calldata, Wert, Gasbudget, Aufruferidentitaet |
| **Frame** | ein Ausfuehrungsrahmen mit eigenem Codezeiger, Lokalspeicher, Rueckgabepuffer und Schreib-Set |
| **Slot** | persistente 32-Byte-Zelle nach dem ABI-Layout des Vertrags |
| **Beleg (Receipt)** | Ergebnis einer Ausfuehrung: Status, Rueckgabedaten, Gasverbrauch, Zustandswurzelbezug |
| **Zustandswurzel** | kryptografische Zusammenfassung des bestaetigten Zustands |

## 4. Normative Objekte

Der Vertrag definiert acht Objekte. Jedes hat genau eine kodierte Form und genau
eine Version.

1. **Artifact** (`SCE-ARTIFACT`): Strom plus Huelle. Die Huelle traegt Magic,
   Generation, Stromlaenge, Layout-Hash und SHA-256. Die Huelle ist **kein**
   zweiter Instruktionssatz: die Bytes des Stroms sind exakt die Bytes nach
   ATC-BC-001.
2. **CallEnvelope** (`SCE-CALL`): Adresse, Selektor, Calldata, Wert, Gasbudget,
   Aufruferidentitaet, Noncebezug.
3. **ExecutionResult** (`SCE-RESULT`): Statuscode, Rueckgabepuffer, Gasverbrauch,
   Ereignisse, Schreib-Set-Bezug.
4. **GasContract** (`SCE-GAS`): Kennung, Instruktionskosten, Speicherkosten,
   Deploymentkosten, Verbrauchsregel.
5. **StateLayout** (`SCE-LAYOUT`): Vertragsadresse, Layout-Hash, Slotliste,
   Sichtbarkeit je Slot.
6. **IdentityValue** (`SCE-IDENT`): 32-Byte-Wort mit der 20-Byte-Adresse nach
   SEC-P0-005.
7. **AdmissionRef** (`SCE-ADMIT`): Verweis auf die Freigabe nach
   ATC-SEC-EXEC-001, damit der Kettenpfad dieselbe Entscheidung prueft wie das
   Deployment.
8. **BlockExecution** (`SCE-BLOCK`): Belegliste, Zustandswurzel vorher/nachher,
   Aufnahmeverweis, Gasabrechnung je Transaktion.

## 5. Anforderungen

### 5.1 Ein Format (D1)

- **REQ-SCE-001:** Es gibt genau einen normativen Instruktionsstrom, ATC-BC-001.
  Jede Implementierung, die einen anderen Strom liest, ist nicht konform.
- **REQ-SCE-002:** Die Huelle hat ein eigenes Magic und ist nicht mit dem Strom
  verwechselbar. Sie traegt Generation, Stromlaenge, Layout-Hash und SHA-256.
- **REQ-SCE-003:** Ein Artefakt deklariert seine Generation. Ein Verifier lehnt
  Befehle ab, die in der deklarierten Generation nicht existieren, und liest kein
  Artefakt einer fremden Generation still um.
- **REQ-SCE-004:** Die Laengenangabe der Huelle und die tatsaechliche Stromlaenge
  muessen uebereinstimmen; Abweichung ist ein Fehler, keine Warnung.

### 5.2 Ein Frontend (D7)

- **REQ-SCE-005:** Im Produktionspfad entsteht jeder Vertrag aus ATCLang. Ein
  anderer Weg ist ausschliesslich Testwerkzeug und im Register als solches gefuehrt.
- **REQ-SCE-006:** Ein Vertrag, der im Kettenziel nicht abbildbar ist, wird
  abgelehnt; er wird nicht still vereinfacht.

### 5.3 Aufruf (D2)

- **REQ-SCE-007:** Ein Aufruf nennt Vertragsadresse, Selektor, Calldata, Wert,
  Gasbudget und Aufruferidentitaet. Fehlt eines dieser Felder, ist der Aufruf
  ungueltig.
- **REQ-SCE-008:** Der Selektor ist die ersten 4 Byte des SHA-256 der kanonischen
  Funktionssignatur aus dem Vertrags-ABI.
- **REQ-SCE-009:** Calldata ist eine kanonische Wortfolge mit expliziter Laenge.
  Eine falsche Laenge ist ein Fehler, kein Abschneiden.
- **REQ-SCE-010:** Ein unbekannter Selektor ist ein Fehler mit eigenem Statuscode,
  kein Leerlauf.
- **REQ-SCE-011:** Ein Vertrag-zu-Vertrag-Aufruf ist erst zulaessig, wenn die
  Ruecklauf-Semantik der Generation ratifiziert ist. Ohne Ruecklauf kein Aufruf.
- **REQ-SCE-012:** Die Rueckgabe ist ein Puffer mit Status, Laenge und Daten. Eine
  zu grosse Rueckgabe bricht ab.

### 5.4 Speicher (D3)

- **REQ-SCE-013:** Lokale Variablen leben in einem begrenzten Frame und sind nach
  dem Aufruf nicht mehr vorhanden.
- **REQ-SCE-014:** Persistent sind ausschliesslich Slots nach dem ABI-Layout des
  Vertrags, 32 Byte je Slot, adressiert ueber den Layout-Hash.
- **REQ-SCE-015:** Der Layout-Hash steht im Artefakt. Passt er nicht zum Zustand,
  wird nicht ausgefuehrt.
- **REQ-SCE-016:** Nicht abbildbare Typen (Maps, dynamische Felder) werden im
  Kettenziel abgelehnt, solange kein dafuer spezifizierter Befehl existiert.
- **REQ-SCE-017:** Schreibzugriffe werden je Frame gesammelt und erst bei Erfolg
  uebernommen (Schreib-Set).

### 5.5 Identitaet (D6)

- **REQ-SCE-018:** Die Aufruferidentitaet ist ein 32-Byte-Wort mit der 20-Byte-
  Adresse aus SEC-P0-005. Eine numerische Ersatzkennung gibt es nicht.
- **REQ-SCE-019:** Die Adresse ist an den oeffentlichen Schluessel gebunden; eine
  Schluesselrotation darf die bestehende Adresse nicht rueckwirkend aendern.
- **REQ-SCE-020:** Ohne gueltige Bindung zwischen Schluessel, Adresse und
  Transaktion wird nicht ausgefuehrt.

### 5.6 Gas und Gebuehren (D4)

- **REQ-SCE-021:** Es gibt genau einen Gastvertrag mit genau einer Kennung je
  Generation. Zwei parallel gueltige Kostentabellen sind unzulaessig.
- **REQ-SCE-022:** Das Budget stammt aus der Transaktion. Ein Systemstandardwert
  darf es nicht ersetzen.
- **REQ-SCE-023:** Kosten werden vor der Wirkung belastet. Ein Fehlschlag erstattet
  keine Ausfuehrungszeit.
- **REQ-SCE-024:** Der Verbrauch steht im Beleg, auch bei Erfolg.
- **REQ-SCE-025:** Eine Gebuehr wird abgebucht, nicht nur berechnet. Ohne
  Kontofuehrung ist der Gastvertrag nicht erfuellt.
- **REQ-SCE-026:** Erschoepftes Budget ist ein Ruecklauf mit vollstaendigem
  Verbrauch, kein Teilzustand.

### 5.7 Ausfuehrung im Block (D5)

- **REQ-SCE-027:** Ein Block traegt die Belege seiner Vertragsausfuehrungen und die
  Zustandswurzel nach der Ausfuehrung.
- **REQ-SCE-028:** Jeder Validator rechnet die Ausfuehrung nach. Ein abweichendes
  Ergebnis macht den Block ungueltig. Ein Hash ueber die Transaktionsliste ist
  kein Ersatz.
- **REQ-SCE-029:** Die Zustandswurzel ist deterministisch ueber Konten und Slots
  gebildet, mit Domain-Trennung und ohne Umgebungsabhaengigkeit.
- **REQ-SCE-030:** Die Aufnahme (Admission) nach ATC-SEC-EXEC-001 wird im
  Kettenpfad geprueft, nicht nur beim Deployment.
- **REQ-SCE-031:** Ein Block, dessen Ausfuehrung nicht nachvollziehbar ist, wird
  abgelehnt. "Nicht nachvollziehbar" umfasst fehlende Belege.

### 5.8 Determinismus

- **REQ-SCE-032:** Verboten in jeder ausfuehrenden Stufe sind: Systemzeit,
  Zufall, Netzwerkzugriff, Dateisystemzugriff, Umgebungsvariablen, nicht
  deterministische Iterationsreihenfolgen, Zeigerwerte und Adressen von
  Speicherobjekten.
- **REQ-SCE-033:** Arithmetik hat genau eine Semantik je Generation, entweder
  geprueft mit Abbruch oder umlaufend. Ein Mischbetrieb zwischen Compiler und VM
  ist unzulaessig.
- **REQ-SCE-034:** Ressourcengrenzen sind Teil des Vertrags und werden vor der
  Ausfuehrung geprueft: Operationszahl, Stapeltiefe, Slotzahl, Call-Tiefe,
  Calldata-, Rueckgabe- und Ereignisgroesse.
- **REQ-SCE-035:** Zwei Laeufe mit gleichem Zustand, gleichem Artefakt und gleichem
  Aufruf liefern denselben Status, denselben Rueckgabepuffer, denselben Verbrauch
  und dieselbe Zustandswurzel.

### 5.9 Versionierung

- **REQ-SCE-036:** Stromgeneration, Huellenformat, Layout-Version, Gastvertrag und
  dieser Vertrag sind getrennt versioniert. Eine Aenderung an einem zieht keine
  stille Aenderung an einem anderen nach sich.
- **REQ-SCE-037:** Eine Kette fuehrt je Vertrag genau eine Generation. Ein Wechsel
  ist ein Ereignis mit Beleg, kein Zustand nebenbei.
- **REQ-SCE-038:** Ein eingefrorener Standard wird nicht umgedeutet (ATC-STD-000
  §30). Abweichungen laufen ueber SCR.

## 6. Entscheidungs-Slots

Dieser Vertrag ist absichtlich an sieben Stellen offen. Er ist erst ratifizierbar,
wenn alle sieben gefuellt sind.

| Slot | Entscheidung | Blockiert |
|---|---|---|
| SCE-D1 | Format und Huellenname (Vorschlag: Strom ATC-BC-001, Huelle `ATCE`) | alles |
| SCE-D7 | kanonisches Frontend | Compilerziel |
| SCE-D2 | Aufrufkonvention, Stufe 1 und Stufe 2 | Ausfuehrung |
| SCE-D3 | Speichertrennung und Layout-Bindung | Zustand |
| SCE-D6 | Aufruferidentitaet | Berechtigungen |
| SCE-D4 | Gastvertrag und Gebuehrenabbuchung | Abrechnung |
| SCE-D5 | Ausfuehrung als Blockbestandteil | Konsens |

Die Ausarbeitung liegt in `docs/decisions/AD-EXEC-D1-D7.md`.

## 7. Freigabe-Gates

| Gate | Kriterium | Nachweisform |
|---|---|---|
| SCE-G1 | Genau ein normatives Bytecodeformat, gelesen von Verifier und VM | Rundlauf: ein Artefakt, zwei Leser, gleiches Ergebnis |
| SCE-G2 | Genau ein Frontend im Produktionspfad | jeder Vertrag im Pfad entsteht aus ATCLang |
| SCE-G3 | Aufrufvertrag vollstaendig, inklusive Fehlerfaelle | Pflichtfallmatrix aus Abschnitt 11 |
| SCE-G4 | Lokale und Slots getrennt, Layout gebunden | Lokale ueberschreibt keinen Slot, Layout-Drift bricht ab |
| SCE-G5 | Aufruferidentitaet nach SEC-P0-005 | Adresse aus Pubkey, Rotation ohne Adresswechsel |
| SCE-G6 | Ein Gastvertrag, Verbrauch im Beleg, Abbuchung | gleiche Kosten bei zwei Laeufen, Fehlschlag verbraucht alles |
| SCE-G7 | Ausfuehrung im Konsens mit zwei Validatoren | zwei Knoten, gleiche Zustandswurzel, Negativfall |
| SCE-G8 | Kein alternativer Ausfuehrungspfad | `.ops` nicht im Produktionspfad, im Register als Testwerkzeug |
| SCE-G9 | Keine Umgebungsabhaengigkeit | Suche und Test: keine Uhr, kein Zufall, kein Netz, kein FS im Ausfuehrungspfad |
| SCE-G10 | Identische State-Roots unabhaengig erzeugt | zwei Prozesse, zwei Maschinen, gleiche Wurzel |

Kein Gate gilt als erfuellt durch lokale Tests allein. Jede Zeile braucht die
Evidenzstufe, die in Abschnitt 10 verlangt ist.

## 8. Was ausdruecklich verboten bleibt

- Eine zweite VM.
- Ein zweiter Compiler.
- Eine parallele Storage-Implementierung.
- Eine stille Umdeutung eines eingefrorenen Standards.
- Ein Produktionspfad, der ohne Typpruefung und ABI auskommt.
- Ein "gruener" Testlauf, der als Integrationsnachweis ausgegeben wird, ohne dass
  ein Validator die Ausfuehrung geprueft hat.

## 9. Verhaeltnis zu bestehenden Dokumenten

| Dokument | Verhaeltnis |
|---|---|
| ATC-BC-001 | bleibt normativ und unveraendert; dieser Vertrag referenziert ihn, aendert ihn nicht |
| ATC-BC-002 | liefert Ruecklauf und Generationen; Voraussetzung fuer SCE-D2 Stufe 2 |
| ATC-SEC-EXEC-001 | liefert Aufnahme und Ausfuehrungsfreigabe; dieser Vertrag verlangt deren Verankerung im Kettenpfad |
| ATC-STD-600 | liefert Chain-, Netz- und Genesis-Identitaet als Ausfuehrungskontext |
| ATC-STD-SC-001..020 | liefern Vertrags-, Audit- und Registry-Anforderungen |
| ATC-14 | Determinismus- und Sicherheitsziele; der Wasm-Teil (ATC-21) steht in Spannung zur Stack-Maschine und braucht eine eigene SCR |
| ATC-STD-000 | Verfassung: Freigabe nach §9, Immutabilitaet nach §30 |

## 10. Evidenzstufen

| Stufe | Bedeutung |
|---|---|
| NORMATIVE | freigegeben nach §9, verbindlich |
| IMPLEMENTED | Code vorhanden, verhaltensgleich mit diesem Vertrag |
| TESTED | Pflichtfaelle aus Abschnitt 11 laufen lokal gruen |
| CI_VERIFIED | derselbe Lauf ist in CI gruen, am gebundenen Commit |
| E2E_VERIFIED | mindestens zwei unabhaengige Validatoren erzeugen dieselbe Zustandswurzel |
| VERIFIED | alle vorigen Stufen erreicht |

Der heutige Stand der Kette erreicht IMPLEMENTED auf Teilen, TESTED lokal, und in
keinem Punkt CI_VERIFIED oder E2E_VERIFIED.

## 11. Pflicht-Testmatrix (verbindlich bei Implementierung)

| # | Fall | Erwartung |
|---|---|---|
| T-01 | Gueltiges Artefakt, Gueltiger Aufruf | deterministisches Ergebnis, Beleg vollstaendig |
| T-02 | Stromlaenge in der Huelle falsch | Abbruch vor der Ausfuehrung |
| T-03 | Fremde Generation im Artefakt | Abbruch, keine stille Umdeutung |
| T-04 | Unbekannter Selektor | eigener Statuscode, keine Wirkung |
| T-05 | Calldata zu kurz oder zu lang | Abbruch |
| T-06 | Rueckgabe zu gross | Abbruch |
| T-07 | Lokale mit gleichem Namen wie ein Slot | Slot unveraendert |
| T-08 | Layout-Hash passt nicht zum Zustand | Abbruch |
| T-09 | Gasbudget exakt ausreichend | Erfolg, Verbrauch gleich Budget |
| T-10 | Gasbudget ein Gas zu klein | Ruecklauf, keine Zustandsaenderung, Budget voll verbraucht |
| T-11 | Fehlschlag nach halber Schleife | keine Teilwirkung |
| T-12 | Aufruf an unbekannte Adresse | Abbruch |
| T-13 | Aufrufer ohne gueltige Bindung | Abbruch |
| T-14 | Zwei Validatoren, gleicher Block | gleiche Zustandswurzel |
| T-15 | Validator mit abweichender Ausfuehrung | Block ungueltig |
| T-16 | Manipuliertes Artefakt | Abbruch vor jeder Wirkung |
| T-17 | Wiederholung desselben Aufrufs | gleiches Ergebnis, gleicher Verbrauch |
| T-18 | Ausfuehrung ohne Freigabe | Abbruch |
| T-19 | Vertrag im Kettenziel nicht abbildbar | Ablehnung beim Deployment, nicht still |
| T-20 | Messung: gleiche Wurzel ueber zwei Laeufe mit verschiedenen Prozessen | identisch |

## 12. Offene Punkte, die die Ratifizierung blockieren

1. Die sieben Entscheidungs-Slots aus Abschnitt 6.
2. Zahlen fuer Gas und Gebuehren.
3. Verhaeltnis zur Wasm-Forderung aus ATC-14 / ATC-21 (Standardkonflikt K-11).
4. Kontofuehrung, ohne die REQ-SCE-025 nicht erfuellbar ist.
5. Nachweis, dass die Aufnahme (ATC-SEC-EXEC-001) auch im Kettenpfad greift.

Bis dahin gilt: dieser Vertrag ist ein Entwurf, der die Arbeit ordnet. Er ist
kein Freibrief, und er ist kein Nachweis.
