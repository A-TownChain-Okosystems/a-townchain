# ATC-29 — Decentralized AI Marketplace & Model Registry
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 3.0 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-29
> **Tier:** 4 (Decentralized AI / Inferenz-Layer)
> **Referenzen:** ATC-11 (Fungible), ATC-13 (Fractional), ATC-17 (DAO), ATC-24 (Task Orch.), ATC-27 (Model Auditing), ATC-28 (Federated Learning), Issue #13 (Marketplace), #50 (AI Kernel)
> **Quelldatei:** Atc-29.docx (urspruengliche Spezifikation)
> **Kategorie:** AI Orchestration  

---

## Abstract

ATC-29 definiert die Decentralized AI Marketplace & Model Registry. Dies ist
der zentrale Handelsplatz des A-TownChain-Oekosystems (KAI-OS), an dem KI-
Modelle, Datensaetze und Inferenz-Dienste nicht nur verwaltet, sondern auch
oekonomisch gehandelt und bereitgestellt werden.

Waehrend ATC-24 die Orchestrierung von Aufgaben uebernimmt und ATC-28 das
dezentrale Lernen ermoeglicht, ist ATC-29 das wirtschaftliche Interface fuer
den "AI App Store" von KAI-OS.

> **ATC-29 = Der KI-Marktplatz — "AI App Store" von KAI-OS.**
> Tier 4 erweitert: Ausfuehrung (24-28) + Oekonomie der KI (29).

---

## 1. Kernkonzepte

### 1.1 Modell-Registry (On-Chain)
Jeder KI-Agent oder Entwickler kann ein Modell in der Registry anmelden. Diese
Registrierung enthaelt den kryptografischen Fingerabdruck (gemaess ATC-27), die
Lizenzbedingungen und die Zugriffsparameter. Der Eintrag erfolgt unveraenderlich
im Ledger.

**Implementation:** PARTIAL — Basis da
- Smart Contract Registry (`smart_contract_registry.py`) fuer Contract-Verwaltung
- Marketplace Contract (`marketplace_contract.py`) — Issue #13 abgeschlossen
- ATC-04 DAG fuer On-Chain Registrierung
- **Geplant:** KI-Modell-spezifische Registry mit ATC-27 Fingerprint

### 1.2 Service-Abonnement & Inferenz-Gebuehren
ATC-29 definiert, wie Nutzer oder andere Agenten fuer die Nutzung eines Modells
bezahlen. Dies geschieht oft ueber Smart Contracts, die automatisch Gebuehren
(in Token gemaess ATC-11) von der anfragenden Partei an den Modellbesitzer
ausschuettet, sobald die Inferenz erfolgreich abgeschlossen wurde.

