// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Enhanced Search functionality
import { BlockBrowser, BlockInfo } from './blocks';
import { TransactionBrowser, TxInfo } from './transactions';
import { AddressLookup, AddressInfo } from './addresses';
import { MempoolBrowser, MempoolTx } from './mempool';

export interface SearchResult {
  blocks: BlockInfo[];
  txs: TxInfo[];
  address: AddressInfo | null;
  mempoolTx: MempoolTx | null;
  type: 'block' | 'tx' | 'address' | 'mempool' | 'none';
}

export class Search {
  constructor(
    private blocks: BlockBrowser,
    private txs: TransactionBrowser,
    private addresses?: AddressLookup,
    private mempool?: MempoolBrowser
  ) {}

  query(term: string): SearchResult {
    const trimmed = term.trim();
    if (!trimmed) {
      return { blocks: [], txs: [], address: null, mempoolTx: null, type: 'none' };
    }

    // Check if block height (numeric)
    const height = parseInt(trimmed, 10);
    if (!isNaN(height) && /^\d+$/.test(trimmed)) {
      const block = this.blocks.getBlock(height);
      if (block) {
        return { blocks: [block], txs: [], address: null, mempoolTx: null, type: 'block' };
      }
    }

    // Check transaction hash
    const tx = this.txs.getTx(trimmed);
    if (tx) {
      return { blocks: [], txs: [tx], address: null, mempoolTx: null, type: 'tx' };
    }

    // Check address lookup
    if (this.addresses) {
      const addr = this.addresses.lookup(trimmed);
      if (addr) {
        return { blocks: [], txs: [], address: addr, mempoolTx: null, type: 'address' };
      }
    }

    // Check mempool
    if (this.mempool) {
      const mTx = this.mempool.getTx(trimmed);
      if (mTx) {
        return { blocks: [], txs: [], address: null, mempoolTx: mTx, type: 'mempool' };
      }
    }

    // Check block hash (string search in blocks)
    const latestBlocks = this.blocks.getBlocks(100);
    const blockByHash = latestBlocks.find(b => b.hash.toLowerCase() === trimmed.toLowerCase());
    if (blockByHash) {
      return { blocks: [blockByHash], txs: [], address: null, mempoolTx: null, type: 'block' };
    }

    return { blocks: [], txs: [], address: null, mempoolTx: null, type: 'none' };
  }
}
