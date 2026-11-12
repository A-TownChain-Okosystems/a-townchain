# Vier Architekturfragen vor dem Abschluss der Integration (D-0)

> **Status: beantwortet.** Die vier Fragen sind am 12.11.2026 je mit Option **A**
> entschieden; die Antworten, Folgen und abgeleiteten Anforderungen stehen in
> [AD-EXEC-D0-FOUR-DECISIONS-2026-11-12.md](AD-EXEC-D0-FOUR-DECISIONS-2026-11-12.md).
> Dieses Dokument bleibt als Vorlage erhalten (Ist-Stand, Optionen, Empfehlung).
> `ATCE/0x02` bleibt trotzdem `OPEN`: Abschluss verlangt P1a (Blockimport) und
> einen CI-Lauf, nicht nur die Entscheidung.

## F-1 Containergrenze

**Frage.** Ist `ATCE`/`0x02` kuenftig der verbindliche externe Eingangsvertrag fuer
saemtliche ausfuehrbaren Artefakte?

**Ist-Stand.** Der Deploy-Weg ist umgestellt: `System::deploy_container` nimmt
ausschliesslich die Huelle an, `System::execute` liest den Strom nur ueber die
Huelle, und eine `ATCB`-Eingabe ergibt `E-007` (Nachweis
`EXEC-D0A-SYSTEM-INGRESS-EVIDENCE-2026-11-12.md`). **Nicht** umgestellt sind die
Kettenseite und der Knoten: `Block.payload` ist ein String
(`components/node/src/chain.rs`), der keinen Container transportiert, und ein
Block-Import prueft heute keinen Container.

**Optionen.**

| Option | Inhalt | Folge |
|---|---|---|
| A (Empfehlung) | Huelle ist der einzige externe Eingang: Deploy, RPC, Block-Import und Bootstrap nehmen ausschliesslich `ATCE` an | ein Eingangsvertrag, keine Umgehung; erfordert Block-/Payload-Format und Importpfad anzupassen (eigene Entscheidung, beruehrt ATC-BC-002) |
| B | Huelle nur im Deploy-Pfad, Block-Payload bleibt unveraendert | Integritaet des Deploy-Wegs ja, aber der Import bleibt ein zweiter Weg ohne Huellenpruefung |
| C | Huelle optional, `ATCB` weiter zulaessig | zwei Eingangsformate, genau die Vermischung, die D-0 ausschliesst |

**Empfehlung A**, mit ausdruecklicher Konsequenz: die Payload-Frage ist Teil der
Antwort. Solange B gilt, ist die Sicherheitsgrenze nur im Deploy-Weg durchgesetzt.

## F-2 Interne Grenze

**Frage.** Darf `ATCB` weiterhin innerhalb der VM als internes Artefaktformat
verwendet werden?

**Ist-Stand.** Ja, und zwar ausschliesslich intern: `Artifact::parse` weist `ATCE`
mit `E-007` ab, `Envelope::parse` weist `ATCB` mit `E-007` ab, `into_artifact()`
ist der einzige Uebergang. Der Compiler/Backend erzeugt `ATCB` aus der Quelle, die
Ausfuehrung (`Vm::from_artifact_with_context`) arbeitet auf `Artifact`.

**Optionen.**

| Option | Inhalt | Folge |
|---|---|---|
| A (Empfehlung) | `ATCB` bleibt internes Ausfuehrungsformat und wird nie von aussen angenommen | eine Dekodierungsautoritaet, keine Formatkopie; die VM bleibt frei von Huellenwissen |
| B | auch intern nur noch Huelle fuehren | jede Ausfuehrung muesste die Huelle neu pruefen; Digest-Arbeit im Ausfuehrungspfad, kein Sicherheitsgewinn |

**Empfehlung A.**

## F-3 Authentizitaet

**Frage.** Wo wird die Signatur eines Artefaktherausgebers geprueft, falls eine
solche erforderlich ist?

