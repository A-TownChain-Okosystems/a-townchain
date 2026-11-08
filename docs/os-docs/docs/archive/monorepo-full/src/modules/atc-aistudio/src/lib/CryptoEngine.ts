// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import * as ed from '@noble/ed25519';
//@ts-ignore
import { sha512 } from '@noble/hashes/sha2.js';

// noble/ed25519 v3 uses hashes for sha512 sync implementation
// @ts-ignore
ed.hashes.sha512 = sha512;
// @ts-ignore
ed.hashes.sha512Async = (...m) => Promise.resolve(sha512(ed.etc.concatBytes(...m)));

export function toHex(bytes: Uint8Array): string {
  return Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
}

export const CryptoEngine = {
  generate_keypair: async () => {
    // noble/ed25519 v3 uses randomSecretKey
    // @ts-ignore
    const privKey = ed.utils.randomSecretKey ? ed.utils.randomSecretKey() : ed.utils.randomPrivateKey();
    const pubKey = await ed.getPublicKeyAsync(privKey);
    return {
      privateKey: privKey as Uint8Array,
      publicKey: pubKey as Uint8Array,
      privateHex: toHex(privKey),
      publicHex: toHex(pubKey)
    };
  },
  sign_message: async (message: string, privateKey: Uint8Array) => {
    const msgBytes = new TextEncoder().encode(message);
    const sig = await ed.signAsync(msgBytes, privateKey);
    return {
      signature: sig,
      signatureHex: toHex(sig)
    };
  },
  verify_message: async (signatureHex: string, message: string, publicKey: Uint8Array) => {
    const msgBytes = new TextEncoder().encode(message);
    const sigBytes = ed.etc.hexToBytes(signatureHex);
    return await ed.verifyAsync(sigBytes, msgBytes, publicKey);
  }
};
