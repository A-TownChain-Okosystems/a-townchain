# ATC-86 — ECDSA Signature (secp256k1)

> **Standard-ID:** ATC-86 (ehemals ATC-1005)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.1
> **Sprint:** 2.1 | **Issue:** #74 | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Consensus Primitives  
> **Tier:** Tier 1 — Consensus Primitives  

---

## 1. Überblick

ECDSA-Signaturen über secp256k1 — Standard-Signaturverfahren für Transaktionen, Blöcke und Konsens-Votes. Nonce-sicher (RFC 6979), mit Recovery-Byte für Address-Ableitung.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/crypto/signatures.atc — ECDSA Sign/Verify/Recover
modules/crypto/keys.atc — Key-Generation
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `generate_keypair() -> (PrivateKey, PublicKey)` | Generiert secp256k1 Key-Pair | 5000 |
| `sign(priv: &PrivateKey, msg: &Hash) -> Signature` | Signiert Hash mit Private-Key (RFC 6979) | 2000 |
| `verify(pub: &PublicKey, msg: &Hash, sig: &Signature) -> bool` | Verifiziert Signatur | 3000 |
| `recover(msg: &Hash, sig: &Signature) -> PublicKey` | Stellt Public-Key aus Signatur wieder her | 3000 |
| `derive_address(pub: &PublicKey) -> Address` | Leitet Address aus Public-Key ab (SHA-256) | 50 |

---

## 3. Konfiguration

```toml
curve: secp256k1 | hash: SHA-256 | canonical: true | nonce: RFC 6979
```

---

## 4. Context-Isolation

| Context | Verfügbar |
|---------|-----------|
| **Node** | ✅ Full |
| **Smart Contract** | ✅ (über API) |
| **Test** | ✅ Mock |

---

## 5. Testing

8+ Unit-Tests: Sign/Verify, Recover, Key-Generation, Address-Derivation, Invalid-Signature, Edge-Cases

### Test-Dateien

```
tests/consensus/
├── test_atc-86.atc    # Unit-Tests
└── test_atc-86_integration.atc  # Integration-Tests
```

### Coverage-Ziel: 90%+

---

## 6. Abhängigkeiten

| Abhängigkeit | Issue | Status |
|--------------|-------|--------|
| ATCLang Compiler (ATC-92) | #72 | 📋 |
| ATCLang VM (ATC-93) | #73 | 📋 |
| ATCLang Stdlib (ATC-94) | #81 | 📋 |

---

## 7. Sprint-Zuweisung

- **Sprint 2.1** — #74
- **Priorität:** HIGH (Sprint 2.1 Blocker für alle ATCLang-Migrationen)
- **Deliverable:** Implementierte Module + Unit-Tests

---

## Referenzen

- **Roadmap v2.0:** Sprint 2.1
- **Wiki:** Kap. 31 (Issue-Registry), Kap. 37 (P2P), Kap. 38 (Konsens)
- **AGENT_MASTERRULES.md:** Regel 0 (ATCLang First), Regel 1 (Reality-Check)

---

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-86 v1.0.0*
