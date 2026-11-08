# ATC-16 — Referral & Multi-Tier Rewards Logic
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.5 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-16
> **Tier:** 2 (Logik & Oekonomie)
> **Referenzen:** ATC-03 (DID/Identity), ATC-04 (DAG), ATC-11 (Fungible Rewards), ATC-14 (Deterministic Exec), ATC-17 (DAO, geplant), Issue #5 (Explorer)
> **Quelldatei:** Atc-16.docx (urspruengliche Spezifikation)
> **Kategorie:** Economy & Assets  

---

## Abstract

ATC-16 definiert die Referral & Multi-Tier Rewards Logic. Innerhalb des
KAI-OS-Oekosystems ist dies das Protokoll, das virales Wachstum und
Anreizstrukturen direkt auf der Protokollebene verankert, ohne dass dies durch
zentralisierte Marketing-Datenbanken gesteuert werden muss.

> **ATC-16 = Virales Wachstum auf der Blockchain.**
> ATC-11 bis 15 = Oekonomie, Assets, Ausfuehrung, Mining (Tier 2).
> ATC-16 = Die virale Wachstums- und Belohnungslogik (Tier 2).

---

## 1. Kernkonzepte

### 1.1 On-Chain Referral-Tracing
ATC-16 ermoeglicht es, eine "Parent-Child"-Beziehung zwischen Wallets oder
Identitaeten (basierend auf ATC-03) direkt auf der Blockchain zu registrieren.
Wenn ein neuer Nutzer dem Netzwerk beitritt, wird er einem "Empfehler" zugewiesen.
Diese Verknuepfung ist unveraenderlich und wird als Graph-Struktur innerhalb
des DAG (ATC-04) gespeichert.

**Aktueller Stand:** Nicht implementiert. Die Basis-Komponenten sind da:
- ATC-03 Identity (ECDSA-Wallet-Adressen, ATC-Prefix)
- ATC-04 DAG (Chain-Struktur fuer Graph-Speicherung)
- Governance Contract (`governance_contract.py`) mit Stakeholder-Verwaltung
- **Geplant:** Referral-Graph als Smart Contract

### 1.2 Multi-Tier-Belohnungslogik
Der Standard definiert, wie Belohnungen ueber mehrere Stufen (Tiers) hinweg
verteilt werden koennen. Wenn ein Nutzer eine Aktion ausfuehrt (z. B.
Inferenz-Gebuehren zahlt oder am Mining teilnimmt), wird automatisch ein Teil
der Gebuehr gemaess ATC-16 an die Kette der Empfehler zurueckgefuehrt — ueber
beliebig viele Stufen hinweg.

**Konzept:**
```
Nutzer A zahlt Inferenz-Gebuehr (100 ATC)
  -> Tier 1 Empfehler (B): 5% = 5 ATC
  -> Tier 2 Empfehler (C): 3% = 3 ATC
  -> Tier 3 Empfehler (D): 1% = 1 ATC
  -> Rest (91 ATC): An System/Miner
```

> **Geplant:** Multi-Tier-Reward Contract in `blockchain/contracts/referral/`

### 1.3 Deterministische Ausschuettung
Da die Belohnungslogik nach ATC-14 deterministisch ist, erfolgt die Auszahlung
der Belohnungen (in Token gemaess ATC-11) bei jeder Transaktion sofort und ohne
menschliches Eingreifen.

**Bezug:** ATC-14 Determinismus + ATC-11 ATCoin = Automatische, deterministische
Reward-Verteilung bei jeder Transaktion.

---

## 2. Warum ATC-16 fuer KAI-OS essenziell ist

### 2.1 Wachstums-Hebel
KAI-OS nutzt ATC-16, um ein "Growth-Hacking"-Protokoll zu implementieren. Da die
Anreize nativ in den Smart Contracts verankert sind, motiviert das System Nutzer
und Inferenz-Anbieter automatisch dazu, neue Teilnehmer ins Netzwerk zu bringen.

