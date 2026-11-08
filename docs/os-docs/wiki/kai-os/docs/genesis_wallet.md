# Genesis-Wallet & Wallet-Logik — A-TownChain OS

> Stand: 2026-06-14 | Version: v3.2.1 | Status: **GENESIS_READY** ✅
> Generiert von Aurora Superagent v3.2

---

## 🏦 Genesis-Wallet

Die Genesis-Wallet ist der Ursprung aller 21.000.000 ATC im Mainnet.
Im Genesis-Block (Block #0) werden alle Token dieser Adresse zugewiesen.

```
Adresse:    ATCf9327118a7dfb30f72ba6aa82e1186078c42232884
Public Key: 44c87e482de99b8b622afea4e8469a6eeba0407dc72cc97d156ac92109606a26
Chain-ID:   9001
Generiert:  2026-06-14T12:12:41Z
```

> ⛔ **Der Private Key ist AUSSCHLIESSLICH lokal in `PRIVATE_KEYS.txt`.**
> Niemals in Git, E-Mail oder Cloud speichern.

---

## 💰 Token-Verteilung (21.000.000 ATC)

| Empfänger            | Betrag        | Anteil | Vesting       |
|----------------------|---------------|--------|---------------|
| Founder Wallet       | 3.150.000 ATC | 15%    | 24 Monate     |
| Team Multisig        | 2.100.000 ATC | 10%    | 18 Monate     |
| Community Reserve    | 10.500.000 ATC| 50%    | DAO-Governance|
| Validator Rewards    | 2.100.000 ATC | 10%    | Staking-Pool  |
| Reserve Pool         | 3.150.000 ATC | 15%    | Notfall-Reserve|

---

## 🔑 Validator Keys (5 Nodes)

Jeder Validator braucht **10.000 ATC Bond** aus der Genesis-Wallet.

| # | Node-ID | Adresse |
|---|---------|---------|
| 0 | `334753d0b0a0ddacef3f03018e6d21c2` | `ATC6831e22d0ea67250bca887fbad133bb1fe254a582f` |
| 1 | `b162ee498fff23e38bb7f58dac029ca8` | `ATCc53eb191b010ee991ffe363a15676a4c80dd200371` |
| 2 | `2078096d4169c94eb45e813a312e1ba2` | `ATC8b432a4514ad7f9f196084b776cd67d1d258bf5d1b` |
| 3 | `8fee65a71c98acc23747093b219d2083` | `ATCcda0d85e8dc023312db92cf2a60f003fb20f9c4b07` |
| 4 | `b1196e53fba080f687b44d1c368ae7f4` | `ATCc613387a963a90d78369b296116e9f310b4ef51687` |

---

## ⚙️ Genesis-Block Struktur

```json
{
  "block_number": 0,
  "chain_id": 9001,
  "genesis_wallet": "ATCf9327118a7dfb30f72ba6aa82e1186078c42232884",
  "coinbase_amount": 2100000000000000,
  "validator_count": 5,
  "required_signatures": 3
}
```

---

## 🛠️ Setup ausführen

```bash
# Genesis-Setup (lokal, offline ausführen!)
python3 scripts/generate_validators.py --count 5 --chain-id 9001

# Outputs:
#   config/mainnet_genesis.json  ← befüllt ✅
#   config/PRIVATE_KEYS.txt      ← nur lokal ⛔
```

---

## 📋 Mainnet Checkliste

- [x] Genesis-Wallet generiert
- [x] 5 Validator Keys generiert
- [x] `config/mainnet_genesis.json` befüllt
- [x] Chain-ID 9001 festgelegt
- [x] Tokenomics definiert
- [ ] Bootstrap-Node IP eintragen
- [ ] Validator-Bonds funded (5 × 10.000 ATC)
- [ ] SSL-Zertifikate installiert
- [ ] Security-Audit abgeschlossen
- [ ] Mainnet Launch 🚀

---

## 🔗 Referenzen

- [Issue #52](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/52)
- [generate_validators.py](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/scripts/generate_validators.py)
- [mainnet_genesis.json](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/config/mainnet_genesis.json)
- [Wallet Module](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/blockchain/wallet)

---

*Aurora v3.2 · 2026-06-14*
