# ATC-93 — ATCLang VM Bytecode Specification v1.0

**Standard:** ATC-93  
**Version:** 1.0.0  
**Datum:** 2026-07-05  
**Status:** STABLE  

---

## 1. Übersicht

Die ATCVM ist eine Stack-basierte virtuelle Maschine für ATCLang-Bytecode. Sie führt kompilierten ATCLang-Code deterministisch auf jeder Node aus.

### Eigenschaften
- **Stack-basiert:** Operanden auf Stack, Results zurück auf Stack
- **80+ Op-Codes:** Arithmetik, Logik, Crypto, Blockchain, Netzwerk
- **Gas-System:** Jeder Op-Code hat definierte Gas-Kosten
- **Deterministisch:** Gleicher Code + State → gleiches Resultat

---

## 2. Op-Code Referenz

### 2.1 Stack-Operationen (Gas: 1)

| Op-Code | Stack-Effekt | Beschreibung |
|---------|-------------|-------------|
| `PUSH <val>` | [] → [val] | Wert auf Stack legen |
| `POP` | [a] → [] | Oberstes Element entfernen |
| `DUP` | [a] → [a, a] | Oberstes duplizieren |
| `SWAP` | [a, b] → [b, a] | Oberste zwei tauschen |
| `ROT` | [a, b, c] → [b, c, a] | Oberste drei rotieren |

### 2.2 Arithmetik (Gas: 3)

| Op-Code | Stack-Effekt | Beschreibung |
|---------|-------------|-------------|
| `ADD` | [a, b] → [a+b] | Addition |
| `SUB` | [a, b] → [a-b] | Subtraktion |
| `MUL` | [a, b] → [a*b] | Multiplikation |
| `DIV` | [a, b] → [a/b] | Division |
| `MOD` | [a, b] → [a%b] | Modulo |
| `POW` | [a, b] → [a**b] | Potenz |
| `NEG` | [a] → [-a] | Negation |
| `BITAND` | [a, b] → [a&b] | Bitweises AND |
| `BITOR` | [a, b] → [a|b] | Bitweises OR |
| `BITXOR` | [a, b] → [a^b] | Bitweises XOR |
| `BITNOT` | [a] → [~a] | Bitweises NOT |
| `SHL` | [a, b] → [a<<b] | Shift Left |
| `SHR` | [a, b] → [a>>b] | Shift Right |

### 2.3 Vergleiche (Gas: 2)

| Op-Code | Stack-Effekt | Beschreibung |
|---------|-------------|-------------|
| `EQ` | [a, b] → [a==b] | Gleich |
| `NEQ` | [a, b] → [a!=b] | Ungleich |
| `LT` | [a, b] → [a<b] | Kleiner |
| `GT` | [a, b] → [a>b] | Größer |
| `LTE` | [a, b] → [a<=b] | Kleiner-Gleich |
| `GTE` | [a, b] → [a>=b] | Größer-Gleich |

### 2.4 Logik (Gas: 2)

| Op-Code | Stack-Effekt | Beschreibung |
|---------|-------------|-------------|
| `AND` | [a, b] → [a&&b] | Logisches AND |
| `OR` | [a, b] → [a||b] | Logisches OR |
| `NOT` | [a] → [!a] | Logisches NOT |

### 2.5 Variablen (Gas: 1-3)

| Op-Code | Stack-Effekt | Gas | Beschreibung |
|---------|-------------|-----|-------------|
| `LOAD <name>` | [] → [val] | 1 | Lokale Variable |
| `STORE <name>` | [val] → [] | 1 | Lokale Variable setzen |
| `LOAD_IDX` | [obj, key] → [obj[key]] | 3 | Index-Zugriff |
| `STORE_IDX` | [obj, key, val] → [] | 3 | Index-Zuweisung |
| `LOAD_GLOBAL` | [] → [val] | 2 | Globale Variable |
| `STORE_GLOBAL` | [val] → [] | 2 | Globale Variable setzen |
| `DEL_VAR` | [name] → [] | 1 | Variable löschen |

### 2.6 Sprünge (Gas: 1)

| Op-Code | Beschreibung |
|---------|-------------|
| `JUMP <addr>` | Unbedingter Sprung |
| `JUMP_IF <addr>` | Sprung wenn Stack-Top true |
| `JUMP_NOT <addr>` | Sprung wenn Stack-Top false |

### 2.7 Funktionen (Gas: 5-10)

| Op-Code | Beschreibung |
|---------|-------------|
| `CALL <name> <argc>` | Funktion aufrufen |
| `RETURN` | Funktion verlassen, Result auf Stack |
| `CALL_EXT` | ATC:: Stdlib-Aufruf |
| `CALL_METHOD` | obj.method(args) |
| `MAKE_FN` | Closure erstellen |

### 2.8 Objekte & Collections (Gas: 3-10)

