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
denselben Wert ausgeben. Das belegt **eine** Sache: dass beide Ausgaben heute
dieselbe Funktion benutzen. Es belegt **keine** unabhaengige Strom-Hash-Berechnung,
denn eine solche gibt es nicht. Wer den Strom-Hash braucht, muss ihn definieren und
implementieren, nicht umleiten.

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

### 1.7 Zusatzfrage: Aendert die Domaenen-Trennung den bestehenden Artefakt-Hash?

Der Review verlangt eigene Domaenen fuer beide Hashes (S1-T-19) und weist richtig
darauf hin, dass eine nachtraegliche Domaenen-Trennung den bisherigen Wert von
`Artifact::hash()` aendern kann. Das ist exakt der Fall, und zwar unvermeidbar:
jedes zusaetzliche Byte vor der Eingabe aendert den Digest.

**Ist-Stand.** `artifact.rs:273-284` baut die Eingabe als
`MAGIC(4) + CONTAINER_VERSION(1) + generation(1) + Anzahl(4, LE) + Opcodes`.
Es gibt **keinen** Domaenen-Praefix. Der Containerkopf ist damit heute die
faktische Domaine des Artefakt-Hashes.

**Und ein belegbarer Unterschied.** Ein Container beginnt immer mit `0x41`
(`"ATCB"`). Ein Strom beginnt mit einem Opcode; `artifact.rs:130-151` kennt 19
Opcodes von `0x01` bis `0x13`, `0x41` ist keiner davon. Die beiden Bytebereiche
koennen sich daher heute nicht vermischen, auch ohne Praefix. (Ein leerer Strom
hat kein erstes Byte; er ist von jedem Container verschieden, weil dieser
mindestens neun Byte traegt.)

**Drei Varianten, alle mit Option A vereinbar.** Die Wahl A (drei Identitaeten)
bleibt unberuehrt; offen ist nur, wie die Trennung hergestellt wird.

| Variante | Artefakt-Hash | Strom-Hash | Wert des Artefakt-Hashes |
|---|---|---|---|
| S1-1a | ohne Praefix, Trennung strukturell ueber den Containerkopf | eigener Praefix, z. B. `ATC-STREAM-V1\0` | **unveraendert** |
| S1-1b | eigener Praefix `ATC-ARTIFACT-V1\0` | eigener Praefix | **aendert sich fuer jedes Artefakt** |
| S1-1c | Praefix nur fuer neue Containerfassung, Altfassung ohne | eigener Praefix | haengt an der Containerfassung |

**Folgen.**

| Bereich | S1-1a | S1-1b | S1-1c |
|---|---|---|---|
| elf abhaengige Stellen | Werte bleiben, nur die Zustandswurzel wechselt | Werte aendern sich alle | zwei Lesepfade dauerhaft |
| Containerformat | unveraendert | Formatfassung muss steigen | Formatfassung steuert den Pfad |
| Auditierbarkeit | Trennung haengt an einer stillen Invariante (kein Opcode `0x41`) | eine Regel fuer alle Identitaeten | eine Regel mit Ausnahme |
| Risiko | ein spaeterer Opcode `0x41` hebt die Trennung auf, ohne dass es auffaellt | einmalige Wertänderung, heute ohne Bestand | dauerhaft zwei Wahrheiten ueber denselben Gegenstand |

**Technische Einschaetzung (keine Entscheidung).** S1-1a ist nur so lange sicher,
wie die Opcode-Menge `0x41` nicht enthaelt. Diese Bedingung ist nirgends
aufgeschrieben und wuerde bei einer Erweiterung des Instruktionssatzes niemandem
auffallen. S1-1b kostet heute nur eine Neuverifikation, weil kein Artefakt-Hash
gespeichert ist, und kauft dafuer eine Regel ohne Ausnahme. S1-1c traegt zwei
Wahrheiten dauerhaft weiter.

**Zusatzpruefungen.**

