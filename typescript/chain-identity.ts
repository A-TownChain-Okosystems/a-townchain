// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
// ATC-STD-600 SDK identity and transaction-domain primitives.

export type NetworkId = "devnet" | "testnet" | "mainnet";

export interface ChainIdentity {
  chain_id: "atc";
  network_id: NetworkId;
  genesis_id: string;
}

export interface RuntimeContext extends ChainIdentity {
  protocol_version: string;
  vm_version: string;
}

export interface TransactionDomain extends RuntimeContext {
  transaction_type: string;
}

export const ATC_CHAIN_ID = "atc" as const;
export const ATC_TX_DOMAIN = "ATC-TX-DOMAIN" as const;

export function validateIdentity(identity: ChainIdentity): void {
  if (identity.chain_id !== ATC_CHAIN_ID) throw new Error("ATC-STD-600: invalid chain_id");
  if (!["devnet", "testnet", "mainnet"].includes(identity.network_id)) throw new Error("ATC-STD-600: invalid network_id");
  if (!/^[0-9a-fA-F]{64}$/.test(identity.genesis_id)) throw new Error("ATC-STD-600: invalid genesis_id");
}

function field(out: number[], key: string, value: Uint8Array): void {
  const encoder = new TextEncoder();
  const k = encoder.encode(key);
  out.push((k.length >>> 24) & 255, (k.length >>> 16) & 255, (k.length >>> 8) & 255, k.length & 255);
  out.push(...k);
  const hi = Math.floor(value.length / 0x100000000);
  const lo = value.length >>> 0;
  out.push((hi >>> 24) & 255, (hi >>> 16) & 255, (hi >>> 8) & 255, hi & 255, (lo >>> 24) & 255, (lo >>> 16) & 255, (lo >>> 8) & 255, lo & 255);
  out.push(...value);
}

export function canonicalSigningPreimage(domain: TransactionDomain, tx: {
  nonce: bigint; sender: Uint8Array; recipient: Uint8Array; value: bigint; fee: bigint; payload: Uint8Array;
}): Uint8Array {
  validateIdentity(domain);
  if (!domain.protocol_version || !domain.transaction_type) throw new Error("ATC-STD-600: invalid transaction domain");
  const encoder = new TextEncoder();
  const out: number[] = [];
  const text = (s: string) => encoder.encode(s);
  field(out, "domain", text(ATC_TX_DOMAIN));
  field(out, "chain_id", text(domain.chain_id));
  field(out, "network_id", text(domain.network_id));
  field(out, "protocol_version", text(domain.protocol_version));
  field(out, "transaction_type", text(domain.transaction_type));
  field(out, "nonce", text(tx.nonce.toString()));
  field(out, "sender", tx.sender);
  field(out, "recipient", tx.recipient);
  field(out, "value", text(tx.value.toString()));
  field(out, "fee", text(tx.fee.toString()));
  field(out, "payload", tx.payload);
  return Uint8Array.from(out);
}