### 2.2 Transparenz
Da die gesamte Referral-Struktur On-Chain einsehbar ist, gibt es keine
"versteckten" Partnerprogramme. Jeder Teilnehmer kann den Status seiner Rewards
und seiner Struktur im Blockchain-Explorer (siehe Issue #5) transparent
nachvollziehen.

**Bezug:** Issue #5 (Explorer) — Blockchain-Explorer fuer transparente Referral-
Strukturen.

### 2.3 Synergie mit KI-Netzwerken
Im Bereich des Social-Media-KI-Franchise ist ATC-16 der Mechanismus, mit dem
Content-Ersteller und KI-Modell-Anbieter fuer die erfolgreiche Verbreitung und
Nutzung ihrer Inhalte entlohnt werden.

**Bezug:** Franchise Factory als organisatorischer Hub. ATC-16 als Belohnungs-
Mechanismus fuer Franchise-Partner.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-03 (Decentralized Identity)
Die Identitaet des Empfehlers ist der Ankerpunkt fuer die Belohnungszuweisung.

### 3.2 ATC-11 (Fungible Assets)
ATC-16-Rewards werden in Form von ATC-11-Token (ATCoin) ausgeschuettet.

### 3.3 ATC-17 (DAO Governance)
Oftmals koennen Nutzer ihre Reputation oder Referral-Erfolge (gemaess ATC-16)
nutzen, um ihr Stimmgewicht innerhalb der DAO zu erhoehen.

> ATC-17 ist ein zukuenftiger Standard — DAO Governance mit Reputation weighting.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| On-Chain Referral-Tracing | Parent-Child Graph im DAG | Nicht implementiert | PARTIAL Geplant |
| Multi-Tier Rewards | Belohnung ueber mehrere Stufen | Nicht implementiert | PARTIAL Geplant |
| Deterministische Ausschuettung | ATC-14 + ATC-11 | Basis da (Contract + Token) | PARTIAL Basis da |
| ATC-03 Identity | Ankerpunkt fuer Referral | ECDSA-Wallets implementiert | OK Implementiert |
| ATC-11 Rewards | ATCoin als Reward-Token | ATCoin (21M, ATC-89) | OK Implementiert |
| Explorer-Transparenz | On-Chain Referral einsehbar | Explorer (Issue #5) | PARTIAL Basis da |
| Sybil-Attack-Prevention | Fake-Referral verhindern | Nicht implementiert | PARTIAL Geplant |
| Franchise-Integration | Content-Ersteller Belohnung | Franchise Factory Hub | PARTIAL Basis da |

> **Fazit:** Die Infrastruktur (Identity, Token, DAG, Governance) ist da.
> Der Referral-Graph und die Multi-Tier-Reward-Logik sind der fehlende
> Smart Contract — ein klar definierter Entwicklungsschritt.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #1 | Smart Contracts (Base) | Done | ATC-16 Contract-Basis |
| #5 | Explorer | Done | ATC-16 Transparenz |
| #9 | Governance | Done | ATC-16 Stakeholder-Verwaltung |
| #39 | DAO-Governance Live | Done | ATC-16 Reputation-Voting |
| Sprint 2.5 | Referral-Graph Contract | Geplant | ATC-16 On-Chain Tracing |
| Sprint 2.5 | Multi-Tier Reward Logic | Geplant | ATC-16 Belohnungslogik |
| Sprint 2.5 | Sybil-Attack Prevention | Geplant | ATC-16 Fake-Referral-Schutz |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Referral-Graph Smart Contract: `blockchain/contracts/referral/referral.py`
- [ ] register_referral(new_user, referrer) — Parent-Child auf Chain
- [ ] Multi-Tier-Reward: Konfigurierbare Tiers (Prozentsatz, Tiefe)
- [ ] Auto-Distribution: Bei jeder Transaktion Referral-Rewards verteilen
- [ ] Sybil-Prevention: ATC-03 Reputation-Score als Filter fuer Fake-Referrals
- [ ] Referral-Cap: Max-Tiers und Max-Reward pro Transaktion
- [ ] Explorer-Integration: Referral-Graph visualisieren
- [ ] Franchise-Rewards: Content-Ersteller und KI-Anbieter belohnen
- [ ] ATC-17 Integration: Referral-Erfolge als DAO-Stimmgewicht
- [ ] Analytics: Referral-Statistiken On-Chain

---

## 7. Sybil-Attack-Prevention

Die grosste Herausforderung fuer Referral-Systeme sind Sybil-Attacken — Angreifer
erstellen hunderte Fake-Identitaeten, um Referral-Belohnungen abzugreifen.

### Schutzmechanismen (geplant):
1. **ATC-03 Reputation-Score:** Neue Identitaeten haben Score 0 — keine Referral-
   Rewards bis Mindest-Score erreicht
2. **Stake-Requirement:** Empfehler muss ATC-11 Token staken, um Rewards zu erhalten
3. **Activity-Proof:** Referral nur gueltig nach erster echter Transaktion des
   geworbenen Nutzers
4. **Tier-Limit:** Max 3-5 Tiers, kein unendliches Ausbreiten
5. **Time-Delay:** Rewards werden ueber Zeit freigegeben (Vesting)

---

*Dieses Dokument wurde aus der urspruenglichen Atc-16.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:00 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| register_referral | 20000 |
| claim_reward | 10000 |
| calculate_tier | 200 |

### Sprint-Zuweisung

- **Sprint 2.5** — geplant
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

4+ Unit-Tests: Referral-Registration, Reward-Claim, Tier-Calculation

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
