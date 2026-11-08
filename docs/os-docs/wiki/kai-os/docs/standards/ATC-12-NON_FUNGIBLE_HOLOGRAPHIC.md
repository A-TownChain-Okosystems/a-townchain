# ATC-12 — Non-Fungible & Holographic Asset Standard
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.5 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-12
> **Tier:** 2 (Logik & Oekonomie)
> **Referenzen:** ATC-11 (Fungible Assets), ATC-14 (Deterministic Exec, geplant), Issue #11 (Shivamon Breeding), Issue #13 (Marketplace)
> **Quelldatei:** Atc-12.docx (urspruengliche Spezifikation)
> **Kategorie:** Economy & Assets  

---

## Abstract

ATC-12 definiert den Non-Fungible & Holographic Asset Standard. Waehrend ATC-11
fuer austauschbare Werte steht, ist ATC-12 das Rueckgrat fuer alles, was im
KAI-OS einzigartig ist.

Die Besonderheit im KAI-OS ist das Attribut "Holographic", das ueber den
klassischen NFT-Standard (wie ERC-721) hinausgeht.

> **ATC-11 = Fungible Werte. ATC-12 = Einzigartige, holographische Objekte.**
> Tier 2 definiert die wirtschaftliche Logik und Asset-Spezifikationen.

---

## 1. Kernkonzepte

### 1.1 Einzigartigkeit (Non-Fungible)
Jedes Asset hat eine weltweit eindeutige ID (TokenID) und kann nicht durch ein
anderes gleiches Asset ersetzt werden. Dies ist die Grundlage fuer digitale
Sammlerstuecke, In-Game-Items (wie die Shivamon-NFTs) oder spezifische
Nutzungsrechte.

**Implementation:** OK Implementiert
- `blockchain/contracts/shivamon/shivamon_contract.py` — Shivamon NFT Contract
  - Eindeutige TokenID pro Shivamon
  - Metadaten: Name, Level, Attribute, Genetik
  - ATC-90 Standard (NFT)
- `blockchain/contracts/marketplace/marketplace_contract.py` — NFT Marketplace
  - Listing, Buying, Bidding fuer einzigartige Assets
  - Issue #13 abgeschlossen

### 1.2 Holographische Struktur
Dies ist das Alleinstellungsmerkmal von ATC-12. Ein holographisches Asset ist
kein statisches Bild oder Datenpaket, sondern ein dynamisches Objekt. Es enthaelt:

**Metadaten-Referenzen:** Links zu On-Chain-Daten oder dezentralem Speicher (IPFS).
**Verhaltens-Skripte:** Das Asset kann eigene Logik ausfuehren (z. B. ein digitales
Haustier, das "waechst" oder "agiert").
**Status-Container:** Ein ATC-12-Asset kann andere Assets (oder Token via ATC-11)
"in sich tragen" (Parent-Child-Relationship).

