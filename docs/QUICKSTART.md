---
title: Quickstart
summary: In zehn Minuten vom Klon zum laufenden ATC-Devnet mit finalisierenden Bloecken.
---

# Quickstart

Ziel: ein laufender Devnet-Node, der Bloecke finalisiert und auf RPC antwortet. Kein Mainnet, keine Validator-Keys, kein Docker-Zwang.

## Voraussetzungen

- Rust (stable) über [rustup](https://rustup.rs)
- Python 3.11 oder neuer (für die Prüfskripte)
- Optional: Docker für den Compose-Weg

## Weg 1: lokal

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain.git
cd a-townchain
./build.sh          # prueft Voraussetzungen, baut target/release/atc-node
./start.sh          # startet den Node, wartet auf RPC-Bereitschaft
./tools/devnet/smoke.sh
```

Erwartete Ausgabe des Smoke-Tests:

```text
  ✓ ping       pong
  ✓ chain_id   atc
  ✓ boot_hash  ...
  ✓ peers      2
  ✓ did_count  0
  ✓ height     2
  ✓ finalisiert Hoehe 2
  ✓ consensus  height=2 rounds=2 validators=3 quorum=2
PASS — Node antwortet, Konsens finalisiert.
```

Drei Nodes statt einem: `./start.sh --nodes 3`. Beenden mit `./start.sh stop`, Zustand mit `./start.sh status`.

## Weg 2: Docker

```bash
docker compose -f docker-compose.devnet.yml up --build
```

Drei Nodes, RPC auf 39471, 39481 und 39491.

## RPC von Hand

Der Node spricht zeilenweise JSON-RPC auf der RPC-Adresse:

```bash
printf '{"jsonrpc":"2.0","method":"consensus","id":1}\n' | nc 127.0.0.1 39471
```

Methoden: `ping`, `chain_id`, `boot_hash`, `peers`, `height`, `consensus`, `did_count`, `did_epoch`.

## Was das Devnet heute kann

- Genesis-Boot mit Peer-Tabelle und deterministischem Boot-Hash
- Konsens-Loop mit drei Validatoren, 2/3-Quorum und Finalisierung im Zwei-Sekunden-Takt
- Chain-State mit Höhe, Best-Hash und vollständiger Kettenprüfung
- DID-Registry mit Epochenzähler
- Pull-Sync über den Gossip-Dienst

## Was es noch nicht kann

- Persistenz: nach einem Neustart beginnt die Kette wieder bei Höhe 0
- Transaktions-Eingang über RPC: der Mempool existiert, die Ingress-Schnittstelle fehlt
- Vote-Austausch zwischen Maschinen (der Konsens läuft derzeit in einem Prozess)
- TLS, Authentisierung, Rate-Limits

Diese Grenzen sind bewusst dokumentiert. Wer mehr erwartet, wird enttäuscht, und das wäre nicht ehrlich.
