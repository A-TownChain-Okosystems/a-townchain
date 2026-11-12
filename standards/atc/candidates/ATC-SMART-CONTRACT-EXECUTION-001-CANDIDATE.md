# ATC-SMART-CONTRACT-EXECUTION-001 — Integrationsvertrag Compiler, VM und Blockchain

> **Standard-ID:** ATC-SMART-CONTRACT-EXECUTION-001
> **Kategorie:** Execution / Smart Contracts
> **Tier:** 0 (Kandidat fuer foundational)
> **Status:** KANDIDAT, **nicht normativ.** Keine Implementierungspflicht, keine
> Ratifizierung, keine Registry-Bindung vor der Owner-Freigabe nach ATC-STD-000 §9.
> **Version:** 1.9.0-draft
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
| 1.9.0-draft | 2026-11-12 | Drei Befunde des Owners eingearbeitet: (1) C-1 war nicht ausfuehrbar definiert und umfasste mit GV-5 ein ausdruecklich abzulehnendes Altformat sowie fuenf Hash-Eingaben, die keine Container sind; C-1 umfasst jetzt genau GV-1, GV-3 und GV-7 als rekonstruierte Container, GV-5 gehoert nach C-3, GV-2/4/6/8/9 sind ausdruecklich keine Parser-Eingaben. (2) Die Behauptung, die Codeliste sei deckungsgleich mit `ArtifactError`, war falsch: zehn von zwoelf Codes haben eine Variante, `EMPTY_STREAM` und `COUNT_MISMATCH` fehlen; die VM verarbeitet ausserdem noch `ATCB`/`0x01` und ist damit kein konformer zweiter Dekoder. (3) Zaehlfehler in der Matrix berichtigt. Neuer Test S1-T-48 fuer die Korpusauswahl. |
| 1.8.0-draft | 2026-11-12 | Parser-Fehlersemantik und Versionsisolation spezifiziert (§19, REQ-SCE-078 bis 084): geschlossene Codeliste E-001 bis E-012 passend zur VM-Fehleraufzaehlung, zehnstufige Pruefreihenfolge mit Praezedenz (erstes Problem gewinnt, Magic vor Version, deshalb ergibt der Altcontainer `ATCB` `E-007` und nicht `E-003`), Tail-Integritaet byte-genau, Versionsisolation ohne Fallback und als reine Funktion, Differentialtest mit acht Korpusklassen. Neun neue Tests S1-T-39 bis S1-T-47, alle `OPEN`. |
| 1.7.0-draft | 2026-11-12 | Owner-Entscheidung: Container-Magic `ATCE`, Fassung `0x02`, `ATCB` bleibt der Strom (REQ-SCE-067, Widerspruch zu §5 und Audit IV-03 beseitigt). Zweiter Befund des Owners behoben: der angehaengte Integritaetsdigest war als SHA-256 ohne Domaene beschrieben und ist jetzt exakt `artifact_hash` (REQ-SCE-062), ohne Zirkel, weil der Tail nicht in die Eingabe eingeht. Neues Fehlerbild `ATC-SCE-E-007 UNKNOWN_MAGIC`, verbindliche Pruefreihenfolge Laenge/Magic/Version/Generation/Struktur/Tail. Sechs Vektoren neu berechnet: GV-1, GV-3, GV-5, GV-7, GV-9 (enthaelt GV-1) und NV-2; GV-2, GV-4, GV-6, GV-8 unveraendert. Keine Implementierungsfreigabe. |
| 1.6.0-draft | 2026-11-12 | Spezifikation Stufe 1 byte-genau: drei Domaenenlabels mit abschliessendem Nullbyte (Laengen nachgezaehlt 16/14/13), Eingaben fuer `artifact_hash`, `code_hash` und `state_root` exakt, neun Goldvektoren und sechs Negativvektoren (mit reinem SHA-256-Werkzeug erzeugt), Containerfassung `0x02` mit Namensraumtrennung fuer das Magic, zwoelf Fehlercodes, getypte Policy-Schluesselraeume mit achtstufiger Pruefsequenz, Erstattungswortlaut samt Fall der kuenftigen Kontobelastung; REQ-SCE-061 bis 077, Testzuordnung in 18.6. Kein Test dadurch bestanden. |
| 1.5.1-draft | 2026-11-12 | Nur Hinweise, keine Anforderung geaendert: Aufruftiefe 8 als offener Konflikt gegen die Owner-Entscheidung 64 in ATC-BC-002 markiert (R-901); Grenzwerte als heute nicht pruefbar gekennzeichnet, weil kein Aufruf existiert (R-902); Verweise auf Abhaengigkeitsinventur und Entscheidungsvorlage. |
| 1.5.0-draft | 2026-11-12 | REQ-SCE-060 praezisiert: lokaler Knotenfehler und Konsensresultat strikt getrennt (kein Beleg, kein Blockstatus, keine Belastung, deterministische Wiederaufnahme); Reproduzierbarkeit entscheidet ueber die Einordnung. REQ-SCE-054 fuehrt zwei getrennte Code-Raeume (Beleg-Status `0x00` bis `0x03`, Knotenfehler `ATC-NODE-INFRA-001`). Testmatrix T-42 bis T-44. |
| 1.4.0-draft | 2026-11-12 | Drei Spezifikationsluecken geschlossen: Domainlaenge auf 15 Byte und Eingabe auf 75 Byte korrigiert (vorher 16/76, war falsch); IdentityValue mit einem kanonischen 32-Byte-Layout (Kennzeichen 0..3, reservierte Nullbytes 4..11, Adresse 12..31) und fail-closed-Regeln, REQ-SCE-057 bis 059; vierte Fehlerklasse fuer interne Infrastrukturfehler ohne Nutzerbelastung, REQ-SCE-060; Erstattungsformeln als ausdruecklicher Vorschlag. Testmatrix T-31 bis T-41. |
| 1.3.0-draft | 2026-11-12 | Vier Befunde der Owner-Pruefung eingearbeitet: SCX-003 beschreibt Strom plus getrennte Huelle SCE-ARTIFACT (ATCB nicht mehr als Magic des VM-Containers); Verantwortungsmatrix trennt Knoten-Orchestrierung von VM-Interpretation und ergaenzt den Validator; Identitaeten `origin`, `caller` und Vertragsadresse getrennt (REQ-SCE-051 bis 053); Fehlerklassen und Erstattung getrennt (REQ-SCE-047, 054, 055); Grundregel SCE-P1 eine Wahrheit je Funktionalitaet; Kanonisierung der Signatur praezisiert; Adressableitung mit Bytebreiten und Kollisionsregel; Grenzwerte mit eigener Begruendung und Grenzwerttests T-21 bis T-30, Status weiterhin Vorschlag. |
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

**Grundregel SCE-P1 (eine Wahrheit je Funktionalitaet).** Jede Komponente besitzt
ihre eigene Verantwortung, aber keine Komponente darf eine zweite Wahrheit ueber
dieselbe Funktionalitaet etablieren. Konkret: eine Sprache (ATCLang), eine VM
(ATC-VM), ein Instruktionsstrom (ATC-BC-001), eine Huelle (SCE-ARTIFACT), eine
Gastabelle je Generation, eine Zustandswurzel. Zwei Implementierungen derselben
Sache sind kein Redundanzgewinn, sondern eine Divergenzquelle.

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
| ATC-VM | Interpretation und Ausfuehrung der Vertragslogik, Determinismus, Gas, Speichergrenzen | Zeit, Zufall, Netzwerk, Signaturen |
| A-TownChain (Knoten) | Ausfuehrungsauftrag, Kontext, Reihenfolge, Konsens, Zustand | Vertragslogik selbst interpretieren, eine zweite Ausfuehrungslogik |
| Validator | Aufruf der normierten VM und Vergleich des Ergebnisses | eigene Semantik, eigene Gasrechnung, eigene Zustandswurzel |

**Praezisierung zu D5 (Befund vom 12.11.2026):** "Der Validator rechnet nach"
heisst nicht, dass die Kette Vertragslogik auslegt. Der Knoten orchestriert:
Auftrag bilden, Reihenfolge festlegen, die normierte VM mit dem vereinbarten
Zustand aufrufen, Ergebnis und Zustandswurzel vergleichen. Die Interpretation der
Vertragslogik liegt ausschliesslich in der VM. Zwei Implementierungen derselben
Vertragslogik waeren eine zweite Wahrheit und sind unzulaessig (REQ-SCE-056).

## 5. Lifecycle mit Pruefpunkten (aus v0.1.0)

