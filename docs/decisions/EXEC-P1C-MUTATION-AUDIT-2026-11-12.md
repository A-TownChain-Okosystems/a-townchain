# P1c — Mutationsgrenzen, Adoption und Devnet-Verdrahtung

> **Status:** `IMPLEMENTED` / `TESTED` (lokal). **CI_VERIFIED:** nein.
> **Datum:** 2026-11-12. **Grundlage:** P1b-Nachweis und die drei Prüfpunkte des
> Owners (Sichtbarkeit, Quelltext-Guard, Devnet-Integration).

## 1 Inventur: jeder Weg in den kanonischen Blockbestand

`Chain` hält `blocks: Vec<Block>` als privates Feld. Vor P1c gab es sechs
Methoden mit Schreibzugriff und einen Weg daran vorbei:

| Weg | Art | Prüfung vorher | Stand nach P1c |
|---|---|---|---|
| `Chain::produce` | lokal erzeugt | Payload-Verbot (`\|`, `;`), Kapazität | unverändert (lokale Produktion) |
| `Chain::commit_payload` | lokal, ruft `produce` | wie `produce` + Ledger | unverändert |
| `Chain::produce_tx_block` | lokal, ruft `produce` | Tx-Prüfung + `produce` | unverändert |
| `Chain::import_tx_block` | fremd, ruft `produce` | Höhe + Tx-Prüfung, **keine Payload-Prüfung** | offen (Befund F3) |
| `Chain::import_block` | fremd | `I-001`…`I-005` | **neu in P1b/P1c** |
| `Chain::adopt_verified` | ganze Peer-Kette | Genesis-Bindung + Payload je Block | **neu in P1c** |
| `Chain::from_blocks` | baut Kandidaten | nur `verify()` (Hashes, Verkettung) | bleibt Baustein von `adopt_verified` |
| `gossip::sync_pull` | **`*kette = kandidat`** | Hashes + Genesis — **keine Payload-Prüfung** | **Befund F1, behoben**: nutzt jetzt `adopt_verified` |

## 2 Befund F1 (behoben): Adoption war der offene Weg

`sync_pull` ersetzte die Kette vollständig (`*kette = kandidat`), nachdem
`Chain::from_blocks` nur Hashes, Höhen und Verkettung geprüft hatte. Damit konnte
ein Peer beliebige Payloads liefern — auch `ATCB`-Ströme oder Müll — und der
Knoten hätte sie als kanonischen Bestand übernommen. Das ist genau der
„alternative Weg", nach dem die Prüfung fragt: nicht ein zweiter `import_block`,
sondern eine **Ersetzung**, die an der Payload-Prüfung vorbeiführt.

Jetzt gilt: `sync_pull(kette, peer, validator)` → `Chain::adopt_verified`. Ohne
Validator `I-001`, bei abgelehnter Payload `I-005`, und **keine Teilübernahme**:
erst nach Prüfung aller neuen Blöcke wird der Bestand ersetzt.

## 3 Befund F2 (behoben): kein crate-weiter Schreiber

In P1b lag `import_block` im Modul `import` und brauchte dort
`pub(crate) fn append_validated`. `pub(crate)` heißt: jedes Modul des Crates darf
anhängen. Aufgelöst, indem der Schreibvorgang dorthin wandert, wo das Feld liegt:
`import_block` und `adopt_verified` sind jetzt Methoden in `chain.rs`,
`append_validated` ist entfallen, `has_capacity` ist privat. `self.blocks.push`
existiert an genau zwei Stellen, `self.blocks = ` an genau einer — alle drei in
`chain.rs`.

Nachweis (strukturell, nicht per Textsuche allein):

* `kein_zweiter_importpfad_im_quelltext` prüft **jede** Schreibform
  (`blocks.push`, `blocks =`, `clear`, `retain`, `truncate`, `insert`, `remove`,
  `extend`) in allen Modulen des Crates und verlangt genau drei Treffer, alle in
  `chain.rs`.
