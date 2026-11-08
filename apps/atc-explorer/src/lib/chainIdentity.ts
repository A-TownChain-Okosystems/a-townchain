// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
// ATC-STD-600 display/ingest validation.

export type NetworkId = "devnet" | "testnet" | "mainnet";
export interface ChainIdentity { chain_id: string; network_id: NetworkId; genesis_id: string; protocol_version: string; vm_version: string; }

export function assertAtcIdentity(identity: ChainIdentity): void {
  if (identity.chain_id !== "atc") throw new Error("ATC-STD-600 identity mismatch: chain_id");
  if (!/^(devnet|testnet|mainnet)$/.test(identity.network_id)) throw new Error("ATC-STD-600 identity mismatch: network_id");
  if (!/^[0-9a-f]{64}$/i.test(identity.genesis_id)) throw new Error("ATC-STD-600 identity mismatch: genesis_id");
  if (!identity.protocol_version || !identity.vm_version) throw new Error("ATC-STD-600 runtime context incomplete");
}