| Code | Stufe | Eingang | Ausgang | Zustaendig |
| --- | --- | --- | --- | --- |
| SCX-001 | Source | Vertragsquelle | geparste Einheit | ATCLang |
| SCX-002 | Compiler | Einheit | typgeprueftes IR | ATCLang |
| SCX-003 | Bytecode | IR | Instruktionsstrom nach ATC-BC-001, verpackt in die getrennte Huelle SCE-ARTIFACT (eigenes Magic, eigene Versionskennung) | ATCLang |
| SCX-004 | Verifikation | Artefakt | gueltig oder Ablehnung | ATC-VM Verifier |
| SCX-005 | Deployment | signierte Transaktion | Contract ID + Storage-Wurzel | A-TownChain |
| SCX-006 | Aufruf | Transaktion + Kontext | Ausfuehrungsauftrag | A-TownChain |
| SCX-007 | Ausfuehrung | Bytecode + Zustand | Ergebnis, Gas, Events | ATC-VM |
| SCX-008 | Zustandswechsel | Ergebnis | neuer Zustand oder Abbruch | A-TownChain |
| SCX-009 | Konsens | Vorschlag | bestaetigter Block | A-TownChain |
| SCX-010 | Finalitaet | Block | dauerhafter Zustand | A-TownChain |

Regel: Keine Stufe darf die Pruefung einer spaeteren Stufe ersetzen.
Eine gueltige Compilierung ist kein Ausfuehrungsrecht.

**Klarstellung zu SCX-003 (Befund vom 12.11.2026):** Die v0.1.0-Formulierung
"Artefakt mit Magie ATCB + Version" ist ueberholt und widersprach D1. Es gibt zwei
getrennte Dinge, die nicht denselben Namen tragen duerfen:

| Ebene | Was | Kennung |
|---|---|---|
| Instruktionsstrom | Bytes nach ATC-BC-001, unveraendert, eingefroren | ATC-BC-001 |
| Huelle | Verpackung des Stroms: eigenes Magic `ATCE` (festgelegt 12.11.2026), eigene Generation, Laenge, Layout-Hash, SHA-256 | SCE-ARTIFACT |

`ATCB` bezeichnet damit weiterhin ausschliesslich den eingefrorenen Strom. Ein
Container mit demselben Magic und anderer Semantik ist untersagt; genau dieser
Doppelkopf war Audit-Befund IV-03.

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
  des SHA-256 der **kanonischen** Funktionssignatur. Ein Deployment, dessen ABI zwei
  gleiche Selektoren enthaelt, wird abgelehnt. Eine Kollision wird nicht durch eine
  laengere Laufzeitform aufgeloest, weil das die Semantik je Generation verschieben
  wuerde.
  **Kanonisierung (Praezisierung 12.11.2026):** Die Signaturform ist
  `name(typ,typ,...)`, ohne Leerzeichen, mit einer festen Schreibweise je Typ aus
  der Typentabelle des Standards (u8 bis u256, i8 bis i256, bool, address,
  bytes32, string, `list<T>`); keine Aliase, keine Kurzformen, keine
  Quellextschreibweise. Die kanonische Form wird aus dem ABI erzeugt, nicht aus dem
  Vertragstext gelesen. Das ABI-Gate lehnt unbekannte Typnamen und zwei gleiche
  kanonische Signaturen ab.
- **REQ-SCE-040:** Die Wortgroesse ist 32 Byte, die Reihenfolge big-endian.
  Calldata ist die kanonische Verkettung von Woertern ohne Auffuellung.
  **Grenzen (Status: VORSCHLAG, nicht bestaetigt; Begruendung unten, Grenzwerttest
  T-21 bis T-24 verbindlich):** Calldata hoechstens 8 192 Byte, Rueckgabepuffer
  hoechstens 4 096 Byte, Aufruftiefe hoechstens 8.

  **Konflikt, offen (12.11.2026, R-901):** ATC-BC-002 nennt die Aufruftiefe an
  vier Stellen als 64 und reserviert `E06` ausdruecklich dafuer; diese 64 ist eine
  Owner-Entscheidung vom 2026-11-09, der Standard selbst ist nicht ratifiziert.
  Die 8 in diesem Kandidaten ist ein Vorschlag und kann die Owner-Entscheidung
  nur durch deren ausdrueckliche Aenderung ersetzen, nicht stillschweigend.
  Optionen und Auswirkungen: `docs/decisions/EXEC-DECISION-TEMPLATE-2026-11-12.md`,
  E-1. Bis zur Aufloesung gilt keine der beiden Zahlen als bestaetigt.

  **Nicht pruefbar (12.11.2026, R-902):** Calldata und Rueckgabepuffer setzen
  einen Aufruf voraus. Weder der Rust-Instruktionssatz (19 Befehle) noch
  ATC-BC-002 Abschnitt 16 kennen einen Aufruf, deshalb sind T-21 bis T-24 heute
  nicht ausfuehrbar. Die Werte bleiben Vorschlag, auch wenn sie rechnerisch
  begruendet sind.

  Begruendung, die nicht aus den bestehenden VM-Grenzen abgeleitet ist, sondern aus
  drei eigenen Bindungen:

  1. **Speicher je Frame.** Ein Frame traegt Stack (4 096 Eintraege zu 8 Byte =
     32 KiB) plus Calldata plus Rueckgabepuffer. Bei 8 192 + 4 096 Byte sind das
     rund 44 KiB je Frame; bei Tiefe 8 also hoechstens rund 352 KiB je Transaktion.
     Diese Zahl ist ohne Messung rechenbar und begrenzt den Speicherbedarf
     deterministisch, unabhaengig von der Gastabelle.
  2. **Belegroesse.** Der Rueckgabepuffer wird vollstaendig in den Beleg kopiert.
     4 096 Byte je Aufruf halten einen Beleg in einer Groessenordnung, die sich
     hashen und speichern laesst; groessere Rueckgaben gehoeren in Ereignisse oder
     Zustand, nicht in den Rueckgabepuffer.
  3. **Kostenverhaeltnis.** 8 192 Byte sind 256 Woerter; selbst bei 10 Gas je Wort
     bleibt die Dekodierung unter 3 000 Gas und damit klein gegen jede
     Ausfuehrungsrechnung. Die Grenze soll Speicher begrenzen, nicht Kosten
     erzeugen; waere sie der dominante Kostenfaktor, waere sie falsch dimensioniert.

  **Was ausdruecklich fehlt und vor der Bestaetigung nachzuholen ist:** ein
  Grenzwerttest je Parameter (T-21 bis T-24) und eine Messung, die zeigt, dass die
  gewaehlten Werte reale Vertraege nicht aussperren. Uebereinstimmung mit
  `MAX_STACK_ITEMS` ist kein Argument; sie ist hoechstens ein Nebenbefund.
- **REQ-SCE-041:** Die Vertragsadresse ist die ersten 20 Byte von
  `SHA-256(domain || deployer_20 || nonce_u64_le || code_hash_32)`, dargestellt als
  `ATC:` plus Hex, identisch zur Adressform aus SEC-P0-005.
  **Praezisierung 12.11.2026 (Befund A):**


| Feld | Laenge | Kodierung |
|---|---|---|
| `domain` | **15 Byte** | ASCII `ATC-CONTRACT-V1` (`ATC` 3 + `-` 1 + `CONTRACT` 8 + `-V1` 3), keine Laengenvorspannung, kein Nullbyte, kein Auffuellzeichen |
| `deployer_20` | 20 Byte | Adresse nach SEC-P0-005, rohe Bytes ohne `ATC:`-Praefix |
| `nonce_u64_le` | 8 Byte | unsigned, little-endian, Monotonie je Deployer |
| `code_hash_32` | 32 Byte | SHA-256 ueber den kanonischen Instruktionsstrom (nicht ueber die Huelle) |
| Eingabe gesamt | **75 Byte** | Verkettung in dieser Reihenfolge, keine Trenner |

**Korrektur 12.11.2026 (Befund A):** Die vorherige Fassung gab 16 Byte Domain und
76 Byte Gesamtlaenge an. `ATC-CONTRACT-V1` ist 15 Byte lang; die Eingabe ist 75 Byte.
Eine Implementierung MUSS beide Laengen pruefen (`len(domain) == 15`,
`len(input) == 75`) und bei Abweichung abbrechen, damit kein Byte stillschweigend
entsteht. Ein zusaetzliches Byte (etwa eine Version oder ein Trenner) waere eine
eigene Formatentscheidung und ist hier ausdruecklich nicht enthalten.


  **Kollision:** Ist die abgeleitete Adresse bereits belegt, wird das Deployment
  abgelehnt. Es gibt kein stilles Weiterzaehlen der Nonce und keine Ausweichadresse,
  weil beides die Ableitung mehrdeutig machen wuerde; der Deployer muss eine neue
  Transaktion mit der naechsten Nonce senden. Die Adresse wird bei jeder
  Kettenverarbeitung aus ihren Eingaben neu berechnet und nicht als Wahrheit
  gespeichert.

**Identitaet (D6)**

- **REQ-SCE-048:** Vor jeder Freigabe von SCE-G5 sind vier Nachweise zu fuehren:
  (a) DID und Kontoidentitaet verwenden dieselbe kanonische Ableitung, belegt durch
  einen Test, der aus einem Schluessel beide Werte erzeugt und vergleicht;
  (b) keine Identitaet ist ueber Transaktionsfelder einschleusbar, belegt durch
  einen Negativtest mit abweichendem Aufruferfeld; (c) die Signaturpruefung
  authentifiziert die Identitaet, belegt durch einen Negativtest mit fremdem
  Schluessel; (d) interne Aufrufe bilden den unmittelbaren Aufrufer ab, belegt
  nach BC-002.
