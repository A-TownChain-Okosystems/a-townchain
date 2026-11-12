# ATC-SMART-CONTRACT-EXECUTION-001 — Integrationsvertrag Compiler, VM und Blockchain

> **Standard-ID:** ATC-SMART-CONTRACT-EXECUTION-001
> **Kategorie:** Execution / Smart Contracts
> **Tier:** 0 (Kandidat fuer foundational)
> **Status:** KANDIDAT, **nicht normativ.** Keine Implementierungspflicht, keine
> Ratifizierung, keine Registry-Bindung vor der Owner-Freigabe nach ATC-STD-000 §9.
> **Version:** 1.2.0-draft
> **Datum:** 2026-11-12
> **Autor:** Michael Wroblewski (Owner-Vorgabe), Entwurf durch Aurora
> **Vorgaenger:** v0.1.0 vom 2026-11-09 (Commit `18730d431`). Diese Fassung
> uebernimmt v0.1.0 **vollstaendig**: Verantwortungsmatrix, Lifecycle SCX-001 bis
> SCX-010, den `require`-Blocker, die Determinismusklausel, die Kodierungsregeln
> und die Evidenzleiter. Gestrichen wurde nichts.
> **Grundlage:** `docs/audits/ATCLANG-VM-CHAIN-INTEGRATION-2026-11-11.md`,
> `docs/audits/CONTRACT-EXECUTION-INVENTORY-2026-11-12.md`,
> `docs/decisions/AD-EXEC-D1-D7.md`
> **Abhaengigkeiten:** ATC-BC-001 (`SPEC-FROZEN`), ATC-BC-002 (Kandidat),
> ATC-SEC-EXEC-001 (Kandidat), ATC-STD-600 (approved), ATC-STD-SC-001..020,
> ATC-14 (Determinismus), ATC-STD-000 (Verfassung, §9, §30)

**Changelog**

| Version | Datum | Aenderung |
|---|---|---|
| 0.1.0 | 2026-11-09 | erster Kandidat: Lifecycle, Verantwortungsmatrix, `require`-Blocker |
| 1.0.0-draft | 2026-11-12 | Integrationsteil: acht Objekte, 38 Anforderungen, zehn Gates, sieben Entscheidungs-Slots, Testmatrix |
| 1.1.0-draft | 2026-11-12 | v0.1.0 vollstaendig wieder eingearbeitet. Zwischenzeitlich war die Datei durch den Entwurf 1.0.0 ersetzt worden; v0.1.0 ist aus Commit `18730d431` zurueckgeholt und in die neue Gliederung ueberfuehrt. Die Kennungen SCX-001 bis SCX-010 bleiben unveraendert gueltig. |
| 1.2.0-draft | 2026-11-12 | Owner-Entscheidung P1 erfasst: D1, D3, D5, D7 freigegeben, D2, D4, D6 bedingt freigegeben, alle sieben DECIDED. Auflagen als REQ-SCE-039 bis REQ-SCE-050 aufgenommen, Entscheidungs-Slots durch das Register ersetzt, Zustandsleiter DECIDED/SPECIFIED/IMPLEMENTED ergaenzt. Zahlen und Gates bleiben offen. |

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

## 2. Geltungsbereich und Nicht-Ziele

**Im Geltungsbereich:** jede Stelle, an der ein Vertrag entsteht, geladen,
ausgefuehrt, abgerechnet oder als Zustand bestaetigt wird, im A-TownChain-L1 und
im Devnet.

**Nicht im Geltungsbereich:** Sprache und Typsystem (ATCLang-Spec), Aufnahme- und
Lizenzrecht im Detail (ATC-SEC-EXEC-001, ATC-LIC), Oekonomie der Gebuehrenhoehe,
Konsensverfahren selbst (ATC-STD-202, `components/algorithm`), Kernel und
Produkt-Apps.

**Nicht-Ziele (aus v0.1.0, unveraendert):**

- Keine zweite Sprache neben ATCLang.
- Keine zweite VM neben der ATC-VM.
- Keine Konsensregeln in der VM.
- Keine Aenderung eingefrorener Formate ohne Aenderungsverfahren.

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

