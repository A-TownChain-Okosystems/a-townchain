# 📜 Smart Contracts Referenz

## FranchiseRegistry (registry.atc)
Verwaltet alle Franchise-Lizenzen als NFTs (ATC-9000 Standard).

### Funktionen
```atclang
register(brand_name, category, region, revenue_share, duration_days) -> u64
transfer(license_id, to) -> bool
suspend(license_id)
is_active(license_id) -> bool
get_license(id) -> FranchiseLicense
licenses_of(addr) -> Vec<u64>
total_licenses() -> u64
```

### Events
```
LicenseIssued(id, owner, brand_name, region)
LicenseTransferred(id, from, to)
LicenseSuspended(id)
```

---

## RevenueShare (revenue.atc)
Automatische Gewinnverteilung zwischen Franchisor und Franchisee.

### Funktionen
```atclang
record_revenue(amount, memo)
payout_franchisor() -> u128
payout_franchisee() -> u128
get_stats() -> Map<string, u128>
```

### Berechnung
```
Franchisor-Anteil = amount * share_pct / 100
Franchisee-Anteil = amount - Franchisor-Anteil
```

---

## FranchiseToken (token.atc)
ATC-8300 kompatibler Utility Token.
- Symbol: **FFT**
- Decimals: 18
- Max Supply: 100.000.000 FFT
