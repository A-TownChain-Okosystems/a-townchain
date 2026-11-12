# P1d-A — Parallel-Gossip (F4) geschlossen, Abweichungsprüfung

> **Status:** `IMPLEMENTED` / `TESTED` (lokal). **CI_VERIFIED:** nein.
> **Datum:** 2026-11-12. **Auslöser:** Befund F4 (P0) und die gemeldete
> Abweichung zwischen Dokument und abrufbarem Quellstand.

## 1 Abweichungsprüfung: was wirklich in `main` steht

Geprüft über die Contents-API auf `main`, nicht aus dem Gedächtnis:

| Datei | Befund am abrufbaren Stand | Bewertung |
|---|---|---|
| `components/node/src/chain.rs` (sha `e5bc956a…`) | `append_validated` **nicht vorhanden**, `adopt_verified` vorhanden | Die Meldung „zeigt dort noch `append_validated` mit `pub(crate)`" trifft auf den aktuellen Stand **nicht** zu. `append_validated` wurde in P1c entfernt; `has_capacity` ist privat. |
| `chain.rs`, Zeichenfolge `blocks = kandidat` | vorhanden, aber als `self.blocks = kandidat.blocks;` **innerhalb** von `adopt_verified` | Kein Rest des alten Pfades: die Zuweisung ist die Adoption selbst und steht **hinter** der Prüfung aller neuen Blöcke. |
| `components/node/src/gossip.rs` | `*kette = kandidat` **nur im Kommentar**, der die Entfernung erklärt | Keine Code-Stelle mehr; die Suche nach der Zeichenfolge findet den Kommentar. |
| `apps/atc-node/src/gossip.rs` | `sync_pull` mit `*kette = kandidat` **im Code**, ungeprüfte Adoption | **Der Befund war real** — und zwar in der Parallelkopie. |

Damit ist der gemeldete Verdacht an der entscheidenden Stelle bestätigt und an
der anderen widerlegt: Der Bypass lebte in `apps/atc-node`, nicht in
`components/node`. Beides ist jetzt nachprüfbar, weil der Test die Zeichenfolgen
**ohne** Kommentarzeilen über das ganze Repo prüft.

**Wichtige Teilursache:** `apps/atc-node` ist **kein Mitglied** der
Workspace-Liste in `Cargo.toml`. Eine Kopie, die nicht gebaut wird, wird von
keiner CI und keinem Test erreicht — deshalb überlebte sie die P1b-/P1c-Runden.
Das ist selbst ein Prozessbefund: Parallelkopien müssen entweder gebaut oder
entfernt werden, sonst ist „getestet" nur für den gebauten Teil wahr.

## 2 Was geändert wurde (P1d-A)

| Datei | Änderung |
|---|---|
| `apps/atc-node/src/gossip.rs` | `sync_pull`, `peer_antwort`, `parse_bloecke` und `SyncReport` **entfernt**. Es bleibt der Dienst `serve_gossip` (STATUS/BLOCKS-Antworten). Der Kopf nennt die kanonische Implementierung und den Grund. |
| `apps/atc-node/tests/two_node_devnet.rs` | Der Test, der eine Adoption nachwies, ist ersetzt: `keine_adoption_im_modul` prüft, dass die Kopie weder `pub fn sync_pull` noch `= kandidat` noch `Chain::from_blocks` enthält. |
| `components/node/src/gossip.rs` | Neuer Regressionstest `manipulierter_peer_mit_gueltigen_hashes_wird_abgewiesen`: ein Peer liefert eine strukturell einwandfreie Kette (Hashes, Höhen, Verkettung, Genesis) mit der Payload `atcb:…`. Adoption wird verweigert, Höhe und Best-Hash bleiben, `verify()` bleibt grün; ohne Validator ebenso. |
| `components/node/tests/block_import.rs` | Neuer repo-weiter Wächter `keine_zweite_adoptionsstelle_im_repo`: durchsucht alle `.rs`-Dateien (Kommentare ausgenommen) nach `Chain::from_blocks(`, `kette = kandidat` und einem zweiten `pub fn sync_pull`. Ausgenommen sind nur die beiden Wächter selbst. |

Der Wächter ist bewusst **repo-weit** und nicht crate-weit: F4 wäre von einem
crate-lokalen Test nie gefunden worden.

## 3 Was ausdrücklich offen bleibt

| Befund | Stand | Warum nicht in P1d-A |
|---|---|---|
| F3 — `System` hält eine eigene `chain` und ruft `produce` in `deploy` | offen | Architekturentscheidung: wer führt den kanonischen Bestand. Deine Zielarchitektur (System erzeugt Payloads und Ausführungsergebnisse, die Kette führt Blöcke) bedeutet einen Schnitt durch `System`, `state` und die Deploy-Pfade. Das ist ein eigener Schritt mit Migrationsfolgen, keine Nebenänderung. |
| Standalone `components/node/src/main.rs` | offen | Der Prozess kann `atc-system` nicht importieren (Zyklus) und lehnt Fremdimporte deshalb mit `I-001` ab. Deine Präferenz (Binary bei `atc-system`, das `atc-node` einbindet) ist umsetzbar; sie verschiebt aber RPC-, Konsens- und Gossip-Loop in die Kompositionswurzel und braucht einen eigenen Nachweis. |
| `import_tx_block` | offen, eingegrenzt | Der Weg prüft Höhe und **alle** Transaktionen und bildet die Payload **lokal** aus den geprüften Tx-IDs (`encode_tx_ids`). Eine beliebige fremde Payload ist damit nicht einschleusbar — anders als bei F1. Er umgeht aber die Payload-Prüfung des Ports und ist ein zweiter Annahmepfad; er muss auf denselben Port geführt oder als reine Devnet-Funktion gekennzeichnet werden (P1d-D). |
| Devnet-E2E mit manipulierten Peers | offen | Der neue Test benutzt einen simulierten Peer im Prozess, keinen laufenden Devnet-Verbund mit echten Prozessen. E2E bleibt unvergeben. |

## 4 Nachweis (lokal)

```
cargo test --workspace        748 grün, 0 rot (P1c: 746)
node: block_import 11 Tests (inkl. repo-weiter Wächter, Adoption, Quelltextform)
node: gossip        +1 Regressionstest (manipulierter Peer, gültige Hashes)
```

`CI_VERIFIED`, `E2E_VERIFIED`, `VERIFIED` bleiben offen. Die 748 Tests sind ein
Zwischenstand, kein Sicherheitsmaßstab — der Maßstab ist die Vollständigkeit der
Mutationsgrenzen, und drei davon sind noch nicht geschlossen.