| Op-Code | Beschreibung |
|---------|-------------|
| `NEW_MAP` | Leere Map erstellen |
| `NEW_LIST` | Leere Liste erstellen |
| `NEW_OBJ` | Struct/Contract-Instanz |
| `GET_FIELD` / `SET_FIELD` | Feld-Zugriff |
| `HAS_KEY` | key in map |
| `DEL_KEY` | del map[key] |
| `LIST_PUSH` / `LIST_POP` | List-Operationen |
| `LIST_LEN` | Listenlänge |
| `MAP_KEYS` / `MAP_VALUES` / `MAP_ITEMS` | Map-Views |
| `CONTAINS` | Element in Collection |
| `CAST` | Typ-Konvertierung |

### 2.9 String-Operationen (Gas: 1-5)

| Op-Code | Beschreibung |
|---------|-------------|
| `STR_LEN` | Stringlänge |
| `STR_SLICE` | Teilstring |
| `STR_UPPER` / `STR_LOWER` | Groß-/Kleinschreibung |
| `STR_SPLIT` | String teilen |
| `STR_JOIN` | Strings verbinden |
| `STR_FORMAT` | Format-String |

### 2.10 Blockchain-Op-Codes (Gas: 10-100)

| Op-Code | Gas | Beschreibung |
|---------|-----|-------------|
| `EMIT` | 10 | Event auslösen |
| `REQUIRE` | 1 | Assertion |
| `TRANSFER` | 50 | ATC Token-Transfer |
| `MINT` | 100 | Token minten |
| `BURN` | 50 | Token burnen |
| `STAKE` / `UNSTAKE` | 50 | Staking |
| `VOTE` | 20 | Governance-Vote |

### 2.11 Crypto / Hash (Gas: 20-50)

| Op-Code | Gas | Beschreibung |
|---------|-----|-------------|
| `HASH_SHA256` | 20 | SHA-256 Hash |
| `HASH_SHA3` | 30 | SHA3-256 Hash |
| `HASH_SHA3_ATC` | 30 | ATC-spezifischer SHA3 |
| `CRYPTO_SIGN` | 50 | Signatur erstellen |
| `CRYPTO_VERIFY` | 50 | Signatur verifizieren |
| `RAND_BYTES` | 10 | Zufallsbytes |
| `RAND_INT` | 5 | Zufallsinteger |
| `LEADING_ZEROS` | 5 | Leading-Zero-Count (Mining) |

### 2.12 Netzwerk / P2P (Gas: 10-50)

| Op-Code | Beschreibung |
|---------|-------------|
| `NET_SEND` | Nachricht senden |
| `NET_RECV` | Nachricht empfangen |
| `NET_BROADCAST` | An alle Peers |
| `NET_CONNECT` | Peer verbinden |
| `NET_PEERS` | Peer-Liste |

### 2.13 Speicher / Persistenz (Gas: 5-50)

| Op-Code | Beschreibung |
|---------|-------------|
| `STORE_PERSIST` | Auf Chain schreiben |
| `LOAD_PERSIST` | Von Chain laden |
| `FS_WRITE` / `FS_READ` | Dateisystem |
| `FS_MKDIR` | Verzeichnis erstellen |

### 2.14 Concurrency / Async (Gas: 10-20)

| Op-Code | Beschreibung |
|---------|-------------|
| `ASYNC_CALL` | Asynchroner Aufruf |
| `AWAIT` | Auf Result warten |
| `SPAWN` | Kernel-Prozess starten |
| `CHANNEL_SEND` / `CHANNEL_RECV` | Channel-Kommunikation |

### 2.15 System (Gas: 0-5)

| Op-Code | Beschreibung |
|---------|-------------|
| `HALT` | Ausführung stoppen |
| `NOP` | No-Operation |
| `PRINT` | Debug-Ausgabe |
| `ASSERT` | Assertion |
| `DEBUG` | Stack-Dump |
| `GAS_CHECK` | Gas explizit prüfen |
| `TIMESTAMP` | block.timestamp |
| `BLOCK_NUM` | block.number |
| `CALLER` | caller-Adresse |
| `LOG` | Strukturiertes Logging |

---

## 3. Instruction-Format

```
Instruction {
    op: OP,           # Op-Code
    arg: Any | None,  # Optionaler Operand
    line: int,        # Quell-Zeile (Debug)
    gas: int,         # Gas-Kosten
}
```

---

## 4. Call-Frame

```
CallFrame {
    instructions: List[Instruction],
    locals: Dict[str, Any],
    stack: List[Any],
    pc: int,              # Program Counter
    gas: int,             # Verbleibendes Gas
    return_stack: List[int],
}
```

---

## 5. Gas-Modell

- **Default Gas:** 1.000.000 pro Transaktion
- **Min Gas:** 21.000 (Basis-Overhead)
- **Out-of-Gas:** Transaktion wird reverted
- **Gas-Refund:** Unverbrauchtes Gas wird zurückerstattet

---

*ATC-93 — ATCLang VM Bytecode Spec v1.0 — 2026-07-05*
