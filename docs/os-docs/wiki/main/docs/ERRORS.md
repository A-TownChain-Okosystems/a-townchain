# Fehlerdefinitionen & Fehlercodes

## ATCLang VM — Laufzeitfehler

| Code | Name | Ursache | Lösung |
|------|------|---------|--------|
| `ATC-VM-001` | `StackOverflowError` | Call-Depth > 128 | Rekursion begrenzen |
| `ATC-VM-002` | `StackUnderflowError` | Pop auf leerem Stack | Compiler-Bug melden |
| `ATC-VM-003` | `ArithmeticOverflow` | Integer-Overflow | `safe_add/sub/mul` verwenden |
| `ATC-VM-004` | `DivisionByZero` | Division durch 0 | `require(divisor > 0)` |
| `ATC-VM-005` | `InvalidOpcode` | Unbekannter Bytecode | Compiler aktualisieren |
| `ATC-VM-006` | `GasExhausted` | Gas > 10.000.000 | Schleife optimieren |
| `ATC-VM-007` | `InvalidAddress` | Adresse ≠ ATC+32Hex | `is_valid_address()` prüfen |
| `ATC-VM-008` | `ReentrancyGuard` | Rekursiver Contract-Aufruf | emit nach State-Update |

## ATCLang Compiler — Parse-Fehler

| Code | Name | Ursache | Lösung |
|------|------|---------|--------|
| `ATC-PARSE-001` | `UnexpectedToken` | Syntax-Fehler | Syntax prüfen |
| `ATC-PARSE-002` | `UndeclaredVariable` | Variable nicht deklariert | `let x: Typ = Wert` |
| `ATC-PARSE-003` | `TypeMismatch` | Falscher Typ | Explizite Konversion |
| `ATC-PARSE-004` | `MissingReturn` | fn ohne return | `return wert` hinzufügen |
| `ATC-PARSE-005` | `FileTooLarge` | Datei > 1 MB | Datei aufteilen |
| `ATC-PARSE-006` | `NullByteDetected` | Null-Byte in Source | Datei bereinigen |

## Smart Contract — Require-Fehler

| Fehler-String | Bedeutung | Contract |
|--------------|-----------|---------|
| `"Nur Owner"` | caller ≠ self.owner | Alle Contracts |
| `"Contract pausiert"` | self.paused == true | Transfer-Contracts |
| `"Unzureichendes Guthaben"` | balance < amount | ATC-8300 |
| `"Max Supply erreicht"` | supply + amount > MAX | ATC-8300 Mint |
| `"Token nicht vorhanden"` | token_id ungültig | ATC-9000 |
| `"Nicht Token-Eigentümer"` | owner_of(id) ≠ caller | ATC-9000 Transfer |
| `"ReentrancyGuard: Kein rekursiver Aufruf"` | Reentrancy-Angriff | BaseContract |
| `"Bereits abgestimmt"` | voter hat proposal_id schon | Governance |
| `"Voting läuft noch"` | finalize vor Ende | Governance |
| `"Betrag zu groß"` | Tages-Bridge-Limit | Bridge |

## Gateway — HTTP-Fehler

| HTTP | Code | Bedeutung |
|------|------|-----------|
| 400 | `invalid_request` | Fehlende/falsche Parameter |
| 401 | `missing_api_key` | X-API-Key fehlt |
| 401 | `invalid_signature` | ECDSA-Signatur falsch |
| 403 | `forbidden` | Nicht berechtigt |
| 404 | `not_found` | Ressource existiert nicht |
| 409 | `nonce_too_low` | TX-Nonce bereits verwendet |
| 422 | `invalid_address` | Keine gültige ATC-Adresse |
| 429 | `rate_limit_exceeded` | Zu viele Requests |
| 500 | `internal_error` | Server-Fehler |

## P2P Netzwerk — Fehler

| Code | Name | Ursache | Lösung |
|------|------|---------|--------|
| `P2P-001` | `PeerTimeout` | Peer antwortet nicht in 120s | Peer entfernen |
| `P2P-002` | `RateLimitExceeded` | > 100 Msgs/60s von Peer | Peer vorübergehend bannen |
| `P2P-003` | `MessageTooLarge` | Nachricht > 64 KB | Nachricht splitten |
| `P2P-004` | `InvalidSignature` | ECDSA-Prüfung fehlgeschlagen | Block/TX verwerfen |
| `P2P-005` | `DuplicateBlock` | Block bereits gesehen | Ignorieren |
| `P2P-006` | `ForkDetected` | Zwei konkurrierende Blöcke | Longest-Chain Regel |

## Bekannte Bottlenecks & Lösungen

| Bottleneck | Problem | Lösung | Status |
|-----------|---------|--------|--------|
| VM Label-Lookup | War O(n) pro Sprung | `_label_cache` dict O(1) | ✅ Behoben |
| Orchestrator Retry | Unbegrenzte Wartezeit | Cap bei 5s | ✅ Behoben |
| P2P Broadcast | Kein Rate-Limit | 100/60s Limit | ✅ Behoben |
| Compiler Memory | Kein Source-Limit | MAX_SOURCE_SIZE=1MB | ✅ Behoben |
| BaseContract | Kein Reentrancy-Schutz | `_locked` Flag | ✅ Behoben |
| REPL os.system | Shell-Injection möglich | ANSI-Escape statt os.system | ✅ Behoben |
| Testnet Memory | Alle Nodes 1 Prozess | Docker-Isolation | ✅ Fix #18 |
| Bridge Replay | Keine TX-ID-Prüfung | `used_tx_ids` Set | ✅ Fix #10 |
| Governance Quorum | Kein Mindest-Stake | 10.000 ATC Min | ✅ Fix #9 |
