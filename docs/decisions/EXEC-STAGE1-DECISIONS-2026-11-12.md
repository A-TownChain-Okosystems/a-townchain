# Stufe 1 — vier Entscheidungen mit Optionen, Folgen, Rueckwaertskompatibilitaet und Akzeptanztests

**Status:** Entscheidungsvorlage. Trifft **keine** Entscheidung und erteilt **keine**
Freigabe.
**Datum:** 2026-11-12 · **Basis:** `a-townchain` main `a4617667`
**Bezug:** `docs/audits/EXEC-DEPENDENCY-INVENTORY-2026-11-12.md`,
`docs/decisions/EXEC-DECISION-TEMPLATE-2026-11-12.md`

## 0. Kennungen, damit nichts doppelt heisst

Die Vorlage vom 12.11. benutzt E-1 fuer die **Aufruftiefe**. Die Stufe-1-Arbeit
beginnt aber mit der **Identitaetsfrage**. Damit dieselbe Kennung nicht zwei
Gegenstaende bezeichnet, fuehrt diese Vorlage eigene Kennungen und ordnet sie zu:

| Stufe-1-Kennung | Gegenstand | entspricht |
|---|---|---|
| S1-1 | drei Identitaeten trennen und benennen | B-2, Teil 1 |
| S1-2 | welche Identitaet bindet Policy, Widerruf, Mindestversion | B-2, Teil 2 |
| S1-3 | Erstattung: Grundsatz | E-3 (Register D4) |
| S1-4 | Testschichtung und Entkopplung P1a/P2 | B-3 |

Die Aufruftiefe (E-1 im bisherigen Sprachgebrauch) ist **kein** Gegenstand dieser
Stufe. Sie gehoert zu ATC-BC-002 und bleibt als R-901 offen.

## 1. S1-1 — Drei Identitaeten trennen

### 1.1 Frage

Bleibt es bei einem Hash, der drei Aufgaben traegt, oder bekommt jede Aufgabe ihre
eigene Identitaet mit eigenem Namen?

### 1.2 Ist-Stand, belegt

- `components/system/src/system.rs:223`: `let code_hash = artifact.hash();` Der
  Code-Hash **ist** der Container-Hash.
- `components/vm/src/artifact.rs:297-298`: `Artifact::hash()` = SHA-256 ueber
  `ohne_hash()`, also ueber Magic, Containerformat, Generation, Anzahl und Strom.
- Es existiert in **diesem** Repository **keine** zweite Hashfunktion: `grep`
  ueber `components/` findet nur `sha256_daten`, `Artifact::hash`, die
  Prueffunktion und die davon unabhaengigen Miner- und Block-Hashes. Der
  Strom-Hash aus REQ-SCE-041 ist damit nicht unbeneannt, sondern **nicht
  vorhanden**. (Das atclang-Repository fuehrt ein eigenes Bytecode-Modell mit
  `compute_hash`; ob dieser Wert als Strom-Hash taugt, ist eine eigene Frage und
  hier nicht geprueft.)
- `components/system/src/state.rs:12`, `43-52`: die Zustandswurzel nimmt den
  Code-Hash als Eingabe, neben der Domaine `ATC-STATE-V1\0`.

**Lokaler Lauf des Demo-Binaries am 12.11.2026** (Arbeitsbaum = `main`, keine
CI-Evidenz) zeigt die Kopplung unmittelbar:

```
Kompilat     : 19 ATCLang-Instruktionen -> 20 ATCB-Operationen
Artefakt     : 138 Bytes
Code-Hash    : 5f2c38c847b3c84188bbbbf766f8aa9cc2c4d86ca53e6f60afc04b75eec75f18
Artefakt-Hash: 5f2c38c847b3c84188bbbbf766f8aa9cc2c4d86ca53e6f60afc04b75eec75f18
Zustand      : 552e5a0b4b644bd6 -> 0c02e59d68a644fb
```

Die Zeilen "Code-Hash" und "Artefakt-Hash" des Programms sind identisch, weil sie
denselben Wert ausgeben. Das ist der sichtbare Beweis, dass es heute nur **eine**
Identitaet gibt.

### 1.3 Optionen

**Option A — drei Identitaeten, drei Namen.**

| Identitaet | Berechnung | Verwendung |
|---|---|---|
| `artifact_hash` | SHA-256 ueber die ganze Huelle | Integritaet, Containerpruefung |
| `code_hash` | SHA-256 ueber den Strom (ohne Huelle) | Vertragsidentitaet, Adressableitung |
| `state_root` | SHA-256 ueber Domaine, Identitaet, Wortzahl, Woerter | Ausfuehrungscommitment |

**Option B — drei Identitaeten, andere Zuordnung.** `code_hash` bleibt der
Container-Hash; die Vertragsadresse wird aus `artifact_hash` gebildet; der
Strom-Hash entfaellt. REQ-SCE-041 muesste dann auf den Container zeigen.