**Implementation:** PARTIAL — Basis da
- ATC-11 Token (ATCoin) als Zahlungsmittel implementiert
- Smart Contracts koennen Zahlungen verarbeiten
- LLMRouter mit Per-User API-Key Strategy (Issue #2)
- **Geplant:** Automatische Inferenz-Gebuehren-Ausschuettung

```python
# GEPLANT: AI Marketplace Payment Flow
# 1. User/Agent requesting inference -> pays ATC-11 tokens
# 2. Smart Contract holds payment in escrow
# 3. Node executes inference -> returns result
# 4. Smart Contract releases payment to model owner
# 5. Event logged in DAG (ATC-04)
```

### 1.3 Governance der Modelle
Da Modelle "Produkte" im KAI-OS-Marktplatz sind, koennen ihre Eigenschaften
(Preis, Zugriffsbeschraenkungen) durch Governance-Abstimmungen (ATC-17)
beeinflusst werden, oder die Community kann als Ganzes entscheiden, bestimmte
Modelle in den "Core-Set"-Standard des Betriebssystems aufzunehmen.

**Bezug:** `governance_contract.py` (ATC-17) — DAO kann Modell-Listings
beeinflussen, Preise festlegen oder Core-Set-Modelle bestimmen.

---

## 2. Warum ATC-29 fuer KAI-OS essenziell ist

### 2.1 Monetarisierung fuer KI-Entwickler
Entwickler haben einen direkten Anreiz, leistungsfaehige KI-Modelle fuer das
KAI-OS zu erstellen, da sie durch ATC-29 automatisch an jeder Inferenz-Nutzung
verdienen koennen.

**Bezug:** ATC-15 (Proof-of-AI Mining) — Miner werden fuer Inferenz bezahlt.
ATC-29 erweitert dies auf Modell-Eigentuemer.

### 2.2 Service-Discovery
Ein Anwender, der eine spezielle KI-Aufgabe loesen moechte (z. B. "Analysiere
mein privates Finanz-Portfolio"), kann ueber den Marktplatz nach ATC-29-
konformen Agenten suchen, die genau diese Faehigkeiten besitzen.

**Bezug:** AI-Kernel (`ai_kernel.py`) + LLMRouter — Agent-Discovery bereits
implementiert. ATC-29 erweitert dies zum Marketplace.

### 2.3 Rechtssicherheit
Ueber ATC-29 werden Nutzungsrechte und Lizenzen fuer KI-Modelle On-Chain
verankert. Dies ist entscheidend fuer Unternehmen, die KI-Dienste in ihre
eigenen Produkte einbauen wollen und nachweisen muessen, dass sie das Modell
legal nutzen.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-27 (Model Auditing)
Ein Modell darf nur dann im Marktplatz gelistet werden, wenn sein
Fingerabdruck durch den Audit-Prozess von ATC-27 bestaetigt wurde.

> ATC-27 = Modell ist unmanipuliert. ATC-29 = Modell ist handelbar.

### 3.2 ATC-11 (Fungible Assets)
Die Zahlungen fuer Inferenz-Leistung im Marktplatz erfolgen in Token, die
diesem Standard entsprechen (ATCoin).

### 3.3 ATC-13 (Fractional Asset Ownership)
ATC-29 ermoeglicht es, dass ein besonders wertvolles KI-Modell (z. B. ein
Large Language Model) von einer Gruppe von Investoren (via ATC-13) gehalten
wird, die sich die laufenden Einnahmen aus dem Marktplatz teilen.

### 3.4 ATC-19 (AMM)
ATC-19 AMM Pools koennen fuer automatische Preisbildung von Inferenz-Diensten
genutzt werden. Dynamic Pricing basierend auf Angebot/Nachfrage.

### 3.5 ATC-24 (Task Orchestration)
ATC-24 verteilt die Aufgaben. ATC-29 definiert, wie die dafuer genutzten
Modelle abgerechnet werden.

### 3.6 ATC-28 (Federated Learning)
Durch ATC-28 verbesserte Modelle koennen neue Versionen im Marktplatz
veroeffentlichen. ATC-27 auditiert die neue Version, ATC-29 listet sie.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Modell-Registry | On-Chain mit Fingerprint | smart_contract_registry.py | PARTIAL Basis da |
| Marketplace Contract | Modelle handeln | marketplace_contract.py (#13) | OK Implementiert |
| Inferenz-Gebuehren | Auto-Ausschuettung ATC-11 | ATC-11 Token + Smart Contracts | PARTIAL Basis da |
| Service-Discovery | Agent-Suche | LLMRouter + AI-Kernel | OK Implementiert |
| Lizenz-Management | On-Chain Nutzungsrechte | Nicht implementiert | PARTIAL Geplant |
| Governance-Integration | DAO beeinflusst Modelle | governance_contract.py | PARTIAL Basis da |
| Core-Set-Modelle | Community-Standard-Modelle | Nicht implementiert | PARTIAL Geplant |
| ATC-13 Fraktional | Co-Ownership von Modellen | ATC-13 konzeptionell | PARTIAL Geplant |
| Dynamic Pricing | AMM-basierte Gebuehren | ATC-19 AMM implementiert | PARTIAL Basis da |
| Version-Management | Modell-Upgrades verwalten | Nicht implementiert | PARTIAL Geplant |

> **Fazit:** Die Basis (Marketplace Contract, Smart Contract Registry, ATC-11
> Token, LLMRouter, Governance) ist implementiert. Die KI-Modell-spezifischen
> Features (Modell-Registry mit Fingerprint, Inferenz-Gebuehren-Auto-Pay,
> Lizenz-Management) sind der naechste Schritt.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #1 | Smart Contracts (Base) | Done | ATC-29 Contract-Basis |
| #2 | Gemini AI Integration | Done | ATC-29 KI-Agent Basis |
| #13 | Marketplace | Done | ATC-29 Marketplace Contract |
| #50 | AI Kernel | Done | ATC-29 AI-Kernel + LLMRouter |
| #69 | Security-Audit | Open | ATC-29 Marketplace-Sicherheit |
| Sprint 3.0 | KI-Modell-Registry | Geplant | ATC-29 + ATC-27 Fingerprint |
| Sprint 3.0 | Inferenz-Gebuehren-Auto-Pay | Geplant | ATC-29 + ATC-11 Escrow |
| Sprint 3.0 | Lizenz-Management | Geplant | ATC-29 On-Chain Lizenzen |
| Sprint 3.0 | Core-Set-Modelle | Geplant | ATC-29 + ATC-17 DAO |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] KI-Modell-Registry: On-Chain mit ATC-27 Fingerprint + Lizenz + Parameter
- [ ] Inferenz-Gebuehren-Escrow: Auto-Pay bei erfolgreicher Inferenz
- [ ] Lizenz-Management: Nutzungsrechte On-Chain (Open Source, Commercial, API)
- [ ] Core-Set-Modelle: DAO-Voting fuer Standard-Modelle des OS
- [ ] Modell-Versioning: Upgrade-Pfad mit ATC-27 Audit pro Version
- [ ] Dynamic Pricing: ATC-19 AMM fuer Inferenz-Gebuehren
- [ ] ATC-13 Co-Ownership: Fraktionalisierte Modell-Eigentumsanteile
- [ ] Reputation-Listing: ATC-03 Score sichtbar im Marktplatz
- [ ] Datensatz-Marketplace: Trainingsdaten als handelbares Asset
- [ ] API-Key-Marketplace: Per-User Keys als Managed Service
- [ ] Modell-Reviews: Community-Bewertungen und Tests
- [ ] Revenue-Sharing: ATC-28 Federated Learning Beitraege werden verguetet

---

## 7. AI App Store Architektur

```
AI APP STORE (ATC-29)
├── Model Registry (On-Chain)
│   ├── Modell A (Fingerprint, Lizenz, Preis)
│   ├── Modell B (Fingerprint, Lizenz, Preis)
│   └── Modell C (Core-Set, DAO-approved)
├── Payment Layer (ATC-11)
│   ├── User -> Escrow -> Model Owner
│   ├── Dynamic Pricing (ATC-19 AMM)
│   └── Revenue Sharing (ATC-13 Co-Ownership)
├── Discovery Layer (ATC-24 + LLMRouter)
│   ├── Service-Search nach Capabilities
│   ├── Reputation-Filter (ATC-03)
│   └── Governance-Filter (ATC-17 Core-Set)
├── Audit Layer (ATC-27)
│   ├── Model-Fingerprint Verification
│   ├── Post-Update Audit (ATC-28)
│   └── Listing-Approval Workflow
└── Training Layer (ATC-28)
    ├── Federated Learning Updates
    ├── New Version -> Audit -> Registry
    └── Continuous Improvement
```

---

*Dieses Dokument wurde aus der urspruenglichen Atc-29.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:36 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| list_model | 50000 |
| rent_model | variable |
| rate_model | 1000 |

### Sprint-Zuweisung

- **Sprint 3.0** — #80
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** MEDIUM

### Testing

5+ Unit-Tests: List, Rent, Rate, Payment, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
