// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
// Canonical L1 transaction signing primitives.
// The byte layout MUST remain identical to the Rust L1 kernel signing_bytes().

export type NetworkId = "devnet" | "testnet" | "mainnet";
export const ATC_CHAIN_ID = 658467 as const;
export const ATC_TX_DOMAIN_V2 = "ATC-TX-DOMAIN-V2" as const;

export interface ChainIdentity {
  chain_id: typeof ATC_CHAIN_ID;
  network_id: NetworkId;
  genesis_id: string;
}

export interface RuntimeContext extends ChainIdentity {
  protocol_version: string;
  vm_version: string;
}

export interface TransactionSigningInput {
  chain_id: typeof ATC_CHAIN_ID;
  tx_type: number;
  sender_did: string;
  recipient_did?: string | null;
  amount: bigint;
  gas_price: bigint;
  gas_limit: bigint;
  nonce: bigint;
  timestamp: bigint;
  payload: Uint8Array;
  poh_hash: Uint8Array;
}

export function validateIdentity(identity: ChainIdentity): void {
  if (identity.chain_id !== ATC_CHAIN_ID) throw new Error("ATC-L1: invalid numeric chain_id");
  if (!["devnet", "testnet", "mainnet"].includes(identity.network_id)) {
    throw new Error("ATC-STD-600: invalid network_id");
  }
  if (!/^[0-9a-fA-F]{64}$/.test(identity.genesis_id)) {
    throw new Error("ATC-STD-600: invalid genesis_id");
  }
}

function pushU32BE(out: number[], value: number): void {
  out.push((value >>> 24) & 0xff, (value >>> 16) & 0xff, (value >>> 8) & 0xff, value & 0xff);
}

function pushU64BE(out: number[], value: bigint): void {
  if (value < 0n || value > 0xffffffffffffffffn) throw new RangeError("u64 out of range");
  for (let shift = 56n; shift >= 0n; shift -= 8n) {
    out.push(Number((value >> shift) & 0xffn));
  }
}

function pushBytes(out: number[], value: Uint8Array): void {
  pushU32BE(out, value.length);
  out.push(...value);
}

function pushOptionalString(out: number[], value: string | null | undefined): void {
  if (value == null) {
    out.push(0);
    return;
  }
  out.push(1);
  pushBytes(out, new TextEncoder().encode(value));
}

function assert32Bytes(name: string, value: Uint8Array): void {
  if (value.length !== 32) throw new Error(`${name} must be exactly 32 bytes`);
}

/**
 * Exact byte representation used by the Rust L1 kernel:
 * ATC-TX-DOMAIN-V2 || chain_id(u64 BE) || tx_type(u8) ||
 * sender_did || recipient_did(optional) || amount(u64 BE) ||
 * gas_price(u64 BE) || gas_limit(u64 BE) || nonce(u64 BE) ||
 * timestamp(u64 BE) || payload || poh_hash(32 bytes).
 */
export function canonicalSigningPreimage(tx: TransactionSigningInput): Uint8Array {
  if (tx.chain_id !== ATC_CHAIN_ID) throw new Error("ATC-L1: invalid numeric chain_id");
  if (!Number.isInteger(tx.tx_type) || tx.tx_type < 0 || tx.tx_type > 255) {
    throw new RangeError("tx_type must fit u8");
  }
  if (!tx.sender_did) throw new Error("ATC-L1: sender_did must not be empty");
  assert32Bytes("poh_hash", tx.poh_hash);

  const out: number[] = [];
  out.push(...new TextEncoder().encode(ATC_TX_DOMAIN_V2));
  pushU64BE(out, BigInt(tx.chain_id));
  out.push(tx.tx_type);
  pushBytes(out, new TextEncoder().encode(tx.sender_did));
  pushOptionalString(out, tx.recipient_did);
  pushU64BE(out, tx.amount);
  pushU64BE(out, tx.gas_price);
  pushU64BE(out, tx.gas_limit);
  pushU64BE(out, tx.nonce);
  pushU64BE(out, tx.timestamp);
  pushBytes(out, tx.payload);
  out.push(...tx.poh_hash);
  return Uint8Array.from(out);
}
