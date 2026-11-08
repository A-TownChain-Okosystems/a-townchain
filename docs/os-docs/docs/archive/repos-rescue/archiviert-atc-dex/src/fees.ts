// Fee calculation
export class FeeCalculator {
  constructor(private rate: number = 0.003) {}

  calculateFee(amount: number): number { return amount * this.rate; }
  calculateNet(amount: number): number { return amount - this.calculateFee(amount); }
  setRate(rate: number): void { this.rate = rate; }
  getRate(): number { return this.rate; }
}
