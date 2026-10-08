// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Unified Explorer Engine (L7 UI/L5 API layer)

import { BlockBrowser, BlockInfo } from './blocks';
import { TransactionBrowser, TxInfo } from './transactions';
import { AddressLookup, AddressInfo } from './addresses';
import { MempoolBrowser, MempoolTx } from './mempool';
import { StatsDashboard, ChainStats } from './stats';
import { Search, SearchResult } from './search';
import { ApiClient } from './api';
import { ChainIdentity, assertAtcIdentity } from './lib/chainIdentity';

export interface ExplorerOverview {
  identity: ChainIdentity | null;
  latestBlock: BlockInfo | null;
  stats: ChainStats;
  mempoolSize: number;
}

export class ExplorerEngine {
  public blocks: BlockBrowser = new BlockBrowser();
  public transactions: TransactionBrowser = new TransactionBrowser();
  public addresses: AddressLookup = new AddressLookup();
  public mempool: MempoolBrowser = new MempoolBrowser();
  public stats: StatsDashboard = new StatsDashboard();
  public search: Search = new Search(this.blocks, this.transactions, this.addresses, this.mempool);
  public apiClient: ApiClient;

  private identity: ChainIdentity | null = null;
  private blockListeners: Array<(block: BlockInfo) => void> = [];
  private txListeners: Array<(tx: TxInfo) => void> = [];

  constructor(apiBaseUrl: string = 'http://localhost:4000', identity?: ChainIdentity) {
    this.apiClient = new ApiClient(apiBaseUrl);
    if (identity) {
      this.setChainIdentity(identity);
    }
  }

  public setChainIdentity(identity: ChainIdentity): void {
    assertAtcIdentity(identity);
    this.identity = identity;
  }

  public getChainIdentity(): ChainIdentity | null {
    return this.identity;
  }

  public ingestBlock(block: BlockInfo, blockTxs: TxInfo[] = [], blockTimeMs: number = 2000, gasUsed: number = 0): void {
    this.blocks.addBlock(block);
    
    let txCount = block.txCount;
    if (blockTxs.length > 0) {
      txCount = blockTxs.length;
      for (const tx of blockTxs) {
        this.ingestTx(tx);
      }
    }

    if (block.validator) {
      const existingVal = this.addresses.lookup(block.validator);
      if (!existingVal) {
        this.addresses.add({
          address: block.validator,
          balance: 0,
          txCount: 0,
          isValidator: true,
          firstSeen: block.timestamp,
        });
      }
    }

    this.stats.recordBlock(blockTimeMs, txCount, gasUsed);

    for (const listener of this.blockListeners) {
      listener(block);
    }
  }

  public ingestTx(tx: TxInfo): void {
    this.transactions.addTx(tx);
    this.mempool.removeTx(tx.hash);

    // Update sender
    const sender = this.addresses.lookup(tx.from);
    if (sender) {
      this.addresses.updateBalance(tx.from, Math.max(0, sender.balance - tx.amount - tx.fee));
    } else {
      this.addresses.add({
        address: tx.from,
        balance: 0,
        txCount: 1,
        isValidator: false,
        firstSeen: tx.timestamp,
      });
    }
    this.stats.recordAddress(tx.from);

    // Update receiver
    const receiver = this.addresses.lookup(tx.to);
    if (receiver) {
      this.addresses.updateBalance(tx.to, receiver.balance + tx.amount);
    } else {
      this.addresses.add({
        address: tx.to,
        balance: tx.amount,
        txCount: 1,
        isValidator: false,
        firstSeen: tx.timestamp,
      });
    }
    this.stats.recordAddress(tx.to);

    for (const listener of this.txListeners) {
      listener(tx);
    }
  }

  public ingestMempoolTx(tx: MempoolTx): boolean {
    return this.mempool.addTx(tx);
  }

  public onBlock(listener: (block: BlockInfo) => void): () => void {
    this.blockListeners.push(listener);
    return () => {
      this.blockListeners = this.blockListeners.filter(l => l !== listener);
    };
  }

  public onTx(listener: (tx: TxInfo) => void): () => void {
    this.txListeners.push(listener);
    return () => {
      this.txListeners = this.txListeners.filter(l => l !== listener);
    };
  }

  public getOverview(): ExplorerOverview {
    return {
      identity: this.identity,
      latestBlock: this.blocks.getLatestBlock(),
      stats: this.stats.getStats(),
      mempoolSize: this.mempool.size(),
    };
  }

  public searchQuery(query: string): SearchResult {
    return this.search.query(query);
  }

  public getPaginatedBlocks(offset: number = 0, limit: number = 10): { items: BlockInfo[]; total: number } {
    const all = this.blocks.getBlocks(1000);
    return {
      items: all.slice(offset, offset + limit),
      total: all.length,
    };
  }

  public getPaginatedTxs(offset: number = 0, limit: number = 10): { items: TxInfo[]; total: number } {
    const pending = this.transactions.getPending();
    return {
      items: pending.slice(offset, offset + limit),
      total: pending.length,
    };
  }
}
