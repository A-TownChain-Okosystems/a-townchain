// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Order matching engine
import { Order, Orderbook } from './orderbook';

export class OrderMatcher {
  match(newOrder: Order, book: Orderbook): { matched: Order[] } {
    const matches: Order[] = [];
    const opposite = newOrder.side === 'buy' ? book.getAsks() : book.getBids();

    for (const order of opposite) {
      if (newOrder.side === 'buy' && order.price <= newOrder.price) matches.push(order);
      else if (newOrder.side === 'sell' && order.price >= newOrder.price) matches.push(order);
    }
    return { matched: matches };
  }
}