| ID | Pruefung | Bestehen |
|---|---|---|
| S1-T-25 | erste Bytes | Strom beginnt in `0x01..0x13`, Container mit `0x41`; die Pruefung ist Test, nicht Kommentar |
| S1-T-26 | Schutz der Trennung | ein Opcode `0x41` oder eine neue Containerfassung ohne Domaenenangabe wird von einem Gate abgewiesen |
| S1-T-27 | Wertwechsel | aendert sich der Artefakt-Hash, sind alte Werte als ueberholt dokumentiert und der Goldvektor ist erneuert |
| S1-T-28 | Benennung | kein Codepfad und kein Dokument verwendet `hash` ohne Qualifizierung (`code_hash`, `artifact_hash`, `state_root`) |

**Zum Zustandsbaum.** `ATC-STATE-V2` ist nicht wegen des Praefixes noetig, sondern
weil sich die **Eingabe** der Wurzel aendert: heute der Artefakt-Hash, kuenftig der
Strom-Hash (`state.rs:43-52`). Auch bei S1-1a, das den Artefakt-Hash unveraendert
laesst, wechselt damit die Wurzel.

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

### 2.6 Umsetzbarkeit der asymmetrischen Bindung heute

Der Review verlangt getrennte Schluesselraeume und Vorrangregeln. Am Code geprueft
ist das heute **nicht** darstellbar, ohne die Datenstruktur zu aendern:

| Stelle | Stand |
|---|---|
| `exec-admission/src/lib.rs:191` | `policies: BTreeMap<[u8; 32], ExecutionPolicy>`, Schluessel ist `policy.code_hash` |
| `:335` | `revoked: BTreeSet<[u8; 32]>`, flach, ohne Typangabe |
| `:334` | `min_versions: BTreeMap<[u8; 32], u64>` |
| `:486` | `min_versions.get(&policy.code_hash)` |
| `:500` | `revoked.contains(&policy.code_hash)` |

Zwei Folgen, die die Option C zu einer Strukturaenderung machen und nicht zu einem
zweiten Eintrag:

1. **Falscher Blickwinkel.** Beide Abfragen pruefen `policy.code_hash`, nie
   `request.observed_hash`. Ein Widerruf, der auf den Artefakt-Hash zeigt, wuerde
   heute nie gefunden, weil niemand den beobachteten Hash fragt.
2. **Typverlust.** `revoked` ist eine Menge aus 32 Byte ohne Kennzeichnung. Ein
   Artefakt-Hash in dieser Menge waere von einem Code-Hash nicht unterscheidbar und
   wuerde als Code-Widerruf gelesen, also genau der Fehler, den die Option
   ausschliessen soll.

Ein Symptom dafuer steht schon im Code: `:434` vergleicht `observed_hash` mit
`policy.code_hash` und meldet dabei "Artefakt-Hash stimmt nicht mit der Freigabe
ueberein". Die Meldung nennt bereits den Artefakt-Hash, wo ein Code-Hash gemeint
ist.

**Damit Option C tragfaehig ist**, braucht sie: getypte Schluesselraeume (zwei
Mengen oder ein getaggter Schluessel), eine zweite Abfrage auf
`request.observed_hash`, Ablehnung bei Treffer auf einer der beiden Sperren, je
einen eigenen Statuscode, damit ein Treffer nicht der falschen Sperre zugeschrieben
wird, und die Zurueckweisung ungetypter Altbelegungen beim Start.

| ID | Pruefung | Bestehen |
|---|---|---|
| S1-T-29 | Schluesselraum | jeder Eintrag nennt seine Identitaet; ein Eintrag ohne Angabe wird abgelehnt |
| S1-T-30 | beide Abfragen | Code- und Artefakt-Sperre werden beide geprueft; ein Treffer auf einer davon blockiert |
| S1-T-31 | Trennung der Wirkung | eine Artefakt-Sperre sperrt nicht den Code in anderer Verpackung, eine Code-Sperre nicht das Artefakt einer anderen Fassung, wie entschieden |
| S1-T-32 | Begruendung | der Ablehnungsgrund nennt die Sperre, die getroffen hat, und nicht die andere |

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
| S1-T-17 | Ehrlichkeit | ein L3-Fall kann nicht als `TESTED` gemeldet werden, solange B-1 offen ist (Status `BLOCKED`) |
| S1-T-18 | Dekoder | L1 braucht einen reinen Dekoder; ohne ihn ist L1 `EMPTY`, nicht `TESTED` und nicht "teilweise bestanden" |

