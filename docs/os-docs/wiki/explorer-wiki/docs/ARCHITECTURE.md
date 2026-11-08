# 🏗️ Architektur — atc-explorer

Der Block Explorer liest Blockchain-Zustaende via RPC ab und speichert indizierte Daten in einer Lese-optimierten Datenbank.

## Datenfluss

```
[ A-TownChain Node ] ---> (RPC Indexer) ---> [ PostgreSQL / Cache ] ---> (FastAPI) ---> [ React Frontend ]
```