**Option C — zwei Identitaeten.** Wie heute, aber mit klarer Benennung:
`artifact_hash` fuer Integritaet und Identitaet, `state_root` fuer den Zustand. Der
Unterschied zu B liegt nur in der Benennung.

### 1.4 Folgen

| Bereich | A | B | C |
|---|---|---|---|
| Instruktionen | keine | keine | keine |
| Hashes | neue Funktion noetig | keine neue Funktion | keine neue Funktion |
| Zustandswurzel | Eingabe wechselt, Domaine muss versioniert werden | unveraendert | unveraendert |
| Vertragsadresse | aus dem Strom, wie SCE-041 es verlangt | aus der Huelle, SCE-041 zu aendern | aus der Huelle |
| Kompatibilitaet | Aenderung von Wurzeln und Schluesseln | keine Aenderung | keine Aenderung |
| Aussagekraft | Strom ist identisch, auch wenn die Huelle neu serialisiert wird | Huelle ist identisch, auch wenn der Strom gleich bleibt | grob |

Der Unterschied zwischen A und B ist die Frage, **was** einen Vertrag identifiziert:
sein Code oder seine Verpackung. A ist die strengere Aussage, B die billigere.

### 1.5 Rueckwaertskompatibilitaet und Migrationsverfahren

Heute ist keine Zustandswurzel ausserhalb eines Demo-Laufs gespeichert, keine
Policy registriert und kein Artefakt-Hash als Goldwert festgehalten. Eine
Migration ist damit **heute** ohne Bestandsdaten moeglich, aber nur mit Verfahren:

1. **Identitaetsversion.** Die Domaine der Zustandswurzel traegt die Fassung
   (`ATC-STATE-V1` heute). Ein Wechsel der Eingabe ist eine neue Domaine
   (`ATC-STATE-V2`), damit alte und neue Wurzeln nicht verwechselbar sind.
2. **Neuregistrierung statt Uebernahme.** Policies, Widerrufe und
   Mindestversionen werden unter der neuen Identitaet neu eingetragen. Keine
   automatische Uebernahme, kein stilles Uebersetzen.
3. **Startpruefung.** Ein gemischt belegter Schluesselraum ist ein Fehler, kein
   Sonderfall: der Start bricht ab, wenn ein Eintrag seine Identitaet nicht nennt.
4. **Protokoll.** Die Umstellung wird als Deployment-Record gefuehrt, mit den
   Vorher- und Nachherwerten des Demo-Laufs als Bezug.

### 1.6 Akzeptanztests

| ID | Pruefung | Bestehen |
|---|---|---|
| S1-T-01 | gleicher Strom, anderes Magic | `code_hash` gleich, `artifact_hash` verschieden |
| S1-T-02 | Stream-Hash | unabhaengig von Generation und Operationszahl |
| S1-T-03 | Zustandswurzel | nennt genau eine Identitaet und eine versionierte Domaine |
| S1-T-04 | Goldvektor | eine Datei enthaelt fuer einen Vertrag beide Hashes, festgeschrieben |
| S1-T-05 | Grep-Gate ueber `components/` | kein Pfad setzt `code_hash` aus dem Container-Hash |
| S1-T-06 | Reproduzierbarkeit | derselbe Quelltext erzeugt die Werte aus 1.2 erneut |

## 2. S1-2 — Welche Identitaet bindet die Freigabe

### 2.1 Frage

Freigabe, Widerruf und Mindestversion haengen heute am `code_hash`. Welche der
drei Identitaeten bindet kuenftig diese drei, und duerfen sie asymmetrisch binden?

### 2.2 Ist-Stand, belegt

- `components/exec-admission/src/lib.rs:73`, `200-204`: `code_hash` ist der
  Schluessel der Policy-Registry.
- `:434`: `observed_hash != policy.code_hash` ist die Ablehnung; `observed_hash`
  kommt aus `system.rs:276` und ist der Container-Hash.
- `:368-373`: Mindestversion und Widerruf haengen an demselben Schluessel.

### 2.3 Optionen

**Option A — alles am Strom.** Policy, Widerruf und Mindestversion binden
`code_hash`. Die Huelle ist reine Integritaetsfrage und kann neu serialisiert
werden, ohne die Freigabe zu verlieren.

**Option B — alles an der Huelle.** Alle drei binden `artifact_hash`. Eine
Neuverpackung desselben Stroms entwertet dann die Freigabe und den Widerruf.

**Option C — asymmetrisch.** Policy und Mindestversion binden `code_hash`,
der Widerruf bindet zusaetzlich `artifact_hash`. Damit laesst sich eine konkrete
verseuchte Verpackung sperren, ohne den Code zu sperren, und umgekehrt.

