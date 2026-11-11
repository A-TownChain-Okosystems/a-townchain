# ATC-NODE-001 — Betriebsspezifikation Node (KANDIDAT)

> **Status:** KANDIDAT, nicht normativ, nicht ratifiziert
> **Datum:** 11.11.2026 (Rev. 3 — Hash-Entscheidung, leere Wurzeln, Verweis ATC-HASH-002 Rev. 3)
> **Geltungsbereich:** `components/node` (atc-node) und alles, was einen Block erzeugt, annimmt oder ausführt
> **Verhältnis zu bestehenden Standards:** ordnet sich unter ATC-01, ATC-BC-001/002, ATC-SEC-EXEC-001 (Kandidat) und ATC-STD-NET-001…008. Bei Widerspruch gilt der ältere normative Standard, **außer** die in Abschnitt 3a dokumentierte Owner-Entscheidung ersetzt ihn ausdrücklich (ATC-HASH-001 als Konsenshash).

---

## 0. Zweck, Abgrenzung und die eine Unterscheidung

Dieses Dokument legt fest, was ein A-TownChain-Node **tun muss**, um als betriebsfähig zu
gelten. Es beschreibt keine Implementierung und ändert keinen Code.

Verbindlich eingeführt wird die Unterscheidung, die bisher nirgends normativ steht:

| Betriebsart | Definition | Nachweis |
|---|---|---|
| **DEVNET** | Ein Prozess, Genesis im Speicher, Blöcke ohne Transaktionen, Konsens durch lokal erzeugte Stimmen, Datenverlust bei Neustart erlaubt | Prozess läuft, Kette verifiziert innerhalb des Prozesses |
| **BETRIEB** | Mehrere unabhängige Validatoren auf getrennten Maschinen, dauerhafte Blöcke, echte Transaktionsausführung, Neustart ohne Zustandsverlust | Evidenz nach Abschnitt 10, gebunden an einen Commit |

**Ein erfolgreicher Konsenslauf innerhalb eines Prozesses ist kein Nachweis für verteilten
Konsens.** Er belegt die Mechanik der Runde und nichts darüber hinaus. Jede Evidenz muss
die Betriebsart benennen, auf die sie sich bezieht.

---

## 1. Abgleich mit bestehenden Standards

| Standard | Was er fordert | Stand im Code (11.11.2026) | Abweichung |
|---|---|---|---|
| ATC-01 §1.1 | P2P-Mesh, Port 4001, Bootstrap 5005, RPC 9944 | RPC `127.0.0.1:39471`, Gossip `127.0.0.1:39472`, kein Mesh | **Ports und Topologie weichen ab** |
| ATC-01 §1.3 | `HELLO`/`HELLO_ACK` mit Version, Rolle, `chain_id` | kein Handshake, kein Rollenbegriff | **fehlt** |
| ATC-01 §2.3 | Identität als ECDSA (secp256k1), Adresse `ATC` + 32 Zeichen | Node-Identität vorhanden, Rolle ungebunden | teilweise |
| ATC-01 §2.2 | pro Nachricht signiert (statt Noise) | Gossip-Pull ohne Signatur | **fehlt** |
| ATC-STD-NET-001…003 | Devnet 658469, Testnet 658468, Mainnet 658467, Mainnet-Genesis unveränderlich | `Genesis::devnet()` | Konfiguration fehlt |
| ATC-BC-001/002 | genau ein Bytecode-Vertrag, genau eine Ausführungssemantik | `atc-node` hängt nicht an `atc-vm` | **keine Ausführung** |
| ATC-HASH-001 | TownHash-256, Devnet-Grade, F-067-Gate vor Mainnet | Blockhash nutzt `atc_hash` | **wird ersetzt, s. 3a** |
| ATC-SEC-EXEC-001 (Kandidat) | Code-Admission vor Ausführung | kein Admission-Punkt im Node | **fehlt** |

---

## 2. Netzidentität, Rollen, Verbindungsaufbau

**N-2.1 (MUSS)** Die Chain-ID kommt aus der Konfiguration und ist kein Literal im Code.
Devnet 658469, Testnet 658468, Mainnet 658467 (ATC-STD-NET). Ein Node mit falscher
Chain-ID darf keinen Block annehmen und keine Stimme senden.

**N-2.2 (MUSS)** Jeder Node hat genau eine Rolle: `FULL`, `LIGHT`, `VALIDATOR`, `MINER`
(ATC-01 §1.3). Rollen sind im Handshake deklariert und werden durchgesetzt, nicht nur
angezeigt. Ein `LIGHT`-Node stimmt nicht ab.

