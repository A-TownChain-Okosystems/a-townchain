# ATC-19 — Automated Market Maker (AMM) Logic
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.5 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-19
> **Tier:** 2 (Logik & Oekonomie)
> **Referenzen:** ATC-11 (Fungible Assets), ATC-13 (Fractional), ATC-14 (Deterministic Exec), ATC-15 (Mining), ATC-18 (MultiSig), Issue #34 (DEX/AMM)
> **Quelldatei:** Atc-19.docx (urspruengliche Spezifikation)
> **Kategorie:** Governance  

---

## Abstract

ATC-19 definiert die Automated Market Maker (AMM) Logic innerhalb des
A-TownChain-Oekosystems (KAI-OS). In einem dezentralen Betriebssystem wie KAI-OS
ist der Handel mit Assets (Token, Inferenz-Credits, NFT-Anteile) nicht auf
zentralisierte Boersen angewiesen, sondern erfolgt direkt "On-Chain" durch
algorithmische Liquiditaetspools.

> **ATC-19 = Das finanzielle "Schmiermittel" des Betriebssystems.**
> ATC-11 bis 18 = Oekonomie, Assets, Mining, Governance, Sicherheit (Tier 2).
> ATC-19 = Automatisierte Liquiditaet und Preisbildung.

---

## 1. Kernkonzepte

### 1.1 Liquiditaetspools statt Orderbuecher
ATC-19 legt fest, dass der Handel ueber Liquiditaetspools stattfindet. Anstatt
Kaeufer und Verkaeufer zusammenzubringen, handeln Nutzer gegen einen Smart
Contract, der beide Assets (z. B. ATCoin und Inferenz-Credits) in einem Pool
haelt.

**Implementation:** OK Implementiert
- DEX/AMM Logik in `blockchain/contracts/` — Issue #34 abgeschlossen
- `blockchain/contracts/solidity/contracts/KAIMarketplace.sol` — EVM-kompatibel
- ATC-88 Standard (AMM Constant-Product) in Registry

### 1.2 Konstanter Produkt-Algorithmus
Das Herzstueck von ATC-19 ist eine mathematische Formel (meist x * y = k), die
den Preis eines Assets basierend auf dem aktuellen Angebot und der Nachfrage im
Pool automatisch anpasst. Dies garantiert, dass zu jedem Zeitpunkt ein Preis fuer
ein Asset berechenbar ist.

**Implementation:** OK Implementiert
- Constant-Product Formel: `x * y = k`
- ATC-88 v1.0 Standard (AMM Constant-Product) in Registry
- Deterministische Preisberechnung (ATC-14 kompatibel)

```python
# AMM Constant-Product
# Pool: reserve_A (x) * reserve_B (y) = k (konstant)
# Swap: amount_A_in -> amount_B_out = y - (k / (x + amount_A_in))
# Preis wird automatisch durch Pool-Verhaeltnis bestimmt
```

### 1.3 Liquidity Provider Anreize
ATC-19 spezifiziert, wie Nutzer, die dem System Liquiditaet zur Verfuegung
stellen (indem sie ihre Token in den Pool einzahlen), durch Gebuehren belohnt
werden, die von jedem Handel innerhalb dieses Pools abgezogen werden.

**Implementation:** OK Implementiert
- LP-Token als Anteils-Beweis am Pool
- Trading-Fee wird an Liquidity Provider verteilt
- Fee-Struktur konfigurierbar

---

## 2. Warum ATC-19 fuer KAI-OS essenziell ist

### 2.1 Dezentraler Handel
ATC-19 ermoeglicht den Austausch von Assets ohne Zwischenhaendler. KI-Agenten
(Tier 4) koennen diese Pools nutzen, um autonom Assets zu tauschen (z. B. um
Inferenz-Credits gegen Utility-Token zu tauschen), was das System hochgradig
autonom macht.

**Bezug:** AI-Kernel (`ai_kernel.py`) — KI-Agenten koennen AMM-Swaps
autonom ausfuehren. ATC-15 Mining + ATC-19 AMM = Autonome KI-Oekonomie.

### 2.2 Liquiditaet fuer alle Assets
Egal ob ATC-11 (Token) oder ATC-13 (Fraktionierte Assets), ATC-19 bietet das
Framework, um fuer jedes dieser Assets einen Markt zu schaffen.

### 2.3 Integrierte Wirtschaft
Da die AMM-Logik direkt im Betriebssystem verankert ist, koennen Smart Contracts
auf die Pool-Daten zugreifen. Dies ermoeglicht fortgeschrittene Finanzanwendungen
wie automatisiertes Rebalancing von Portfolios durch KI-Agenten.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-11 (Fungible Assets)
ATC-19 Pools bestehen aus zwei oder mehr ATC-11-konformen Token (z. B. ATCoin +
Inferenz-Credits).

### 3.2 ATC-13 (Fractional Asset Ownership)
ATC-19-Pools koennen genutzt werden, um Liquiditaet fuer fraktionierte NFT-
Anteile bereitzustellen.

