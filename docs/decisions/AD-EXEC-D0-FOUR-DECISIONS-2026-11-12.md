# D0 — die vier Architekturentscheidungen zur Containergrenze

> **Status:** entschieden (Owner, 12.11.2026), je Option **A**. Intern verbindlich
> fuer dieses Repository. **Keine** Ratifikation des Kandidaten, **keine**
> Freigabe fuer Ausfuehrung oder Release, **kein** `CI_VERIFIED`.
> Register: `docs/decisions/exec-decision-register.yaml` (D-0, Teilauflagen
> D0-1 bis D0-4).

Die vier Fragen stammen aus der Vorlage
[EXEC-D0-ARCH-QUESTIONS-TEMPLATE-2026-11-12.md](EXEC-D0-ARCH-QUESTIONS-TEMPLATE-2026-11-12.md).
Hier stehen die Antworten, ihre Folgen und was daraus umgesetzt werden muss.

## D0-1 Containergrenze

**Entscheidung (A).** `ATCE`/`0x02` ist der **verbindliche externe Eingang** fuer
saemtliche ausfuehrbaren Artefakte. Jeder Weg von aussen (Deploy, RPC,
Block-Import, Bootstrap) nimmt denselben Containervertrag an.

**Begruendung.** Ein einziger externer Vertrag ist pruefbar; zwei Eingangsformen
waeren genau die Vermischung, die D-0 ausschliesst (Audit IV-03: gleiches Magic
mit anderer Semantik ist untersagt).

**Folgen.**

1. `Block.payload` muss kuenftig den Container transportieren. Heute ist der
   Payload ein String (`components/node/src/chain.rs`), der keinen Container
   fasst: das ist die verbleibende Luecke.
2. Block-Import und Systemeingang verwenden **denselben** Vertrag. Die Luecke
   darf **nicht** durch eine zweite akzeptierte Eingangsform geschlossen werden.
3. `ATCB` erhaelt keine externe Route, auch nicht als "Notpfad" im Import.

**Abgeleitete Anforderungen (vorzuschlagen, nicht ratifiziert).**

| ID | Anforderung |
|---|---|
| D0-1-A1 | Externe ausfuehrbare Artefakte werden ausschliesslich als `ATCE` angenommen |
| D0-1-A2 | Der Block-Payload transportiert den Container; die Kette fuehrt keinen nackten `ATCB`-Strom |
| D0-1-A3 | Import und Systemeingang rufen dieselbe Huellenpruefung auf; keine zweite Implementierung |
| D0-1-A4 | Ein ungueltiger Container erzeugt weder Zustand noch Block, auf keinem Weg |

**Offene Umsetzung.** P1a: Payload-Format und Importpfad. Eigene Aufgabe, eigene
Matrix, weil sie das Blockformat beruehrt (ATC-BC-002 ist nicht ratifiziert).

## D0-2 Interne Grenze

**Entscheidung (A).** `ATCB`/`0x01` bleibt **internes** Artefakt- und
Ausfuehrungsformat und ist von aussen nicht erreichbar.

**Begruendung.** Damit bleibt genau eine Bytecodeform, eine Opcode-Definition,
eine Operandenbreite und eine Strompruefung. Der VM-Kern wird nicht angefasst,
das bestehende Artefaktmodell bleibt rueckwaertskompatibel (Fixwert
`0dee1036…` unveraendert).

**Folgen.**

1. `into_artifact()` ist der **einzige** Uebergang von aussen nach innen.
2. `Envelope::parse` und `Artifact::parse` weisen das Magic des jeweils anderen
   mit `E-007` ab; kein Fallback, kein spekulatives Parsen.
3. Der Compiler/Backend erzeugt `ATCB` aus der Quelle: das ist Erzeugung, kein
   externer Eingang.

**Abgeleitete Anforderungen.**