### 2.4 Folgen

| Bereich | A | B | C |
|---|---|---|---|
| Freigabe | bleibt bei neuem Container gueltig | wird ungueltig | wie A |
| Widerruf | sperrt den Code | sperrt die Verpackung | sperrt beides, getrennt |
| Schlankheit | ein Schluessel je Objekt | ein Schluessel | zwei Schluessel, zwei Absichten |
| Missbrauchsrisiko | Code mit gleichem Strom in neuer Huelle laeuft weiter, absichtlich | Widerruf trifft nicht den Code | Verwechslung der beiden Schluessel |

### 2.5 Akzeptanztests

| ID | Pruefung | Bestehen |
|---|---|---|
| S1-T-07 | Altbelegung | Registrierung ohne genannte Identitaet wird beim Start abgelehnt, nicht still zugeordnet |
| S1-T-08 | Widerruf | sperrt genau die gewaehlte Identitaet; das andere Objekt verhaelt sich wie entschieden |
| S1-T-09 | Gleichlauf | Mindestversion und Widerruf nennen ihre Identitaet im Datensatz |
| S1-T-10 | Umbenennung | Aenderung des Magic kann eine Freigabe weder umgehen noch vortaeuschen; das Ergebnis ist definiert |

## 3. S1-3 — Erstattung: Grundsatz

### 3.1 Frage

Bleibt es beim Rueckfluss in Gas, oder kommt eine Geldrueckzahlung an den Absender
hinzu?

### 3.2 Ist-Stand, belegt

ATC-BC-002 verneint Erstattung (Zeile 127), entscheidet den Rueckfluss des
Teilbudgets an den Aufrufer (Zeile 200) und sagt "einmal belastet bleibt belastet"
(Zeile 197). Ein Geldmodell fehlt vollstaendig. Der Beleg traegt `gas_limit`
(`system.rs:131`), aber kein `gas_used`, kein `gas_refund` und keinen Betrag. Die
Transaktion traegt `gas_price` und `gas_limit` (`wallet/src/tx.rs:31-32`), der
Knoten liest beide nicht.

### 3.3 Optionen

**Option A — nur Rueckfluss.** `gas_refund` ist eine Gasmenge, die ein Frame an
seinen Aufrufer zurueckgibt. Ungenutztes Transaktionsbudget verfaellt. Kein
Geldbetrag im Beleg.

**Option B — Rueckfluss und Geld.** Zusaetzlich `refund_amount` nach der
SCE-Formel. Das verlangt ein Konto, eine Abbuchung und eine ausdrueckliche
Aufhebung des Wortlauts "keine Erstattung".

### 3.4 Folgen

| Bereich | A | B |
|---|---|---|
| Transaktionsformat | unveraendert | unveraendert |
| Beleg | Felder `gas_used`, `gas_refund` in Gas | zusaetzlich Betrag, Bezug auf ein Konto |
| Konten | nicht benoetigt | benoetigt, existiert heute nicht |
| Wirtschaft | Rechenzeit ist knapp, nicht bezahlt | Gebuehr mit Rueckzahlung |
| Konsens | einfacher, keine Geldbewegung im Beleg | Geldbewegung deterministisch zu belegen |

### 3.5 Akzeptanztests

| ID | Pruefung | Bestehen |
|---|---|---|
| S1-T-11 | Revert nach 1 000 von 10 000 Gas (T-28) | `gas_used` 1 000, Bestand unveraendert, `gas_refund` nach Regel |
| S1-T-12 | Erschoepfung (T-29) | `gas_used` 10 000, `gas_refund` 0, Bestand unveraendert |
| S1-T-13 | Feldschnitt | `gas_refund` in Gas; ein Geldbetrag ist entweder nicht vorhanden (A) oder getrennt benannt (B) |
| S1-T-14 | ungenutztes Budget | verfaellt (A) oder wird gutgeschrieben (B), im Beleg sichtbar |

## 4. S1-4 — Testschichtung

### 4.1 Frage

Wie werden die Grenzwerte pruefbar, ohne die fehlende Aufrufarchitektur
vorwegzunehmen?

### 4.2 Drei Schichten

| Schicht | Inhalt | Voraussetzung |
|---|---|---|
| L1 | reine Funktionen ohne Ausfuehrung: Wortgroesse, Wortreihenfolge, Laengenpruefung, Ablehnung zu grosser Calldata, Artefaktlaenge und Magic | ein reiner Dekoder, der **heute nicht existiert** |
| L2 | Semantik eines Frames im Harness: Rueckfluss, Revert, Rueckgabegroesse, Aufruftiefe | Frame-Modell, also B-1 als Entwurf |
| L3 | echte Vertragsaufrufe im Block | B-1 entschieden und umgesetzt |

