# ATC-13 — Fractional Asset Ownership & Liquidity Standard
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.5 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-13
> **Tier:** 2 (Logik & Oekonomie)
> **Referenzen:** ATC-11 (Fungible Assets), ATC-12 (Non-Fungible Assets), ATC-19 (AMM, geplant), Issue #13 (Marketplace)
> **Quelldatei:** Atc-13.docx (urspruengliche Spezifikation)
> **Kategorie:** Economy & Assets  

---

## Abstract

ATC-13 definiert den Fractional Asset Ownership & Liquidity Standard. Waehrend
ATC-11 fungible Token (wie Waehrungen) und ATC-12 einzigartige Assets (wie NFTs)
abdeckt, ermoeglicht ATC-13 die fraktionale Eigentuemerschaft an hochwertigen
oder unteilbaren Assets.

> **ATC-11 = Fungible. ATC-12 = Non-Fungible. ATC-13 = Bruecke zwischen beiden.**
> ATC-13 schliesst die Luecke zwischen exklusiven Sammlerstuecken und
> hochliquiden Finanzmaerkten.

---

## 1. Kernkonzepte

### 1.1 Fraktionierung von Unikaten
ATC-13 erlaubt es, einen nicht-fungiblen Vermoegenswert (einen ATC-12-NFT,
z. B. ein hochpreisiges KI-Modell oder ein digitales Grundstueck) in eine grosse
Anzahl kleiner, fungibler Anteile (ATC-11-Token) zu zerlegen. Dies macht teure
Assets fuer ein breiteres Publikum handelbar.

**Aktueller Stand:** Nicht implementiert. Die Basis-Komponenten sind da:
- ATC-12 NFTs (`shivamon_contract.py`) als fraktionierbare Assets
- ATC-11 Fungible Tokens (`atc8300_token.py`) als Anteils-Token
- Marketplace (`marketplace_contract.py`) als Handelsplattform
- **Geplant:** Vault-Smart-Contract fuer Fraktionierung

### 1.2 Vaulting-Mechanismus
Das urspruengliche ATC-12-Asset wird in einem sogenannten "Vault" (einem Smart
Contract) gesichert. Sobald es dort liegt, werden die entsprechenden
"Anteils-Token" (ATC-13-konform) an die urspruenglichen Besitzer oder Investoren
ausgegeben.

**Konzept:**
```
ATC-12 NFT -> [Vault Smart Contract] -> ATC-11 Anteils-Token (x1000)
                                      |
                                      +-> Original-NFT gesperrt im Vault
                                      +-> Anteils-Token frei handelbar
                                      +-> Defraktionierung: Alle Anteile zurueck -> NFT freigeben
```

> **Geplant:** Vault Contract in `blockchain/contracts/fractional/` — sperrt NFT,
> mintet ATC-11 Anteils-Token, ermoeglicht Defraktionierung.

### 1.3 Liquidity Provision
Durch die Aufteilung in kleinere Einheiten wird die Liquiditaet erhoeht. Anstatt
einen gesamten NFT verkaufen zu muessen, kann ein Investor einfach einen Teil
seiner Anteile an einer Boerse oder einem automatisierten Liquiditaetspool
(ATC-19) veraeussern.

**Bezug:** ATC-19 (AMM/Liquidity Pools) ist ein zukuenftiger Standard. Die
DEX/AMM Logik ist bereits im Codebase implementiert (v3.0.0 Features).

---

## 2. Warum ATC-13 fuer KAI-OS entscheidend ist

### 2.1 Demokratisierung von KI-Ressourcen
Ein KI-Modell der Enterprise-Klasse kann extrem teuer sein. ATC-13 ermoeglicht es
einer Gemeinschaft von Nutzern, gemeinsam Anteile an einem solchen Modell zu
erwerben, an dessen Inferenz-Einnahmen sie dann anteilig partizipieren koennen.

**Bezug:** AI-Kernel (`ai_kernel.py`) mit LLMRouter. Ein KI-Modell als ATC-12
NFT, fraktioniert in ATC-11 Anteils-Token — Investoren erhalten anteilig
Inferenz-Einnahmen.

