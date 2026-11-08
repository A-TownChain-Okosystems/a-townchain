// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Liquidity pool
export class LiquidityPool {
  private reserveA: number;
  private reserveB: number;
  private lpTokens: Map<string, number> = new Map();

  constructor(reserveA: number, reserveB: number) {
    this.reserveA = reserveA;
    this.reserveB = reserveB;
  }

  swap(amountA: number): number {
    const k = this.reserveA * this.reserveB;
    this.reserveA += amountA;
    const newB = k / this.reserveA;
    const out = this.reserveB - newB;
    this.reserveB = newB;
    return out;
  }

  addLiquidity(provider: string, amountA: number, amountB: number): number {
    const totalLp = (this.reserveA * this.reserveB) ** 0.5;
    const lpMinted = (amountA * amountB) ** 0.5;
    this.reserveA += amountA;
    this.reserveB += amountB;
    this.lpTokens.set(provider, (this.lpTokens.get(provider) ?? 0) + lpMinted);
    return lpMinted;
  }

  getReserves(): [number, number] { return [this.reserveA, this.reserveB]; }
  getPrice(): number { return this.reserveB / this.reserveA; }
}