### 4.3 Zuordnung der vorhandenen Testfaelle

| Fall | heute ausfuehrbar | Schicht |
|---|---|---|
| T-21 Calldata 8 192 | nur die Laengenannahme | L1 (Laenge), L2 (Annahme im Frame) |
| T-22 Calldata 8 193 | nur die Laengenablehnung | L1 (Laenge), L2 (kein Gas) |
| T-23 Rueckgabe 4 096 im Beleg | nein | L2 |
| T-24 Rueckgabe 4 097 oder Tiefe 9 | nein | L1 (Laenge), L3 (Tiefe) |
| T-28, T-29 Gas | nein, solange kein `gas_used` existiert | L2 |

### 4.4 Optionen

**Option A — Schichtung, L1 und L2 zaehlen fuer P1a, L3 bleibt gesperrt.** P1a
wird abschliessbar mit der Auflage, dass L3 nach B-1 nachgezogen wird.

**Option B — Schichtung, nur L1 zaehlt fuer P1a.** Strenger; Grenzwerte, die nur
im Frame pruefbar sind, bleiben offen.

**Option C — keine Schichtung.** P1a bleibt gesperrt, bis B-1 entschieden und
umgesetzt ist. Das ist der heutige Zustand.

### 4.5 Folgen und Akzeptanz

| Option | P1a | P2 | Risiko |
|---|---|---|---|
| A | abschliessbar mit Auflage | bleibt gesperrt | L2-Harness koennte zur verdeckten Implementierung werden |
| B | teilweise abschliessbar | bleibt gesperrt | Grenzwerte bleiben ohne Nachweis |
| C | nicht abschliessbar | bleibt gesperrt | Stillstand |

Akzeptanz der Schichtung selbst:

| ID | Pruefung | Bestehen |
|---|---|---|
| S1-T-15 | Vollstaendigkeit | jeder Fall T-21 bis T-24 ist genau einer Schicht zugeordnet, keiner ohne |
| S1-T-16 | Trennung | L2 laeuft ohne Produktionsfreigabe und ohne Netz |
| S1-T-17 | Ehrlichkeit | ein L3-Fall kann nicht als bestanden gemeldet werden, solange B-1 offen ist |
| S1-T-18 | Dekoder | L1 braucht einen reinen Dekoder; ohne ihn bleibt L1 leer und ist als leer zu melden |

## 5. Reihenfolge und Zuordnung zum Register

Die Reihenfolge folgt der Abhaengigkeit, nicht der Wichtigkeit:

| Schritt | Gegenstand | Ergebnis |
|---|---|---|
| 1 | S1-1 und S1-2 | Hash-Semantik und Policy-Identitaet festgelegt |
| 2 | S1-3 | Grundsatz zu Gas und Erstattung |
| 3 | S1-4 | entkoppeltes Testverfahren |
| 4 | B-1 mit R-906 | Autoritaet und Aufrufarchitektur |
| 5 | E-2 und E-4 | abhaengige Details |

**Zuordnung, am Register geprueft** (`docs/decisions/exec-decision-register.yaml`):

| Frage | Registerbezug | Beleg |
|---|---|---|
| Grenzwerte Calldata und Rueckgabe (E-2) | **D2** | Bedingungen "Wortgroesse und Calldata-Encoding festgelegt", "maximale Rueckgabepuffergroesse festgelegt" |
| Aufruftiefe (E-1) | **kein D-Eintrag** | steht nur in ATC-BC-002 Zeilen 50, 133, 146, 197, 282 gegen SCE-REQ-040 |
| Erstattung (E-3) | **D4** | Bedingungen "gas_refund in Gaseinheiten, Geldbetrag getrennt benannt", "Erstattungsregel normativ festlegen und mit T-28 belegen" |
| `caller` (E-4) | **D6** | Bedingungen "origin, caller und Vertragsadresse getrennt definiert", "caller bei Vertrag-zu-Vertrag ist die Vertragsadresse, nicht die Wallet" |
| Identitaeten und Policy-Bindung (S1-1, S1-2) | **D1 und D2** | D1 Format und Kopf, D2 "deterministische Vertragsadressableitung festgelegt" |

Damit ist die Zuordnung nicht aus einer Zusammenfassung abgeleitet, sondern aus
den Bedingungen des Registers selbst. Die Aufruftiefe ist der einzige Punkt ohne
Registerbezug; sie ist eine Entscheidung zwischen zwei Standards, nicht innerhalb
dieses Registers.

## 6. Was diese Vorlage nicht tut

Sie waehlt keine Option, aendert keine Anforderung und setzt keinen Grenzwert.
Alle sieben Entscheidungen im Register bleiben Kandidaten und nicht ratifiziert.
P1a ist offen, P2 ist gesperrt, eine Produktionsfreigabe ist nicht erteilt.
