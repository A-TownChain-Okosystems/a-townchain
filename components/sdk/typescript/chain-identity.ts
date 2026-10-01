// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
// Canonical L1 transaction signing primitives.
// This client representation MUST remain byte-identical to the frozen protocol contract.

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
  if (!Number.isSafeInteger(value) || value < 0 || value > 0xffffffff) {
    throw new RangeError("u32 out of range");
  }
  out.push((value >>> 24) & 0xff, (value >>> 16) & 0xff, (value >>> 8) & 0xff, value & 0xff);
}

function pushUIntBE(out: number[], value: bigint, bytes: number): void {
  const max = (1n << BigInt(bytes * 8)) - 1n;
  if (value < 0n || value > max) throw new RangeError(`u${bytes * 8} out of range`);
  for (let shift = BigInt((bytes - 1) * 8); shift >= 0n; shift -= 8n) {
    out.push(Number((value >> shift) & 0xffn));
  }
}

function pushU64BE(out: number[], value: bigint): void {
  pushUIntBE(out, value, 8);
}

function pushU128BE(out: number[], value: bigint): void {
  pushUIntBE(out, value, 16);
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
 * Canonical transaction preimage:
 * domain || chain_id(u64 BE) || tx_type(u8) || sender || recipient? ||
 * amount(u128 fixed 16-byte BE) || gas_price(u128 fixed 16-byte BE) || gas_limit(u64 BE) ||
 * nonce(u64 BE) || timestamp(u64 BE) || payload || poh_hash(32).
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
  pushU128BE(out, tx.amount);
  pushU128BE(out, tx.gas_price);
  pushU64BE(out, tx.gas_limit);
  pushU64BE(out, tx.nonce);
  pushU64BE(out, tx.timestamp);
  pushBytes(out, tx.payload);
  out.push(...tx.poh_hash);
  return Uint8Array.from(out);
}


export function bytesToHex(value: Uint8Array): string {
  return Array.from(value, byte => byte.toString(16).padStart(2, "0")).join("");
}

// Frozen cross-language conformance vector shared with the canonical Rust wallet.
export const CANONICAL_TX_V2_VECTOR = {
  preimageHex:
    "4154432d54582d444f4d41494e2d563200000000000a0c23000000000a4154432d73656e646572010000000d4154432d726563697069656e7400000000000000000000000000000064000000000000000000000000000000000100000000000003e80000000000000007000000006553f1000000000568656c6c6f0909090909090909090909090909090909090909090909090909090909090909",
  sha256Hex: "8608d1530c0c8dd02207903ec6b24071878b36fca0299b2534cef76e79d340be",
} as const;

export function assertCanonicalTxV2Vector(): void {
  const preimage = canonicalSigningPreimage({
    chain_id: ATC_CHAIN_ID,
    tx_type: 0,
    sender_did: "ATC-sender",
    recipient_did: "ATC-recipient",
    amount: 100n,
    gas_price: 1n,
    gas_limit: 1000n,
    nonce: 7n,
    timestamp: 1_700_000_000n,
    payload: new TextEncoder().encode("hello"),
    poh_hash: new Uint8Array(32).fill(9),
  });
  if (bytesToHex(preimage) !== CANONICAL_TX_V2_VECTOR.preimageHex) {
    throw new Error("ATC-TX-V2 conformance vector mismatch");
  }
}
