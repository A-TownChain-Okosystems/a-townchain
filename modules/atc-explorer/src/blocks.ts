// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Block browser
export interface BlockInfo {
  height: number;
  hash: string;
  prevHash: string;
  timestamp: number;
  txCount: number;
  validator: string;
}

export class BlockBrowser {
  private blocks: Map<number, BlockInfo> = new Map();

  addBlock(block: BlockInfo): void { this.blocks.set(block.height, block); }
  getBlock(height: number): BlockInfo | null { return this.blocks.get(height) ?? null; }
  getLatestBlock(): BlockInfo | null {
    const heights = [...this.blocks.keys()].sort((a, b) => b - a);
    return heights.length > 0 ? this.blocks.get(heights[0]) ?? null : null;
  }
  getBlocks(limit: number = 10): BlockInfo[] {
    return [...this.blocks.values()].sort((a, b) => b.height - a.height).slice(0, limit);
  }
}
