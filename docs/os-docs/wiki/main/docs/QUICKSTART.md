# Quickstart Guide

## Installation
```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os
cd a-townchain-os
pip install -r requirements.txt
python start.py
```

## System läuft auf:
- API Gateway: http://localhost:4000
- P2P Network: Port 4001
- Bootstrap: Port 5005
- WebSocket: Port 9944

## Erste Wallet
```bash
curl -X POST http://localhost:4000/api/wallet/generate
# → {"address": "ATC...", "public_key": "...", ...}
```

## Ersten Smart Contract deployen
```atclang
contract Hello {
    fn greet(name: string) -> string {
        return "Hello, " + name + " from A-TownChain!"
    }
}
```