**N-2.3 (MUSS)** Der Verbindungsaufbau folgt ATC-01: `HELLO` → `HELLO_ACK` mit
Protokollversion, Chain-ID, Genesis-Hash, Rolle, Zeitstempel, Signatur über den kanonischen
Nachrichtenrumpf. Abweichende Genesis oder Chain-ID beendet die Verbindung vor jedem
weiteren Austausch.

**N-2.4 (MUSS)** Konsensrelevante Nachrichten (Proposal, Vote, Block) sind einzeln
signiert; der Empfänger prüft vor der Verarbeitung. Unsignierte Konsensnachrichten werden
verworfen und gezählt, nicht still ignoriert.

**N-2.5 (MUSS) — Ports.** Die Portfrage wird **nicht** durch Anpassen der Konfiguration
erledigt. Zuerst ist die Dienstezuordnung normativ zu klären, dann folgt die Konfiguration.
Die offene Zuordnung:

| ATC-01 | Vorgesehene Funktion | Heute im Code | Zu entscheiden |
|---|---|---|---|
| 4001 | P2P-Kommunikation (Mesh, Gossip) | Gossip-Pull auf 39472 | Ist Pull-Sync die P2P-Funktion oder nur ein Übergang? |
| 5005 | Bootstrap-Node | kein Bootstrap-Dienst | Braucht Devnet einen Bootstrap-Dienst oder DNS-Seed? |
| 9944 | WebSocket-RPC | Zeilen-RPC + JSON-RPC auf 39471, kein WS | Bleibt WS normativ, oder wird HTTP-RPC standardisiert? |

Ergebnis dieser Klärung ist eine Tabelle „Dienst → Port → Pflicht/Nicht-Pflicht", die in
ATC-01 nachgezogen wird. Erst danach werden Standardwerte im Code gesetzt.

---

## 3. Blockheader (normativ)

Heute trägt `Block` vier Felder: `height`, `prev_hash`, `payload`, `hash`. Der Header MUSS
mindestens tragen:

| Feld | Typ / Kodierung | Bedeutung |
|---|---|---|
| `version` | u16 BE | Headerformat, nicht Protokollversion |
| `chain_id` | u64 BE | Netzidentität |
| `height` | u64 BE | Höhe, Genesis = 0 |
| `prev_hash` | 32 Byte | Hash des Vorgängers, Genesis bindet Genesis-Hash |
| `tx_root` | 32 Byte | Wurzel über die Transaktions-IDs in kanonischer Reihenfolge |
| `state_root` | 32 Byte | Zustandswurzel **nach** Ausführung aller Transaktionen |
| `receipt_root` | 32 Byte | Wurzel über Ergebnisse und Gasverbrauch |
| `gas_used` | u64 BE | Summe des verbrauchten Gases |
| `timestamp` | u64 BE | Zeitbasis nach ATC-10 (Median Time Past), nicht Wanduhr |
| `proposer` | 33 Byte | komprimierter SEC1-Schlüssel des Proposers |
| `signature` | 64 Byte | ECDSA über den kanonischen Header ohne dieses Feld |

### 3a. Hashgrundlage (Owner-Entscheidung 11.11.2026)

**ENTSCHIEDEN: SHA-256 ist die Hashgrundlage für Blockhashes, Merkle-Wurzeln,
Zustandswurzeln und Transaktions-IDs im Mainnet-Protokoll.**

Begründung (Owner): langjährige Analyse, breite Implementierungs- und Testbasis, bewährte
Verwendung in Blockchain-Systemen, Interoperabilität, keine Notwendigkeit, eine eigene
kryptografische Konstruktion sicherheitstechnisch zu rechtfertigen. Eigene Entwicklung ist
dort richtig, wo sie einen nachweisbaren technischen Vorteil bringt, nicht bei bereits
etablierten Sicherheitsgrundlagen.

Damit gilt verbindlich:

**N-3a.1 (MUSS)** Jede Konsensfläche nutzt SHA-256 mit **Domain Separation** und fester
Eingabekodierung. Vorgabe:

