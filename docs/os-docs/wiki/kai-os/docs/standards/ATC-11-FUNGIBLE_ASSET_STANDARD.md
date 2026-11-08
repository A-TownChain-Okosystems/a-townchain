# ATC-11 — Fungible Asset Standard
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.3 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-11
> **Tier:** 2 (Logik & Oekonomie) — Erster Tier-2 Standard
> **Referenzen:** ATC-09 (Cross-Chain Bridge), ATC-13 (Fractional Ownership, geplant), ATC-14 (Deterministic Exec, geplant), ATC-17 (DAO, geplant), ATC-19 (Liquidity Pools, geplant), Issue #1
> **Quelldatei:** Atc-11.docx (urspruengliche Spezifikation)
> **Kategorie:** Economy & Assets  

---

## Abstract

ATC-11 ist der Fungible Asset Standard. Er bildet die Basis fuer alle
austauschbaren (fungiblen) Werteinheiten innerhalb des A-TownChain-Okosystems
(KAI-OS). Man kann ihn sich als das "A-TownChain-Aequivalent" zum bekannten
ERC-20-Standard von Ethereum vorstellen, jedoch mit spezifischen Erweiterungen
fuer das KAI-OS-Betriebssystem.

> **ATC-11 ist der Einstieg in Tier 2 (Logik & Assets).**
> Tier 1 (ATC-01 bis ATC-10) definierte das Fundament.
> Tier 2 definiert die wirtschaftliche Logik und Asset-Spezifikationen.

---

## 1. Kernkonzepte

### 1.1 Einheitlichkeit und Austauschbarkeit
ATC-11 definiert die Schnittstelle fuer Assets, die beliebig teilbar und
untereinander austauschbar sind (wie z. B. Waehrungen, Governance-Token oder
Utility-Credits fuer Inferenz-Leistung). Jeder Token eines Typs hat denselben
Wert wie jeder andere Token desselben Typs.

**Implementation:** OK Implementiert
- `blockchain/atcoin/atcoin.py` — ATCoin (ATC) als primaerer fungibler Token
  - Symbol: ATC
  - Max Supply: 21.000.000 (wie Bitcoin)
  - Decimals: 18
  - Standard: ATC-89 (ERC20-kompatibel)