## 4. Verantwortungsmatrix (aus v0.1.0)

| System | Darf verantworten | Darf nicht |
| --- | --- | --- |
| ATCLang | Syntax, Typen, IR, Bytecode-Erzeugung | Konsens, Persistenz, Netzwerk |
| ATC Smart-Contract | Vertragsregeln, Storage-Schema, ABI | Eigene VM, eigene Execution |
| ATC-VM | Determinismus, Gas, Speichergrenzen | Zeit, Zufall, Netzwerk, Signaturen |
| A-TownChain | Transaktion, Konsens, Zustaende | Vertragslogik interpretieren |

## 5. Lifecycle mit Pruefpunkten (aus v0.1.0)

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

**Zuordnung der Stufen zu den Anforderungen dieser Fassung:**

| SCX | Anforderung | Gate |
|---|---|---|
| SCX-001 bis SCX-002 | REQ-SCE-005, REQ-SCE-006 (ein Frontend) | SCE-G2 |
| SCX-003 | REQ-SCE-001 bis REQ-SCE-004 (ein Format, Huelle, Generation) | SCE-G1 |
| SCX-004 | REQ-SCE-003, REQ-SCE-015, REQ-SCE-034 | SCE-G1, G4 |
| SCX-005 | REQ-SCE-012 bis REQ-SCE-015 (Deployment, Layout, Adresse) | SCE-G3, G4 |
| SCX-006 | REQ-SCE-007 bis REQ-SCE-012 (Aufruf) | SCE-G3 |
| SCX-007 | REQ-SCE-021 bis REQ-SCE-026, REQ-SCE-032 bis REQ-SCE-035 | SCE-G6, G9 |
| SCX-008 | REQ-SCE-017, REQ-SCE-026, REQ-SCE-033 | SCE-G4, G6 |
| SCX-009 | REQ-SCE-027 bis REQ-SCE-031 | SCE-G7 |
| SCX-010 | REQ-SCE-028, REQ-SCE-029, REQ-SCE-035 | SCE-G7, G10 |

Regel aus v0.1.0, unveraendert: **Keine Stufe darf die Pruefung einer spaeteren
Stufe ersetzen. Eine gueltige Compilierung ist kein Ausfuehrungsrecht.**

### 5.10 Auflagen aus der Owner-Freigabe vom 12.11.2026

Diese Anforderungen setzen die Bedingungen der bedingten Freigaben um. Sie sind
Teil dieses Vertrags, nicht Zusatzmaterial.

**Aufruf (D2)**

- **REQ-SCE-039:** Der Selektor ist 4 Byte breit und besteht aus den ersten 4 Byte
  des SHA-256 der kanonischen Funktionssignatur. Ein Deployment, dessen ABI zwei
  gleiche Selektoren enthaelt, wird abgelehnt. Eine Kollision wird nicht durch eine
  laengere Laufzeitform aufgeloest, weil das die Semantik je Generation verschieben
  wuerde.
- **REQ-SCE-040:** Die Wortgroesse ist 32 Byte, die Reihenfolge big-endian.
  Calldata ist die kanonische Verkettung von Woertern ohne Auffuellung. Grenzen
  (Vorschlag, Bestaetigung im Rahmen von P1b offen, angelehnt an die bestehenden
  Grenzen `MAX_PROGRAM_OPS` 65 536, `MAX_STACK_ITEMS` 4 096,
  `MAX_STORAGE_SLOTS` 65 536): Calldata hoechstens 8 192 Byte, Rueckgabepuffer
  hoechstens 4 096 Byte, Aufruftiefe hoechstens 8.
- **REQ-SCE-041:** Die Vertragsadresse ist die ersten 20 Byte von
  `SHA-256("ATC-CONTRACT-V1" || deployer_20 || nonce_u64_le || code_hash_32)`,
  dargestellt als `ATC:` plus Hex, identisch zur Adressform aus SEC-P0-005. Sie ist
  ohne Netzwerkzugriff und ohne Zeitbezug reproduzierbar.

**Identitaet (D6)**