| Fläche | Domain-Präfix | Eingabe |
|---|---|---|
| Blockhash | `ATC-BLOCK-V2` | kanonischer Header ohne Signatur |
| Transaktions-ID | `ATC-TXID-V1` | kanonische Transaktionsbytes (ohne Signatur) |
| Merkle-Knoten | `ATC-MERKLE-N-V1` | linkes Kind ‖ rechtes Kind |
| Merkle-Blatt | `ATC-MERKLE-L-V1` | Transaktions-ID |
| Zustandswurzel | `ATC-STATE-V1` | kanonisch sortierte Zustandseinträge |
| Belegwurzel | `ATC-RECEIPT-V1` | Belege in Blockreihenfolge |
| Leere Tx-Menge | `ATC-MERKLE-EMPTY-V1` | keine Blätter |
| Leerer Zustand | `ATC-STATE-EMPTY-V1` | keine Einträge |

**N-3a.2 (MUSS)** Kein Knoten, kein Blatt und kein Block wird ohne Domain-Präfix gehasht.
Dadurch sind Blatt- und Knotenhashes nicht gegeneinander austauschbar (Second-Preimage-
Schutz der Merkle-Konstruktion).

**N-3a.3 (MUSS)** Die Ausgabelänge ist 32 Byte. Eine längere Ausgabe ist eine
Protokolländerung, kein Implementierungsdetail.

**N-3a.4 (MUSS)** Die Hashfunktion ist versioniert. Ein Wechsel ist eine
Protokollaktivierung mit Höhe und Zeitpunkt, nie ein Softwareupdate.

**N-3a.5 (SOLL)** Für Nicht-Konsensflächen (Cache-Schlüssel, Log-Indizes, Prüfsummen von
Werkzeugen) ist SHA-256 ebenfalls der Standard, damit im Ökosystem genau eine
Hashgrundlage gilt. Abweichungen werden begründet und inventarisiert.

### 3b. TownHash-256 (Bestandsschutz, keine Konsensfläche)

**N-3b.1 (MUSS)** `ATC-HASH-001` / TownHash-256 wird **nicht** für Blockhashes,
Merkle-Wurzeln, Zustandswurzeln oder Transaktions-IDs eingesetzt — weder im Mainnet noch
im Testnet.

**N-3b.2 (MUSS)** Die Implementierung bleibt erhalten, wird aber als **nicht
kryptografisch validierte Eigenentwicklung** gekennzeichnet: im Modulkopf, im
Standardsregister und in jeder Verwendungsstelle. Historische Testergebnisse bleiben
erhalten, sie gelten nur für diesen Status.

**N-3b.3 (MUSS)** Devnet darf TownHash-256 weiter nutzen, solange die Kennzeichnung
sichtbar ist. Ein Devnet-Hash ist kein Bestandteil einer Mainnet-Evidenz.

**N-3b.4 (SOLL)** Eine spätere unabhängige Analyse bleibt möglich und sinnvoll, ist aber
kein Ersatz für ein etabliertes Verfahren im Mainnet. Eine externe Prüfung allein
garantiert keine Sicherheit.

### 3c. Offener Abgleich mit `atc-standards` (blockierend für Schritt 2)

Befund, geprüft am 11.11.2026:

1. `components/algorithm/src/hash.rs` nennt als Spezifikation
   `docs/SPEC-ATC-HASH-001.md`. Diese Datei existiert **weder** in `a-townchain` **noch**
   in `a-townchain-os-docs` am genannten Pfad (beide HTTP 404).
2. Damit hat die standardisierte Kennung `ATC-HASH-001` aktuell **kein normatives
   Dokument** an der Stelle, auf die der Code verweist.

Konsequenz für die Umsetzung: Die Hashentscheidung braucht ein **neues, nummeriertes
normatives Dokument** (Vorschlag `ATC-HASH-002 — Konsenshashgrundlage SHA-256`), und
`ATC-HASH-001` wird als „Devnet, nicht konsensfähig, ersetzt durch ATC-HASH-002"
gekennzeichnet. Solange dieses Dokument fehlt, ist Schritt 2 der Umsetzungsreihenfolge
nicht abgeschlossen, auch wenn die Owner-Entscheidung selbst klar ist.

---

## 4. Dauerhaftigkeit

**N-4.1 (MUSS)** Blöcke und Ledger liegen auf dauerhaftem Speicher. Nach einem Neustart ist
die Kette ab dem letzten bestätigten Block verfügbar, ohne Genesis-Verlust.

**N-4.2 (MUSS)** Der Abschluss eines Blocks ist atomar: Header, Transaktionen, Zustand und
Zeiger auf den Best-Block werden gemeinsam bestätigt oder gar nicht.

