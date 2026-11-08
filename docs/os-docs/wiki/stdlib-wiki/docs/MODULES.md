# 📚 Standard Library Modules Reference

> **Repository:** [atc-stdlib](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-stdlib)  
> **Stand:** 2026-08-05  

---

## 📂 Verfügbare Module

### 1. `std::primitives`
- Types & Casting
- `to_int(val)`, `to_string(val)`, `is_null(val)`, `assert(cond, msg)`

### 2. `std::math`
- Math operations
- `abs(x)`, `pow(base, exp)`, `sqrt(x)`, `min(a, b)`, `max(a, b)`

### 3. `std::string`
- String utilities
- `length(s)`, `concat(s1, s2)`, `substring(s, start, len)`, `split(s, delim)`

### 4. `std::collections`
- Data structures
- `List::new()`, `Map::new()`, `Set::new()`, `Stack::new()`

### 5. `std::crypto`
- Hash & Signatures
- `sha256(data)`, `keccak256(data)`, `ed25519_verify(pubkey, sig, msg)`

### 6. `std::chain`
- Blockchain State APIs
- `get_block_height()`, `get_balance(addr)`, `emit_event(name, payload)`

### 7. `std::wallet`
- Wallet & Keys
- `get_address()`, `sign_message(data)`