- **REQ-SCE-048:** Vor jeder Freigabe von SCE-G5 sind vier Nachweise zu fuehren:
  (a) DID und Kontoidentitaet verwenden dieselbe kanonische Ableitung, belegt durch
  einen Test, der aus einem Schluessel beide Werte erzeugt und vergleicht;
  (b) keine Identitaet ist ueber Transaktionsfelder einschleusbar, belegt durch
  einen Negativtest mit abweichendem Aufruferfeld; (c) die Signaturpruefung
  authentifiziert die Identitaet, belegt durch einen Negativtest mit fremdem
  Schluessel; (d) interne Aufrufe bilden den unmittelbaren Aufrufer ab, belegt
  nach BC-002.

**Gas (D4)**

- **REQ-SCE-045:** Die Abrechnung fuehrt fuenf getrennte Felder:
  `gas_limit` (Budget), `gas_used` (Verbrauch), `gas_price` (Preis je Einheit),
  `fee_charged` (tatsaechlich belastet), `gas_refund` (nicht verbrauchtes Budget).
  Kein Feld darf aus einem anderen abgeleitet werden, ohne dass die Ableitung
  normiert ist.
- **REQ-SCE-046:** Die Gebuehr wird unabhaengig vom Rollback des Vertragszustands
  belastet. Ein Rollback der Ausfuehrung ist kein Rollback der Abrechnung.
- **REQ-SCE-047:** Ein fehlgeschlagener Vertrag verbraucht sein Budget vollstaendig.
  Rechenzeit ohne Belastung ist unzulaessig; das gilt auch fuer Fehlschlaege im
  Loader und im Backend.

**Ausfuehrung und Block (D5, Zusatzanforderungen)**

- **REQ-SCE-042:** Das Artefakt und der Block deklarieren die Semantikversion der
  VM. Ein Validator, dessen Version abweicht, lehnt den Block ab, statt ihn mit
  anderer Semantik zu bewerten. Kein stiller Moduswechsel.
- **REQ-SCE-043:** Jede fehlgeschlagene Ausfuehrung rollt ihren Vertragszustand
  vollstaendig zurueck, ueber alle Frames einer Transaktion. Ein Teilzustand nach
  Fehlschlag ist unzulaessig.
- **REQ-SCE-044:** Dieselbe Transaktion darf durch erneute Verarbeitung keine
  doppelten Zustandsaenderungen verursachen. Die Bindung erfolgt ueber Chain-Domain
  und Nonce; ein bereits verarbeiteter Beleg ist nicht erneut anwendbar.
- **REQ-SCE-049:** Die Transaktionsreihenfolge im Block ist deterministisch
  festgelegt, nicht von Ankunftszeit, Map-Reihenfolge oder Zufall abhaengig.
- **REQ-SCE-050:** Receipts sind byte-gleich reproduzierbar. Zwei Laeufe derselben
  Transaktion auf demselben Zustand erzeugen identische Bytes.

## 6. Normative Objekte

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

## 7. Anforderungen

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

## 8. Entscheidungs-Slots (am 12.11.2026 beantwortet)

Die sieben Slots sind entschieden. Vier sind freigegeben (D1, D3, D5, D7), drei
bedingt freigegeben (D2, D4, D6); die Auflagen stehen in Abschnitt 5.10. Alle
sieben stehen auf **DECIDED**, keine auf **SPECIFIED** im Sinne einer Ratifizierung,
keine auf **IMPLEMENTED**.

<!-- BEGIN DECISION-REGISTER (erzeugt aus docs/decisions/exec-decision-register.yaml, nicht von Hand aendern) -->

**Entscheidungsregister** (Owner, 2026-11-12; Quelle: `docs/decisions/exec-decision-register.yaml`)