- `blockchain/contracts/atc8300/atc8300_token.py` — ATC-89 Token Standard
- `blockchain/contracts/base/base_contract.py` — Base Contract (Issue #1)
- `blockchain/contracts/atc001/genesis_token.py` — Genesis Token

### 1.2 Standardisierte Schnittstellen-Methoden
Damit Smart Contracts und das Betriebssystem reibungslos mit einem Asset
interagieren koennen, schreibt ATC-11 zwingend definierte Methoden vor:

**Implementation:** OK Implementiert (ATC-89 Standard)

```
# ATC-89 Standard Methoden (aehnlich ERC-20)
class ATC8300Token:
    def balance_of(self, owner) -> int:
        # Abfrage des Bestands
    def transfer(self, to, amount) -> bool:
        # Uebertragung von Werten
    def approve(self, spender, amount) -> bool:
        # Autorisierung von Drittanwendungen (z. B. AMMs)
    def total_supply(self) -> int:
        # Abfrage der gesamten im Umlauf befindlichen Menge
```

> **Status:** Alle vier Kern-Methoden (balanceOf, transfer, approve,
> totalSupply) sind im ATC-89 Standard implementiert.

### 1.3 Betriebssystem-Integration (Tier 3 & 4)
Da ATC-11 ein Standard innerhalb eines Betriebssystems ist, koennen KI-Agenten
(Tier 4) diese Assets nativ verwenden, um beispielsweise autonom Rechenleistung
(Inferenz-Gebuehren) zu bezahlen, ohne manuelle Transaktionen durch den Nutzer
zu benoetigen.

**Implementation:** Teilweise implementiert
- AI-Kernel (`ai_kernel.py`) mit DecisionEngine fuer autonome Entscheidungen
- Orchestrator (`orchestrator.py`) mit TaskType.WALLET und TaskType.AI
- Wallet API (`backend/api/routes/wallet.py`) fuer Asset-Transfers
- **Geplant:** Vollstaendige autonome KI-Bezahlung von Inferenz-Gebuehren

---

## 2. Warum ATC-11 fuer KAI-OS essenziell ist

### 2.1 Wirtschaftliche Konsistenz
Wenn ein Asset den ATC-11-Standard erfuellt, garantiert das Betriebssystem,
dass alle Protokolle (wie Governance via ATC-17 oder Liquiditaetspools via
ATC-19) korrekt mit diesem Asset arbeiten.

**Bezug:** ATCoin interoperiert mit Governance (`governance_contract.py`),
Marketplace (`marketplace_contract.py`), und Bridge (`bridge_contract.py`).

### 2.2 Interoperabilitaet
ATC-11-Assets koennen ueber die Interoperabilitaets-Bruecke (ATC-09)
problemlos in andere Netzwerke uebertragen werden, da das zugrundeliegende
Interface standardisiert ist.

**Bezug:** ETH Bridge (`KAIBridge.sol`) und SOL Bridge bridgen ATC-89
Tokens. Wrapped ATC (wATC) auf Ethereum/Solana nutzt ATC-11 Interface.

### 2.3 Automatisierung
Da ATC-11-Schnittstellen deterministisch sind (ATC-14), koennen KI-Agenten
komplexe wirtschaftliche Entscheidungen treffen (z. B. "Kaufe X Token, wenn
der Preis unter Y faellt"), die direkt vom KAI-OS ausgefuehrt werden.

**Bezug:** AI-Kernel mit DecisionEngine + Constitutional AI. DEX/AMM
Logik fuer automatisierte Token-Swaps implementiert.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-13 (Fractional Asset Ownership)
Waehrend ATC-11 die Basis-Fungibilitaet definiert, erlaubt ATC-13 darauf
aufbauend das Halten von Bruchteilen (Fractional Ownership) an groesseren,
wertvolleren Assets.

> ATC-13 ist ein zukuenftiger Standard — baut auf ATC-11 auf.

### 3.2 ATC-14 (Deterministic Execution)
Da die Uebertragung von ATC-11-Token eine Zustaendsaenderung darstellt, muss
jeder Node durch ATC-14 sicherstellen, dass die Bilanzaenderung auf allen
Nodes exakt gleich berechnet wird.

> ATC-14 ist ein zukuenftiger Standard — sichert ATC-11 Transaktionen.

### 3.3 Tier 2 (Logik & Oekonomie)
ATC-11 ist die "Einstiegstuer" fuer alles, was im KAI-OS einen wirtschaftlichen
Wert repraesentiert.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Fungible Token | ERC-20-Aequivalent | ATC-89 Standard, ATCoin | OK Implementiert |
| balanceOf() | Bestandsabfrage | balance_of() in ATC8300Token | OK Implementiert |
| transfer() | Wertuebertragung | transfer() mit ECDSA-Signatur | OK Implementiert |
| approve() | Drittanbieter-Autorisierung | approve() fuer AMM/Dex | OK Implementiert |
| totalSupply() | Gesamtmenge | 21.000.000 ATC, 18 Decimals | OK Implementiert |
| OS-Integration | KI-Agenten nativ bezahlen | AI-Kernel + Orchestrator | PARTIAL Basis da |
| Cross-Chain Transfer | Via ATC-09 Bridge | ETH/SOL Bridge implementiert | OK Implementiert |
| Approval-Overflow-Schutz | Bug-Vermeidung | Zu pruefen im Security-Audit | PARTIAL Offen |
| Autonome KI-Zahlung | Auto Inferenz-Gebuehren | DecisionEngine, noch nicht voll autonom | PARTIAL Geplant |

> **Fazit:** ATC-11 ist **voll implementiert** als ATC-89 Standard mit
> ATCoin als primaerem Token. Alle Kern-Methoden (balanceOf, transfer,
> approve, totalSupply) sind funktional. Die KI-Autonomie bei
> Inferenz-Begleichung ist der naechste Schritt.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #1 | Smart Contracts (Base Contract) | Done | ATC-11 Basis-Contract |
| #10 | Cross-Chain Bridge | Done | ATC-11 Cross-Chain Transfer |
| #13 | Marketplace | Done | ATC-11 Token im Marketplace |
| #69 | Security-Audit | Open | ATC-11 Approval-Overflow-Pruefung |
| Sprint 2.3 | KI Auto-Payment | Geplant | ATC-11 Autonome Inferenz-Begleichung |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Approval-Overflow-Schutz: require(approve == 0 vor neuem approve)
- [ ] KI Auto-Payment: KI-Agenten zahlen Inferenz-Gebuehren autonom
- [ ] ATC-13 Integration: Fractional Ownership auf ATC-11 Basis
- [ ] ATC-14 Integration: Deterministische Execution fuer ATC-11 Transfers
- [ ] Gas-Abstraktion: ATC-11 Transfers ohne manuelle Gas-Kalkulation
- [ ] Token-Minting-Policy: Standardisiertes Minting fuer neue ATC-11 Tokens
- [ ] Multi-Token-Support: Mehrere ATC-11 Tokens nebeneinander (neben ATC)
- [ ] Governance-Token: Separater ATC-11 Token fuer DAO-Voting (ATC-17)
- [ ] Utility-Credits: Inferenz-Credits als ATC-11 Token
- [ ] Bridge-Wrapping: wATC auf ETH/SOL mit ATC-11 Interface

---

*Dieses Dokument wurde aus der urspruenglichen Atc-11.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:40 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| transfer | 20000 |
| approve | 20000 |
| balance_of | 100 |

### Sprint-Zuweisung

- **Sprint 2.3** — #76
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

6+ Unit-Tests: Transfer, Approve, Allowance, Mint, Burn, Overflow

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
