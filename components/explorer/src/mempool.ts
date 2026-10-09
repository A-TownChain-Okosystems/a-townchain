// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Mempool view & management module

export interface MempoolTx {
  hash: string;
  from: string;
  to: string;
  amount: number;
  fee: number;
  gasPrice: number;
  nonce: number;
  timestamp: number;
  payload?: string;
}

export class MempoolBrowser {
  private mempool: Map<string, MempoolTx> = new Map();

  addTx(tx: MempoolTx): boolean {
    if (this.mempool.has(tx.hash)) {
      return false;
    }
    this.mempool.set(tx.hash, tx);
    return true;
  }

  removeTx(hash: string): boolean {
    return this.mempool.delete(hash);
  }

  getTx(hash: string): MempoolTx | null {
    return this.mempool.get(hash) ?? null;
  }

  getPending(limit: number = 50): MempoolTx[] {
    return [...this.mempool.values()]
      .sort((a, b) => b.gasPrice - a.gasPrice || a.nonce - b.nonce || a.timestamp - b.timestamp)
      .slice(0, limit);
  }

  getTxsBySender(address: string): MempoolTx[] {
    return [...this.mempool.values()]
      .filter((tx) => tx.from === address)
      .sort((a, b) => a.nonce - b.nonce);
  }

  size(): number {
    return this.mempool.size;
  }

  clear(): void {
    this.mempool.clear();
  }

  evictStale(maxAgeMs: number, currentTime: number = Date.now()): number {
    let evicted = 0;
    for (const [hash, tx] of this.mempool.entries()) {
      if (currentTime - tx.timestamp > maxAgeMs) {
        this.mempool.delete(hash);
        evicted++;
      }
    }
    return evicted;
  }
}
