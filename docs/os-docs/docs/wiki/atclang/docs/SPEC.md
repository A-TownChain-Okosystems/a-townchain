# ATCLang Sprachspezifikation v0.2.0

## Basis-Typen
| Typ | Bits | Bereich |
|-----|------|---------|
| `u8` | 8 | 0 – 255 |
| `u16` | 16 | 0 – 65.535 |
| `u32` | 32 | 0 – 4.294.967.295 |
| `u64` | 64 | 0 – 18,4 · 10¹⁸ |
| `u128` | 128 | 0 – 3,4 · 10³⁸ |
| `bool` | — | true / false |
| `string` | — | UTF-8, max 64 KB |
| `bytes32` | 256 bit | Hash-Werte |
| `Address` | 35 Zeichen | ATC + 32 hex |

## Contract-Syntax
```atclang
use ATC::Types::Address
use ATC::Crypto::sha256

enum Status { ACTIVE, PAUSED }

struct Data {
    id:    u64,
    owner: Address,
    ts:    u64,
}

contract MyContract {
    state owner:   Address
    state counter: u64 = 0
    state data:    Map<u64, Data>

    fn init(owner_addr: Address) {
        self.owner = owner_addr
        emit Initialized(owner_addr)
    }

    fn increment() {
        require(caller == self.owner, "Nur Owner")
        self.counter = safe_add(self.counter, 1)
        emit Incremented(caller, self.counter)
    }

    fn get() -> u64 { return self.counter }
}
```

## Sicherheits-Primitiven
- `safe_add(a, b)` — Overflow-geschützte Addition
- `safe_sub(a, b)` — Underflow-geschützte Subtraktion
- `safe_mul(a, b)` — Overflow-geschützte Multiplikation
- `require(cond, msg)` — Guard-Assertion (revert bei false)
- `is_valid_address(s)` — ATC-Adress-Validierung
- `caller` — Authentifizierter Aufrufer (unveränderlich)
