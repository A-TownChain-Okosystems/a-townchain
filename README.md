# atc-algorithm

> **ATC-Algorithmus** — der proprietaere Hybrid Consensus der A-TownChain: **PoH + PoS + PoW**.

**Prioritaet:** P0 (Konsens-Kern, AD-044) | **Chain-ID:** 658467 (AD-004) | **Org:** [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems)

> ## Fuer KI-Agenten - Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo [`a-townchain-os-docs`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs):
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md)
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md)
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) - insb. AD-001 (SHA-256), AD-004 (Chain-ID), AD-023 (kein Mainnet-Termin), AD-044 (dieses Repo)

---

## Rolle im Oekosystem

Der ATC-Algorithmus ist der proprietare Konsens der A-TownChain — eine
Hybrid-Kombination aus drei Komponenten:

| Komponente | Funktion |
|---|---|
| **PoH** (Proof of History) | Kryptografische Zeitstempel-Kette (Solana-inspiriert) — verifizierbare Ereignis-Reihenfolge vor der Konsens-Entscheidung |
| **PoS** (Proof of Stake) | Validator-Gewichtung — 5 Validator-Keys, Staking-basierte Block-Produktion |
| **PoW** (Proof of Work) | Schwierigkeitsanteil — Schutz gegen Stake-Zentralisierung |

```
Transaktionen -> PoH (Zeitstempel-Kette, SHA-256 — AD-001)
                  |
                  v
           PoS/PoW-Hybrid-Auswahl (Validator-Gewichtung)
                  |
                  v
           Block finalisierung -> a-townchain (Chain) -> atc-vm (Contract-Ausfuehrung)
```

## Abgrenzung (AD-044)

| Zustaendigkeit | Repo |
|---|---|
| **Konsens-Algorithmus (PoH + PoS + PoW, Hybrid-Auswahl)** | **`atc-algorithm` (dieses Repo)** |
| Chain-Konsens-Orchestrierung, Block-/Tx-Struktur | `a-townchain` |
| Kernel (Prozess-/Speicher-Isolation) | `atc-shivacore` |
| Vertrags-Ausfuehrung | `atc-vm` |
| Netzwerk-Propagation | `a-townchain` (atcnet) |

Die bisherige PoH-Implementierung in `a-townchain` (ProofOfHistory-Modul)
dient als Referenz; die kanonische Rust-Implementierung entsteht hier
(AD-021 Rust-first). Die Modul-Migration ist als offener Punkt im
DECISIONS_REGISTER (AD-044) gefuehrt.

## Status (AD-020-Rebuild-Aera)

- **Stand:** R1-Skeleton (ATC-STD-201) — Struktur + Governance stehen,
  Implementierung folgt im qualitaetsgetriebenen Rebuild (AD-023).
- **Naechste Schritte:** (1) PoH-Spezifikation aus a-townchain/Wiki
  konsolidieren, (2) Hybrid-Auswahl-Regeln formal spezifizieren
  (Gewichtung, Finalitaet, Fork-Resolution), (3) Rust-Implementierung
  PoH-Tick-Kette, (4) Fuzzing + Konsistenz-Tests gegen Referenz.
- **Qualitaets-Gates:** Konsens = S4-kritisch — G18 Security-Audit vor
  jedem Freeze (AD-023); SHA-256 als einziger Hash-Algorithmus (AD-001).

## Lizenz

Proprietaer - All Rights Reserved (ATC-LIC). Siehe [LICENSE](LICENSE).
