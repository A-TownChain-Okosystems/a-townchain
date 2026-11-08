# Häufige Fragen (FAQ)

## Allgemein

**Was ist A-TownChain?**
Ein vollständig proprietäres, dezentrales Blockchain-Ökosystem — entwickelt von ShivaCore.
Kein Fork, kein Klon. Eigene Programmiersprache (ATCLang), eigenes OS (ShivaOS), eigene Standards.

**Was ist ATCLang?**
Eine proprietäre Blockchain-Programmiersprache mit eigenem Compiler, VM und REPL.
Für Smart Contracts, OS-Module und System-Prozesse.

**Wie starte ich einen Node?**
```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os
cd a-townchain-os
pip install -r requirements.txt
python start.py
```

**Wie erzeuge ich eine Wallet?**
```bash
curl -X POST http://localhost:4000/api/wallet/generate
# → {"address":"ATC...","mnemonic":"word1 word2 ... word24"}
```

## ATCLang

**Warum keine Solidity?**
A-TownChain ist kein EVM-Fork. ATCLang ist proprietär und auf die ATC-Standards ausgerichtet.

**Wie prüfe ich meinen Contract auf Sicherheit?**
```bash
python atclang/security/analyzer.py my_contract.atc
```

**Was bedeutet `safe_add(a, b)`?**
Overflow-geschützte Addition. Wirft einen Fehler bei Integer-Overflow statt still zu wrappen.

## Shivamon

**Wie viele Shivamon gibt es maximal?**
Max Supply: 10.000.000 (ATC-9000 Standard).

**Was kostet ein Shivamon zu minten?**
0,1 ATC Gas + Mint-Fee (je nach Rarity).

**Wann kommt Breeding?**
v2.2.0 — Multi-Node Testnet Release.

## Fehlersuche

**Port 4000 nicht erreichbar?**
```bash
python start.py  # Gateway läuft auf Port 4000
# Prüfen: curl http://localhost:4000/health
```

**`ModuleNotFoundError`?**
```bash
pip install -r requirements.txt
```
