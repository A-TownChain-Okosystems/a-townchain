// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Address lookup
export interface AddressInfo {
  address: string;
  balance: number;
  txCount: number;
  isValidator: boolean;
  firstSeen: number;
}

export class AddressLookup {
  private addresses: Map<string, AddressInfo> = new Map();

  add(info: AddressInfo): void { this.addresses.set(info.address, info); }
  lookup(address: string): AddressInfo | null { return this.addresses.get(address) ?? null; }
  updateBalance(address: string, balance: number): void {
    const info = this.addresses.get(address);
    if (info) { info.balance = balance; }
  }
}
