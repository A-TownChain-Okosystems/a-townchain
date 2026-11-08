# 🔒 ATCLang Security Modell

## Übersicht der Schutzmechanismen

### Integer-Overflow-Schutz
ATCLang erzwingt `safe_add()`, `safe_sub()`, `safe_mul()` für alle Berechnungen.
Direktes `+`, `-`, `*` auf Blockchain-Token-Beträgen ist verboten.

### Reentrancy-Schutz
VM ist single-threaded. Kein Re-Entrancy möglich.

### Access Control
`require(caller == self.owner, ...)` — strenge Caller-Validierung.

### Gas-Limit
Max 10.000.000 Gas/TX. Verhindert DoS durch Endlosschleifen.

### Typ-Sicherheit
Statisch typisiert. `u64`, `u128`, `u256` — kein implizites Casting.

### ECDSA-Signatur
Alle TXs: secp256k1, SHA3-256. Replay-Schutz via Nonce + Chain-ID 9000.

### Adress-Validierung
`is_valid_address()` — Pflicht für alle externen Adressen.

## Bekannte Risiken (unter Bearbeitung)
- Front-Running: Wird durch PoH-Zeitstempel gemildert
- Oracle-Manipulation: Dezentrale Preis-Feeds in v2.2
