# 🔒 ATCLang Security — Sicherheitsmodell

## ATCLang Hack-Schutz

### 1. Integer Overflow/Underflow — GESCHÜTZT
```atclang
// UNSICHER (verboten in ATCLang):
let x: u64 = a + b   // Direkte Addition → Overflow möglich

// SICHER (erzwungen):
let x: u64 = safe_add(a, b)   // Wirft OverflowError bei Überlauf
let y: u64 = safe_sub(a, b)   // Wirft ValueError bei Unterlauf
let z: u64 = safe_mul(a, b)   // Wirft OverflowError
```

### 2. Reentrancy — GESCHÜTZT
ATCLang VM ist single-threaded — kein paralleler Contract-Aufruf möglich.
State-Updates passieren IMMER vor externen Calls.

### 3. Access Control — ERZWUNGEN
```atclang
fn admin_action() {
    require(caller == self.owner, "Nur Owner")
    // Ohne require() → automatischer Revert
}
```

### 4. Address Validierung — AUTOMATISCH
```atclang
require(is_valid_address(to_string(addr)), "Ungültige ATC-Adresse")
// ATC-Adressen müssen: ATC + 32 Zeichen = 35 Zeichen gesamt
```

### 5. Gas-Limit — ERZWUNGEN
Jeder Opcode verbraucht Gas. Endlos-Schleifen terminieren automatisch.
Max Gas: 10.000.000 pro TX.

### 6. Integer Types — STRIKT
```atclang
let x: u64  = 100        // 0 bis 2^64-1
let y: u128 = 10_000     // 0 bis 2^128-1 (für Token-Beträge)
// Typen werden zur Compile-Zeit geprüft
```

### 7. Immutable State nach Deployment
Contract-Bytecode ist nach Deployment unveränderlich.
Nur State-Variablen können über Funktionen geändert werden.

### 8. Keine Delegate-Calls
ATCLang hat kein delegatecall equivalent — verhindert Proxy-Angriffe.

### 9. SHA3-ATC Hashing
Alle Hashes verwenden SHA3-256 (nicht SHA-256) — resistenter gegen Preimage-Angriffe.

### 10. ECDSA secp256k1 Signaturen
Alle Transaktionen müssen valide ECDSA-Signaturen haben.
Replay-Schutz: eindeutiger Nonce pro TX + Chain-ID 9000.