- **REQ-SCE-051:** Drei Identitaeten sind getrennt zu fuehren und getrennt zu
  benennen: `origin` (Urheber der Transaktion, ueber den gesamten Aufrufstapel
  unveraendert), `caller` (unmittelbar Aufrufender des aktuellen Frames) und die
  Vertragsadresse als technischer Aufrufer. `origin` ist nicht ersetzbar, `caller`
  ist frame-lokal.
- **REQ-SCE-052:** Bei einem Aufruf von Vertrag zu Vertrag ist `caller` die
  Vertragsadresse des aufrufenden Vertrags, nicht die Wallet des Urhebers. Eine
  Implementierung, die `caller` dauerhaft auf `origin` stehen laesst, ist
  unzulaessig, weil sie jede Berechtigungspruefung aushebelt. Die Semantik wird mit
  ATC-BC-002 abgestimmt, bevor Vertrag-zu-Vertrag-Aufrufe freigegeben werden.
- **REQ-SCE-053:** Eine Wallet-Identitaet darf niemals als interne Aufruferidentitaet
  eines Vertrags erscheinen, und eine Vertragsadresse darf sich nicht als Wallet
  ausgeben. Die Unterscheidung ist im Identitaetswort sichtbar (Typkennzeichen),
  nicht nur in der Dokumentation.
- **REQ-SCE-057:** `IdentityValue` hat genau ein kanonisches Layout von 32 Byte.
  Es gibt keine zweite Form und keine je Komponente verschiedene Kodierung.

| Byte | Inhalt | Regel |
|---|---|---|
| 0..3 (4 Byte) | Typkennzeichen, unsigned 32 Bit big-endian | `0x00000001` Wallet, `0x00000002` Vertrag |
| 4..11 (8 Byte) | reserviert | MUSS `0x00` sein; ein einziges Nicht-Nullbyte macht das Wort ungueltig |
| 12..31 (20 Byte) | Adresse | rohe Bytes nach SEC-P0-005 (Wallet) bzw. REQ-SCE-041 (Vertrag), ohne Praefix |

  Damit ist die fruehere Beschreibung "20-Byte-Adresse mit linksseitiger
  Nullauffuellung" ersetzt: die linke Seite ist kein undefinierter Fuellraum mehr,
  sondern besteht aus Typkennzeichen und reservierten Nullbytes. Eine Kollision
  zwischen Kennzeichen und Auffuellung ist strukturell ausgeschlossen.
- **REQ-SCE-058:** Ungueltige Woerter werden zurueckgewiesen, nicht umgedeutet:
  Kennzeichen `0x00000000`, unbekanntes Kennzeichen, nicht-null reservierte Bytes,
  20 Adressbytes alle null, oder das Wort insgesamt null. Ein unbekanntes
  Kennzeichen wird niemals als Wallet behandelt (fail-closed). Das Kennzeichen setzt
  der Frame, nicht der Aufrufer: Deployment setzt `Vertrag`, die Signaturpruefung
  setzt `Wallet`. Ein Aufrufer kann seinen Typ nicht waehlen.
- **REQ-SCE-059:** Identitaetswoerter werden als Bytes verglichen, nicht als Zahl
  behandelt. Arithmetik, Inkrement, Dekrement und Ueberlauf sind auf ihnen
  unzulaessig, damit kein Kennzeichen durch Rechnung entstehen kann. Fuer Kodierung
  und Dekodierung existiert genau eine Implementierung (Modul `identity`), die
  Wallet, VM und Knoten gemeinsam nutzen; jede Kopie waere eine zweite Wahrheit
  (SCE-P1). Die gemeinsamen Testvektoren liegen versioniert in
  `standards/atc/conformance/identity-value-vectors.json` (T-31 bis T-39, Status
  candidate, Schema `atc.conformance.identity-value.v1`); alle drei Komponenten
  pruefen gegen dieselbe Datei. Die Datei wird mit `tools/identity_vectors_check.py`
  geprueft (Exitcode 0/1); der Pruefer ist kein Produktionsdekoder und ersetzt die
  gemeinsame Implementierung nicht.

**Gas (D4)**

- **REQ-SCE-045:** Die Abrechnung fuehrt fuenf getrennte Felder:
  `gas_limit` (Budget), `gas_used` (Verbrauch), `gas_price` (Preis je Einheit),
  `fee_charged` (tatsaechlich belastet), `gas_refund` (nicht verbrauchtes Budget).
  Kein Feld darf aus einem anderen abgeleitet werden, ohne dass die Ableitung
  normiert ist.
- **REQ-SCE-046:** Die Gebuehr wird unabhaengig vom Rollback des Vertragszustands
  belastet. Ein Rollback der Ausfuehrung ist kein Rollback der Abrechnung.
- **REQ-SCE-047:** Ein fehlgeschlagener Vertrag verbraucht mindestens die tatsaechlich
  geleistete Rechenzeit. Rechenzeit ohne Belastung ist unzulaessig; das gilt auch
  fuer Fehlschlaege im Loader und im Backend. **Praezisierung 12.11.2026 (Befund 4):
  "fehlgeschlagen" ist nicht gleich "Budget weg".** Es gelten vier Klassen:


| Klasse | Verbrauch | Erstattung |
|---|---|---|
| Gaserschoepfung | volles `gas_limit` | keine, `gas_refund = 0` |
| Revert oder logischer Vertragsfehler | nur `gas_used` | ungenutztes Budget, nach der Erstattungsregel |
| Ablehnung vor der Ausfuehrung (Struktur, Aufnahme, Signatur) | kein Gas, keine Zustandsaenderung | nicht anwendbar; die Transaktion wird nicht gebucht |
| Interner Fehler der Ausfuehrungsinfrastruktur (Loader, Backend, VM-Interna) | kein Nutzeranteil | keine Belastung des Urhebers |

**Korrektur 12.11.2026 (Befund C):** Die vierte Klasse fehlte. Ein Fehler der
Infrastruktur ist weder Revert noch Erschoepfung; wuerde er als eines von beiden
abgerechnet, erzeugten identische Transaktionen je nach Zustand der Infrastruktur
unterschiedliche Gebuehren, und der Nutzer zahlt fuer einen Fehler, den er nicht
verursacht hat.

**REQ-SCE-060 (praezisiert 12.11.2026): lokaler Knotenfehler und Konsensresultat
sind strikt getrennt.**

1. **Lokal.** Tritt der Fehler nur auf einem Knoten auf, bricht dieser Knoten die
   Verarbeitung ab. Er erzeugt keinen Konsensbeleg, schreibt keinen Beleg-Status und
   nimmt den Block nicht als erfolgreich verarbeitet an. Fuer ihn gilt der Block als
   **nicht verarbeitet**, nicht als verarbeitet mit Fehler.
2. **Konsens.** Ein Infrastrukturfehler darf niemals als deterministisches
   Vertragsergebnis in den Block geschrieben werden. Er ist kein vierter erzeugbarer
   Beleg-Status, sondern ein Knotenfehlercode (`ATC-NODE-INFRA-001`): kein Beleg,
   keine Zustandsaenderung, kein Einfluss auf Gasfelder.
3. **Gebuehren und Zustand.** Keine lokale Belastung, keine lokale
   Zustandsaenderung. Die Transaktion wird nicht deshalb als erfolgreich oder
   fehlgeschlagen abgerechnet; ueber sie ist noch nicht entschieden.
4. **Wiederaufnahme.** Nach Behebung des Fehlers muss der Knoten denselben Block auf
   demselben Ausgangszustand deterministisch erneut ausfuehren und zum selben
   Ergebnis und derselben Zustandswurzel kommen wie ein Knoten ohne Fehler. Kein
   Teilergebnis des abgebrochenen Laufs darf weiterwirken.

**Reproduzierbarkeit entscheidet ueber die Einordnung.** Tritt derselbe Fehler bei
wiederholter Ausfuehrung auf demselben Ausgangszustand erneut auf, ist er kein
Infrastrukturfehler, sondern ein Defekt in Spezifikation oder Implementierung: Der
Block wird von keinem Knoten angenommen, es wird nichts abgerechnet, und der Defekt
ist mit Vorfallmeldung zu beheben. Ein Fehler, der nur einmal und nur lokal auftritt,
ist ein Betriebsfehler. Ohne diese Unterscheidung koennte ein einzelner Knoten den
Status einer Transaktion bestimmen, also genau die zweite Wahrheit erzeugen, die
SCE-P1 verbietet.


  Die Erstattungsregel ist normativ festzulegen und zu testen; ohne sie waere jede
  einfache Vertragsablehnung so teuer wie eine Erschoepfung, was den Anreiz
  verzerrt.

  **Vorschlag zur Erstattungsregel (Status: VORSCHLAG, nicht beschlossen,
  Owner-Entscheidung offen).** Fuer den regulaeren Revert:

  ```
  gas_refund    = gas_limit - gas_used
  refund_amount = gas_refund * gas_price
  fee_charged   = gas_used * gas_price
  ```

  Bei Gaserschoepfung gilt `gas_used = gas_limit`, damit `gas_refund = 0`. Bei
  Ablehnung vor der Ausfuehrung wird nichts gebucht, bei einem Infrastrukturfehler
  ebenfalls nichts. Kein prozentualer Abzug, keine Mindestgebuehr. Die Formeln
  gelten erst, wenn T-28, T-29 und T-40 gruen sind.