**Implementation:** Teilweise implementiert
- Shivamon NFTs haben dynamische Metadaten (Level, XP, Attribute)
- Shivamon Breeding (Issue #11) generiert neue NFTs mit geerbten Genen
- **Geplant:** Wasm-basierte Verhaltens-Skripte innerhalb von NFTs
- **Geplant:** Parent-Child-Relationship (NFT enthaelt andere NFTs/Token)

### 1.3 Vollstaendige Interoperabilitaet
Wie bei ATC-11 schreibt ATC-12 eine standardisierte ABI (Application Binary
Interface) vor, damit das Betriebssystem, Marktplaetze (siehe Issue #13) und
KI-Agenten die Eigenschaften des Assets (Name, Faehigkeiten, aktuelle Zustaende)
konsistent lesen und schreiben koennen.

**Implementation:** OK Implementiert
- `ownerOf(tokenId)` — Besitzerabfrage
- `transferFrom(from, to, tokenId)` — Uebertragung
- `tokenURI(tokenId)` — Metadaten-Referenz
- `mint(to, metadata)` — Neues NFT erstellen
- Marketplace nutzt diese Interface-Methoden fuer sicheren Handel

---

## 2. Warum ATC-12 fuer KAI-OS essenziell ist

### 2.1 Komplexitaet der KI-Services
KI-Agenten oder digitale Avatare sind komplexe Einheiten. Ein ATC-12-Standard
erlaubt es, einen KI-Avatar nicht nur als "Besitz" zu fuehren, sondern ihn als
aktives Objekt im KAI-OS zu behandeln, das mit anderen Objekten interagieren kann.

**Bezug:** AI-Kernel (`ai_kernel.py`) mit DecisionEngine. KI-Agenten als
ATC-12 NFTs mit Verhaltens-Skripten — konzeptionell fuer Zukunft.

### 2.2 Shivamon-Oekosystem
Das in der Roadmap erwaehnte Shivamon-Breeding (Issue #11) nutzt ATC-12, um neue,
einzigartige Shivamon-NFTs zu generieren, die ihre genetischen Merkmale
(Metadaten) aus den Eltern-Assets erben und dauerhaft im Ledger verankern.

**Implementation:** OK Implementiert
- `shivamon_contract.py` — Shivamon NFT Contract mit Genetik-System
- Breeding: Eltern-NFTs -> Kind-NFT mit geerbten Attributen
- Issue #11 abgeschlossen

### 2.3 Sichere Besitzverhaeltnisse
Da KAI-OS ein OS ist, dient ATC-12 auch zur Verwaltung von System-Ressourcen-Rechten.
Ein Nutzer kann beispielsweise das "Besitzrecht an einem KI-Modell" als ATC-12-Asset
halten.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-14 (Deterministic Execution)
Da ein holographisches Asset oft Logik ausfuehrt (z. B. bei einer Interaktion),
muss diese Logik auf allen Nodes deterministisch ablaufen.

> ATC-14 ist ein zukuenftiger Standard — sichert ATC-12 Verhaltens-Skripte.

### 3.2 Issue #11 (Shivamon Breeding) & #13 (Marketplace)
Diese Features sind direkte Anwendungen des ATC-12-Standards. Der Marktplatz
nutzt die ATC-12-Interface-Methoden (ownerOf, transferFrom), um den Handel
dieser Unikate sicher abzuwickeln.

**Status:** Beide Issues abgeschlossen (#11 Breeding, #13 Marketplace).

### 3.3 Tier 2 (Logik & Oekonomie)
ATC-12 ist die technologische Erweiterung von ATC-11 fuer Unikate.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Non-Fungible Token | ERC-721-Aequivalent | ATC-90 Standard, Shivamon | OK Implementiert |
| Eindeutige TokenID | Weltweit eindeutige ID | Shivamon Contract mit TokenID | OK Implementiert |
| Metadaten-Referenzen | Links zu On-Chain/IPFS | Metadaten in NFT (Name, Level, Gene) | OK Implementiert |
| Verhaltens-Skripte | Wasm-Logik im NFT | Statische Metadaten (noch kein Wasm) | PARTIAL Geplant |
| Status-Container | Parent-Child Relationship | Noch nicht implementiert | PARTIAL Geplant |
| ownerOf() | Besitzerabfrage | Implementiert | OK Implementiert |
| transferFrom() | Uebertragung | Implementiert | OK Implementiert |
| tokenURI() | Metadaten-URI | Metadaten direkt on-chain | OK Implementiert |
| Marketplace Integration | Handel mit Unikaten | marketplace_contract.py (Issue #13) | OK Implementiert |
| Shivamon Breeding | Genetische Vererbung | shivamon_contract.py (Issue #11) | OK Implementiert |
| Holographische Logik | Dynamisches Objekt | Dynamische Metadaten (Level/XP) | PARTIAL Basis da |

> **Fazit:** Die Kern-Funktionalitaet (NFT-Minting, Metadaten, Breeding,
> Marketplace) ist voll implementiert. Die "holographische" Erweiterung
> (Wasm-Verhaltens-Skripte, Parent-Child-Container) ist der naechste Schritt.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #3 | Battle UI | Done | ATC-12 Shivamon Darstellung |
| #11 | Shivamon Breeding | Done | ATC-12 Genetik-Vererbung |
| #13 | Marketplace | Done | ATC-12 NFT-Handel |
| #69 | Security-Audit | Open | ATC-12 NFT-Sicherheit |
| Sprint 2.5 | Wasm-Verhaltens-Skripte | Geplant | ATC-12 Holographische Logik |
| Sprint 2.5 | Parent-Child-Container | Geplant | ATC-12 Status-Container |
| Sprint 2.5 | KI-Avatar als NFT | Geplant | ATC-12 KI-Agent als Asset |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Wasm-Verhaltens-Skripte: NFTs fuehren eigene Logik aus (z. B. "wachsen")
- [ ] Parent-Child-Relationship: NFT enthaelt andere NFTs/Token
- [ ] IPFS-Metadaten: Externe Metadaten-Speicherung fuer grosse Assets
- [ ] KI-Avatar NFT: KI-Agent als ATC-12 Asset mit Verhaltens-Skript
- [ ] ATC-14 Integration: Deterministische Execution fuer NFT-Logik
- [ ] Dynamic NFT Evolution: Assets veraendern sich basierend on On-Chain-Events
- [ ] NFT-Staking: ATC-12 Assets in ATC-11 Yield-Pools nutzen
- [ ] Cross-Chain NFT Bridge: ATC-12 NFTs via ATC-09 auf andere Chains
- [ ] NFT-Fragmentierung: ATC-13 Fractional Ownership fuer teure NFTs
- [ ] Holographic-Explorer UI: Visuelle Darstellung der dynamischen Assets

---

*Dieses Dokument wurde aus der urspruenglichen Atc-12.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:44 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| mint_nft | 50000 |
| transfer_nft | 25000 |
| owner_of | 100 |

### Sprint-Zuweisung

- **Sprint 2.5** — geplant
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

6+ Unit-Tests: Mint, Transfer, Owner-Of, Metadata, Double-Mint

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
