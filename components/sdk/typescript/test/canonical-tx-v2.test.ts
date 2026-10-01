import { strict as assert } from "node:assert";
import { assertCanonicalTxV2Vector, bytesToHex, canonicalSigningPreimage, CANONICAL_TX_V2_VECTOR, ATC_CHAIN_ID } from "../chain-identity.ts";

assertCanonicalTxV2Vector();

const preimage = canonicalSigningPreimage({
  chain_id: ATC_CHAIN_ID, tx_type: 0, sender_did: "ATC-sender", recipient_did: "ATC-recipient",
  amount: (1n << 128n) - 1n, gas_price: 0n, gas_limit: 0n, nonce: 0n, timestamp: 0n,
  payload: new Uint8Array(), poh_hash: new Uint8Array(32),
});

assert.equal(preimage.length, 146);
assert.equal(bytesToHex(preimage.slice(43, 59)), "ffffffffffffffffffffffffffffffff");
assert.equal(CANONICAL_TX_V2_VECTOR.sha256Hex.length, 64);