- **REQ-SCE-054:** Die Fehlerklassen nach REQ-SCE-047 sind im Ergebnis und im Beleg
  unterscheidbar, damit ein Validator die Abrechnung nachrechnen kann, ohne die
  Absicht zu raten. Es gibt genau zwei Code-Raeume, und sie duerfen sich nicht
  vermischen:

| Raum | Codes | Wo erlaubt |
|---|---|---|
| Beleg-Status (deterministisch, Konsensbestandteil) | `0x00` SUCCESS, `0x01` REVERT, `0x02` OUT_OF_GAS, `0x03` REJECTED_PRE_EXECUTION | nur im Beleg |
| Lokaler Knotenfehler (nicht deterministisch, nicht im Block) | `ATC-NODE-INFRA-001` | nur in der Knotenablage und im Betriebsprotokoll |

  Ein Beleg mit einem Statuscode ausserhalb von `0x00` bis `0x03` ist ungueltig.
  `ATC-NODE-INFRA-001` ist kein Beleg-Status und darf niemals in einen Beleg oder in
  die Zustandswurzel gelangen.
- **REQ-SCE-055:** `gas_refund` ist eine Anzahl Gaseinheiten, kein Geldbetrag. Der
  zurueckerstattete Betrag ist `gas_refund * gas_price` und traegt einen eigenen
  Namen; die beiden Groessen duerfen nicht in einem Feld vermischt werden.

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
- **REQ-SCE-056:** Der Knoten orchestriert, interpretiert aber nicht. Er bildet den
  Ausfuehrungsauftrag, legt die Reihenfolge fest, ruft die normierte VM mit dem
  vereinbarten Zustand auf und vergleicht Ergebnis, Verbrauch und Zustandswurzel.
  Eine eigene Auslegung der Vertragslogik im Knoten, im Backend oder im Loader ist
  unzulaessig (Grundregel SCE-P1).

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

| ID | Entscheidung (Titel) | Owner-Entscheidung | Ratifikation | Spezifikation | spezifiziert in | Umsetzung | Gate |
|---|---|---|---|---|---|---|---|
| D1 | Bytecode-Kopf: eigene VM-Huelle, ATC-BC-001 unveraendert | freigegeben | nicht ratifiziert (Kandidat) | SPECIFIED | Kandidat §5.1, §18.1-18.3 und §19.1-19.5 (nicht ratifiziert) | OPEN | SCE-G1 |
| D2 | Aufrufkonvention: ABI-Selektor, Calldata, Rueckgabepuffer | bedingt freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.3, §5.10 (nicht ratifiziert) | OPEN | SCE-G3 |
| D3 | Storage: Frame-lokal und persistente S1-Slots | freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.4 (nicht ratifiziert) | OPEN | SCE-G4 |
| D4 | Gas: eine Tabelle, Transaktionsbudget, Gebuehrenabbuchung | bedingt freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.6, §5.10 (nicht ratifiziert) | OPEN | SCE-G6 |
| D5 | Blockausfuehrung: deterministische Validator-Ausfuehrung | freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.7, §5.10 (nicht ratifiziert) | OPEN | SCE-G7 |
| D6 | Identitaet: 32-Byte-Identitaetswort | bedingt freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.5, §5.10 (nicht ratifiziert) | OPEN | SCE-G5 |
| D7 | Frontend: ATCLang kanonisch, .ops nur Testwerkzeug | freigegeben | nicht ratifiziert (Kandidat) | OPEN | Kandidat §5.2 (nicht ratifiziert) | OPEN | SCE-G2 |

Zustandsleiter: **DECIDED** (Entscheidung getroffen) → **SPECIFIED** (vollstaendig normativ spezifiziert) → **IMPLEMENTED** (im bestehenden System nachgewiesen). Kein Zustand ersetzt den naechsten automatisch; die Gates bleiben bis zum jeweiligen Nachweis offen.

Drei Ebenen: **Owner-Entscheidung** ist intern verbindlich, auch wenn der zugehoerige Standard nicht ratifiziert ist. **Ratifikation** sagt, ob der Standard in Kraft ist; "nicht ratifiziert (Kandidat)" bindet niemanden. Ein Wert im Kandidaten bleibt **Vorschlag**, solange der Owner ihn nicht ausdruecklich beschliesst.

Offen und nicht Teil dieser Freigabe: Numerische Gaspreise (Messung je Instruktionsklasse und Vertragsbenchmark); Endgueltiges Transaktionsgaslimit; Endgueltiges Blockgaslimit; Implementierungs- und CI-Gates; Konflikt Aufruftiefe: Owner-Entscheidung 64 vom 09.11.2026 in ATC-BC-002 (Kandidat, nicht ratifiziert) gegen SCE-Vorschlag 8 (R-901); Fehlende CALL-Instruktion und Identitaetstyp auf dem u64-Stapel (R-902, R-907); Autoritaet der beiden Instruktionssaetze (R-906).

**Stufe-1-Ratifikation** (Owner, 2026-11-12; Quelle: `docs/decisions/exec-decision-register.yaml`)

| ID | Gegenstand | Option | ratifiziert | Basis | Wirkung |
|---|---|---|---|---|---|
| S1-1 | Drei getrennte Identitaeten: artifact_hash, code_hash, state_root | A mit Domaenenvariante 1b | 2026-11-12 | `a63e41a93` | zweite Hashidentitaet wird berechnet, nicht nur benannt; neue Containerfassung ist Teil derselben Entscheidung; ATC-STATE-V2 unabhaengig von der Variante |
| S1-2 | Policy-Bindung: welche Identitaet Freigabe, Widerruf, Mindestversion bindet | C, asymmetrisch | 2026-11-12 | `a63e41a93` | Freigabe und Mindestversion am code_hash, Widerruf getrennt fuer Code und Artefakt; getypte Schluesselraeume, zwei Abfragen, nur als vollstaendige Einheit (S1-T-38) |
| S1-3 | Erstattung: Grundsatz | A, ausschliesslich Gasrueckgabe | 2026-11-12 | `a63e41a93` | gas_refund in Gaseinheiten, kein Geldbetrag; ungenutztes Budget verfaellt, auch nach Einfuehrung einer Kontobelastung |
| S1-4 | Testschichtung L1/L2/L3 | B, nur L1 zaehlt fuer P1a | 2026-11-12 | `a63e41a93` | L1 muss nicht ausfuehrbar sein, um zu zaehlen; L2 und L3 bleiben bis B-1 gesperrt; L1 ist heute EMPTY, P1a bleibt offen |

Durch die Stufe-1-Ratifikation entfallen: B-2 Artefaktidentitaet im Identitaetsteil (R-903); Container-Magic am 12.11.2026 entschieden: ATCE fuer die Huelle; B-3 Verfahren (R-905): Schichtung entschieden, L2 und L3 bleiben bis B-1 gesperrt.

Eine Stufe-1-Ratifikation entscheidet die Option, nicht den Standard: der Kandidat bleibt Kandidat, P1a bleibt offen und P2 bleibt gesperrt.

Nicht freigegeben sind weiterhin Implementierung und Release.


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
| T-21 | Calldata genau 8 192 Byte | angenommen, dekodiert |
| T-22 | Calldata 8 193 Byte | abgelehnt vor der Ausfuehrung, kein Gas |
| T-23 | Rueckgabe genau 4 096 Byte | angenommen, vollstaendig im Beleg |
| T-24 | Rueckgabe 4 097 Byte oder Aufruftiefe 9 | abgelehnt, Statuscode eindeutig |
| T-25 | ABI mit zwei gleichen Selektoren | Deployment abgelehnt |
| T-26 | Zwei kanonische Signaturen, die sich nur in Alias-Schreibweise unterscheiden | gleicher Selektor, Deployment abgelehnt |
| T-27 | Adresskollision: gleicher Deployer, gleiche Nonce, gleicher Code-Hash | zweites Deployment abgelehnt |
| T-28 | Revert nach 1 000 Gas bei Budget 10 000 | `gas_used` 1 000, Erstattung nach Regel, Zustand unveraendert |
| T-29 | Gaserschoepfung bei Budget 10 000 | `gas_used` 10 000, `gas_refund` 0, Zustand unveraendert |
| T-30 | Aufruf mit falschem Statuscode fuer den Beleg | Block abgelehnt |
| T-31 | Wallet-Wort: Kennzeichen `00000001`, reserviert 0, Adresse A | gueltig, dekodiert zu A |
| T-32 | Vertragswort: Kennzeichen `00000002`, reserviert 0, Adresse A | gueltig, dekodiert zu A |
| T-33 | Kennzeichen `00000000` | abgelehnt |
| T-34 | unbekanntes Kennzeichen `00000003` | abgelehnt, nicht als Wallet behandelt |
| T-35 | reservierte Bytes 4..11 nicht null | abgelehnt |
| T-36 | Wort vollstaendig null | abgelehnt |
| T-37 | Adressbytes alle null bei gueltigem Kennzeichen | abgelehnt |
| T-38 | gleiche 20 Adressbytes, einmal Wallet-, einmal Vertragskennzeichen | zwei verschiedene Identitaeten, keine Eskalation |
| T-39 | Vertrag ruft Vertrag: `caller` traegt Vertragskennzeichen, `origin` bleibt Wallet | gueltig (erst nach BC-002) |
| T-40 | Loaderfehler nach 500 Gas bei Budget 10 000 | kein Beleg, keine Belastung, kein Zustandswechsel, `ATC-NODE-INFRA-001` nur lokal |
| T-42 | Wiederaufnahme: derselbe Block nach behobenem Infrastrukturfehler | identisches Ergebnis und identische Zustandswurzel wie ohne Fehler |
| T-43 | Reproduzierbarer Fehler bei zwei Laeufen auf demselben Zustand | kein Infrastrukturfehler; kein Knoten nimmt den Block an, Defektmeldung |
| T-44 | Beleg mit Statuscode ausserhalb `0x00` bis `0x03` | Block abgelehnt |
| T-41 | Adressableitung: Domain 15 Byte, Eingabe 75 Byte | reproduzierbar; 16-Byte-Domain wird abgelehnt |

