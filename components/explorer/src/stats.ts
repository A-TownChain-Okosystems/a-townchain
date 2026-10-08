// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Stats Dashboard module

export interface ChainStats {
  totalBlocks: number;
  totalTransactions: number;
  currentTps: number;
  avgBlockTimeMs: number;
  activeAddresses: number;
  validatorCount: number;
  totalGasUsed: number;
}

export class StatsDashboard {
  private totalBlocks: number = 0;
  private totalTransactions: number = 0;
  private blockTimes: number[] = [];
  private recentTxCounts: number[] = [];
  private activeAddressesSet: Set<string> = new Set();
  private validatorCount: number = 0;
  private totalGasUsed: number = 0;

  recordBlock(blockTimeMs: number, txCount: number, gasUsed: number = 0): void {
    this.totalBlocks++;
    this.totalTransactions += txCount;
    this.totalGasUsed += gasUsed;

    this.blockTimes.push(blockTimeMs);
    if (this.blockTimes.length > 100) this.blockTimes.shift();

    this.recentTxCounts.push(txCount);
    if (this.recentTxCounts.length > 20) this.recentTxCounts.shift();
  }

  recordAddress(address: string): void {
    this.activeAddressesSet.add(address);
  }

  setValidatorCount(count: number): void {
    this.validatorCount = count;
  }

  getStats(): ChainStats {
    const avgBlockTimeMs =
      this.blockTimes.length > 0
        ? this.blockTimes.reduce((a, b) => a + b, 0) / this.blockTimes.length
        : 0;

    const totalRecentTxs = this.recentTxCounts.reduce((a, b) => a + b, 0);
    const totalRecentTimeSec = (avgBlockTimeMs * this.recentTxCounts.length) / 1000;
    const currentTps = totalRecentTimeSec > 0 ? parseFloat((totalRecentTxs / totalRecentTimeSec).toFixed(2)) : 0;

    return {
      totalBlocks: this.totalBlocks,
      totalTransactions: this.totalTransactions,
      currentTps,
      avgBlockTimeMs: Math.round(avgBlockTimeMs),
      activeAddresses: this.activeAddressesSet.size,
      validatorCount: this.validatorCount,
      totalGasUsed: this.totalGasUsed,
    };
  }

  reset(): void {
    this.totalBlocks = 0;
    this.totalTransactions = 0;
    this.blockTimes = [];
    this.recentTxCounts = [];
    this.activeAddressesSet.clear();
    this.validatorCount = 0;
    this.totalGasUsed = 0;
  }
}