**N-4.3 (MUSS)** Beim Start wird die Kette gegen ihre eigenen Hashes geprüft. Bei
Korruption bricht der Start mit klarer Meldung ab; er repariert nicht stillschweigend und
springt nicht auf Genesis zurück.

**N-4.4 (MUSS)** Das 64-Block-Limit der Devnet-Produktion wird entfernt. Eine
Protokollgrenze ersetzt es nur, wenn sie normativ begründet und getestet ist.

**N-4.5 (SOLL)** Wiederherstellung ist testbar: Neustart nach jedem Schreibschritt,
absichtlich beschädigte Datei, halb geschriebener Block. Jeder Fall hat einen erwarteten
Ausgang.

---

## 5. Konsens

**N-5.1 (MUSS)** Validatoren laufen als getrennte Prozesse auf getrennten Maschinen. Der
Validator-Schlüssel eines Nodes liegt nicht im Prozess eines anderen.

**N-5.2 (MUSS)** Proposals und Votes laufen über das Netz. Ein Vote ist eine signierte
Nachricht mit Runde, Höhe, Blockhash und Validator-Identität.

**N-5.3 (MUSS)** Die Quorumprüfung zählt nach Validator-Gewicht, nicht nach eingegangenen
Nachrichten. Doppelte Stimmen desselben Validators für dieselbe Runde zählen einmal.

**N-5.4 (MUSS)** Ein Validator stimmt pro Runde höchstens einmal ab, und diese Zusage
überlebt einen Neustart (dauerhafte Vote-Historie). Ein Vote für eine bereits abgestimmte
Runde mit anderem Hash ist Äquivokation und wird als Beweis gespeichert.

**N-5.5 (MUSS)** Round-Wechsel und Timeout sind definiert, deterministisch und
aufgezeichnet. Ein Timeout ist ein Zustandsübergang, kein Stillstand.

**N-5.6 (MUSS)** Fork-Wahl ist normativ festgelegt (Vorgabe: höchste finalisierte Höhe,
dann Quorum-Nachweis). Kein Reorg über einen finalisierten Block.

**N-5.7 (SOLL)** Nach einem Neustart nimmt ein Validator seine Rolle nur wieder auf, wenn
Vote-Historie und letzte finalisierte Höhe gelesen wurden.

---

## 6. Transaktionspipeline

**N-6.1 (MUSS)** Der Node nimmt Transaktionen an, prüft sie und hält sie im Mempool. Der
Konsens-Loop erhält die geprüfte Menge; eine leere Liste ist gültig, aber kein
hinreichender Betriebszustand.

**N-6.2 (MUSS)** Aufnahme setzt voraus: gültige Signatur (ATC-TX-DOMAIN-V2, kanonisch),
korrekte Chain-ID, gültige Nonce gegen den Ledger, ausreichendes Guthaben für das Limit,
nicht abgelaufen, kein Duplikat. Ablehnung mit stabilem Code.

**N-6.3 (MUSS)** Die Transaktions-ID ist SHA-256 über die kanonischen Transaktionsbytes mit
Domain `ATC-TXID-V1` und enthält die Signatur nicht. Zwei byte-gleiche Transaktionen haben
dieselbe ID; unterschiedliche Signaturen derselben Absicht nicht.

**N-6.4 (MUSS)** Reihenfolge im Block ist deterministisch und für alle Validatoren gleich.
Sie wird durch die Spezifikation festgelegt, nicht durch Eingangszeit oder Zufall.

**N-6.5 (MUSS)** Ungültige Transaktionen werden ausgeschlossen, nicht in den Block
geschrieben. Scheitert eine Transaktion bei der Ausführung, wird sie als gescheitert
verbucht und belegt Gas, ohne den Block zu blockieren.

**N-6.6 (SOLL)** Gebührenmodell und Mempool-Verdrängung sind festgelegt, bevor ein
öffentliches Netz startet.

---

## 7. Zustandsausführung

**N-7.1 (MUSS)** Der Node führt Vertragsbytecode über `atc-vm` aus, nicht über eigenen Code.
`components/system` ist der einzige Orchestrierungspfad: Quelle → Compiler → Artefakt →
Freigabe → VM → Block. Keine zweite VM, keine zweite Transaktionsimplementierung, keine
zweite Ledger-Wahrheit im Node.

**N-7.2 (MUSS)** Jede Transaktion läuft über den einen Ausführungseinstieg mit Snapshot und
Rücknahme. Teilzustand nach Fehler ist verboten.