| ID | Anforderung |
|---|---|
| D0-2-A1 | Eine autoritative Bytecode-Dekodierung (`dekodiere_strom`); Huellen bilden nur Fehler ab |
| D0-2-A2 | `ATCB` ist kein externer Eingang; jede externe Zufuhr ergibt `E-007` |
| D0-2-A3 | Interne Erzeugung aus der Quelle ist zulaessig und erzeugt dieselbe Bytefolge wie die Kapselung |

## D0-3 Authentizitaet

**Entscheidung (A).** Eine Herausgeber-Signatur ist eine **eigene Ebene
ausserhalb** des Containers, gebunden an den freigegebenen `artifact_hash`. Sie
wird **nicht** in den Container eingebettet.

**Begruendung.** Eine Pruefsumme belegt Unversehrtheit, nicht Herkunft. Laege die
Signatur im Container, muesste der Digest sie ein- oder ausschliessen; beides
erzeugt einen Zirkel oder ein Format, das an der Signatur haengt.

**Folgen und Auflagen.**

1. Signaturpruefung, Schluesselidentitaet, Widerruf und Vertrauensrichtlinien
   werden **getrennt** spezifiziert (eigener Standard, nicht Teil von §18/§19).
2. Bindung ist eindeutig: Signatur ueber `artifact_hash` mit eigener Domaene.
   Ein gueltiger SHA-256-Wert allein gilt **nie** als Herkunftsnachweis.
3. Schluesselrollen bleiben getrennt (K7): Modell-Release-Schluessel duerfen
   keine Artefakte signieren; Freigabeschluessel (Policy) und Herausgeber sind
   nicht dasselbe.
4. Ohne Bedrohungsmodell keine Umsetzung. Wenn es keinen vom
   Freigabeschluessel verschiedenen Herausgeber gibt, ist "keine
   Herausgeber-Signatur" die ehrlichere Antwort und muss so entschieden werden.

**Abgeleitete Anforderungen.**

| ID | Anforderung |
|---|---|
| D0-3-A1 | Signatur ueber `artifact_hash`, eigene Domaene, ausserhalb des Containers |
| D0-3-A2 | Zwei getrennte Ergebnisse: Integritaet (Digest) und Herkunft (Signatur); nie verschmolzen |
| D0-3-A3 | Widerruf und Rotation sind Teil der Vertrauensrichtlinie, nicht des Containers |

**Status.** Entscheidung getroffen, Umsetzung **nicht** freigegeben: sie braucht
Bedrohungsmodell und Schluesselrichtlinie.

## D0-4 Versionierung

**Entscheidung (A).** Je Containerfassung ein **eigener Parser** in eigener
Stufe. Zusaetzlich verbindlich:

1. **Keine** automatische Versionskonvertierung.
2. **Keine** stillschweigende Interpretation unbekannter Fassungen (`E-003`).
3. Opcode-Aenderungen **nur** ueber definierte Generationen (`E-011`, `E-009`).
4. Alte Parser bleiben fuer ihre Fassungen **deterministisch** und unveraendert.

**Begruendung.** Der Bytecode-Hash bindet Strom und Generation, nicht die
Containerfassung. Solange beide Versionsraeume getrennt bleiben, kann eine neue
Huelle eingefuehrt werden, ohne bestehende Bytecode-Hashes oder Artefakte
ungueltig zu machen.

**Folgen.**

1. Formatidentitaet ist das Tupel (Magic, Fassung, Struktur), nie das Magic
   allein; Pruefreihenfolge Laenge, Magic, Fassung, Generation, Struktur, Tail.
2. Eine neue Fassung ist eine Entscheidung mit eigenem Nachweis, eigener Matrix
   und eigenem Vektor-Satz, kein Nebenprodukt einer Aenderung.
3. Ein Fassungswechsel ersetzt die alte Fassung **nicht**: Lesbarkeit bleibt bis
   zum ausdruecklichen Widerruf.

**Abgeleitete Anforderungen.**

