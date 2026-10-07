import { test } from "node:test";
import assert from "node:assert/strict";
import { ATC_CHAIN_ID, ATC_TX_DOMAIN_V2, ATC_TX_ID_V2, canonicalSigningPreimage, canonicalTransactionId } from "../chain-identity.ts";

const tx = (amount: bigint) => ({
  chain_id: ATC_CHAIN_ID,
  tx_type: 0,
  sender_did: "ATC-sender",
  recipient_did: "ATC-recipient",
  amount,
  gas_price: 1n,
  gas_limit: 1000n,
  nonce: 7n,
  timestamp: 1700000000n,
  payload: new TextEncoder().encode("hello"),
  poh_hash: new Uint8Array(32).fill(9),
});

test("canonical V2 preimage matches Rust reference vector", () => {
  const hex = Buffer.from(canonicalSigningPreimage(tx(100n))).toString("hex");
  assert.equal(
    hex,
    "4154432d54582d444f4d41494e2d563200000000000a0c23000000000a4154432d73656e646572010000000d4154432d726563697069656e7400000000000000000000000000000064000000000000000100000000000003e80000000000000007000000006553f1000000000568656c6c6f0909090909090909090909090909090909090909090909090909090909090909",
  );
});

test("amount uses fixed-width u128 encoding", () => {
  const bytes = canonicalSigningPreimage(tx(2n ** 128n - 1n));
  const amountOffset = new TextEncoder().encode(ATC_TX_DOMAIN_V2).length + 8 + 1 + 4 + 10 + 1 + 4 + 13;
  assert.deepEqual(Array.from(bytes.slice(amountOffset, amountOffset + 16)), new Array(16).fill(255));
});

test("u128 overflow is rejected", () => {
  assert.throws(() => canonicalSigningPreimage(tx(2n ** 128n)), /u128 out of range/);
});

test("canonical V2 transaction ID is byte-stable and signature-independent", () => {
  assert.equal(
    Buffer.from(canonicalTransactionId(tx(100n))).toString("hex"),
    "043ebe7545f44320394af8211da323afe8d27fe9e99bff5da3aafd2e5c99b9dc",
  );
  assert.equal(ATC_TX_ID_V2, "ATC-TX-ID-V2");
});
