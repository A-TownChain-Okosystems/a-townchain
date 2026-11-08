# ATCLang — Vollständige Syntax-Referenz

## Schlüsselwörter
```
contract  state    fn       return   if      else
for       in       let      emit     require  use
true      false    self     caller   async    await
import    export   match    struct   event    error
```

## Operatoren
```
+  -  *  /  %          Arithmetik
== != <  >  <= >=       Vergleich
&&  ||  !               Logik
=   +=  -=  *=  /=      Zuweisung
->                      Return-Typ
::                      Namespace
```

## Vollständige Contract-Vorlage
```atclang
use ATC::Types::Address
use ATC::Crypto::sha256
use ATC::Stdlib::safe_math

/// @title SafeToken — ATC-8300 konformes Token
/// @version 1.0.0
/// @author ShivaCore
contract SafeToken {

    // ── State ──────────────────────────────────────
    state owner:         Address
    state paused:        bool    = false
    state name_:         string  = "SafeToken"
    state symbol_:       string  = "STK"
    state decimals_:     u8      = 18
    state total_supply_: u128    = 0
    state MAX_SUPPLY:    u128    = 21_000_000_000_000_000_000_000_000

    state balances:      Map<Address, u128>
    state allowances:    Map<Address, Map<Address, u128>>

    // ── Events ─────────────────────────────────────
    event Transfer(from: Address, to: Address, amount: u128)
    event Approval(owner: Address, spender: Address, amount: u128)
    event Mint(to: Address, amount: u128)
    event Burn(from: Address, amount: u128)
    event Paused(by: Address)
    event Unpaused(by: Address)
    event Initialized(owner: Address)

    // ── Constructor ────────────────────────────────
    fn init(owner_addr: Address) {
        require(is_valid_address(to_string(owner_addr)), "Ungültige Adresse")
        self.owner = owner_addr
        emit Initialized(owner_addr)
    }

    // ── Views ──────────────────────────────────────
    fn name()         -> string { return self.name_ }
    fn symbol()       -> string { return self.symbol_ }
    fn decimals()     -> u8     { return self.decimals_ }
    fn total_supply() -> u128   { return self.total_supply_ }

    fn balance_of(addr: Address) -> u128 {
        return self.balances[addr]
    }

    fn allowance(owner: Address, spender: Address) -> u128 {
        return self.allowances[owner][spender]
    }

    // ── Transfers ──────────────────────────────────
    fn transfer(to: Address, amount: u128) -> bool {
        require(!self.paused,                              "Pausiert")
        require(is_valid_address(to_string(to)),           "Ungültige Adresse")
        require(self.balances[caller] >= amount,           "Unzureichend")
        require(amount > 0,                                "Betrag > 0")
        self.balances[caller] = safe_sub(self.balances[caller], amount)
        self.balances[to]     = safe_add(self.balances[to], amount)
        emit Transfer(caller, to, amount)
        return true
    }

    fn approve(spender: Address, amount: u128) -> bool {
        self.allowances[caller][spender] = amount
        emit Approval(caller, spender, amount)
        return true
    }

    fn transfer_from(from_: Address, to: Address, amount: u128) -> bool {
        require(!self.paused,                                    "Pausiert")
        require(self.allowances[from_][caller] >= amount,        "Nicht genehmigt")
        require(self.balances[from_] >= amount,                  "Unzureichend")
        self.allowances[from_][caller] = safe_sub(self.allowances[from_][caller], amount)
        self.balances[from_]           = safe_sub(self.balances[from_], amount)
        self.balances[to]              = safe_add(self.balances[to], amount)
        emit Transfer(from_, to, amount)
        return true
    }

    // ── Mint / Burn ────────────────────────────────
    fn mint(to: Address, amount: u128) -> bool {
        require(caller == self.owner,                            "Nur Owner")
        require(safe_add(self.total_supply_, amount) <= self.MAX_SUPPLY, "Max Supply")
        self.balances[to]   = safe_add(self.balances[to], amount)
        self.total_supply_  = safe_add(self.total_supply_, amount)
        emit Mint(to, amount)
        return true
    }

    fn burn(amount: u128) -> bool {
        require(self.balances[caller] >= amount, "Unzureichend")
        self.balances[caller] = safe_sub(self.balances[caller], amount)
        self.total_supply_    = safe_sub(self.total_supply_, amount)
        emit Burn(caller, amount)
        return true
    }

    // ── Admin ──────────────────────────────────────
    fn pause()   { require(caller == self.owner, "Nur Owner"); self.paused = true;  emit Paused(caller) }
    fn unpause() { require(caller == self.owner, "Nur Owner"); self.paused = false; emit Unpaused(caller) }
}
```

## VM-Opcodes (30+)
| Opcode | Stack-Effekt | Beschreibung |
|--------|-------------|-------------|
| `PUSH x` | → x | Wert auf Stack |
| `POP` | x → | Wert entfernen |
| `ADD` | a b → a+b | Addition |
| `SUB` | a b → a-b | Subtraktion |
| `MUL` | a b → a*b | Multiplikation |
| `DIV` | a b → a/b | Division |
| `MOD` | a b → a%b | Modulo |
| `EQ` | a b → bool | Gleichheit |
| `LT` | a b → bool | Kleiner als |
| `GT` | a b → bool | Größer als |
| `AND` | a b → bool | Logisches UND |
| `OR` | a b → bool | Logisches ODER |
| `NOT` | a → !a | Negation |
| `JUMP lbl` | → | Unbedingter Sprung |
| `JUMPI lbl` | cond → | Bedingter Sprung |
| `CALL fn` | args → ret | Funktionsaufruf |
| `RETURN` | val → | Rückgabe |
| `LOAD var` | → val | Variable laden |
| `STORE var` | val → | Variable speichern |
| `EMIT ev` | args → | Event auslösen |
| `REQUIRE` | cond msg → | Assertion |
| `SHA256` | data → hash | Hashing |
| `ADDR_CHECK` | addr → bool | Adresse prüfen |
| `SAFE_ADD` | a b → c | Overflow-sichere Addition |
| `SAFE_SUB` | a b → c | Overflow-sichere Subtraktion |
| `SAFE_MUL` | a b → c | Overflow-sichere Multiplikation |
| `MAP_GET` | map key → val | Map-Zugriff |
| `MAP_SET` | map key val → | Map-Update |
| `VEC_PUSH` | vec val → | Vec hinzufügen |
| `VEC_LEN` | vec → len | Vec-Länge |
