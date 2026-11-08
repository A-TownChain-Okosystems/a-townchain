# 🏗️ Architektur — atc-wallet

Das Wallet verarbeitet Kryptographie, Schlüsselgenerierung und Signatur-Pipelines.

## HD-Wallet Derivation

Pfade nach BIP44 Standard:
`m / 44' / 8300' / account' / change / address_index`

## Kryptographie

- **Signaturalgorithmus**: Ed25519 (RFC 8032)
- **Hashfunktion**: SHA-256 / SHA3-256
- **Verschlüsselung**: AES-256-GCM für lokale Keys