| ID | Entscheidung (Titel) | Owner-Entscheidung | Spezifikation | Umsetzung | Gate |
|---|---|---|---|---|---|---|
| D1 | Bytecode-Kopf: eigene VM-Huelle, ATC-BC-001 unveraendert | freigegeben | SPECIFIED | OPEN | SCE-G1 |
| D2 | Aufrufkonvention: ABI-Selektor, Calldata, Rueckgabepuffer | bedingt freigegeben | SPECIFIED | OPEN | SCE-G3 |
| D3 | Storage: Frame-lokal und persistente S1-Slots | freigegeben | SPECIFIED | OPEN | SCE-G4 |
| D4 | Gas: eine Tabelle, Transaktionsbudget, Gebuehrenabbuchung | bedingt freigegeben | SPECIFIED | OPEN | SCE-G6 |
| D5 | Blockausfuehrung: deterministische Validator-Ausfuehrung | freigegeben | SPECIFIED | OPEN | SCE-G7 |
| D6 | Identitaet: 32-Byte-Identitaetswort | bedingt freigegeben | SPECIFIED | OPEN | SCE-G5 |
| D7 | Frontend: ATCLang kanonisch, .ops nur Testwerkzeug | freigegeben | SPECIFIED | OPEN | SCE-G2 |

Zustandsleiter: **DECIDED** (Entscheidung getroffen) → **SPECIFIED** (vollstaendig normativ spezifiziert) → **IMPLEMENTED** (im bestehenden System nachgewiesen). Kein Zustand ersetzt den naechsten automatisch; die Gates bleiben bis zum jeweiligen Nachweis offen.

Offen und nicht Teil dieser Freigabe: Numerische Gaspreise (Messung je Instruktionsklasse und Vertragsbenchmark); Endgueltiges Transaktionsgaslimit; Endgueltiges Blockgaslimit; Implementierungs- und CI-Gates.

<!-- END DECISION-REGISTER -->

Die Ausarbeitung liegt in `docs/decisions/AD-EXEC-D1-D7.md`. Das Register ist die
einzige Quelle; beide Dokumente tragen denselben, erzeugten Block
(`tools/decision_register.py --check`).

## 9. Freigabe-Gates

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

## 10. Blocker aus v0.1.0: `require` ist heute wirkungslos

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

**Stand 12.11.2026:** Der Blocker besteht unveraendert. Er ist im Audit vom
11.11.2026 als Befund IV-12 gefuehrt und bleibt Voraussetzung fuer SCE-D2 Stufe 2
(Call zwischen Vertraegen). Die dort genannte Empfehlung A (ATCB-v2 mit ASSERT und
REVERT) entspricht `standards/atc/candidates/ATC-BC-002-CANDIDATE.md`.

## 11. Determinismus (SCX-007)

Zwei Validatoren mit gleichem Ausgangszustand und gleicher Transaktion
erzeugen identisches Ergebnis, identisches Gas, identische Events und
identisches Fehlerverhalten. Verboten in der Ausfuehrung: Systemzeit,
Zufall, Netzwerk, Dateisystem, Umgebungsvariablen, Iterationsreihenfolge
nicht deterministischer Strukturen.

Ergaenzt durch REQ-SCE-032 bis REQ-SCE-035 (Arithmetik je Generation,
Ressourcengrenzen, Reproduzierbarkeit von Status, Rueckgabe, Verbrauch und Wurzel).

## 12. Kanonische Kodierung und ABI (aus v0.1.0)

- Jede signierte Nutzlast traegt Domain-Praefix und Laengenfelder.
- ABI ist versioniert; Aufrufe nennen Version und Funktionsauswahl explizit.
- Die Blockchain uebernimmt keine vom Client behaupteten Ergebnisse.
- Gas wird von der VM berechnet, nie vom Aufrufer geliefert.

Ergaenzt durch REQ-SCE-008 bis REQ-SCE-012 (Selektor, Calldata, Rueckgabe,
Adresse, Grenzen).

## 13. Was ausdruecklich verboten bleibt

- Eine zweite VM.
- Ein zweiter Compiler.
- Eine parallele Storage-Implementierung.
- Eine stille Umdeutung eines eingefrorenen Standards.
- Ein Produktionspfad, der ohne Typpruefung und ABI auskommt.
- Ein "gruener" Testlauf, der als Integrationsnachweis ausgegeben wird, ohne dass
  ein Validator die Ausfuehrung geprueft hat.

## 14. Verhaeltnis zu bestehenden Dokumenten

