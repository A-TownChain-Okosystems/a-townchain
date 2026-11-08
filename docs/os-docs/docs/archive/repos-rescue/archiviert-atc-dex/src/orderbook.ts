// Order book management
export interface Order {
  id: string;
  price: number;
  amount: number;
  side: 'buy' | 'sell';
  timestamp: number;
}

export class Orderbook {
  private bids: Order[] = [];
  private asks: Order[] = [];

  addOrder(order: Order): void {
    if (order.side === 'buy') {
      this.bids.push(order);
      this.bids.sort((a, b) => b.price - a.price);
    } else {
      this.asks.push(order);
      this.asks.sort((a, b) => a.price - b.price);
    }
  }

  removeOrder(id: string): void {
    this.bids = this.bids.filter(o => o.id !== id);
    this.asks = this.asks.filter(o => o.id !== id);
  }

  getBestBid(): Order | null { return this.bids[0] ?? null; }
  getBestAsk(): Order | null { return this.asks[0] ?? null; }
  getSpread(): number {
    const bid = this.getBestBid()?.price ?? 0;
    const ask = this.getBestAsk()?.price ?? Infinity;
    return ask - bid;
  }
  getBids(): Order[] { return [...this.bids]; }
  getAsks(): Order[] { return [...this.asks]; }
}
