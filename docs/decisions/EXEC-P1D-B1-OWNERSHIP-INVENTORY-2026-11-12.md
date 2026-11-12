# P1d-B1 — Eigentum und Commit-Verhältnisse von Chain, Zustand und Freigaberegister

> **Status:** `ANALYSIERT` (Inventur abgeschlossen, **kein Umbau**).
> **Datum:** 2026-11-12. **Freigabe für den Umbau:** offen — sie folgt erst nach
> dieser Inventur, wie vom Owner verlangt.

## 0 Phasenordnung (verbindlich, mit alter Zuordnung)

Damit die Phasen nicht nachträglich ihre Bedeutung wechseln:

| Neu | Inhalt | Abnahme | vorher geführt als |
|---|---|---|---|
| **P1d-B1** | Chain aus `System` herauslösen | eine autoritative Chain | P1d-B (Teil 1) |
| **P1d-B2** | State-Root-Vorbedingung | veralteter Ausgangszustand wird abgewiesen | P1d-B (Teil 2) |
| **P1d-C** | Atomarer State-/Chain-Commit (optimistische Konkurrenzkontrolle) | kein partieller Commit | P1d-D / „atomare Zustandsänderungen" |
| **P1d-D** | Standalone Node in der Kompositionswurzel | eine Runtime-Wurzel | P1d-C (alt) |
| **P1d-E** | `import_tx_block` vereinheitlichen | kein Port-Bypass | P1d-D (alt) |
| **P1d-F** | E2E-Sicherheitsaudit (manipuliert **und** konkurrierend) | Angriffe abgewiesen | P1d-F (alt) |

Erledigt bleiben: **P1d-A** (Parallel-Gossip/F4) und **P1d-B, Governance-Teil**
(Build-Abdeckung G-01).

## 1 Inventur: wer schreibt was, wann, mit welchem Commit-Punkt

| # | Pfad | Eigentümer | Art | Commit-Punkt | Folge bei Abbruch danach |
|---|---|---|---|---|---|
| 1 | `System.chain` (`system.rs:184`) | `System` (privat, lesend über `chain()`) | Blockbestand | `self.chain.produce` in `execute` (`:478`) | Block existiert, Zustand des Aufrufers nicht fortgeschrieben |
| 2 | `System.registry` (`:185`) | `System` (lesend über `registry()`) | Freigaberegister `code_hash → Policy` | `self.registry.register(...)` in `deploy` (`:295`) und im Container-Pfad (`:357`) | Policy registriert, **kein Block** — eigener Drift-Fall |
| 3 | `System.authority` (`:186`) | `System` (lesend über `audit_count()`) | Audit-Log, Obergrenzen, Widerruf | `authorize_signer`/`set_ceiling`/Audit-Eintrag | Audit-Eintrag ohne Block |
| 4 | `ContractState` | **Aufrufer**, nicht `System` | Wörter + `state_root(code_hash)` (`state.rs:16-43`) | `*state = after` in `execute` (`:479`) | Aufrufer hält alten Zustand, Block behauptet neuen |
| 5 | `DevnetNode.chain` (`node_import.rs`) | `DevnetNode` | **zweiter** Blockbestand im selben Prozess | `import_block`/`adopt_chain` | eigener Bestand neben `System.chain` |
| 6 | State-Persistenz | **existiert nicht** | — | — | nach Neustart ist jeder Zustand neu zu setzen |

Beobachtungen, die aus der Inventur folgen:

* **Es gibt keinen Zustandsspeicher.** `ContractState` lebt im Aufrufer
  (`main.rs` hält `state` im Speicher). „Neustart-Konsistenz" ist heute nicht
  definiert, weil es nichts Dauerhaftes gibt, gegen das man sie prüfen könnte.
  Das ist keine Lücke des Umbaus, sondern eine **fehlende Voraussetzung** für
  P1d-C: eine Transaktion braucht ein Ziel, das sie committen kann.
* **`deploy` schreibt zweimal, ohne Block:** das Freigaberegister und ggf. das
  Audit-Log. Ein Deployment, das nie ausgeführt wird, hinterlässt eine Policy.
  Dieselbe Klasse von Inkonsistenz wie Problem A, nur ohne Chain-Bezug.
* **`DevnetNode` hat eine eigene Kette** (`node_import.rs`, in P1d-A eingeführt).
  Damit existieren im selben Prozess derzeit **zwei** Blockbestände: der von
  `System` (Produktion in `execute`) und der von `DevnetNode` (Annahme). Das ist
  genau die zweite Wahrheit, die P1d-B1 auflösen soll — und sie ist durch meine
  eigene P1d-A-Änderung entstanden, nicht durch Altcode. Sie ist bis zur
  Zusammenführung **kein** Sicherheitsleck (beide führen geprüfte bzw. produzierte
  Blöcke), aber sie ist der Grund, warum B1 vor B2 kommen muss.
