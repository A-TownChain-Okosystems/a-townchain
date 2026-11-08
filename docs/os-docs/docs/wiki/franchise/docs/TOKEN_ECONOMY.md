# Franchise Factory — Token Economy

## Token-Übersicht
| Token | Standard | Verwendung |
|-------|---------|-----------|
| **ATC** | Native | Lizenzgebühren, Gas, Staking |
| **FFT** | ATC-8300 | Governance, Revenue-Sharing |
| **Franchise-NFT** | ATC-9000 | Lizenz-Nachweis |

## FFT Token Verteilung
| Kategorie | Anteil | Vesting |
|-----------|--------|---------|
| Team | 15% | 4 Jahre, 1 Jahr Cliff |
| Community | 40% | Keine |
| Ökosystem-Fonds | 20% | 2 Jahre |
| Reserve | 15% | DAO-kontrolliert |
| Liquidität | 10% | Sofort |

**Max Supply: 100.000.000 FFT**

## Revenue-Share Mechanismus
1. Franchisee erwirtschaftet Umsatz
2. `record_revenue(amount)` → Contract speichert
3. Franchisor-Anteil: `amount × share_pct / 100`
4. `payout_franchisor()` → automatische Auszahlung in ATC
5. FFT-Holder erhalten Anteil via Governance-Beschluss

## Lizenz-Gebührenstruktur
| Paket | Lizenzgebühr | Revenue-Share | Laufzeit |
|-------|-------------|--------------|---------|
| Starter | 500 ATC | 5% | 1 Jahr |
| Standard | 1.000 ATC | 10% | 2 Jahre |
| Premium | 2.500 ATC | 15% | 5 Jahre |
| Master | 10.000 ATC | 20% | Unbegrenzt |

## Governance mit FFT
- 1 FFT = 1 Stimme
- Mindest-Stake: 1.000 FFT für Proposal
- Quorum: 10% aller FFT
- Threshold: 51% Ja-Stimmen
- Timelock: 48h nach Voting
