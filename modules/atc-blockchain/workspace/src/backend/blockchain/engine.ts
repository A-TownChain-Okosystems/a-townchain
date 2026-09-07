// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import crypto from 'crypto';

export interface Transaction {
  id: string;
  sender: string;
  recipient: string;
  amount: number;
  signature?: string;
  timestamp: number;
}

export interface Block {
  index: number;
  timestamp: number;
  transactions: Transaction[];
  previousHash: string;
  hash: string;
  nonce: number;
  validator: string;
  consensusMetrics: {
    poH_hash: string;
    poW_difficulty: number;
    poS_stakeWeight: number;
    poI_score: number;
  };
}

export class AtcBlockchainEngine {
  private chain: Block[] = [];
  private pendingTransactions: Transaction[] = [];
  private miningInterval: any = null;
  private tpsStats: number = 0;

  constructor() {
    this.createGenesisBlock();
  }

  private createGenesisBlock() {
    const genesisBlock: Block = {
      index: 0,
      timestamp: Date.now(),
      transactions: [],
      previousHash: "0",
      hash: this.calculateHash(0, "0", Date.now(), [], 0),
      nonce: 0,
      validator: "Genesis",
      consensusMetrics: {
        poH_hash: "genesis_poh",
        poW_difficulty: 2,
        poS_stakeWeight: 100,
        poI_score: 1.0
      }
    };
    this.chain.push(genesisBlock);
  }

  private calculateHash(index: number, previousHash: string, timestamp: number, transactions: Transaction[], nonce: number): string {
    const data = index + previousHash + timestamp + JSON.stringify(transactions) + nonce;
    return crypto.createHash('sha256').update(data).digest('hex');
  }

  public getLatestBlock(): Block {
    return this.chain[this.chain.length - 1];
  }

  public getChainHeight(): number {
    return this.chain.length;
  }

  public getChain(): Block[] {
    return this.chain;
  }

  public getTPS(): number {
    return this.tpsStats;
  }

  public addTransaction(transaction: Transaction) {
    this.pendingTransactions.push(transaction);
  }

  public startConsensus() {
    // PoI + PoS (Proof-of-Intelligence + Proof-of-Stake) Simulation Pipeline
    this.miningInterval = setInterval(() => {
      this.minePendingTransactions("Validator-Node-Alpha");
    }, 10000); // New block every 10 seconds
  }

  public stopConsensus() {
    if (this.miningInterval) {
      clearInterval(this.miningInterval);
    }
  }

  private minePendingTransactions(miningRewardAddress: string) {
    const previousBlock = this.getLatestBlock();
    let nonce = 0;
    const timestamp = Date.now();
    
    // 1. Simulate PoH (Proof of History) - Sequential hashing of time delay
    let pohHash = crypto.createHash('sha256').update(previousBlock.timestamp.toString() + timestamp.toString()).digest('hex');
    for(let i=0; i<50; i++) {
       pohHash = crypto.createHash('sha256').update(pohHash).digest('hex');
    }

    // 2. Simulate PoS (Proof of Stake) - Stake Weight Evaluation
    const stakeWeight = Math.floor(Math.random() * 10000) + 1000;

    // 3. Simulate PoI (Proof of Intelligence) - AI Model Capability Score
    const poiScore = parseFloat(Math.min(1.0, Math.random() + 0.2).toFixed(4));
    
    // Determine dynamic difficulty based on combined PoS + PoI synergy
    // Higher stake and higher intelligence lowers the PoW difficulty target
    let difficulty = 3;
    if (stakeWeight > 8000 && poiScore > 0.8) difficulty = 1;
    else if (stakeWeight > 4000 || poiScore > 0.6) difficulty = 2;
    
    const targetPrefix = "0".repeat(difficulty);
    let hash = this.calculateHash(previousBlock.index + 1, previousBlock.hash, timestamp, this.pendingTransactions, nonce);

    // 4. PoW (Proof of Work) - Fallback/complementary mining loop
    while (hash.substring(0, difficulty) !== targetPrefix) {
      nonce++;
      hash = this.calculateHash(previousBlock.index + 1, previousBlock.hash, timestamp, this.pendingTransactions, nonce);
    }

    const newBlock: Block = {
      index: previousBlock.index + 1,
      timestamp,
      transactions: [...this.pendingTransactions],
      previousHash: previousBlock.hash,
      hash,
      nonce,
      validator: miningRewardAddress,
      consensusMetrics: {
        poH_hash: pohHash.substring(0, 16) + '...',
        poW_difficulty: difficulty,
        poS_stakeWeight: stakeWeight,
        poI_score: poiScore
      }
    };

    if (this.pendingTransactions.length > 0) {
      this.tpsStats = this.pendingTransactions.length / 10; // Transactions per second based on 10s interval
    } else {
      this.tpsStats = 0;
    }

    this.chain.push(newBlock);
    this.pendingTransactions = []; // Reset pending operations
    
    // Simulate ambient network traffic by adding random transactions sometimes
    if (Math.random() > 0.5) {
      const txCount = Math.floor(Math.random() * 50) + 10;
      for (let i = 0; i < txCount; i++) {
        this.addTransaction({
          id: crypto.randomBytes(16).toString('hex'),
          sender: "0x" + crypto.randomBytes(20).toString('hex'),
          recipient: "0x" + crypto.randomBytes(20).toString('hex'),
          amount: Math.floor(Math.random() * 1000),
          timestamp: Date.now()
        });
      }
    }
  }
}