**Ist-Stand.** Es gibt **keine** Herausgeber-Signatur des Artefakts. Vorhanden
sind: `artifact_hash` der Huelle und der Bytecode-Hash im Container (Integritaet,
Domain Separation), die Freigabe (`ExecutionPolicy`) mit Signatur des
Freigabeschluessels, und `atc_exec_admission` als Pruefinstanz. SHA-256 belegt
Unversehrtheit, nicht Herkunft.

**Optionen.**

| Option | Inhalt | Folge |
|---|---|---|
| A (Empfehlung) | Signatur als **eigene** Ebene ausserhalb des Containers, ueber `artifact_hash` mit eigener Domaene und eigenem Schluessel (Modell-Release-Schluessel duerfen Artefakte nicht signieren) | Containerformat bleibt unveraendert, Signatur ist optional und nachruestbar; braucht Schluesselrollen, Widerruf und ein Bedrohungsmodell |
| B | Signatur im Container (Feld im Kopf) | Formatwechsel, jede Fassung haengt an der Signatur, `artifact_hash` muesste die Signatur ein- oder ausschliessen (Zirkelgefahr) |
| C | keine Herausgeber-Signatur, nur Freigabe | ausreichend, wenn das Bedrohungsmodell keinen fremden Herausgeber kennt; muss dann ausdruecklich so entschieden werden |

**Empfehlung A**, aber nur wenn das Bedrohungsmodell einen Herausgeber kennt, der
nicht mit dem Freigabeschluessel identisch ist. Sonst ist C die ehrlichere Antwort.

## F-4 Versionierung

**Frage.** Wie werden kuenftige Containerfassungen eingefuehrt, ohne bestehende
Parser oder Bytecode-Hashes zu veraendern?

**Ist-Stand.** Zwei getrennte Versionsraeume: Containerfassung `bytes[4]` (heute
`0x02`), Generation `bytes[5]` (1 und 2 zulaessig). Fremde Containerfassung ergibt
`E-003` bei 4, unbekannte Generation `E-009` bei 5, Opcodes ab Generation 2 ohne
Generation 2 ergeben `E-011`. Kein Fallback, kein spekulatives Parsen. Der
Bytecode-Hash bindet Strom und Generation, nicht die Containerfassung.

**Optionen.**

| Option | Inhalt | Folge |
|---|---|---|
| A (Empfehlung) | je Containerfassung ein eigener Parser in eigener Stufe; `E-003` fuer alles Fremde; Opcode-Aenderungen nur als neue **Generation**; alte Fassungen bleiben lesbar bis Widerruf | Bytecode-Hashes bleiben stabil, Parser bleiben getrennt; Fassungswechsel ist eine Entscheidung mit eigenem Nachweis und eigener Matrix |
| B | eine Fassung mit Feature-Flags im Kopf | ein Parser mit Verzweigungen, jedes Flag ist eine stille Formatvariante |
| C | Fassung ersetzt die alte | alte Artefakte werden unlesbar, Widerspruch zur Zusage "Bytecode-Hash bleibt unveraendert" |

**Empfehlung A**, ausdruecklich mit der Regel: eine neue Containerfassung aendert
nie den Bytecode-Hash und nie die Bedeutung bestehender Opcodes.

## Was zum Abschluss noetig ist

1. Je Frage eine Entscheidung (A, B oder C) mit Begruendung, eingetragen im
   Register (`docs/decisions/exec-decision-register.yaml`).
2. F-1 zieht, wenn A gewaehlt wird, eine eigene Aufgabe nach sich:
   Block-/Payload-Format und Importpfad.
3. F-3 braucht vor jeder Umsetzung ein Bedrohungsmodell und eine Aussage zu
   Schluesselrollen und Widerruf.
4. Erst danach kann `ATCE/0x02` von `OPEN` weg; `CI_VERIFIED` bleibt davon
   unabhaengig und braucht einen CI-Lauf am exakten HEAD.