## 17. Offene Entscheidungen vor Ratifizierung

**Pruefstand (12.11.2026):** Die vier offenen Punkte sind gegen ATC-BC-002 und die
vorhandenen Vektoren geprueft; Ergebnis und Abhaengigkeiten in
`docs/audits/EXEC-OPEN-POINTS-2026-11-12.md`. Der Bericht entscheidet nichts und
aendert keine Anforderung.

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
15. Grenzwerte bestaetigen: Calldata 8 192, Rueckgabe 4 096 Byte, Tiefe 8 erst nach
    den Grenzwerttests T-21 bis T-24.
16. Erstattungsregel nach Revert normativ festlegen (Vorschlag: volles ungenutztes
    Budget) und mit T-28 belegen.
17. `caller`-Semantik bei Vertrag-zu-Vertrag mit ATC-BC-002 abstimmen (REQ-SCE-052).
18. Eigenen Statuscode fuer Infrastrukturfehler vergeben (REQ-SCE-060); er darf nicht
    mit Revert oder Erschoepfung kollidieren.
20. *(erledigt 12.11.2026)* Container-Magic: `ATCE` fuer die Huelle, `ATCB` bleibt
    der Strom, Fassung `0x02` (REQ-SCE-067). Die Formatidentitaet ist das Tupel aus
    Magic, Version und Struktur, nicht das Magic allein.
19. Testvektoren fuer `IdentityValue` liegen als Datei vor
    (`standards/atc/conformance/identity-value-vectors.json`). Offen bleibt die
    Umsetzung gegen diese Datei: eine Implementierung fuer Wallet, VM und Knoten
    (REQ-SCE-059).

## 18. Spezifikation Stufe 1 (byte-genau)

Dieser Abschnitt setzt die am 12.11.2026 ratifizierten Entscheidungen S1-1 (Option A
mit Domaenenvariante 1b), S1-2 (Option C), S1-3 (Option A) und S1-4 (Option B) in
normative Regeln um. Er ist **Spezifikation, keine Freigabe**: Implementierung und
Release bleiben gesperrt, P1a bleibt offen, P2 bleibt gesperrt.

Alle Werte in diesem Abschnitt sind nachrechenbar: die Sollwerte in 18.2 wurden mit
einem reinen SHA-256-Werkzeug aus den hier definierten Bytes erzeugt, **nicht** mit
Projektcode. Jede Regel nennt ihre Tests aus `docs/decisions/EXEC-STAGE1-TRACKING.md`.

### 18.1 Hash-Domaenen (REQ-SCE-061 bis 064)

**Drei Domaenenlabels**, ASCII, jeweils mit abschliessendem `0x00`:

| Label | Bytes | Laenge | Zweck |
|---|---|---|---|
| `ATC-ARTIFACT-V1\0` | `4154432d41525449464143542d5631 00` | 16 | Huelle (Container) |
| `ATC-STREAM-V1\0` | `4154432d53545245414d2d5631 00` | 14 | Instruktionsstrom |
| `ATC-STATE-V2\0` | `4154432d53544154452d5632 00` | 13 | Zustandswurzel |

**REQ-SCE-061 (Domaenenlabels).** Die Labels sind fest und byte-exakt. Das
abschliessende Nullbyte ist Pflicht, damit kein Label Praefix eines laengeren werden
kann: ohne Nullbyte waere `ATC-STREAM-V1` Praefix von `ATC-STREAM-V10`.

**REQ-SCE-062 (Eingaben).** Es gilt genau:

```
artifact_hash = SHA-256( "ATC-ARTIFACT-V1\0" || container[0 .. n-32] )
code_hash     = SHA-256( "ATC-STREAM-V1\0"   || ops )
state_root    = SHA-256( "ATC-STATE-V2\0"    || code_hash || count(u32 LE) || words(u64 LE)*count )
```

Dabei ist `container` der vollstaendige Container aus 18.3 und `container[0 .. n-32]`
alles ausser dem angehaengten 32-Byte-Hash. `ops` ist die lueckenlose Folge der
kodierten Instruktionen ohne jeden Kopf. Die Anzahl der Instruktionen ist
**absichtlich nicht** Teil der Strom-Eingabe: sie ist aus der Opcode-Tabelle
eindeutig ableitbar, und ihre Aufnahme wuerde Metadaten in die Code-Identitaet
ziehen. Damit gilt: derselbe Strom in zwei verschiedenen Huellen ergibt denselben
`code_hash` und verschiedene `artifact_hash`.

**REQ-SCE-063 (Kodierung).** ASCII ohne BOM. Alle Ganzzahlen Little Endian mit fester
Breite (u32 = 4 Byte, u64 = 8 Byte). Keine Host-Typen, keine Zeiger, keine
Debug-Ausgabe, keine JSON- oder Base64-Darstellung als Hash-Eingabe. Kein Hash ueber
eine Eingabe ohne Domaene.

**REQ-SCE-064 (Fehlerreaktion).** Ein leerer Strom ist kein gueltiges Artefakt:
`ATC-SCE-E-004 EMPTY_STREAM`, kein Digest. Ein abgeschnittener Operand ergibt
`ATC-SCE-E-002 TRUNCATED_OPERAND`, ebenfalls ohne Digest. In beiden Faellen gibt es
keinen Hash, keine Registrierung und keine Ausfuehrung.

Tests: S1-T-19, S1-T-20, S1-T-21, S1-T-28, S1-T-33.

### 18.2 Goldvektoren (REQ-SCE-065)

**REQ-SCE-065.** Diese Vektoren sind verbindlich. Eine Implementierung, die einen
Sollwert nicht reproduziert, ist nicht konform.

**Programm P1** (`Push(1)`, `Halt`): Strom `01010000000000000010` (10 Byte).
**Programm P2** (`Push(7)`, `Push(42)`, `Add`, `Store(0)`, `Halt`):
Strom `010700000000000000012a00000000000000020b10` (21 Byte).

| ID | Gegenstand | Eingabe (hex) | Sollwert |
|---|---|---|---|
| GV-1 | `artifact_hash`, P1, Generation 1, Fassung 2 | `4154432d41525449464143542d563100` + `4154434502010200000001010000000000000010` | `02a62ba7bb1d81722fdf0e8eadd94c8637c37c95f82b794a3a93d63b76208a23` |
| GV-2 | `code_hash`, P1 | `4154432d53545245414d2d563100` + `01010000000000000010` | `d7adc88fc2ceb21136df3397ec7d6bfff57a4f439088798f34ab3c8c3c546cdb` |
| GV-3 | `artifact_hash`, P1, Generation 2 | `4154432d41525449464143542d563100` + `4154434502020200000001010000000000000010` | `29790676ca4b93fa7c9d90880a432b82394ee7b86acad802dd76de0afbcabe44` |
| GV-4 | `code_hash`, P1, Generation 2 | wie GV-2 | `d7adc88fc2ceb21136df3397ec7d6bfff57a4f439088798f34ab3c8c3c546cdb` (**gleich GV-2**) |
| GV-5 | alte Fassung, ohne Praefix, Containerformat `0x01` | `4154434201010200000001010000000000000010` | `3f5bb373c995e6f92ff81722d28f7e5329e4ba670dad5bf962729670edd7484e` (**ueberholt**) |
| GV-6 | `code_hash`, P2 | `4154432d53545245414d2d563100` + `010700000000000000012a00000000000000020b10` | `c829629e93d97b464b58a1a5004c7a17700f482a08fb34caae59a83ef39e30d8` |
| GV-7 | `artifact_hash`, P2, Generation 1, Fassung 2 | `4154432d41525449464143542d563100` + `41544345020105000000` + P2-Strom | `79c1a77bb66101277cd1fe4d38d66b7c7fb11869a694fbc77771e8fc93caaa17` |
| GV-8 | `state_root` V2, `code_hash` = GV-2, Woerter `[7, 42]` | `4154432d53544154452d563200` + GV-2 + `02000000` + `0700000000000000` + `2a00000000000000` | `c3cc531a91ce794025e1f584cc4d6e33dc1dea36accddd8a72cc6e20b26cc184` |
| GV-9 | `state_root` V1 (alte Semantik), `artifact_hash` = GV-1 | `4154432d53544154452d563100` + GV-1 + `02000000` + `0700000000000000` + `2a00000000000000` | `8116c0f71d1ada483836c3b33abd0c21c24a25a144f3bfb0dbb8c096d4f22d28` |

