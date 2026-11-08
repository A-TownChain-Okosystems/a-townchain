// Transaction browser
export interface TxInfo {
  hash: string;
  blockHeight: number;
  from: string;
  to: string;
  amount: number;
  fee: number;
  timestamp: number;
  status: 'pending' | 'confirmed' | 'failed';
}

export class TransactionBrowser {
  private txs: Map<string, TxInfo> = new Map();

  addTx(tx: TxInfo): void { this.txs.set(tx.hash, tx); }
  getTx(hash: string): TxInfo | null { return this.txs.get(hash) ?? null; }
  getAddressTxs(address: string): TxInfo[] {
    return [...this.txs.values()].filter(t => t.from === address || t.to === address);
  }
  getPending(): TxInfo[] { return [...this.txs.values()].filter(t => t.status === 'pending'); }
}
