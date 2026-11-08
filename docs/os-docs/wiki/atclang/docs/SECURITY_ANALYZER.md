# ATCLang Security Analyzer v1.0.0

## Übersicht
Der ATCLang Security Analyzer ist ein statisches Analyse-Tool für `.atc` Quellcode-Dateien.
Er implementiert den **ATC-SEC-0001 Standard** und prüft 15 Sicherheitsregeln.

## 15 Checks
| Code | Schweregrad | Trigger |
|------|------------|---------|
| ATC-SEC-001 | 🟠 HIGH | `self.balance + amount` ohne safe_add |
| ATC-SEC-002 | 🟠 HIGH | `fn transfer()` ohne `require(caller ==` |
| ATC-SEC-003 | 🟡 MEDIUM | `transfer()` ohne let-Zuweisung |
| ATC-SEC-004 | 🟡 MEDIUM | Hardcoded `"ATC..."` Adresse |
| ATC-SEC-005 | 🟠 HIGH | `emit X` gefolgt von `self.y =` |
| ATC-SEC-006 | 🟡 MEDIUM | `/ count` ohne `require(count > 0)` |
| ATC-SEC-007 | 🟠 HIGH | `tx_origin` in Source |
| ATC-SEC-008 | 🔴 CRITICAL | `random()` ohne Blockchain-Entropy |
| ATC-SEC-009 | 🔴 CRITICAL | `self.owner =` ohne require() |
| ATC-SEC-010 | 🟡 MEDIUM | Address-Parameter ohne `is_valid_address` |
| ATC-SEC-011 | 🔴 CRITICAL | `terminate()` ohne require() |
| ATC-SEC-012 | 🟢 LOW | `fn transfer` ohne `!self.paused` |
| ATC-SEC-013 | 🟡 MEDIUM | Kein `state owner:` definiert |
| ATC-SEC-014 | 🟡 MEDIUM | Kein `fn init(` vorhanden |
| ATC-SEC-015 | 🟠 HIGH | Arithmetik ohne safe_add/sub global |
| ATC-PERF-001 | 🟢 LOW | `for x in self.collection` ohne Limit |

## Demo: Unsicherer vs. sicherer Contract

### ❌ Unsicher (7 Issues erkannt)
```atclang
contract VulnerableBank {
    state balances: Map<Address, u128>

    fn deposit(amount: u128) {
        self.balances[caller] = self.balances[caller] + amount  // SEC-001
        emit Deposit(caller, amount)
    }

    fn withdraw(to: Address, amount: u128) {  // SEC-002: kein require
        emit Withdrawal(to, amount)            // SEC-005: emit vor state
        self.balances[caller] = self.balances[caller] - amount
    }

    fn set_owner(new_owner: Address) {
        self.owner = new_owner  // SEC-009: kritisch!
    }

    fn roll_dice() -> u64 {
        return random(6)        // SEC-008: kritisch!
    }
}
```

### ✅ Sicher (0 CRITICAL/HIGH Issues)
```atclang
contract SafeBank {
    state owner:   Address
    state paused:  bool = false
    state balances: Map<Address, u128>

    fn init(owner_addr: Address) {
        self.owner = owner_addr
        emit Initialized(owner_addr)
    }

    fn deposit(amount: u128) {
        require(!self.paused, "Pausiert")
        require(amount > 0, "Betrag > 0")
        self.balances[caller] = safe_add(self.balances[caller], amount)
        emit Deposit(caller, amount)
    }

    fn withdraw(to: Address, amount: u128) {
        require(caller == self.owner, "Nur Owner")
        require(!self.paused, "Pausiert")
        require(is_valid_address(to_string(to)), "Ungültige Adresse")
        require(self.balances[caller] >= amount, "Unzureichend")
        self.balances[caller] = safe_sub(self.balances[caller], amount)
        emit Withdrawal(to, amount)
    }
}
```