### 3.3 ATC-14 (Deterministic Execution)
Da der Preis in einem AMM-Pool durch eine mathematische Formel bestimmt wird,
muss diese auf allen Nodes nach ATC-14 identisch berechnet werden, um Konsens
ueber den aktuellen Preis zu erzielen.

### 3.4 ATC-88 (AMM Constant-Product)
ATC-88 ist die technische Spezifikation des x*y=k Algorithmus in der
Standards Registry.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Liquiditaetspool | Pool statt Orderbuch | DEX in blockchain/contracts | OK Implementiert |
| Constant-Product | x * y = k | ATC-88 Standard | OK Implementiert |
| Swap-Funktion | Token tauschen | swap() in DEX Contract | OK Implementiert |
| Liquidity Provider | LP-Token + Fees | LP-Token und Fee-Verteilung | OK Implementiert |
| Add/Remove Liquidity | Pool einzahlen/abheben | Implementiert | OK Implementiert |
| Preisberechnung | Deterministisch | ATC-14 kompatibel | OK Implementiert |
| KI-Agent Auto-Swap | Autonomes Tauschen | KI-Agent + DEX moeglich | PARTIAL Basis da |
| NFT-Fraktions-Pool | ATC-13 Anteile im AMM | ATC-13 konzeptionell | PARTIAL Geplant |
| Impermanent Loss Schutz | IL-Mitigation | Nicht implementiert | PARTIAL Geplant |
| TWAP-Oracle | Zeitgewichteter Preis | Nicht implementiert | PARTIAL Geplant |

> **Fazit:** ATC-19 ist **voll implementiert** als Constant-Product AMM (DEX).
> Pools, Swaps, LP-Token, Fees und deterministische Preisberechnung sind
> funktional. Erweiterte DeFi-Features (TWAP, IL-Schutz) als Evolution.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #1 | Smart Contracts (Base) | Done | ATC-19 Contract-Basis |
| #13 | Marketplace | Done | ATC-19 NFT-Handel |
| #34 | DEX/AMM | Done | ATC-19 AMM Implementation |
| #69 | Security-Audit | Open | ATC-19 AMM-Sicherheit |
| Sprint 2.5 | TWAP-Oracle | Geplant | ATC-19 Preis-Oracle |
| Sprint 2.5 | Impermanent Loss Schutz | Geplant | ATC-19 IL-Mitigation |
| Sprint 2.5 | KI-Agent Auto-Rebalancing | Geplant | ATC-19 + KI Portfolio |
| Sprint 2.5 | NFT-Fraktions-Pools | Geplant | ATC-19 + ATC-13 |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] TWAP-Oracle: Zeitgewichteter Durchschnittspreis fuer Manipulations-Schutz
- [ ] Impermanent Loss Schutz: IL-Mitigation fuer Liquidity Provider
- [ ] Concentrated Liquidity: LPs wählen Preisbereiche (aehnlich Uniswap v3)
- [ ] KI-Agent Auto-Swap: Autonomes Tauschen von Inferenz-Credits
- [ ] NFT-Fraktions-Pools: ATC-13 Anteile in AMM-Pools
- [ ] Multi-Asset Pools: Pools mit mehr als 2 Token
- [ ] Dynamic Fees: Fee basierend auf Volatilitaet anpassen
- [ ] Pool-Factory: Standardisierte Pool-Erstellung fuer neue Asset-Paare
- [ ] AMM-Explorer: Pool-Statistiken und Preis-Charts im Explorer
- [ ] Cross-Chain AMM: Pools ueber ATC-09 Bridge mit anderen Chains

---

## 7. Impermanent Loss — Risiko und Schutz

### Was ist Impermanent Loss?
Wenn ein Liquidity Provider Token in einen Pool einzahlt und die Preise der
beiden Token sich unterschiedlich entwickeln, verliert der LP im Vergleich zum
reinen Halten (Hold). Dieser Verlust ist "impermanent" — er wird realisiert,
wenn der LP aus dem Pool aussteigt.

### Schutzmechanismen (geplant):
1. **Dynamic Fees:** Hoehere Fees bei hoher Volatilitaet -> IL-Kompensation
2. **Concentrated Liquidity:** LPs waehlen Preisbereiche -> geringeres IL-Risiko
3. **IL-Insurance:** Versicherungspool fuer Liquidity Provider
4. **Single-Sided Staking:** Nur eine Asset-Seite einzahlen (IL-frei)
5. **Rebalancing-Rewards:** Zusaetzliche Rewards fuer LPs bei starker Abweichung

---

*Dieses Dokument wurde aus der urspruenglichen Atc-19.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:07 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| swap | 30000 |
| add_liquidity | 50000 |
| remove_liquidity | 50000 |

### Sprint-Zuweisung

- **Sprint 2.3** — #76
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

6+ Unit-Tests: Swap, Liquidity, Slippage, Price-Calc, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
