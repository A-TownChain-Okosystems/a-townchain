# Security Policy — A-TownChain

## Supported Versions
| Version | Support |
|---------|---------|
| v3.0.x | Active |
| v2.2.x | Security fixes only |
| < v2.2 | End of Life |

## Reporting
Report vulnerabilities to: security@atownchain.io
**Do NOT create public issues for security bugs.**

## Response SLA
Confirmation: 24h | Assessment: 72h | Fix: 30 days | Disclosure: 90 days

## Bug Bounty (Mainnet)
Critical: 50.000 ATC | High: 10.000 ATC | Medium: 2.500 ATC | Low: 500 ATC

## Measures
- ATCLang Security Analyzer (15 rules, ATC-SEC-0001)
- ECDSA secp256k1 FIPS-compliant
- Reentrancy-Guard all state-mutating contracts
- MAX_CALL_DEPTH=128, MAX_SOURCE_SIZE=1MB
- Rate-limiting Gateway+P2P (100/60s, 64KB)
- MultiSig M-of-N Bridge + Franchise Treasury
- Nightly automated security scan (Bandit+Safety)