Das Statusvokabular steht in Abschnitt 5.1. Es gilt fuer alle drei Schichten: ein
nicht ausfuehrbarer Fall ist `BLOCKED`, `EXCLUDED` oder `EMPTY`, niemals `TESTED`.

## 5. Auflagen aus dem Review vom 12.11.2026

Diese Bedingungen sind Voraussetzung dafuer, dass S1-1 ueberhaupt abnahmefaehig ist.
Sie stehen hier als Pruefungen, nicht als gewaehlte Option.

| Bedingung | Pruefung | Test |
|---|---|---|
| Getrennte Hash-Domaenen | Artefakt-Hash und Strom-Hash werden aus verschiedenen, exakt benannten Bytebereichen berechnet, jeder mit eigener Domaine; eine Aenderung ausserhalb des Stroms aendert den Artefakt-Hash und **nicht** den Strom-Hash | S1-T-19 |
| Kanonische Eingabe | keine implizite Konvertierung, keine nicht deterministische Serialisierung; zwei gleichwertige Darstellungen ergeben denselben Hash, zwei Prozesse dasselbe Ergebnis | S1-T-20 |
| Identitaetstrennung | gleicher Strom mit unterschiedlichen Container-Metadaten (Magic, Generation, Anzahl): Strom-Hash gleich, Artefakt-Hash verschieden | S1-T-01, S1-T-21 |
| Zustandswurzel | `ATC-STATE-V2` legt die neue Semantik eindeutig fest; `V1` wird nie stillschweigend als `V2` gelesen; beide Wurzeln sind unterscheidbar | S1-T-03, S1-T-22 |
| Policy-Migration | keine automatische Uebernahme alter Freigabe- oder Widerrufsschluessel; die Neuregistrierung ist nachweisbar (Zaehlung, Protokolleintrag) | S1-T-23 |
| Startpruefung | gemischte V1-/V2-Identitaeten werden erkannt und zurueckgewiesen; kein Rueckfall auf alte Schluessel | S1-T-24 |

### 5.1 Teststatus-Vokabular

| Status | Bedeutung |
|---|---|
| `TESTED` | ausgefuehrt und bestanden |
| `FAILED` | ausgefuehrt und nicht bestanden |
| `BLOCKED` | ausfuehrbar, aber Voraussetzung fehlt (z. B. B-1) |
| `EXCLUDED` | bewusst nicht im Umfang dieser Stufe |
| `EMPTY` | die Schicht enthaelt keine ausfuehrbaren Faelle (z. B. L1 ohne Dekoder) |

Ein nicht ausfuehrbarer Fall wird **nie** als `TESTED` oder "teilweise bestanden"
gefuehrt, sondern als `BLOCKED`, `EXCLUDED` oder `EMPTY`. Damit ist S1-1 nicht
abnehmbar, solange der Strom-Hash nur beschrieben und nicht berechnet ist.

### 5.2 Zusaetzliche Akzeptanztests

| ID | Pruefung | Bestehen |
|---|---|---|
| S1-T-19 | Domaenen | beide Hashes tragen verschiedene Domaenen; Aenderung der Generation aendert nur `artifact_hash` |
| S1-T-20 | Determinismus | gleiche Eingabe, zwei Prozesse, gleicher Hash; keine Serialisierung mit Host-Zeit, Zufall oder Zeigern |
| S1-T-21 | Metadaten | gleicher Strom, andere Metadaten: `code_hash` gleich, `artifact_hash` verschieden |
| S1-T-22 | Versionstreue | eine V1-Wurzel wird nicht als V2 gelesen; die Domaine ist im Datensatz sichtbar |
| S1-T-23 | Neuregistrierung | alter Schluessel wird nicht uebernommen; Zaehlung vor und nach der Umstellung ist belegbar |
| S1-T-24 | Mischzustand | Start bricht bei gemischten Schluesseln ab, ohne Rueckfall |

