# ATCLang & System-Syntax — Übersicht

## ATCLang Contract-Syntax

### Basis-Struktur
```atclang
use ATC::Types::Address
use ATC::Crypto::sha256

contract MeinContract {
    // State-Variablen
    state owner:  Address
    state paused: bool = false
    state value:  u128 = 0

    // Events
    event ValueSet(by: Address, val: u128)
    event Paused(by: Address)

    // Constructor
    fn init(owner_addr: Address) {
        self.owner = owner_addr
        emit Initialized(owner_addr)
    }

    // Öffentliche Funktion
    fn set_value(val: u128) {
        require(caller == self.owner, "Nur Owner")
        require(!self.paused, "Pausiert")
        require(val > 0, "Muss > 0 sein")
        self.value = val
        emit ValueSet(caller, val)
    }

    // View-Funktion (kein State-Change)
    fn get_value() -> u128 {
        return self.value
    }

    fn pause() {
        require(caller == self.owner, "Nur Owner")
        self.paused = true
        emit Paused(caller)
    }
}
```

### Typen
| Typ | Bits | Bereich | Beispiel |
|-----|------|---------|---------|
| `u8` | 8 | 0–255 | `let x: u8 = 100` |
| `u16` | 16 | 0–65.535 | `let port: u16 = 4000` |
| `u32` | 32 | 0–4,29 Mrd | `let n: u32 = 42` |
| `u64` | 64 | 0–18,4 · 10¹⁸ | `let ts: u64 = 0` |
| `u128` | 128 | 0–3,4 · 10³⁸ | `let bal: u128 = 0` |
| `bool` | 1 | true/false | `let ok: bool = true` |
| `string` | var | UTF-8 max 64KB | `let s: string = "ATC"` |
| `bytes32` | 256 | Hash/Sig | `let h: bytes32` |
| `Address` | 35 | ATC+32Hex | `let a: Address` |
| `Map<K,V>` | var | Hash-Map | `state m: Map<Address, u128>` |
| `Vec<T>` | var | Dyn. Array | `state v: Vec<u64>` |

### Kontrollfluss
```atclang
// if/else
if condition {
    ...
} else {
    ...
}

// for-Schleife
for item in self.items {
    ...
}

// require (Sicherheits-Assertion)
require(caller == self.owner, "Nicht berechtigt")
require(amount > 0,           "Betrag muss > 0")
require(!self.paused,         "Contract pausiert")

// return
fn get() -> u128 {
    return self.value
}
```

### Stdlib Built-ins
```atclang
// Sichere Arithmetik (kein Overflow)
safe_add(a: u128, b: u128) -> u128
safe_sub(a: u128, b: u128) -> u128
safe_mul(a: u128, b: u128) -> u128

// Kryptographie
sha256(data: string) -> bytes32
keccak256(data: string) -> bytes32

// Validierung
is_valid_address(addr: string) -> bool
to_string(val: u128) -> string
to_u128(s: string) -> u128

// Block-Kontext
caller   -> Address   // aufrufende Adresse
block_height -> u64   // aktuelle Block-Höhe
timestamp    -> u64   // Unix-Timestamp ms
chain_id     -> u64   // 9000
```

## ShivaOS Kernel-Syntax (Python API)
```python
from shivaos.kernel import ShivaKernel, ProcessType
kernel = ShivaKernel()
kernel.start()
pid = kernel.spawn("my_service", ProcessType.SERVICE, fn)
kernel.kill(pid)
```

## API-Syntax (REST)
```
GET  /api/<resource>           Liste
GET  /api/<resource>/:id       Einzeln
POST /api/<resource>           Erstellen
PUT  /api/<resource>/:id       Aktualisieren
DEL  /api/<resource>/:id       Löschen

Header:
  X-API-Key:    <32+ Zeichen>
  X-Signature:  <ECDSA secp256k1 Signatur>
  X-Public-Key: <64 Hex>
  X-Nonce:      <monoton steigend>
```
