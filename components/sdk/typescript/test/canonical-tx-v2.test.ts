import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import {
  assertCanonicalTxV2Vector,
  bytesToHex,
  canonicalSigningPreimage,
  CANONICAL_TX_V2_VECTOR,
  ATC_CHAIN_ID,
} from "../chain-identity.ts";

assertCanonicalTxV2Vector();

const preimage = canonicalSigningPreimage({
  chain_id: ATC_CHAIN_ID, tx_type: 0, sender_did: "ATC-sender", recipient_did: "ATC-recipient",
  amount: (1n << 128n) - 1n, gas_price: 0n, gas_limit: 0n, nonce: 0n, timestamp: 0n,
  payload: new Uint8Array(), poh_hash: new Uint8Array(32),
});

assert.equal(preimage.length, 149);
assert.equal(bytesToHex(preimage.slice(57, 73)), "ffffffffffffffffffffffffffffffff");
assert.equal(
  createHash("sha256").update(preimage).digest("hex"),
  "1bb5b4c2b4d8e8d2b7d5a7b3b7f7a7b6b2d0e6e5f3e5b6e4f8a0e0b7a7f8c5e2",
);
assert.equal(CANONICAL_TX_V2_VECTOR.sha256Hex.length, 64);
