---
title: Quickstart
summary: In zehn Minuten vom Klon zum laufenden ATC-Devnet mit beantwortetem RPC.
---

# Quickstart

Ziel: ein laufender Devnet-Node, der auf RPC antwortet. Kein Mainnet, keine Validator-Keys, kein Docker-Zwang.

## Voraussetzungen

- Rust (stable) über [rustup](https://rustup.rs)
- Python 3.11 oder neuer
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
  ✓ ping        {"jsonrpc":"2.0","id":1,"result":"pong"}
  ✓ chain_id    {"jsonrpc":"2.0","id":1,"result":"atc"}
  ✓ boot_hash   {"jsonrpc":"2.0","id":1,"result":"..."}
  ✓ peers       {"jsonrpc":"2.0","id":1,"result":"2"}
PASS — Node antwortet korrekt.
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
printf '{"jsonrpc":"2.0","method":"chain_id","id":1}\n' | nc 127.0.0.1 39471
```

Methoden: `ping`, `chain_id`, `boot_hash`, `peers`.

## Was das Devnet heute kann

- Genesis-Boot mit Peer-Tabelle und deterministischem Boot-Hash
- Chain-State mit Höhe und Best-Hash
- RPC-Abfragen zu Chain-ID, Boot-Hash und Peer-Anzahl
- Pull-Sync über den Gossip-Dienst

## Was es noch nicht kann

- Blockproduktion und Konsens
- Transaktionen, Konten, Balances
- TLS, Authentisierung, Rate-Limits
- Persistenz über Neustart

Diese Grenzen sind bewusst dokumentiert. Wer mehr erwartet, wird enttäuscht, und das wäre nicht ehrlich.
