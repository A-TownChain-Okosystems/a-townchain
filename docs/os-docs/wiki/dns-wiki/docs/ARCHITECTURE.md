# 🏗️ System-Architektur — atc-dns

---

## 1. Funktionsweise von .atc Domains

`atc-dns` bildet Namen wie `alice.atc` auf kryptografische Adressen, Gateway URIs und IPFS Hashs ab.

```
[ Domain Lookup: bob.atc ] ──> [ Local TTL Cache ] ──> (Hit) Return Address
                                     │
                                (Cache Miss)
                                     ▼
                      [ On-Chain Registrar State ]
```

## 2. Unterstützte Record Typen

- **`A` / `AAAA`**: Physische Node IP-Adressen.
- **`ATC-URI`**: Interne Dezentrale Ressourcen-Zeiger.
- **`TXT`**: Arbiträre Metadaten & Agenten Identifikationen.
