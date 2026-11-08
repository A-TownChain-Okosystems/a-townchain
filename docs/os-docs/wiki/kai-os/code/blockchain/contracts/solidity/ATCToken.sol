// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";

/**
 * @title ATCToken
 * @dev ATC-8300 Standard — ERC20 Token für A-TownChain
 * Symbol: ATC
 * Max Supply: 21.000.000
 * Decimals: 18
 * Consensus: SHA-256 PoW + PoS + PoH
 */
contract ATCToken is ERC20, Ownable, ERC20Pausable {
    uint256 public constant MAX_SUPPLY = 21_000_000 * 10**18;
    uint256 public constant HALVING_INTERVAL = 210_000; // Blocks
    
    uint256 public blockRewardHalvings = 0;
    uint256 public lastHalvingBlock = 0;
    
    // Miner Registry — nur registrierte Miner dürfen prägen
    mapping(address => bool) public miners;
    
    // Events
    event MinerRegistered(address indexed miner);
    event MinerUnregistered(address indexed miner);
    event TokensMinted(address indexed to, uint256 amount, uint256 blockHeight);
    
    constructor() ERC20("A-Town Coin", "ATC") {
        lastHalvingBlock = block.number;
    }
    
    // ── Minting ────────────────────────────────────────
    /**
     * @dev Nur registrierte Miner dürfen neue ATC prägen (Block Reward)
     * @param to Empfänger der Tokens
     * @param amount Menge der zu prägenden Tokens
     */
    function mint(address to, uint256 amount) external onlyMiner returns (bool) {
        require(totalSupply() + amount <= MAX_SUPPLY, "Max supply exceeded");
        _mint(to, amount);
        emit TokensMinted(to, amount, block.number);
        return true;
    }
    
    /**
     * @dev Block Reward mit Halving-Mechanismus (wie Bitcoin)
     * Block 0-209999:      50 ATC
     * Block 210000-419999: 25 ATC
     * Block 420000-629999: 12.5 ATC
     * usw. bis zur Obergrenze
     */
    function getBlockReward(uint256 blockHeight) public view returns (uint256) {
        uint256 halvingCount = (blockHeight - lastHalvingBlock) / HALVING_INTERVAL;
        uint256 reward = 50 * 10**18; // Initial 50 ATC
        
        for (uint256 i = 0; i < halvingCount; i++) {
            reward = reward / 2;
            if (reward == 0) return 0;
        }
        
        return reward;
    }
    
    /**
     * @dev Triggert das Halving (meist vom Blockchain-Node aufgerufen)
     */
    function triggerHalving() external onlyOwner {
        lastHalvingBlock = block.number;
        blockRewardHalvings++;
    }
    
    // ── Access Control ────────────────────────────────
    /**
     * @dev Registriert eine neue Miner-Adresse
     */
    function registerMiner(address miner) external onlyOwner {
        require(miner != address(0), "Invalid miner address");
        miners[miner] = true;
        emit MinerRegistered(miner);
    }
    
    /**
     * @dev Entfernt eine Miner-Adresse
     */
    function unregisterMiner(address miner) external onlyOwner {
        miners[miner] = false;
        emit MinerUnregistered(miner);
    }
    
    /**
     * @dev Modifier: nur registrierte Miner dürfen diese Funktion aufrufen
     */
    modifier onlyMiner() {
        require(miners[msg.sender], "Only registered miners can mint");
        _;
    }
    
    // ── Pause/Unpause ────────────────────────────────
    /**
     * @dev Pausiert alle Transfers
     */
    function pause() external onlyOwner {
        _pause();
    }
    
    /**
     * @dev Reaktiviert Transfers
     */
    function unpause() external onlyOwner {
        _unpause();
    }
    
    // ── Burn Mechanism ───────────────────────────────
    /**
     * @dev Verbrennt Tokens permanent
     */
    function burn(uint256 amount) external {
        _burn(msg.sender, amount);
    }
    
    /**
     * @dev Verbrennt Tokens einer anderen Adresse (mit Erlaubnis)
     */
    function burnFrom(address account, uint256 amount) external {
        uint256 currentAllowance = allowance(account, msg.sender);
        require(currentAllowance >= amount, "Insufficient allowance");
        approve(account, msg.sender, currentAllowance - amount);
        _burn(account, amount);
    }
    
    // ── Snapshot (Governance) ────────────────────────
    /**
     * @dev Gibt aktuelle Supply-Informationen zurück
     */
    function getSupplyInfo() external view returns (
        uint256 total,
        uint256 max,
        uint256 remaining,
        uint256 holders
    ) {
        return (totalSupply(), MAX_SUPPLY, MAX_SUPPLY - totalSupply(), 0);
    }
    
    // ── Internal Overrides ────────────────────────────
    function _update(
        address from,
        address to,
        uint256 value
    ) internal override(ERC20, ERC20Pausable) {
        super._update(from, to, value);
    }
}