* **Konkurrenz ist heute unkontrolliert:** weder `deploy` noch `execute` prüfen
  Höhe, Best-Hash oder `state_root` des übergebenen Zustands. Zwei Aufrufer mit
  verschiedenen Zuständen erzeugen zwei Blöcke in derselben Kette (Problem B).

## 2 Zielinvariante (Owner-Regel, hier als Vertrag festgehalten)

> Ein Block darf nur auf dem zuletzt bestätigten Chain- und State-Zustand
> aufbauen. Jeder Zustandsübergang ist an eine eindeutige Ausgangsversion
> gebunden. Block und zugehöriger Zustand werden als **eine** logische
> Transaktion übernommen. Fehler, Versionskonflikt oder Prozessabbruch hinterlassen
> keinen partiellen Commit.

Konkret als Commit-Bedingung (optimistische Konkurrenzkontrolle):

```text
Leseversion festhalten:  (hoehe, best_hash, state_root_before)
Ausführen auf isoliertem Zustand (Kopie, kein Schreibzugriff auf das Original)
Vor dem Commit erneut prüfen: ist die Leseversion noch aktuell?
  - nein  -> Abbruch mit Versionskonflikt, kein Schreibvorgang, Aufrufer darf neu ausführen
  - ja    -> Block und Zustand gemeinsam übernehmen (ein Commit-Punkt)
```

Zusatzbedingung des Owners, die ich ausdrücklich übernehme: der `state_root` muss
den **vollständigen** für die Ausführung maßgeblichen Zustand abdecken. Ein
teilweise erfasster `ContractState` (heute: nur Wörter, ohne Slot-/Layout-Bezug
und ohne Kontobestand) wäre keine tragfähige Grundlage für eine
Konkurrenzkontrolle.

## 3 Abnahmetests (verbindlich, je Phase zugeordnet)

| Test | Phase |
|---|---|
| veralteter `state_root` wird abgewiesen | B2 |
| falscher Ausgangs-Blockhash wird abgewiesen | B2 |
| zwei parallele Aufrufer mit unterschiedlichem Zustand: höchstens einer committet | C |
| Fehler **nach** VM-Ausführung verändert keinen dauerhaften Zustand | C |
| Fehler **während** des Block-Commits verändert keinen dauerhaften Zustand | C |
| Neustart nach Commit stellt konsistenten Chain-/State-Zustand her | C (setzt Zustandsspeicher voraus) |
| wiederholte Ausführung erzeugt keinen doppelten Zustandsübergang | C |

## 4 Was als Nächstes entschieden werden muss (Freigabepunkte)

1. **Wo liegt der autoritative Bestand?** Vorschlag nach deiner Zielarchitektur:
   die Kompositionswurzel (`DevnetNode`/Runtime), `System` liefert
   `(payload, after)` und schreibt nichts. Dann entfällt `DevnetNode.chain` als
   Dublette, weil es die einzige Kette ist.
2. **Was ist der Zustandsspeicher?** Ohne Entscheidung (Datei, Log, KV, Ledger)
   ist „atomarer Commit" nicht prüfbar. Mein Vorschlag: kleinster tragfähiger
   Schritt ist ein append-only Commit-Log `(hoehe, block_hash, state_root)` plus
   Zustands-Snapshot je Höhe; das macht Neustart-Test und Doppelübergangs-Test
   überhaupt erst formulierbar.
3. **Was gehört in den `state_root`?** Heute `words` + `code_hash`. Für die
   Konkurrenzkontrolle muss der Wurzelwert den gesamten maßgeblichen Zustand
   abdecken (inkl. Freigaberegister? Konten?). Das ist eine Entscheidung über den
   Zustandsbegriff, nicht über Code.
4. **`deploy`-Schreibvorgänge:** soll die Policy-Registrierung Teil des
   Zustandsübergangs werden (also mitcommit­tet) oder ein vorgelagerter,
   idempotenter Vorgang bleiben?

Ohne diese vier Antworten wäre jeder Umbau ein Ratespiel — deshalb hier Ende der
Analyse und **keine** Codeänderung an `System`.

## 5 Nachweis

```
cargo test --workspace   748 grün, 0 rot   (unverändert: dieser Schritt ändert keinen Code)
pytest tools/tests       40 passed
check_build_coverage     GATE PASS, 77 Manifeste, 7 offen
```