**Negativvektoren.** Sie muessen ungleich sein, sonst ist die Trennung unwirksam:

| ID | Pruefung | Sollwert |
|---|---|---|
| NV-1 | Strom mit Artefakt-Domaene gehasht | `4f67416a21680a0b2029b625904cf88850f43e00a46a0679069059bf57887810`, darf weder GV-1 noch GV-2 sein |
| NV-2 | Container mit Strom-Domaene gehasht | `9408a5daea03a72a77e3a366a3289aa742795e7620dfc1b3ab8333747f08a583`, darf weder GV-1 noch GV-2 sein |
| NV-3 | Label ohne Nullbyte (`ATC-STREAM-V1`) statt mit | Digest weicht von GV-2 ab; eine Implementierung ohne Nullbyte ist nicht konform |
| NV-4 | leerer Strom | `ATC-SCE-E-004`, kein Digest |
| NV-5 | `01` ohne 8-Byte-Operand | `ATC-SCE-E-002`, kein Digest |
| NV-6 | Fassung `0x01` als `0x02` gelesen | `ATC-SCE-E-003`, kein Digest |

**Reproduktion ohne Projektcode**, damit die Werte ohne Vertrauen pruefbar sind. Die
Vektoren liegen zusaetzlich maschinenlesbar in
`standards/atc/conformance/stage1-vectors.json`; diese Datei wird aus diesem
Abschnitt erzeugt und nie von Hand gepflegt:

```
python3 tools/check_stage1_vectors.py          # prueft Dokument gegen SHA-256
python3 tools/check_stage1_vectors.py --write  # erzeugt die Vektordatei neu
```

Eine Zeile genuegt fuer die Gegenprobe:

```
python3 -c "import hashlib;print(hashlib.sha256(bytes.fromhex('4154432d53545245414d2d56310001010000000000000010')).hexdigest())"
# d7adc88fc2ceb21136df3397ec7d6bfff57a4f439088798f34ab3c8c3c546cdb   (= GV-2)
```

Tests: S1-T-04, S1-T-19, S1-T-20, S1-T-21, S1-T-22, S1-T-25, S1-T-34.

### 18.3 Containerfassung (REQ-SCE-066 bis 069)

**REQ-SCE-066 (Fassung).** Das Containerformat steigt von `0x01` auf `0x02`. Das ist
Bestandteil der Entscheidung S1-1/1b und keine spaetere Einzeländerung. Aufbau
unveraendert in den Feldern, geaendert in der Kennung:

| Offset | Laenge | Inhalt |
|---|---|---|
| 0 | 4 | Magic `ATCE` (`41544345`) |
| 4 | 1 | Containerformat `0x02` |
| 5 | 1 | Generation `0x01` oder `0x02` |
| 6 | 4 | Anzahl Instruktionen, u32 LE |
| 10 | n | Instruktionsstrom |
| 10+n | 32 | `artifact_hash` = SHA-256(`ATC-ARTIFACT-V1\0` ++ Byte 0 bis 10+n-1) |

Der Tail ist **genau** der definierte `artifact_hash` aus REQ-SCE-062, kein zweiter
Digest ohne Domaene. Es entsteht kein Zirkel, weil die Eingabe den Tail nicht
enthaelt: gehasht wird Byte 0 bis 10+n-1, der Tail beginnt bei 10+n.

Gesamtlaenge = `10 + n + 32`.

**REQ-SCE-067 (Magic).** Das Container-Magic ist `ATCE` (`41544345`), **nicht**
`ATCB` (Owner-Entscheidung 12.11.2026). Damit ist die Auflage aus D1 erfuellt und der
Widerspruch zu §5 beseitigt: `ATCB` bleibt ausschliesslich die Kennung des
eingefrorenen Instruktionsstroms, die Huelle traegt ihr eigenes Magic. Der Doppelkopf
aus Audit-Befund IV-03 wird nicht wiederholt.

Formatidentitaet ist immer das vollstaendige Tupel aus Magic, Formatversion und
festgelegter Struktur. Kein Parser darf aus dem Magic allein auf Version oder Semantik
schliessen. Die Pruefreihenfolge ist verbindlich: Laenge, dann Magic, dann Version,
dann Generation, dann Struktur und Anzahl, dann Tail.

**REQ-SCE-068 (Ablehnung).** Nur das Magic `ATCE` und nur Fassung `0x02` werden
angenommen. Ein anderes Magic ergibt `ATC-SCE-E-007 UNKNOWN_MAGIC`, Fassung `0x01`
oder jede andere Fassung `ATC-SCE-E-003 UNSUPPORTED_CONTAINER_VERSION`. Es gibt
keinen Fallback, keine Deutung als `0x02`, keine Umrechnung der Identitaet und keine
versionsabhaengige Interpretation vor der Versionspruefung. Ebenso abgelehnt:
eine Anzahl, die nicht zur dekodierten Opzahl passt (`ATC-SCE-E-005 COUNT_MISMATCH`),
und ein angehaengter Hash, der nicht zur Berechnung passt (`ATC-SCE-E-006
INTEGRITY_MISMATCH`).

**REQ-SCE-069 (Generationen).** Die Regeln aus ATC-BC-002 bleiben: Opcodes `0x11` bis
`0x13` nur mit Generation `0x02`, keine Konvertierung zwischen Generationen. Die
Generation ist Teil des `artifact_hash` und **nicht** Teil des `code_hash` (GV-2 =
GV-4). Eine Huelle mit anderer Generation ist ein anderes Artefakt mit demselben
Code.

Tests: S1-T-01, S1-T-02, S1-T-19, S1-T-26, S1-T-27, S1-T-35.

### 18.4 Policy-Schluesselraeume (REQ-SCE-070 bis 074)

**REQ-SCE-070 (getypte Schluessel).** Ein Eintrag nennt seine Identitaet. Vorgabe:
erstes Byte `0xC0` = Code-Identitaet, `0xA0` = Artefakt-Identitaet, danach der
32-Byte-Hash. Ein Eintrag ohne gueltiges Kennzeichen ist ungueltig; er wird **nicht**
geraten und **nicht** als Code gelesen.

**REQ-SCE-071 (Pruefsequenz, verbindlich).** In dieser Reihenfolge, jede Stufe mit
eigenem Statuscode:

| # | Pruefung | Fehler |
|---|---|---|
| 1 | Authority verfuegbar | `ATC-SCE-E-101 AUTHORITY_UNAVAILABLE` |
| 2 | Containerfassung unterstuetzt | `ATC-SCE-E-003` |
| 3 | Struktur und kanonische Kodierung | `ATC-SCE-E-005`, `E-006` |
| 4 | Eintrag zu `code_hash` vorhanden | `ATC-SCE-E-102 UNKNOWN_CODE` |
| 5 | Huellen-Identitaet: `artifact_hash` des Containers gleich registriertem `artifact_hash` | `ATC-SCE-E-103 ARTIFACT_MISMATCH` |
| 6 | Code-Identitaet: `code_hash` gleich registriertem `code_hash` | `ATC-SCE-E-104 CODE_MISMATCH` |
| 7 | Signaturen und Autorisierung | `ATC-SCE-E-105 SIGNER` |
| 8 | Sperren und Mindestversion | `E-110`, `E-111`, `E-112` |

Stufe 5 und 6 sind **zwei** Pruefungen, nicht eine. Heute existiert nur Stufe 6
(`exec-admission/src/lib.rs:434`), und ihre Meldung nennt bereits "Artefakt-Hash",
wo ein Code-Hash gemeint ist.

**REQ-SCE-072 (Sperren).** `ATC-SCE-E-110 CODE_REVOKED` bei Treffer auf die
Code-Identitaet, `ATC-SCE-E-111 ARTIFACT_REVOKED` bei Treffer auf die
Artefakt-Identitaet, `ATC-SCE-E-112 MIN_VERSION` bei Unterschreitung. Beide
Sperrraeume werden immer geprueft; ein Treffer auf einem von beiden blockiert.
Treffer auf beiden: gemeldet wird `E-110`, damit Belege deterministisch bleiben.

