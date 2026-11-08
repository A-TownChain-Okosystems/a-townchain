# Kapitel 33 — Ethereum/EVM Integration

> Version: 1.0.0 | Stand: 2026-06-09 | KAI-OS Wiki
> Status: Geplant (Sprint 2.9, Sprint 3.11)

---

## 33.1 Strategische Entscheidung: Frontier-Pallet

**Gewaehlt: Substrate Frontier EVM-Pallet**

```
Substrate-Runtime
+-- pallet-evm       (Frontier — EVM-Execution-Engine)
+-- pallet-ethereum  (Frontier — Ethereum-Block-Format)
+-- pallet-base-fee  (EIP-1559 kompatibel)
+-- pallet-dynamic-fee

Vorteile:
- EVM direkt in Substrate integriert
- MetaMask/Ethers.js funktionieren ohne Aenderung
- Solidity-Contracts laufen nativ auf KAI-OS
- Bestehende ETH-Tools (Hardhat, Foundry) nutzbar
- EIP-1559, EIP-712, EIP-2930 kompatibel
- Chain-ID: 9000 (eindeutig fuer KAI-OS)
```

---

## 33.2 Frontier-Pallet Cargo.toml

```toml
[dependencies]
pallet-evm = { git = "https://github.com/polkadot-evm/frontier", features = ["forbid-evm-reentrancy"] }
pallet-ethereum = { git = "https://github.com/polkadot-evm/frontier" }
pallet-base-fee = { git = "https://github.com/polkadot-evm/frontier" }
fp-evm = { git = "https://github.com/polkadot-evm/frontier" }
fp-rpc = { git = "https://github.com/polkadot-evm/frontier" }
```

---

## 33.3 Runtime-Konfiguration

```rust
parameter_types! {
    pub BlockGasLimit: U256 = U256::from(75_000_000u64);
    pub const GasLimitPovSizeRatio: u64 = 16;
    pub WeightPerGas: Weight = Weight::from_parts(20_000, 0);
}

impl pallet_evm::Config for Runtime {
    type FeeCalculator = BaseFee;
    type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
    type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
    type AddressMapping = HashedAddressMapping<BlakeTwo256>;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type ChainId = EVMChainId;          // 9000
    type BlockGasLimit = BlockGasLimit;
    type Runner = pallet_evm::runner::stack::Runner<Self>;
    type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}
```

---

## 33.4 Solidity Contract Suite

### Projekt-Struktur (Hardhat)

```
kai-solidity/
+-- contracts/
|   +-- ATC Token.sol          # ERC-20 (ATC-8300 kompatibel)
|   +-- ShivamonNFT.sol       # ERC-721 + ERC-2981 Royalties
|   +-- KAIGovernance.sol     # Governor Bravo kompatibel
|   +-- KAIMarketplace.sol    # ERC-721 Marketplace
|   +-- KAIBridge.sol         # Lock/Unlock Bridge
+-- test/
+-- scripts/deploy.ts
+-- hardhat.config.ts
```

### ATC Token.sol (ERC-20)

```solidity
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

contract ATC Token is ERC20, ERC20Permit, ERC20Votes, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18;

    constructor() ERC20("A-TownCoin", "ATC") ERC20Permit("A-TownCoin") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function mintFromBridge(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds max supply");
        _mint(to, amount);
        emit BridgeMint(to, amount);
    }

    function burnToBridge(address from, uint256 amount, bytes32 substrateAddress)
        external onlyRole(BURNER_ROLE)
    {
        _burn(from, amount);
        emit BridgeBurn(from, amount, substrateAddress);
    }

    event BridgeMint(address indexed to, uint256 amount);
    event BridgeBurn(address indexed from, uint256 amount, bytes32 substrateAddress);
}
```

### ShivamonNFT.sol (ERC-721 + ERC-2981)

```solidity
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Royalty.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract ShivamonNFT is ERC721URIStorage, ERC721Royalty {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIdCounter;

    struct ShivamonData {
        string name;
        string element;
        uint8 level;
        uint16 hp; uint16 attack; uint16 defense; uint16 speed;
        string rarity;
        uint8 generation;
        bytes32 dna;
    }

    mapping(uint256 => ShivamonData) public shivamonData;

    constructor() ERC721("Shivamon", "SHIV") {
        _setDefaultRoyalty(msg.sender, 250); // 2.5%
    }

    function mint(address to, string memory uri, ShivamonData calldata data)
        public returns (uint256)
    {
        uint256 tokenId = _tokenIdCounter.current();
        _tokenIdCounter.increment();
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, uri);
        shivamonData[tokenId] = data;
        emit ShivamonMinted(tokenId, to, data.name, data.rarity);
        return tokenId;
    }

    event ShivamonMinted(uint256 indexed tokenId, address indexed to, string name, string rarity);
}
```

---

## 33.5 Hardhat-Konfiguration

```typescript
const config: HardhatUserConfig = {
  solidity: { version: "0.8.20", settings: { optimizer: { enabled: true, runs: 200 } } },
  networks: {
    kaiDevnet:  { url: "http://localhost:9933", chainId: 9000, accounts: [process.env.DEPLOYER_KEY!] },
    kaiTestnet: { url: "https://rpc.testnet.kai-os.io", chainId: 9000 },
    kaiMainnet: { url: "https://rpc.kai-os.io", chainId: 9000 },
    sepolia:    { url: `https://sepolia.infura.io/v3/${process.env.INFURA_KEY}`, chainId: 11155111 },
  },
  etherscan: {
    customChains: [{ network: "kaiMainnet", chainId: 9000,
      urls: { apiURL: "https://explorer.kai-os.io/api", browserURL: "https://explorer.kai-os.io" }
    }]
  }
};
```

---

## 33.6 MetaMask-Konfiguration

```javascript
await window.ethereum.request({
  method: 'wallet_addEthereumChain',
  params: [{
    chainId: '0x2328',          // 9000 in hex
    chainName: 'KAI-OS Network',
    nativeCurrency: { name: 'A-TownCoin', symbol: 'ATC', decimals: 18 },
    rpcUrls: ['https://rpc.kai-os.io'],
    blockExplorerUrls: ['https://explorer.kai-os.io']
  }]
});
```

---

## 33.7 Gas-Kosten (KAI-OS EVM)

| Operation | Gas | ATC-Kosten (bei 1 GWei) |
|-----------|-----|-------------------------|
| Transfer | 21.000 | 0.000021 ATC |
| ERC-20 Transfer | 65.000 | 0.000065 ATC |
| NFT Mint | 150.000 | 0.00015 ATC |
| Contract Deploy | 500.000-2.000.000 | 0.0005-0.002 ATC |
| DAO Vote | 80.000 | 0.00008 ATC |

---

## 33.8 Roadmap-Integration

| Sprint | Aufgaben | Datum |
|--------|----------|-------|
| 2.9 | Frontier-Pallet einbauen, EVM-RPC, MetaMask | Sep 2026 |
| 3.11 | ATC Token.sol + ShivamonNFT.sol + Tests deployen | Apr 2027 |
| 4.6 | ETH-Bridge Mainnet, Blockscout-Explorer | Okt 2027 |

---

*KAI-OS Wiki Kapitel 33 — Ethereum/EVM Integration | v1.0.0 | 2026-06-09*