**N-7.3 (MUSS)** Gas wird verbraucht, berichtet und im Beleg gebunden. Ein Budget von 0
führt zu vollständiger Ablehnung ohne Zustandsänderung.

**N-7.4 (MUSS)** Die Zustandswurzel wird aus dem resultierenden Zustand mit Domain
`ATC-STATE-V1` berechnet (Abschnitt 3a), in den Header geschrieben und vom Empfänger
nachgerechnet. Abweichung heißt Ablehnung des Blocks.

**N-7.5 (MUSS)** Code wird nur mit Admission ausgeführt (ATC-SEC-EXEC-001). Solange der
Kandidat nicht ratifiziert ist, gilt: kein Fremdcode im Betrieb, der Admission-Punkt bleibt
als offene Lücke dokumentiert.

---

## 8. RPC und Betriebssicherheit

**N-8.1 (MUSS)** RPC ist im Betrieb nur mit TLS erreichbar. Klartext ist Devnet.
**N-8.2 (MUSS)** Jeder RPC-Zugriff ist authentisiert; Lese- und Schreibrechte sind getrennt.
**N-8.3 (MUSS)** Rate Limiting und Ressourcengrenzen sind gesetzt, inklusive
Verbindungszahl und Nachrichtengröße.
**N-8.4 (MUSS)** Validator-Schlüssel sind vom Node-Prozess getrennt geschützt: nicht im
Arbeitsverzeichnis, nicht in Logs.
**N-8.5 (SOLL)** Monitoring erfasst Höhe, Finalität, Vote-Beteiligung, abgelehnte
Nachrichten und Speicherzustand.

---

## 9. Verbote

- Keine zweite VM, kein zweiter Compiler, keine zweite Hash- oder Transaktionsimplementierung im Node.
- Keine Konsenslogik als Ersatz für kryptografische Autorisierung.
- Keine stille Normalisierung zwischen Compiler und VM.
- Keine Evidenz aus einer anderen Betriebsart (Devnet-Lauf belegt keinen Betriebslauf).
- **Kein TownHash-256 auf einer Konsensfläche**, auch nicht vorübergehend „nur zum Testen" in einem Release-Kandidaten.

---

## 10. Nachweisstufen und Testmatrix

Stufen: E1 Ausführung erfolgreich, E2 reproduzierbar, E3 Rechenregel geprüft, E4 Semantik
gegen unabhängige Referenz gleich. Ein Nachweis ohne Betriebsart, Commit und Datum gilt nicht.

| Phase | Nachweis | Mindestumfang |
|---|---|---|
| 0 Protokoll | Header, Hash, Identität, Formate festgelegt | Dokumente gegen ATC-01/BC/HASH abgeglichen |
| 1 Dauerhaftigkeit | Schreiben, Neustart, Korruption | 3 Absturzpunkte, 1 beschädigte Datei, 1 voller Neustart |
| 2 Verteilter Konsens | ≥3 Prozesse, ≥2 Maschinen | Ausfall eines Validators, Verlust von zwei Verbindungen, Wiederverbindung, gleicher finaler Zustand |
| 3 Pipeline | Annahme, Ablehnung, Reihenfolge | je Fehlerklasse ein Fall, doppelte Einreichung, Nonce-Sprung |
| 4 Ausführung | VM-Pfad im Node | gleiche Quelle, gleiches Ergebnis wie Referenz, gleiche Zustandswurzel |
| 5 Betrieb | TLS, Auth, Limits | Zugriff ohne Zertifikat scheitert, ohne Auth scheitert, Limit greift |
| 6 Mainnet | alles zusammen, commitgebunden | ein Commit, alle Phasen grün, Build reproduzierbar |

**Hash-Nachweis (gehört zu Phase 0):** unabhängig erzeugte Sollwerte, keine Laufzeit-
Selbstberechnung, keine Platzhalter. Für SHA-256 sind die FIPS-180-4-Vektoren
(`""`, `"abc"`, Zwei-Block-Nachricht, 1 000 000 × `"a"`) sowie Grenzlängen 55/56/57/63/64/65
zu prüfen. Prüfsummen werden gegen eine zweite, unabhängige Implementierung verglichen.

---

## 11. Offene Entscheidungen

