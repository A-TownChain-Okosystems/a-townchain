# ATC-03 — Decentralized Identity (DID) & Zero-Trust IAM
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.3 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-03
> **Referenzen:** ATC-01 (Handshake/Identity), ATC-02 (State Migration), ATC-0002 (Wallet/ECDSA), ATC-91 (Governance)
> **Quelldatei:** Atc-03.docx (ursprüngliche Spezifikation)
> **Kategorie:** P2P Networking  
> **Tier:** Tier 1 — Blockchain Infrastructure  

---

## Abstract

ATC-03 ist das Identitäts- und Sicherheitsfundament von KAI-OS und steht für
Decentralized Identity (DID) & Zero-Trust IAM (Identity and Access Management).

In einem dezentralen Betriebssystem, in dem jeder Node eigenständig Berechnungen
durchführen kann, muss sichergestellt sein, dass Entitäten (Nutzer, KI-Agenten
oder Hardware-Nodes) genau das sind, was sie vorgeben zu sein — ohne eine
zentrale Instanz (wie z. B. einen OAuth-Server von Google oder Microsoft).

> **Merksatz:** ATC-01 = Verbindung. ATC-02 = Gedächtnis.
> ATC-03 = Verifizierte Identität und autorisierter Zugriff.

---

## 1. Kernkonzept

ATC-03 definiert, wie Identitäten im A-TownChain-Ökosystem erstellt, verifiziert
und autorisiert werden:

### 1.1 Decentralized Identity (DID)
Jeder Akteur besitzt eine kryptografische Identität, die nicht auf einem Server
gespeichert ist, sondern auf der Blockchain verankert bleibt. Diese Identität
kann für den Login, die Signatur von Inferenz-Aufträgen oder die
Governance-Teilnahme verwendet werden.

**Implementation:** ✅ Teilweise implementiert
- `blockchain/wallet/keygen.py` — Generiert kryptografische Identität
  - 256-bit Entropy → 24-Wort Seed Phrase (BIP39-kompatibel)
  - Private Key (SHA-256, 64 hex chars)
  - Public Key (SHA-256 des Private Key)
  - ATC-Adresse: `ATC` + 32 Zeichen
- `blockchain/wallet/ecdsa.py` — ECDSA secp256k1 Signatur-System
  - `generate_keypair()` → (private_key_hex, public_key_hex)
  - `sign(private_key, data)` → Signature
  - `verify(public_key, data, signature)` → bool