| ID | Anforderung |
|---|---|
| D0-4-A1 | Ein Parser je Fassung; fremde Fassung ergibt `E-003` bei Offset 4 |
| D0-4-A2 | Kein Fallback, keine Konvertierung, kein spekulatives Parsen |
| D0-4-A3 | Neue Opcodes nur als neue Generation, nie als stille Erweiterung |
| D0-4-A4 | Bytecode-Hash bleibt ueber Fassungswechsel hinweg stabil |

## Nicht verhandelbare Invarianten

Diese neun Saetze gelten fuer jede weitere Aenderung und werden nicht
verhandelt (Owner, 12.11.2026):

| ID | Invariante |
|---|---|
| I1 | `ATCE` ist die einzige externe ausfuehrbare Eingangsform |
| I2 | `ATCB` bleibt intern und erhaelt keine externe Umgehungsroute |
| I3 | Eine einzige autoritative Bytecode-Dekodierung |
| I4 | Container-Digest und Bytecode-Hash bleiben getrennt |
| I5 | Der bestehende Bytecode-Fixwert wird nicht veraendert |
| I6 | Die Admission Registry bleibt zentral und eindeutig |
| I7 | Kein Zustand und kein Block bei ungueltigem Container |
| I8 | Kein `CI_VERIFIED` ohne aktuellen CI-Nachweis |
| I9 | Keine Statusaufwertung allein aufgrund gruener lokaler Tests |

## Abgleich mit den bestehenden Standards

Geprueft gegen `registry/standards.yaml` in `atc-standards` (Stand 12.11.2026).

| Standard | Status | Beruehrung durch D0 | Ergebnis |
|---|---|---|---|
| ATC-STD-000 (Governance) | approved 1.3.0 | Rangfolge, Domaenen | kein Widerspruch; D0 ist eine Entscheidung, kein neuer Standard |
| ATC-STD-SC-001 (Smart Contract General) | approved 1.0.0 | Registry-Pflichten (SC-019), Audit | kein Widerspruch; der Standard regelt Oekosystem/Registry, nicht Bytecode-Container |
| ATC-STD-204 (Dependency & Interface) | approved 1.0.0 | Abhaengigkeiten der drei Schichten | kein Widerspruch; Huelle/VM sind getrennte Schichten ohne Zusatzabhaengigkeit |
| ATC-STD-203 (Repository Security & Release) | approved 1.0.1 | Evidence-Pflichten | kein Widerspruch |

**Befund (Governance).** Die Dokumente, auf die sich D0 inhaltlich stuetzt —
`ATC-SEC-EXEC-001`, `ATC-BC-001`, `ATC-BC-002` — sind **keine Eintraege** in
`registry/standards.yaml`. Sie existieren als Spezifikation/Kandidat in
`a-townchain`, sind also nicht normativ registriert. Folge: D0 ist intern
verbindlich, aber nicht gegen einen registrierten Standard durchsetzbar, und
"normativ festschreiben" heisst heute: **Entscheidung**, nicht **Standard**.
Wenn die Regeln ueber dieses Repository hinaus gelten sollen, brauchen sie
Registry-Eintraege. Das ist eine Governance-Aufgabe, keine Code-Aufgabe, und
wird hier nicht selbsttaetig vorgenommen.

## Reihenfolge (Owner, 12.11.2026)

| Schritt | Inhalt | Status |
|---|---|---|
| D0 | vier Entscheidungen dokumentieren (dieses Dokument) | erledigt, entschieden |
| S1-2 | F-1 isoliert: Policy-Ueberschreibung verhindern | OPEN, [Analyse](EXEC-S1-2-ADMISSION-REGISTRY-F1.md) |
| P1a | `Block.payload` und Import auf den Containervertrag | OPEN |
| P2 | Differentialtest verpflichtend in der CI | BLOCKED |
| Abschluss | frischer Default-Branch, CI-Lauf, Logs, Artefakt-Hashes | nicht freigegeben |
