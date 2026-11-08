# Fehlerlösungen & Debugging-Guide

## Häufige Fehler + Lösungen

### 1. `ATC-VM-001 StackOverflowError`
```
Fehler: Call-Depth > MAX_CALL_DEPTH (128)
Symptom: RuntimeError bei rekursivem Vertrag
```
**Lösung:**
```atclang
// ❌ Problematisch: unbegrenzte Rekursion
fn factorial(n: u64) -> u64 {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)  // Stack-Overflow bei n > 128
}

// ✅ Lösung: iterativ
fn factorial(n: u64) -> u64 {
    let result: u64 = 1
    let i: u64 = 1
    for i in range(1, n) {
        result = safe_mul(u128(result), u128(i))
    }
    return result
}
```

### 2. `ATC-SEC-005 emit() vor State-Update`
```
Fehler: Reentrancy-Angriff möglich
Symptom: ATCLang Security Analyzer meldet HIGH
```
**Lösung:**
```atclang
// ❌ Unsicher: emit vor State-Change
fn withdraw(amount: u128) {
    emit Withdrawal(caller, amount)         // ← GEFÄHRLICH
    self.balances[caller] -= amount
}

// ✅ Sicher: erst State ändern, dann emit
fn withdraw(amount: u128) {
    require(self.balances[caller] >= amount, "Unzureichend")
    self.balances[caller] = safe_sub(self.balances[caller], amount)
    emit Withdrawal(caller, amount)         // ← SICHER
}
```

### 3. Gateway 429 Rate-Limit
```
Fehler: HTTP 429 Too Many Requests
Symptom: Requests werden blockiert
```
**Lösung:**
```python
import time

def send_with_retry(url, payload, max_retries=3):
    for attempt in range(max_retries):
        r = requests.post(url, json=payload, headers=headers)
        if r.status_code == 429:
            retry_after = int(r.headers.get('X-RateLimit-Reset', 60))
            time.sleep(min(retry_after, 5))  # max 5s warten
            continue
        return r
    raise Exception("Rate-Limit nach 3 Versuchen")
```

### 4. P2P-004 InvalidSignature
```
Fehler: Block/TX-Signatur ungültig
Symptom: Blocks werden vom Netzwerk abgelehnt
```
**Lösung:**
```python
# Signatur korrekt erstellen:
from cryptography.hazmat.primitives.asymmetric.ec import ECDSA
from cryptography.hazmat.primitives.hashes import SHA3_256

private_key = load_private_key(hex_key)
message = tx_hash.encode()
signature = private_key.sign(message, ECDSA(SHA3_256()))
```

### 5. `Bridge: Betrag zu groß`
```
Fehler: Tages-Bridge-Limit überschritten (1.000.000 ATC)
Symptom: bridge.initiate_transfer() schlägt fehl
```
**Lösung:** Transfer auf mehrere Tage aufteilen oder
Multi-Sig-Transaktion mit erhöhtem Limit beantragen (Governance).

### 6. `nonce_too_low` (HTTP 409)
```
Fehler: TX-Nonce bereits verwendet
Symptom: Transaktion wird abgelehnt
```
**Lösung:**
```python
# Aktuelle Nonce abfragen
r = requests.get(f"{BASE_URL}/api/wallet/nonce/{address}")
current_nonce = r.json()['nonce']
# TX mit nonce = current_nonce + 1 senden
```

## Debug-Befehle

```bash
# System-Status prüfen
curl http://localhost:4000/health

# ATCLang Contract prüfen
python atclang/security/analyzer.py my_contract.atc

# Logs ansehen
tail -f logs/gateway.log
tail -f logs/blockchain.log

# Node-Metriken
curl http://localhost:9090/metrics

# Testnet starten
./start_testnet.sh --nodes 5 --verbose

# Test-Suite ausführen
python -m pytest tests/ -v
```