- `blockchain/wallet/multisig.py` — Multi-Signature Wallet (M-of-N, Issue #24)
- `mobile/wallet/biometric_auth.py` — Biometrische Auth (FaceID/TouchID, Issue #46)

> **Status:** Kryptografische Identität (ATC-Adresse + ECDSA) ist voll
> implementiert. Die Verankerung als on-chain DID-Record (mit Metadaten wie
> Name, Rolle, Reputation) ist ein Zukunftsfeature.

### 1.2 Zero-Trust IAM
Das System verfolgt das Prinzip "Never Trust, Always Verify". Jeder Zugriff
auf Ressourcen (z. B. wenn ein KI-Agent GPU-Leistung anfordert) erfordert
eine kryptografische Autorisierung gemäß ATC-03. Ein Node prüft bei jedem
Aufruf die Signatur, den Reputation-Score und die Berechtigung der Identität.

**Implementation:** ✅ Teilweise implementiert
- `modules/gateway/middleware/signature_verify.py` — `require_signature` Decorator
  - Verifiziert ECDSA-Signatur bei jedem API-Aufruf
  - Extrahiert Public Key aus Request-Header
  - Lehnt Requests ohne gültige Signatur ab
- `modules/gateway/middleware/auth.py` — API-Key-Authentifizierung
- `modules/gateway/middleware/rate_limit.py` — Rate-Limiting (100 req/60s/peer)
- `gateway/main.py` — Gateway auf Port 4000, alle Traffic routed durch Middleware

```python
# signature_verify.py
@require_signature
def protected_endpoint():
    # Nur erreichbar mit gültiger ECDSA-Signatur
    ...
```

> **Status:** Signatur-Verifizierung bei API-Aufrufen ist implementiert.
> Reputation-Score-basierte Zugriffskontrolle (z. B. "nur Agenten mit
> Score > 0.8 dürfen GPU-Leistung anfordern") ist geplant.

### 1.3 Verifiable Credentials (VCs)
Identitäten können durch andere Identitäten "beglaubigt" werden (z. B. ein
"Sicherheits-Audit-Zertifikat" für einen Inferenz-Node). Dies geschieht über
offline verifizierbare, signierte Aussagen, die den Vertrauenswert innerhalb
des Mesh-Netzwerks erhöhen.

**Implementation:** 📐 Geplant
- Das Framework für VCs (signierte Aussagen von Identität A über Identität B)
  ist konzipiert aber noch nicht implementiert
- On-Chain Logging im AI-Kernel (`ai_kernel.py`) legt bereits Hashes aller
  KI-Entscheidungen auf der Chain ab — dies könnte als Basis für
  Reputation-VCs dienen

---

## 2. Warum ATC-03 kritisch ist

Ohne ATC-03 wäre das gesamte Netzwerk anfällig für Identitätsdiebstahl oder
unautorisierte Ressourcennutzung:

### 2.1 Schutz vor Sybil-Attacken
Durch ATC-03 wird jede Identität an einen kryptografischen Aufwand gebunden.
Ein Angreifer kann nicht einfach Millionen von Identitäten erstellen, ohne
deren Reputation oder kryptografische Verankerung nachweisen zu müssen.

**Implementation:** ✅ Teilweise
- PoS-Mindeststake: 10.000 ATC (`blockchain/consensus/pos.py`)
- Node-Registrierung erfordert gültige ATC-Adresse + Signatur
- Sybil-Resistenz durch Stake (je mehr Identitäten, desto mehr ATC nötig)
- **Geplant:** PoW für Node-Registrierung (siehe ATCNET Security.md)

### 2.2 Granulare Berechtigungen (RBAC)
ATC-03 erlaubt es, Rollen und Berechtigungen (RBAC — Role Based Access Control)
direkt auf der Chain abzubilden. Ein Inferenz-Node kann z. B. so konfiguriert
werden, dass er nur Aufgaben von Agenten mit einem bestimmten Vertrauenswert
annimmt.

**Implementation:** ⚠️ Teilweise
- Node-Rollen existieren: `FULL`, `LIGHT`, `VALIDATOR`, `MINER` (`blockchain/nodes/node.py`)
- Governance-Rollen: Proposer, Voter (`governance_contract.py`)
- Task-Typen im Orchestrator: Blockchain, Wallet, AI, Game, Governance, etc.
- **Geplant:** On-Chain RBAC mit granularen Berechtigungen pro Identität

### 2.3 Interoperabilität
Da es sich um offene Standards (DID-Spezifikationen) handelt, kann die
Identität eines Nutzers auch über die Grenzen von KAI-OS hinaus für andere
Web3-Dienste verwendet werden.

**Implementation:** 📐 Geplant
- ATC-Adressen sind proprietär (`ATC` + 32 chars)
- ECDSA (secp256k1) ist branchenstandard
- **Geplant:** W3C DID-Spezifikation-konforme DID-Records für Cross-Chain-Kompatibilität

---

## 3. Einordnung in die Architektur

ATC-03 arbeitet eng mit anderen Tiers zusammen:

### 3.1 Tier 1 (Netzwerk)
Die Identität aus ATC-03 wird während des Handshakes (ATC-01) genutzt, um
den Node zu authentifizieren.

**Implementation:** ATC-01 Handshake (`HELLO`/`HELLO_ACK`) überträgt
`sender` (ATC-Adresse) und `signature` (ECDSA). Empfänger validiert die
Signatur vor der Verbindung.

### 3.2 Tier 2 (Smart Contracts)
Die DAO-Governance nutzt ATC-03, um sicherzustellen, dass nur
stimmberechtigte und verifizierte Entitäten über Änderungen im Protokoll
abstimmen.

**Implementation:** ✅ Implementiert
- `governance_contract.py` (ATC-91 DAO, Issue #9)
- Proposal-Status: PENDING → ACTIVE → PASSED/REJECTED → EXECUTED
- Vote-Choices: YES, NO, ABSTAIN
- Stimmgewicht basiert auf Stake (Token-basiert)
- Nur verifizierte ATC-Adressen können Proposals erstellen und abstimmen

### 3.3 Tier 4 (KI-Agenten)
Jeder autonome Agent benötigt eine Identität nach ATC-03, um in seinem Namen
(z. B. bei der Abrechnung via AMM) zu handeln.

**Implementation:** ✅ Teilweise
- `ai_kernel.py` — AIKernel als autonomer Akteur
- On-Chain Decision-Logging: Jede KI-Entscheidung wird gehasht auf der Chain
  gespeichert (Auditierbarkeit)
- MultiSig Wallet für Bridge & Franchise Vault (`multisig.py`)
- **Geplant:** Eigene DID pro KI-Agent mit eigenem Reputation-Score

---

## 4. Spezifikation vs. Implementation — Zusammenfassung

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Decentralized Identity | Blockchain-verankerte DID | ATC-Adresse + ECDSA (secp256k1) | ⚠️ Teilweise |
| Zero-Trust IAM | "Never Trust, Always Verify" | Signature-Verify Middleware | ✅ Implementiert |
| Verifiable Credentials | Signierte Beglaubigungen | On-Chain Decision-Hashes (Basis) | 📐 Geplant |
| Sybil-Schutz | Krypto. Aufwand pro Identity | PoS Stake (10.000 ATC) | ⚠️ Teilweise |
| RBAC | On-Chain Rollen/Berechtigungen | Node-Rollen + Governance-Rollen | ⚠️ Teilweise |
| Interoperabilität | W3C DID-konform | ATC-Adresse (proprietär) + ECDSA | 📐 Geplant |
| ECDSA-Signaturen | Krypto. Autorisierung | ✅ secp256k1 Sign/Verify | ✅ Implementiert |
| MultiSig | M-of-N Schema | ✅ multisig.py (Issue #24) | ✅ Implementiert |
| Biometrische Auth | Mobile FaceID/TouchID | ✅ biometric_auth.py (Issue #46) | ✅ Implementiert |
| Governance-Voting | Nur verifizierte Entitäten | ✅ ATC-91 DAO (Issue #9) | ✅ Implementiert |

> **Fazit:** Die kryptografische Basis (ECDSA, MultiSig, Biometric, Governance)
> ist voll implementiert. Die erweiterten DID-Konzepte (on-chain DID-Records,
> Verifiable Credentials, Reputation-Score, W3C-Interoperabilität) sind
> konzeptionell final und als nächste Entwicklungsphase geplant.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #6 | ECDSA-Implementation | ✅ Done | ATC-03 Krypto. Identity |
| #9 | Governance (ATC-91) | ✅ Done | ATC-03 DAO-Voting |
| #24 | MultiSig Wallet | ✅ Done | ATC-03 M-of-N Auth |
| #46 | Mobile Biometric Auth | ✅ Done | ATC-03 Mobile Identity |
| #69 | Security-Audit | 🟡 Open | ATC-03 Identity-Sicherheit |
| Sprint 2.3 | DID-Records auf Chain | 📐 Geplant | ATC-03 Vollimplementation |
| Sprint 2.3 | Verifiable Credentials | 📐 Geplant | ATC-03 VC Framework |
| Sprint 2.3 | Reputation-Score System | 📐 Geplant | ATC-03 Zero-Trust scoring |

---

## 6. Verbesserungsvorschläge (Zukunft)

- [ ] On-Chain DID-Records (Adresse + Metadaten + Rolle + Reputation)
- [ ] Verifiable Credentials Framework (signierte Aussagen zwischen Identitäten)
- [ ] Reputation-Score basierend on On-Chain-History + VCs
- [ ] RBAC Smart Contract (granulare Berechtigungen pro Identität)
- [ ] W3C DID-Spezifikation-konforme DID-Method (`did:atc:...`)
- [ ] PoW für Node-Registrierung (Sybil-Schutz-Erweiterung)
- [ ] KI-Agent-DID: Eigene Identität + Reputation pro autonomem Agent
- [ ] Cross-Chain Identity Verification (Solana Bridge, ETH Bridge)

---

*Dieses Dokument wurde aus der ursprünglichen Atc-03.docx Spezifikation
abgeleitet und mit der tatsächlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:21 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| create_did | 50000 |
| resolve_did | 100 |
| verify_credential | 200 |
| revoke_did | 10000 |

### Sprint-Zuweisung

- **Sprint 2.6** — #78
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

6+ Unit-Tests: DID-Creation, Resolution, Credential-Verify, Revocation

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
