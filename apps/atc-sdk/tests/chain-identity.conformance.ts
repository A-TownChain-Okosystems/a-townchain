import { strict as assert } from "node:assert";
import { canonicalSigningPreimage, ATC_CHAIN_ID } from "../typescript/chain-identity.ts";

const bytes = canonicalSigningPreimage({
  chain_id: ATC_CHAIN_ID,
  tx_type: 0,
  sender_did: "ATC-sender",
  recipient_did: "ATC-recipient",
  amount: 100n,
  gas_price: 1n,
  gas_limit: 1000n,
  nonce: 7n,
  timestamp: 1_700_000_000n,
  payload: Uint8Array.from([0x68, 0x65, 0x6c, 0x6c, 0x6f]),
  poh_hash: new Uint8Array(32).fill(9),
});

const expected =
  "4154432d54582d444f4d41494e2d563200000000000a0c23000000000a4154432d73656e646572010000000d4154432d726563697069656e7400000000000000000000000000000064000000000000000100000000000003e80000000000000007000000006553f1000000000568656c6c6f0909090909090909090909090909090909090909090909090909090909090909";

assert.equal(Buffer.from(bytes).toString("hex"), expected);

const max = canonicalSigningPreimage({
  chain_id: ATC_CHAIN_ID,
  tx_type: 0,
  sender_did: "ATC-sender",
  recipient_did: null,
  amount: (1n << 128n) - 1n,
  gas_price: (1n << 64n) - 1n,
  gas_limit: 0n,
  nonce: 0n,
  timestamp: 0n,
  payload: new Uint8Array(),
  poh_hash: new Uint8Array(32),
});
assert.equal(max.length, 16 + 8 + 1 + 4 + 10 + 1 + 16 + 8 + 8 + 8 + 8 + 4 + 32);

assert.throws(() => canonicalSigningPreimage({
  chain_id: ATC_CHAIN_ID,
  tx_type: 0,
  sender_did: "ATC-sender",
  amount: 1n << 128n,
  gas_price: 0n,
  gas_limit: 0n,
  nonce: 0n,
  timestamp: 0n,
  payload: new Uint8Array(),
  poh_hash: new Uint8Array(32),
}));

assert.throws(() => canonicalSigningPreimage({
  chain_id: 1 as typeof ATC_CHAIN_ID,
  tx_type: 0,
  sender_did: "ATC-sender",
  amount: 0n,
  gas_price: 0n,
  gas_limit: 0n,
  nonce: 0n,
  timestamp: 0n,
  payload: new Uint8Array(),
  poh_hash: new Uint8Array(32),
}));