**REQ-SCE-073 (Wirkung).** Eine Code-Sperre wirkt auf jede Huelle desselben Stroms.
Eine Artefakt-Sperre wirkt nur auf diese eine Huelle; derselbe Code in anderer
Verpackung bleibt zulaessig, solange er nicht selbst gesperrt ist. Eine
Artefakt-Sperre darf nie als Code-Sperre gelesen werden und umgekehrt.

**REQ-SCE-074 (Migration).** Flache 32-Byte-Altschluessel ohne Kennzeichen werden
nicht interpretiert. Ein solcher Altbestand fuehrt zur **Startverweigerung**
(`ATC-SCE-E-113 LEGACY_KEY_UNTYPED`), mit Zaehlung im Startbericht. Es gibt keine
automatische Umwandlung und keinen Mischbetrieb.

S1-2 gilt nur als vollstaendige Einheit: eine zweite Abfrage ohne getypte
Schluesselraeume, oder getypte Schluesselraeume ohne zweite Abfrage, bestehen das
Gate nicht.

Tests: S1-T-07, S1-T-08, S1-T-09, S1-T-23, S1-T-24, S1-T-29, S1-T-30, S1-T-31,
S1-T-32, S1-T-36, S1-T-37, S1-T-38.

### 18.5 Gasrueckgabe (REQ-SCE-075 bis 077)

**REQ-SCE-075 (Felder).** Vier getrennte Felder im Beleg: `gas_limit`,
`gas_used`, `gas_refund` (Gaseinheiten), `fee_charged` (Geld). `gas_refund` ist
immer eine Gasmenge. Ein Geldbetrag an dieser Stelle ist unzulaessig; eine
Umrechnung im Beleg ebenfalls.

**REQ-SCE-076 (Wortlaut der Regel).** Es gilt `0 <= gas_used <= gas_limit` und

```
gas_refund = gas_limit - gas_used
```

je Fall:

| Fall | `gas_used` | `gas_refund` |
|---|---|---|
| Erfolg | verbrauchte Gase | Restbudget |
| Revert | nur verbrauchtes Gas | Restbudget |
| Erschoepfung | `gas_limit` (volles Budget) | `0` |
| Ablehnung vor Ausfuehrung | `0` | `gas_limit` |

Nicht verbrauchtes Budget **verfaellt**; es entsteht kein Guthaben. Solange kein
Konto belastet wird, ist das folgenlos. Sobald eine tatsaechliche Belastung
eingefuehrt wird, gilt verbindlich: Belastung hoechstens `gas_limit * gas_price`,
Rueckbuchung genau `gas_refund * gas_price`, keine Gutschrift ueber den
Ausgangsstand hinaus, keine Verzinsung, keine Gutschrift fiktiver Betraege.

**REQ-SCE-077 (Determinismus).** Alle Betraege ergeben sich ausschliesslich aus den
Belegfeldern und der Gastabelle. Keine Zeit, kein Zufall, keine Host-Groesse. Zwei
Knoten mit derselben Eingabe erzeugen denselben Beleg.

Der Referenzwert 100 000 Gas bleibt Mess- und Vergleichswert, kein Konsenslimit; die
Gastabelle selbst ist nicht Teil dieser Spezifikation.

Tests: S1-T-11, S1-T-12, S1-T-13, S1-T-14, S1-T-17.

### 18.6 Testzuordnung und Abnahmekriterium

Jeder Abschnitt nennt seine Tests. Umgekehrt muss jeder Test aus der Matrix hier
eine Regel haben; ein Test ohne Regel ist ein Spezifikationsfehler, eine Regel ohne
Test ein Abnahmeverstoss.

| Abschnitt | Anforderungen | Tests |
|---|---|---|
| 18.1 Domaenen | REQ-SCE-061 bis 064 | S1-T-19, 20, 21, 28, 33 |
| 18.2 Goldvektoren | REQ-SCE-065 | S1-T-04, 19, 20, 21, 22, 25, 34 |
| 18.3 Container | REQ-SCE-066 bis 069 | S1-T-01, 02, 19, 26, 27, 35 |
| 18.4 Schluesselraeume | REQ-SCE-070 bis 074 | S1-T-07 bis 10, 23, 24, 29 bis 32, 36, 37, 38 |
| 18.5 Gasrueckgabe | REQ-SCE-075 bis 077 | S1-T-11 bis 14, 17 |

Nicht abgedeckt und deshalb weiterhin offen: S1-T-15 und S1-T-18 (Schichtzuordnung
und L1-Status; sie haengen an einem Dekoder, den es nicht gibt), S1-T-16 (L2, von
B-1 abhaengig).

Der Nachweisstand aendert sich durch diesen Abschnitt **nicht**: die Matrix bleibt
bei 37 offen und 1 blockiert, weil kein Test ausgefuehrt wurde. Spezifiziert ist
nicht implementiert.


---
## 19. Parser-Fehlersemantik und Versionsisolation

Dieser Abschnitt ist die Antwort auf die drei vom Owner am 12.11.2026 benannten
unabhaengigen Pruefungen: Tail-Integritaet, Differentialtests, Versionsisolation.
Er ist Spezifikation, **keine** Freigabe. Er aendert keine Anforderung aus §18 und
ergaenzt keine Vektoren: die neun Gold- und zwei Negativvektoren bleiben gueltig.

Grundsatz: ein Fehler ist ein **Paar aus Code und Offset**, nicht ein Text. Zwei
konforme Dekoder duerfen sich in der Formulierung unterscheiden, nie in Code, Offset
oder Annahme-/Ablehnungsentscheidung.

### 19.1 Fehlercodes (REQ-SCE-078)

**REQ-SCE-078 (geschlossene Codeliste).** Ein Dekoder kennt genau diese Codes und
erfindet keine weiteren. Die Liste ist **normativ**; die Fehleraufzaehlung der VM
(`components/vm/src/artifact.rs`, `ArtifactError`) ist heute **nicht** deckungsgleich
mit ihr. Zehn der zwoelf Codes haben dort eine Variante, zwei fehlen (Tabelle unten).
Die VM ist deshalb derzeit kein konformer zweiter Dekoder.

| Code | Name | Bedeutung | Offset | `ArtifactError` in der VM |
|---|---|---|---|---|
| `E-001` | `TOO_SHORT` | weniger als 42 Byte, kein vollstaendiger Container | Laenge der Eingabe | `TooShort` |
| `E-002` | `TRUNCATED_OPERAND` | Operand reicht nicht bis zum Stromende | erstes Byte des fehlenden Operanden | `TruncatedOperand` |
| `E-003` | `UNSUPPORTED_CONTAINER_VERSION` | Magic stimmt, Version ist nicht `0x02` | 4 | `UnsupportedContainer(u8)` |
| `E-004` | `EMPTY_STREAM` | Strom ohne Instruktion | 10 | **fehlt in der VM** |
| `E-005` | `COUNT_MISMATCH` | deklarierte Anzahl passt nicht zur dekodierten Opzahl | 6 | **fehlt in der VM** |
| `E-006` | `INTEGRITY_MISMATCH` | Tail entspricht nicht `artifact_hash` | 10+n | `BadHash` |
| `E-007` | `UNKNOWN_MAGIC` | erste vier Bytes sind nicht `ATCE` | 0 | `BadMagic` |
| `E-008` | `TRAILING_BYTES` | Bytes zwischen Stromende und Tail | 10 + dekodierte Laenge | `TrailingBytes` |
| `E-009` | `UNKNOWN_GENERATION` | Generation ist nicht `0x01` und nicht `0x02` | 5 | `UnknownGeneration(u8)` |
| `E-010` | `UNKNOWN_OPCODE` | Opcode existiert in keiner Generation | Position des Opcode-Bytes | `UnknownOpcode(u8)` |
| `E-011` | `GENERATION_MISMATCH` | Opcode `0x11` bis `0x13` bei Generation `0x01` | Position des Opcode-Bytes | `GenerationMismatch { .. }` |
| `E-012` | `PROGRAM_TOO_LARGE` | Strom ueberschreitet den Groesstwert | 10 | `ProgramTooLarge { .. }` |

`E-012` ist vergeben, sein Groesstwert ist aber **nicht** festgelegt; die Grenzwerte
bleiben offen (T-21 bis T-24). Ein Dekoder darf `E-012` deshalb heute nur melden,
wenn ein Betreiber einen Wert konfiguriert hat.

**Zur VM als zweitem Dekoder.** `components/vm/src/artifact.rs` verarbeitet heute
Magic `ATCB` mit Containerfassung `0x01`. Sie erfuellt den Vertrag aus §19 damit
nicht: es fehlen die Varianten fuer `E-004` und `E-005`, und solange sie `ATCB`
annimmt, wuerde sie den Altbestand durchlassen, den REQ-SCE-079 ablehnt. Die VM darf
erst dann als unabhaengiger zweiter Dekoder im Differentialtest zaehlen, wenn sie
beides erfuellt. Das ist eine Implementierungsauflage, keine Freigabe.

### 19.2 Pruefreihenfolge und Praezedenz (REQ-SCE-079)

