# NON_CANONICAL — Python-Konsens- und Chain-Code

> **Status:** NICHT AUTORITATIV. Historischer Bestand, keine Produktionsautoritaet.
> Gekennzeichnet am 16.11.2026 auf Grundlage von `SCR-0143` und `ATC-STD-100` v2.0.0
> (REQ-STD-106: Python kontrolliert nicht den TCB der Layer L1 bis L3).

## Was hier liegt

`consensus/` (`shiva_consensus.py` 641 Zeilen, `poh.py`, `poh_integration.py`, `pos.py`, `pow.py`,
`hybrid_consensus.py`), `nodes/` (`bootstrap.py`, `discovery.py`), `wallet/` (`did.py`, `multisig.py`),
`zkp/` (`groth16.py`), daneben eine `.atc`-Linie derselben Gegenstaende und `setup.py`.

## Warum es nicht autoritativ ist

1. **Kein Aufrufpfad.** Org-weite Code-Suche am 16.11.2026: `import atc_blockchain` 0 Treffer,
   `from modules.atc-blockchain` 0 Treffer, `console_scripts` in `a-townchain` 0 Treffer,
   Rust ruft kein Python per `subprocess` auf (0 Treffer). Kein Dockerfile, keine Compose-Datei,
   keine systemd-Unit ausserhalb des Archiv-Snapshots bindet dieses Paket ein.
2. **Nicht installierbar wie deklariert.** `setup.py` erklaert `packages=find_packages(where="src")`,
   aber der Code liegt in `consensus/`, `nodes/`, `wallet/`, `zkp/` und nicht unter `src/`.
   Es gibt kein `src/`-Paket, das die deklarierte Distribution fuellt.
3. **CI-Beruehrung ist generisch, kein Funktionsnachweis.** `a-townchain/.github/workflows/test-suite.yml`
   faehrt `pytest` ueber jedes Verzeichnis mit `requirements.txt`. Ein gruener Lauf dort zeigt, dass die
   Tests des Bestands laufen, nicht dass dieser Code die Chain betreibt.

## Kanonische Implementierung (Rust)

| Zweck | Kanonische Fundstelle | Beleg |
| --- | --- | --- |
| Blockstruktur, Blockhash | `a-townchain/components/node/src/chain.rs` | 32-Byte-`prev_hash`/`hash`, Domaenentag `ATC-BLOCK-V1` |
| Vote- und Proposal-Digest | `a-townchain/components/node/src/consensus.rs` | `vote_digest` ueber `sha2` |
| Proposer-Auswahl, Stake-Registry | `a-townchain/components/node/src/validator.rs` | `proposal_digest`, Registry |
| Node-Identitaet, DID | `a-townchain/components/node/src/did.rs` | DID aus `sha256` des Pubkeys, erste 20 Byte |
| Genesis/Boot | `a-townchain/components/node/src/bootstrap.rs` | `boot_hash256` |
| PoH und Hash-Primitive | `a-townchain/components/algorithm/src/poh.rs`, `.../hash.rs` | `atc_hash` (TownHash-256) |
| P2P-Transport, Nachrichtenausbreitung | `globus-os/modules/atc-shivacore/service_space/src/p2p.rs` | Kernel-Service-Space |
| ATCNet-Protokoll im Service-Space | `globus-os/modules/atc-shivacore/service_space/src/atcnet.rs` | Kernel-Service-Space |
| Chain-Zustand im Service-Space | `globus-os/modules/atc-shivacore/service_space/src/blockchain.rs` | 505 Zeilen, dokumentiert |
| Konsens im Service-Space | `globus-os/modules/atc-shivacore/service_space/src/consensus.rs` | ValidatorRegistry/ConsensusEngine |
| DID im Service-Space | `globus-os/modules/atc-shivacore/service_space/src/did.rs` | Kernel-Service-Space |

Ein `net.rs` gibt es im kanonischen Service-Space **nicht**; die Netzautoritaet liegt in `atcnet.rs` und `p2p.rs`.
`net.rs` existiert ausschliesslich im Archivsnapshot `a-townchain-os-docs/docs/archive/monorepo-full/.../kernel/src/net.rs`
und ist damit keine kanonische Fundstelle.

## Konsequenz

- Kein Sicherheitsfix, keine Protokollaenderung, kein Verhalten wird hier gepflegt.
- Es wird nichts geloescht: der Bestand bleibt als historische Referenz erhalten, bis der Owner
  ueber Loeschen oder Rekonstruktion entscheidet (Owner-Auflage 16.11.2026: keine Loeschung von
  Referenzcode ohne Pruefung).
- Widersprueche zwischen diesem Code und der kanonischen Rust-Implementierung sind **Findings gegen
  diesen Bestand**, nicht gegen die Norm.
- Neue Entwicklung zu Konsens, Chain, Netz oder DID erfolgt ausschliesslich in den oben genannten
  Rust-Fundstellen.
