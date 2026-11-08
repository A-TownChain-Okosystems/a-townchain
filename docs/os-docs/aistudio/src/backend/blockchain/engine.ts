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
      validator: "Genesis"
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
    let hash = this.calculateHash(previousBlock.index + 1, previousBlock.hash, timestamp, this.pendingTransactions, nonce);

    // Simple PoW simulation for the consensus
    while (hash.substring(0, 2) !== "00") {
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
      validator: miningRewardAddress
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