## 6. Reihenfolge und Zuordnung zum Register

Die Reihenfolge folgt der Abhaengigkeit, nicht der Wichtigkeit:

| Schritt | Gegenstand | Ergebnis |
|---|---|---|
| 1 | S1-1 und S1-2 | Hash-Semantik und Policy-Identitaet festgelegt; bei S1-1 zusaetzlich die Domaenenvariante S1-1a/b/c (Abschnitt 1.7), bei S1-2 die getypten Schluesselraeume (Abschnitt 2.6) |
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

## 7. Was jetzt nicht passieren darf

1. Keine Implementierung auf Grundlage einer noch nicht gewaehlten Option.
2. Keine Aenderung an `Artifact::hash()` ohne vollstaendige Erfassung der elf
   Abhaengigkeiten aus der Inventur, Abschnitt 2.
3. Keine Aenderung der Zustandswurzel ohne ausdruecklichen V1/V2-Kompatibilitaetsvertrag.
4. Keine Freigabe von P2, solange B-1 nicht entschieden und die Abhaengigkeiten
   nicht geschlossen sind.
5. Keine Hochstufung eines Kandidaten zur ratifizierten Norm durch blosse
   Dokumentenaenderung.

Diese fuenf Punkte sind keine Empfehlungen, sondern die Grenze dieser Vorlage.

## 8. Was diese Vorlage nicht tut

Sie waehlt keine Option, aendert keine Anforderung und setzt keinen Grenzwert.
Alle sieben Entscheidungen im Register bleiben Kandidaten und nicht ratifiziert.
P1a ist offen, P2 ist gesperrt, eine Produktionsfreigabe ist nicht erteilt.

## 9. Empfehlung des Owners vom 12.11.2026 (nicht bestaetigt)

Der Owner hat eine Optionswahl als **Empfehlung** vorgelegt. Sie ist hier
festgehalten, damit die naechste Pruefung nicht mit einer Vermutung arbeitet. Sie
ist **keine** Ratifikation: das Register ist unveraendert, die Anforderungen sind
unveraendert, Aufgabe 910 bleibt offen, P1a bleibt offen und P2 gesperrt.

| Entscheidung | Empfehlung | Begruendung des Owners |
|---|---|---|
| S1-1 | A, drei Identitaeten | Code und Container muessen unabhaengig identifizierbar sein |
| S1-2 | C, asymmetrische Bindung | Freigabe und Mindestversion binden den Code, Widerruf sperrt gezielt Code oder Verpackung |
| S1-3 | A, nur Gas-Rueckfluss | kein Geldmodell, solange Konten- und Gebuehrenmodell es nicht tragen |
| S1-4 | B, nur L1 zaehlt fuer P1a | keine Frame-Semantik durch einen Harness vorwegnehmen |

**Vier Auflagen, die mit der Empfehlung verbunden sind:**

1. S1-1 verlangt zusaetzlich die Klaerung der Domaenenfrage aus Abschnitt 1.7
   (Varianten S1-1a, S1-1b, S1-1c). Diese Unterfrage ist **offen**; die Empfehlung
   A entscheidet sie nicht mit.
2. S1-2 verlangt getrennte Schluesselraeume und Vorrangregeln; Abschnitt 2.6 zeigt,
   dass die heutige Datenstruktur das nicht hergibt.
3. S1-3 schliesst eine Geldrueckzahlung aus, solange kein Kontenmodell existiert.
4. S1-4 aendert nichts daran, dass L1 heute `EMPTY` ist. Option B macht L1 nicht
   testbar; erst ein reiner Dekoder und ausgefuehrte Tests erzeugen `TESTED`.

Bis zur Bestaetigung durch den Owner gilt: keine Option gewaehlt, keine Umsetzung.