### 2.2 Effiziente Marktplaetze
Der Marktplatz (Issue #13) benoetigt ATC-13, um Orderbuecher fuer fraktionierte
Assets abzubilden. Es ist wesentlich einfacher, Bruchteile von Assets zu handeln,
als den gesamten Markt fuer Unikate auf "Ganz-Stueck-Transaktionen" zu
beschraenken.

**Bezug:** Marketplace (`marketplace_contract.py`, Issue #13 abgeschlossen).
Aktuell fuer ganze NFTs. Fraktionierte Anteile wuerden den Markt erweitern.

### 2.3 Rechteverwaltung
Da KAI-OS ein Betriebssystem ist, koennen Systemrechte (z. B. Rechenleistung oder
Admin-Rechte) ueber ATC-13 zwischen verschiedenen Stakeholdern aufgeteilt werden.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-11 (Fungible Assets)
Die "Anteils-Token", die aus einer Fraktionierung entstehen, sind technisch
gesehen ATC-11-konforme Token. ATC-13 definiert lediglich das Regelwerk, wie
diese Token aus einem ATC-12-Asset entstehen und wie die Rueckabwicklung
(Defraktionierung) funktioniert.

### 3.2 ATC-12 (Non-Fungible Assets)
Das Original-Asset, das fraktioniert wird, ist ein ATC-12-NFT (z. B. Shivamon,
KI-Modell, digitales Grundstueck).

### 3.3 ATC-19 (AMM)
ATC-13-Token werden oft direkt in Liquiditaetspools hinterlegt, um den Handel
mit den fraktionierten Assets zu ermoeglichen.

> ATC-19 ist ein zukuenftiger Standard. DEX/AMM Basis bereits im Codebase.

### 3.4 Issue #13 (Marketplace)
Die Implementierung des Marktplatzes muss beide Welten (NFTs nach ATC-12 und
deren Fraktionen nach ATC-13) unterstuetzen.

**Status:** Marketplace fuer ganze NFTs implementiert. Fraktionierte Anteile
als Erweiterung geplant.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Fraktionierung | NFT -> fungible Anteile | Nicht implementiert | PARTIAL Geplant |
| Vault-Mechanismus | Smart Contract sperrt NFT | Nicht implementiert | PARTIAL Geplant |
| Anteils-Token | ATC-11-konform | ATC-89 Standard verfuegbar | PARTIAL Basis da |
| Defraktionierung | Alle Anteile -> NFT freigeben | Nicht implementiert | PARTIAL Geplant |
| Liquidity Provision | Anteile in AMM-Pools | DEX/AMM Basis implementiert | PARTIAL Basis da |
| Marketplace Integration | Fraktionierte Assets handeln | Marketplace fuer ganze NFTs | PARTIAL Basis da |
| KI-Modell-Fraktionierung | Community-KI-Besitz | AI-Kernel + NFT | PARTIAL Basis da |
| Rechteverwaltung | Systemrechte aufgeteilt | Nicht implementiert | PARTIAL Geplant |

> **Fazit:** Alle Basis-Komponenten (ATC-11, ATC-12, Marketplace, DEX/AMM) sind
> implementiert. Der Vault-Contract als Bindeglied fehlt — er ist der
> technische Schluessel zur Fraktionierung.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #1 | Smart Contracts (Base) | Done | ATC-13 Vault-Contract Basis |
| #11 | Shivamon Breeding | Done | ATC-13 fraktionierbare NFTs |
| #13 | Marketplace | Done | ATC-13 Handel mit Fraktionen |
| #34 | DEX/AMM | Done | ATC-13 Liquidity Provision |
| Sprint 2.5 | Vault Contract | Geplant | ATC-13 Fraktionierung |
| Sprint 2.5 | Defraktionierung | Geplant | ATC-13 Rueckabwicklung |
| Sprint 2.5 | KI-Modell-Fraktionierung | Geplant | ATC-13 Community-KI-Besitz |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Vault Smart Contract: `blockchain/contracts/fractional/vault.py`
- [ ] fractionize(nft_id, shares) — NFT in Vault sperren, Anteils-Token minten
- [ ] defractionize(shares) — Alle Anteile zurueck, NFT freigeben
- [ ] Anteils-Token als ATC-89 mit Referenz auf Original-NFT
- [ ] Marketplace: Orderbuch fuer fraktionierte Anteile
- [ ] DEX/AMM: Liquidity Pool fuer ATC-13 Anteils-Token
- [ ] Revenue Distribution: Inferenz-Einnahmen anteilig verteilen
- [ ] KI-Modell als NFT: Enterprise-KI fraktionierbar machen
- [ ] Rechteverwaltung: System-Rechte als fraktionierte ATC-13 Assets
- [ ] Visualisierung: Fraktionierungs-Status im Explorer

---

*Dieses Dokument wurde aus der urspruenglichen Atc-13.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:48 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| fractionalize | 40000 |
| buy_fraction | 20000 |
| redeem | 30000 |

### Sprint-Zuweisung

- **Sprint 2.3** — #76
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

4+ Unit-Tests: Fractionalize, Buy, Redeem, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
