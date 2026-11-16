# NON_CANONICAL — Python-Netzwerkcode (ATCNet)

> **Status:** NICHT AUTORITATIV. Historischer Bestand, keine Produktionsautoritaet.
> Gekennzeichnet am 16.11.2026 auf Grundlage von `SCR-0143` und `ATC-STD-100` v2.0.0
> (REQ-STD-106: Python kontrolliert nicht den TCB der Layer L1 bis L3).

## Was hier liegt

`atcnet.py` (18.229 Bytes), `p2p_propagation.py` (12.965 Bytes), `node.py`, `bootstrap_client.py`,
dazu eine `.atc`-Linie derselben Gegenstaende und `setup.py`.

## Warum es nicht autoritativ ist

1. **Kein Aufrufpfad.** Code-Suche am 16.11.2026: keine `import`-Stelle ausserhalb von Doku,
   keine `console_scripts`, kein Python-Aufruf aus Rust, keine Container- oder Dienstbindung
   ausserhalb des Archiv-Snapshots.
2. **Nicht installierbar wie deklariert.** `packages=find_packages(where="src")` bei Code in
   `modules/atcnet/*.py` ohne `src/`-Paket.
3. **CI-Beruehrung generisch.** `pytest` laeuft ueber das Verzeichnis, weil `requirements.txt`
   existiert; das ist Testausfuehrung, kein Betriebsnachweis.

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
| ATCNet-Protokoll | `globus-os/modules/atc-shivacore/service_space/src/atcnet.rs` | Kernel-Service-Space |
| DID im Service-Space | `globus-os/modules/atc-shivacore/service_space/src/did.rs` | Kernel-Service-Space |

Ein `net.rs` gibt es im kanonischen Service-Space **nicht**; es existiert ausschliesslich im Archivsnapshot
`a-townchain-os-docs/docs/archive/monorepo-full/.../kernel/src/net.rs` und ist damit keine kanonische Fundstelle.

## Konsequenz

- Netzverhalten wird ausschliesslich im kanonischen Service-Space gepflegt (`atcnet.rs`, `p2p.rs`, `did.rs`).
- Der Bestand bleibt als Referenz erhalten, keine Loeschung ohne Owner-Entscheidung.
- Abweichungen im Wire-Verhalten sind Findings gegen diesen Bestand.