1. ~~Hash für den Blockhash~~ → **entschieden: SHA-256** (Abschnitt 3a).
2. Nummer und Inhalt des neuen Hashstandards (`ATC-HASH-002`?) und Kennzeichnung von `ATC-HASH-001`.
3. Headerfeldmenge: ob `receipt_root` in Version 1 enthalten ist.
4. Zeitbasis: Median Time Past nach ATC-10 oder Proposer-Zeitstempel mit Toleranzfenster.
5. Quorum und Validatorzahl für Testnet und Mainnet.
6. Gebührenmodell und Mempool-Verdrängung.
7. Dienstezuordnung der Ports (Abschnitt 2, N-2.5) — vor jeder Konfigurationsänderung.
8. Chain-ID-Konfiguration je Netz verbindlich verorten.

---

## 12. Befunde aus der Codeprüfung (11.11.2026, main 5a0b147)

1. **Hashfrage geklärt.** `atc_algorithm::hash::atc_hash` ist TownHash-256: eigenständige
   Merkle-Damgård-Konstruktion, 512-Bit-Blöcke, 8×32-Bit-Zustand, 24 Runden, Little-Endian,
   IV und K aus splitmix32 mit Seed `0xA7C0DE01`. **Nicht SHA-256, kein 64-Bit-Traversal.**
   Der Modulkopf in `components/node/src/chain.rs` behauptet „64-Bit-Traversal", der
   Kommentar an `compute_hash` behauptet „SHA-256". Beide Aussagen sind falsch.
2. **Testvektoren in `hash.rs` sind teilweise wirkungslos.** Die Tabelle `faelle` wird
   gebaut und mit `let _ = faelle;` verworfen; geprüft werden drei fest verdrahtete
   Literale, eine Tabellenzeile enthält den Platzhalter „ersetzt-durch-laufzeit".
3. **Spezifikationsverweis tot.** `docs/SPEC-ATC-HASH-001.md` existiert in beiden
   kanonischen Repos nicht (Abschnitt 3c).
4. **Kette im Speicher.** `Chain { blocks: Vec<Block> }`, keine Storage-Abhängigkeit,
   harte Grenze `blocks.len() >= 64`.
5. **Konsens nur im Prozess.** `ConsensusEngine::devnet(3)`; der Vote-Austausch zwischen
   Maschinen ist laut Codekommentar nicht im Gossip-Wire.
6. **Leere Transaktionsliste.** Der Loop ruft `run_round(&mut kette, &[], &mut ledger)`.
7. **Keine Ausführung.** `atc-node` hängt an `atc-algorithm` und `atc_wallet`, nicht an
   `atc-vm`/`atc-system`.
8. **Kopf zu dünn.** Vier Felder, keine Zustandswurzel, keine Proposer-Signatur, kein Zeitstempel.
9. **RPC ohne TLS und ohne Authentisierung** (steht so im Codekommentar von `main.rs`).

Diese Befunde sind **gelesen, nicht ausgeführt**. Die Sandbox-Shell war am 11.11.2026 nicht
verfügbar; es wurde nichts gebaut, gestartet oder gemessen.

---

## 13. Reihenfolge der Umsetzung (Owner-Vorgabe 11.11.2026)

1. Hashentscheidung in diesem Dokument festhalten. **erledigt (3a)**
2. Normativen Hashstandard mit `atc-standards` abgleichen. **teilweise** — der Abgleich hat
   ergeben, dass der referenzierte Standard kein Dokument hat (3c); Dokument selbst fehlt.
3. Testvektortabelle reparieren, unabhängige Referenzvektoren einsetzen. **offen**, braucht
   Build- und Commitzugang.
4. Alle Hashverwendungen im Ökosystem inventarisieren. **offen**, braucht Repository-Zugriff
   (Inventar, nicht Vermutung).
5. TownHash-256- und SHA-256-Verwendungen sauber trennen. **offen**, folgt aus 4.
6. Blockheader und Merkle-/State-Root-Spezifikation festlegen. **teilweise** (Abschnitt 3),
   Details offen.
7. Erst danach die Node-Implementierung beginnen.

Punkt 1 und 2 sind Dokumentationsarbeit und ohne Shell möglich. Punkt 3 bis 5 verlangen
einen Build- und Commitzugang; sie werden nicht vorgetäuscht.

---

## 14. Was dieses Dokument nicht behauptet

Es behauptet nicht, dass der Node betriebsfähig ist. Es behauptet nicht, dass eine der
Phasen begonnen wurde. Es behauptet nicht, dass die Hashentscheidung im Code angekommen
ist — sie ist entschieden und dokumentiert, nicht implementiert.