**REQ-SCE-079 (erste Stufe gewinnt).** Die Pruefungen laufen in dieser Reihenfolge.
Meldet eine Eingabe mehrere Fehler, wird ausschliesslich der Code der **fruehesten**
Stufe gemeldet. Die Reihenfolge ist normativ und nicht implementierungsabhaengig:

| Stufe | Pruefung | Code |
|---|---|---|
| 1 | Laenge mindestens 42 Byte | `E-001` |
| 2 | Magic `ATCE` | `E-007` |
| 3 | Version `0x02` | `E-003` |
| 4 | Generation `0x01` oder `0x02` | `E-009` |
| 5 | Strom dekodierbar (Opcode bekannt, Generation passt, Operand vollstaendig) | `E-010`, `E-011`, `E-002` |
| 6 | keine Bytes zwischen Stromende und Tail | `E-008` |
| 7 | deklarierte Anzahl gleich dekodierter Opzahl | `E-005` |
| 8 | Strom nicht leer | `E-004` |
| 9 | Tail gleich `artifact_hash` | `E-006` |
| 10 | Strom innerhalb des Groesstwerts, sofern konfiguriert | `E-012` |

Bemerkenswert und beabsichtigt: Stufe 2 steht **vor** Stufe 3. Ein alter Container mit
Magic `ATCB` und Version `0x01` ergibt daher `E-007` und nicht `E-003`. Das ist die
einzige Stelle, an der die alte Fassung sichtbar wird, und sie wird nicht als
Versionsfehler getarnt. Ein Container mit `ATCE` und Version `0x01` ergibt `E-003`.

### 19.3 Tail-Integritaet (REQ-SCE-080)

**REQ-SCE-080 (exakter Tail).** Der Tail beginnt **genau** bei `10 + n` und ist
**genau** 32 Byte lang, wobei `n` die dekodierte Laenge des Stroms ist. Es gilt:

- kein Byte vor dem Tail gehoert zum Digest, kein Byte nach dem Tail ist zulaessig;
- die Eingabe des Digest ist Byte 0 bis `10+n-1`, also ohne Tail;
- ein um ein Byte verschobener Digest ergibt `E-006`, nicht `E-008`;
- zusaetzliche Bytes nach dem Tail ergeben `E-008`, nicht `E-006`; die Stufen 6 und 9
  duerfen nicht vertauscht werden;
- fehlende Bytes ergeben `E-001`, wenn die Gesamtlaenge unter 42 liegt, sonst
  `E-002` oder `E-008`, je nachdem, wo die Dekodierung endet.

Der gespeicherte Digest ist der **einzige** Integritaetswert im Container. Ein zweiter
Digest an anderer Stelle ist unzulaessig.

### 19.4 Versionsisolation (REQ-SCE-081 bis 083)

**REQ-SCE-081 (keine Vermischung).** Es gibt genau eine angenommene Kombination:
Magic `ATCE`, Version `0x02`, Generation `0x01` oder `0x02`. Jede andere Kombination
wird abgelehnt. Verboten sind ausdruecklich:

- Fallback-Parsing ("erst `0x02`, sonst `0x01`"),
- spekulatives Parsen mehrerer Fassungen mit Auswahl des besten Ergebnisses,
- Umdeuten eines Containers in eine andere Version,
- jede versionsabhaengige Interpretation vor der Versionspruefung,
- ein gemeinsamer Parserzustand zwischen zwei Aufrufen.

**REQ-SCE-082 (reine Funktion).** Der Dekoder ist eine reine Funktion der Bytes.
Derselbe Aufruf mit denselben Bytes liefert im selben Prozess und in jedem anderen
Prozess dasselbe Ergebnis `(Code, Offset)` beziehungsweise dieselbe Annahme. Kein
Cache, dessen Schluessel nur das Magic ist, keine globalen Veraenderlichen, keine
Abhaengigkeit von der Reihenfolge frueherer Aufrufe.

**REQ-SCE-083 (Generation vor Opcode).** Die Generation stammt ausschliesslich aus dem
Kopffeld. Opcodes `0x11` bis `0x13` sind bei Generation `0x01` unzulaessig
(`E-011`), unbekannte Opcodes sind in jeder Generation unzulaessig (`E-010`). Es gibt
keine stille Umnummerierung und kein Lesen eines Artefakts fremder Generation
(REQ-SCE-003 bleibt in Kraft).

### 19.5 Differentialtests (REQ-SCE-084)

**REQ-SCE-084 (Vergleich statt Behauptung).** Ein Dekoder gilt erst als konform, wenn
zwei unabhaengige Dekoder auf demselben Korpus dasselbe Ergebnis liefern. Der
Vergleich umfasst ausschliesslich `(Code, Offset)` je Eingabe sowie die
Annahme-/Ablehnungsentscheidung; Meldungstexte sind nicht normativ.

Der Korpus ist mindestens:

| Klasse | Inhalt |
|---|---|
| C-1 | die **drei vollstaendigen Artefakt-Goldvektoren** GV-1, GV-3 und GV-7, rekonstruiert als Kopf + Strom + 32-Byte-Tail mit Tail = Sollwert des jeweiligen Vektors; alle drei werden angenommen |
| C-2 | Magic falsch, Version falsch, Generation falsch (je einzeln) |
| C-3 | Altcontainer `ATCB` + `0x01`, darunter GV-5 als Eingabe (muss `E-007` mit Offset `0` ergeben) |
| C-4 | `ATCE` + `0x01`, `ATCE` + `0x03`, `ATCE` + `0xFF` |
| C-5 | abgeschnittene Operanden an jeder Operandenposition |
| C-6 | ein bis drei zusaetzliche Bytes zwischen Stromende und Tail |
| C-7 | um ein Byte verschobener Tail |
| C-8 | deklarierte Anzahl zu klein und zu gross |

**Welche Vektoren Parser-Eingaben sind (REQ-SCE-084, praezisiert 12.11.2026).**
Ein Goldvektor ist nur dann eine Parser-Eingabe, wenn er einen **vollstaendigen
Container** beschreibt. Das ist bei drei Vektoren der Fall:

| Vektor | Art | Rolle im Korpus |
|---|---|---|
| GV-1, GV-3, GV-7 | `artifact_hash`-Eingabe plus Sollwert = Tail | vollstaendiger Container, C-1, angenommen |
| GV-5 | Altformat `ATCB` + `0x01`, Sollwert = alter Hash | kein gueltiger Container, C-3, `E-007`, Offset `0` |
| GV-2, GV-4, GV-6 | `code_hash`-Eingabe (Strom ohne Kopf) | keine Container, **nicht** im Parser-Korpus |
| GV-8, GV-9 | `state_root`-Eingabe | keine Container, **nicht** im Parser-Korpus |

Rekonstruktion fuer GV-1 als Beispiel: der dokumentierte Container ist
`4154434502010200000001010000000000000010`, der Sollwert
`02a62ba7…08a23` ist der Tail. Die vollstaendige Eingabe ist also Kopf + Strom +
Tail, und der Tail wird nicht mitgehasht (19.3). Fuer GV-3 und GV-7 gilt dasselbe
Verfahren.

C-1 hat damit **drei** Eintraege, nicht neun. Ein Korpus, der GV-5 als gueltigen
Container fuehrt, wuerde REQ-SCE-079 verletzen und ist unzulaessig; ein Korpus, der
GV-2, GV-4, GV-6, GV-8 oder GV-9 an einen Containerdecoder gibt, stellt eine Frage,
die diese Vektoren nicht beantworten.

Ergebnisform: eine Zeile je Eingabe mit `id`, `sha256` der Eingabe, `decision`,
`code`, `offset`. Die Datei ist maschinenlesbar; ein Abweichen in einer Zeile ist ein
Fehlschlag, kein Hinweis.

### 19.6 Testzuordnung

| Abschnitt | Anforderungen | Tests |
|---|---|---|
| 19.1 Codes | REQ-SCE-078 | S1-T-42 |
| 19.2 Praezedenz | REQ-SCE-079 | S1-T-42, S1-T-45 |
| 19.3 Tail | REQ-SCE-080 | S1-T-39, 40, 41 |
| 19.4 Versionsisolation | REQ-SCE-081 bis 083 | S1-T-44, 45, 46, 47 |
| 19.5 Differentialtest, Korpusauswahl | REQ-SCE-084 | S1-T-43, S1-T-48 |

Alle neun Tests sind `OPEN`. Kein Test ist damit bestanden; die Matrix aendert nur
ihre Groesse, nicht ihren Befund. Der Dekoder bleibt die Voraussetzung fuer L1
(S1-T-18) und damit fuer P1a.

---
Der Referenzwert 100 000 Gas aus `ATC-BC-002-CANDIDATE` §19.5 (groesstes im Repo
verwendetes Budget, zwei Aufrufe in `tests/atcb_v2.rs`) ist ein Mess- und
Vergleichswert, kein Konsenslimit. `DEFAULT_GAS = 50_000` in
`components/system/src/system.rs:37` ist ein Implementierungswert und ersetzt kein
Transaktionsbudget.

Bis dahin gilt: dieser Vertrag ordnet die Arbeit. Er ist kein Freibrief und kein
Nachweis.