* `blockbestand_ist_ausserhalb_nicht_zugaenglich` prüft, dass außerhalb von
  `chain.rs` nur die Lesezugriffe `.blocks()` und `.blocks_from(…)` vorkommen —
  jeder direkte Feldzugriff ist ein Testfehler.
* Die eigentliche Garantie ist die Feldprivatheit: das Feld ist nicht `pub`,
  damit ist ein Zugriff aus einem anderen Modul ein **Compile-Fehler**, nicht nur
  ein Testfehler. Beide Tests sichern die Randbedingungen drumherum.

## 4 Befund F3 (offen): zweiter Blockbestand in `System`

`System` hält eine eigene `chain` und ruft in `deploy` `self.chain.produce(&payload)`.
Damit existiert ein zweiter Ort, an dem Blöcke mit Payload entstehen, ohne den
Port zu fragen. Das ist keine Umgehung des Imports in `atc-node`, aber eine
zweite Wahrheit über den Blockbestand, solange nicht entschieden ist, welcher
Bestand kanonisch ist. **Nicht** in P1c geändert — das ist eine
Architekturentscheidung (eine Kette je Knoten, `System` erzeugt Payloads, nicht
Blöcke).

## 5 Befund F4 (offen): zweite Knotenimplementierung in `apps/atc-node`

`apps/atc-node/src/gossip.rs` enthält eine Parallelkopie von `gossip.rs`
(eigener `sync_pull`, eigene Tests). Sie ist nicht angepasst und adoptiert
weiterhin ungeprüft. Zwei Implementierungen derselben Funktion verstoßen gegen
die Grundregel „eine Wahrheit je Funktionalität"; die Kopie muss aufgelöst oder
stillgelegt werden.

## 6 Devnet-Verdrahtung

Kompositionswurzel, die heute beide Seiten sieht, ist `atc-system` (es hängt von
`atc-node` ab; umgekehrt wäre es ein Zyklus):

```rust
// components/system/src/node_import.rs
pub struct DevnetNode { system: System, chain: atc_node::chain::Chain }

impl DevnetNode {
    pub fn start() -> Result<Self, SystemError>;          // Validator ist Teil des Knotens
    pub fn import_block(&mut self, Block) -> Result<(), ImportError>;
    pub fn adopt_chain(&mut self, Vec<Block>) -> Result<(), ImportError>;
}
```

`import_block` nimmt **keinen** Validator-Parameter: der Aufrufer kann die
Prüfung weder abwählen noch durch `None` ersetzen. Es gibt keinen permissiven
Entwicklungsmodus. Nachgewiesen in `devnet_knoten_nimmt_nur_geprueft_an` (gültige
Hülle angenommen, `ATCB` `I-005`, falscher Hash `I-002`, Höhe bleibt) und
`devnet_knoten_adoption_prueft_payloads` (keine Teilübernahme).

**Verbleibende Lücke, klar benannt:** der eigenständige Prozess
`components/node/src/main.rs` kann den Validator nicht selbst bauen (er darf
`atc-system` nicht importieren). Wer ihn betreibt, muss ihn injizieren — bis
dahin lehnt dieser Prozess Fremdimporte über `import_block` mit `I-001` ab, weil
er keinen Validator setzt. Der nächste Schritt ist entweder, den Knotenprozess in
die Kompositionswurzel zu ziehen (Binary bei `atc-system`, das `atc-node`
einbindet), oder den Prozess mit einer expliziten Injektion zu starten. Beides ist
eine eigene Entscheidung, keine stille Änderung.

## 7 Nachweis (lokal)

```
cargo test --workspace        746 grün (P1b: 740) — 0 rot
node:  block_import 9 Tests (6 P1b + 3 Adoption), two_node_devnet angepasst
system: node_import 9 Tests (7 P1b + 2 Devnet-Knoten)
```

`CI_VERIFIED`, `E2E_VERIFIED` und `VERIFIED` bleiben offen: kein Actions-Lauf auf
dem Default-Branch-HEAD, kein Devnet-E2E mit laufendem Prozess.
