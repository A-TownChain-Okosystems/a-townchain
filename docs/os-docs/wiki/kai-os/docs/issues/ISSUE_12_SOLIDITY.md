# 📄 Issue #12 — Solidity Smart Contracts (On-Chain)

> **Labels:** enhancement · blockchain · solidity · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #12](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/12)

---

## Ziel

Die Python-Contract-Implementierungen in **Solidity** übersetzen für eine echte EVM-kompatible On-Chain Deployment-Option auf der A-TownChain EVM-Schicht.

---

## Contract-Spezifikationen

### ATC Token.sol (ATC-8300)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

contract ATC Token is ERC20, Ownable, Pausable {

    uint256 public constant MAX_SUPPLY       = 21_000_000 * 10**18;
    uint256 public constant HALVING_INTERVAL = 210_000;
    uint256 public constant INITIAL_REWARD   = 50 * 10**18;

    mapping(address => bool) public miners;

    constructor() ERC20("A-Town Coin", "ATC") Ownable(msg.sender) {}

    function mint(address to, uint256 amount)
        external onlyMiner whenNotPaused {
        require(totalSupply() + amount <= MAX_SUPPLY, "Max supply");
        _mint(to, amount);
    }

    function getBlockReward(uint256 blockHeight)
        public pure returns (uint256) {
        uint256 halvings = blockHeight / HALVING_INTERVAL;
        return halvings >= 64 ? 0 : INITIAL_REWARD >> halvings;
    }

    function burn(uint256 amount) external {
        _burn(msg.sender, amount);
    }

    modifier onlyMiner() {
        require(miners[msg.sender], "Not a miner");
        _;
    }
}
```

### Shivamon.sol (ATC-9000)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract Shivamon is ERC721 {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    uint256 public constant MAX_SUPPLY  = 9900;
    uint256 public          mintingFee  = 10 * 10**18; // 10 ATC

    struct ShivamonData {
        uint8   element;    // 0=Fire 1=Water 2=Earth 3=Air 4=Shadow 5=Neon 6=Quantum
        uint8   rarity;     // 0=Common ... 5=Genesis
        uint8   level;
        uint16  generation;
        uint32  hp; uint32 attack; uint32 defense;
        uint32  speed; uint32 special;
        bytes32 dnaHash;
        uint256 mintedAt;
    }

    mapping(uint256 => ShivamonData) public shivamonData;

    ATC Token public atcToken;

    constructor(address _atcToken) ERC721("Shivamon", "SHV") {
        atcToken = ATC Token(_atcToken);
    }

    function mint(address to, uint8 element, uint8 rarity)
        external returns (uint256) {
        require(_tokenIds.current() < MAX_SUPPLY, "Max supply");
        require(atcToken.transferFrom(msg.sender, address(this), mintingFee),
                "ATC payment failed");

        _tokenIds.increment();
        uint256 newId = _tokenIds.current();
        _mint(to, newId);

        bytes32 dna = keccak256(abi.encodePacked(to, newId, block.timestamp));
        uint256 d   = uint256(dna);

        shivamonData[newId] = ShivamonData({
            element: element, rarity: rarity, level: 1, generation: 1,
            hp:      uint32(50 + (d & 0xFF) % 100),
            attack:  uint32(40 + ((d >> 8) & 0xFF) % 80),
            defense: uint32(40 + ((d >> 16) & 0xFF) % 80),
            speed:   uint32(35 + ((d >> 24) & 0xFF) % 70),
            special: uint32(45 + ((d >> 32) & 0xFF) % 90),
            dnaHash: dna,
            mintedAt: block.timestamp
        });
        return newId;
    }
}
```

---

## Aufgaben

- [ ] `blockchain/contracts/solidity/` Verzeichnis anlegen
- [ ] `ATC Token.sol` schreiben + Tests
- [ ] `Shivamon.sol` schreiben + Tests
- [ ] `ATCGovernance.sol` schreiben + Tests
- [ ] Hardhat-Projekt einrichten (`hardhat.config.js`)
- [ ] Deployment-Script für Testnet
- [ ] ABI-Export nach `config/abis/`
- [ ] Backend: Web3.py On-Chain Queries
- [ ] Test Coverage > 90%

---

## Akzeptanzkriterien

- [ ] Alle Contracts deployen auf Sepolia Testnet
- [ ] ATC Token: Transfer, Mint, Burn, Halving korrekt
- [ ] Shivamon: Mint, Transfer, DNA korrekt
- [ ] ABI im Backend integriert
- [ ] Test Coverage > 90%
