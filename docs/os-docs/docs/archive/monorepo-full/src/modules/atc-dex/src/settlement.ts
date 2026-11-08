// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Trade settlement
export interface Trade {
  id: string;
  buyer: string;
  seller: string;
  price: number;
  amount: number;
  timestamp: number;
}

export class Settlement {
  private trades: Trade[] = [];

  settle(trade: Omit<Trade, 'id' | 'timestamp'>): Trade {
    const full: Trade = {
      ...trade,
      id: `trade-${this.trades.length + 1}`,
      timestamp: Date.now(),
    };
    this.trades.push(full);
    return full;
  }

  getTrades(): Trade[] { return [...this.trades]; }
  getTradeCount(): number { return this.trades.length; }
}