| Dokument | Verhaeltnis |
|---|---|
| ATC-BC-001 | bleibt normativ und unveraendert; dieser Vertrag referenziert ihn, aendert ihn nicht |
| ATC-BC-002 | liefert Ruecklauf und Generationen; Voraussetzung fuer SCE-D2 Stufe 2 |
| ATC-SEC-EXEC-001 | liefert Aufnahme und Ausfuehrungsfreigabe; dieser Vertrag verlangt deren Verankerung im Kettenpfad |
| ATC-STD-600 | liefert Chain-, Netz- und Genesis-Identitaet als Ausfuehrungskontext |
| ATC-STD-SC-001..020 | liefern Vertrags-, Audit- und Registry-Anforderungen |
| ATC-14 | Determinismus- und Sicherheitsziele; der Wasm-Teil (ATC-21) steht in Spannung zur Stack-Maschine und braucht eine eigene SCR |
| ATC-STD-000 | Verfassung: Freigabe nach §9, Immutabilitaet nach §30 |

## 15. Evidenz je Stufe

**Drei Zustaende, die einander nicht ersetzen:**

| Zustand | Bedeutung | Wer setzt ihn |
|---|---|---|
| **DECIDED** | Architekturentscheidung getroffen | Owner |
| **SPECIFIED** | vollstaendig normativ spezifiziert und ratifiziert | Owner nach §9 |
| **IMPLEMENTED** | Umsetzung im bestehenden System nachgewiesen | Evidenz, nicht Beschluss |

Eine Entscheidung erzeugt keine Spezifikation, eine Spezifikation keine Umsetzung.
Die Gates bleiben bis zum jeweiligen Nachweis offen.

Jeder Pruefpunkt braucht Test plus Nachweis, sonst bleibt er unter der
Statusleiter. Reihenfolge: IMPLEMENTED, TESTED, CI_VERIFIED, E2E_VERIFIED,
VERIFIED. Nicht nachgewiesen bleibt BLOCKED.

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

## 16. Pflicht-Testmatrix (verbindlich bei Implementierung)

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

## 17. Offene Entscheidungen vor Ratifizierung

1. ATCB-v2: ASSERT/REVERT einfuehren (Empfehlung) oder Abbildung ohne neues Op.
2. Revert-Semantik: Speicher-Rollback je Rahmen oder je Transaktion.
3. Storage-Modell: Maps als VM-Primitive oder Contract-seitig ueber Slot-Rechnung.
4. ABI-Kodierung: eigene kanonische Form oder etabliertes Format.
5. Gas-Modell: feste Op-Preise oder Tabelle mit Versionsbindung.

**Zusaetzlich aus dem Integrationsteil (12.11.2026):**

6. Die sieben Entscheidungs-Slots aus Abschnitt 8 (D1, D7, D2, D3, D6, D4, D5).
7. Zahlen fuer Gas und Gebuehren.
8. Verhaeltnis zur Wasm-Forderung aus ATC-14 / ATC-21 (Standardkonflikt K-11).
9. Kontofuehrung, ohne die REQ-SCE-025 nicht erfuellbar ist.
10. Nachweis, dass die Aufnahme (ATC-SEC-EXEC-001) auch im Kettenpfad greift.

**Aus der Freigabe vom 12.11.2026 ausdruecklich offen:**

11. Numerische Instruktionskosten (Messung je Instruktionsklasse und Vertragsbenchmark).
12. Endgueltiges Transaktionsgaslimit.
13. Endgueltiges Blockgaslimit.
14. Implementierungs- und CI-Gates.

Der Referenzwert 100 000 Gas aus `ATC-BC-002-CANDIDATE` §19.5 (groesstes im Repo
verwendetes Budget, zwei Aufrufe in `tests/atcb_v2.rs`) ist ein Mess- und
Vergleichswert, kein Konsenslimit. `DEFAULT_GAS = 50_000` in
`components/system/src/system.rs:37` ist ein Implementierungswert und ersetzt kein
Transaktionsbudget.

Bis dahin gilt: dieser Vertrag ordnet die Arbeit. Er ist kein Freibrief und kein
Nachweis.
