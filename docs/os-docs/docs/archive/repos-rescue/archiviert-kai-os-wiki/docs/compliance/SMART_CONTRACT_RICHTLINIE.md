# Smart-Contract-Richtlinie fuer BaFin-Compliance
## ATC-LIC Provisionsabwicklung & Urheberschutz auf Kernel-Ebene

> **Version:** 1.0.0 | **Datum:** 06.07.2026
> **Autor:** Michael Wroblewski / ShivaCore / A-TownChain-Okosystems
> **Status:** OFFIZIELL — Zur Einreichung bei der BaFin
> **Klassifikation:** Vertraulich — Compliance-Dokument
> **Referenz-Standard:** ATC-LIC v1.0

---

## 1. Zweck und Geltungsbereich

Dieses Dokument definiert die Smart-Contract-Richtlinie, die der BaFin
detailliert aufzeigt, wie die automatisierte Provisionsabwicklung und der
Urheberschutz auf Kernel-Ebene (ATVM) rechtssicher erzwungen werden.

**Geltungsbereich:** Alle Smart Contracts und Code-Module, die innerhalb der
A-TownChain Virtual Machine (ATVM) ausgefuehrt werden.

**Adressaten:**
- BaFin (Bundesanstalt fuer Finanzdienstleistungsaufsicht)
- Entwickler im A-TownChain-Oekosystem
- Node-Betreiber (Validator, Compute, Storage, Gateway)
- Auditing-Partner

---

## 2. Rechtliche Grundlage: "Code is Law"

### 2.1 Paradigmenwechsel
Die A-TownChain ersetzt das klassische Justizsystem zur Durchsetzung von
Urheberrechten durch Kryptografie und Smart Contracts. Die Durchsetzung
erfolgt nicht durch Gerichtsurteile, sondern durch die ATVM, die unlizanzierten
Code **physisch nicht ausfuehrt**.

### 2.2 Rechtsaequivalenz
| Klassisches Urheberrecht | A-TownChain (ATC-LIC) |
|--------------------------|----------------------|
| Gerichtliche Klage | ATVM verweigert Ausfuehrung |
| Schadensersatz nach Verletzung | Automatische Royalty vor Verletzung |
| Beweislast beim Urheber | Kryptografischer Beweis (DAG) |
| Juristische Unsicherheit | Deterministisch (ATC-14) |
| Lange Gerichtsverfahren | Echtzeit (Atomar) |
| Regionale Rechtsprechung | Global (Dezentral) |

### 2.3 Rechtssicherheit durch Kryptografie
- **ECDSA (secp256k1)** — Signatur beweist Urheberschaft (ATC-03 DID)
- **SHA-256** — Code-Hash identifiziert geschuetztes Werk eindeutig
- **ATC-04 DAG** — Unveraenderlicher Audit-Trail aller Ausfuehrungen
- **ATC-14** — Deterministische Ausfuehrung (kein Diskretionsspielraum)

---

## 3. Automatisierte Provisionsabwicklung

### 3.1 Royalty-Berechnungsmodell

Die Royalty wird deterministisch berechnet. Es gibt keine manuelle
Nachberechnung, keinen Ermessensspielraum, keine Ausnahmen.

```
ROYALTY = f(license_type, base_price, caller_did, network_state)

Berechnung:
  PER_CALL:       royalty = base_price
  SUBSCRIPTION:   royalty = 0 (wenn aktives Abo) | base_price (wenn abgelaufen)
  PERPETUAL:      royalty = 0 (wenn DID Besitzer) | base_price (wenn nicht)
  REVENUE_SHARE:  royalty = base_price * revenue_of_caller
  FREEMIUM:       royalty = 0 (Basis) | base_price (Premium)
  DAO_GOVERNED:   royalty = dao_resolution(dao_proposal_id)
```

### 3.2 Atomare Transaktion

Die Royalty-Zahlung und die Code-Ausfuehrung erfolgen **atomar** in einer
einzigen DAG-Transaktion. Das bedeutet:

1. **Entweder** Code wird ausgefuehrt UND Royalty wird ueberwiesen
2. **Oder** Code wird NICHT ausgefuehrt UND keine Royalty faellt an

Es gibt keinen Zwischenzustand. Ein "Ausfuehrung ohne Bezahlung" ist
kryptografisch ausgeschlossen.

```
ATVM ATOMIC EXECUTION TRANSACTION
├── BEGIN
│   ├── 1. License Registry Lookup (code_hash)
│   ├── 2. Royalty Calculation (license_type, price, caller)
│   ├── 3. Balance Check (caller_did >= royalty?)
│   │   ├── NO  -> REJECT (atomic rollback, nothing happens)
│   │   └── YES -> continue
│   ├── 4. ATC-11 Token Transfer (caller -> developer)
│   ├── 5. Code Execution (ATVM runtime)
│   ├── 6. Execution Log (ATC-04 DAG entry)
│   └── 7. License Registry Update (call_count++, total_royalties++)
├── COMMIT (all steps succeed) or ROLLBACK (any step fails)
└── Result: Executed + Paid | Blocked + Not Paid (no partial state)
```

### 3.3 Doppelte Buchfuehrung auf der Blockchain

Jede Royalty-Zahlung erzeugt zwei DAG-Eintraege:

1. **Token-Transfer** — ATC-11 von Caller zu Developer
2. **License-Event** — Ausfuehrungs-Log mit Code-Hash, Caller-DID, Royalty

Beide Eintraege sind unveraenderlich und oeffentlich auditierbar.

### 3.4 Abrechnungstransparenz fuer BaFin

Die BaFin kann jederzeit folgende Daten aus dem DAG extrahieren:

| Datenpunkt | Quelle | Verfuegbarkeit |
|-----------|--------|---------------|
| Alle Royalty-Zahlungen | DAG-Eintraege | Oeffentlich |
| Wer hat welchen Code ausgefuehrt | License-Events | Oeffentlich |
| Wie viel Royalty pro Code-Modul | License Registry | Oeffentlich |
| Gesamteinnahmen pro Developer | Registry Aggregation | Oeffentlich |
| Lizenz-Typ pro Code-Modul | Registry | Oeffentlich |
| Zeitstempel jeder Ausfuehrung | DAG | Oeffentlich |

> **Audit-Export:** Die BaFin kann einen vollstaendigen Compliance-Report als
> CSV/JSON aus dem Explorer (#5) oder dem IP & License Dashboard (GlobusOS)
> exportieren.

---

## 4. Urheberschutz auf Kernel-Ebene (ATVM)

### 4.1 Die ATVM als Hardware-nahe Ausfuehrungsumgebung

Die A-Town Virtual Machine (ATVM) ist die virtuelle Maschine, die Smart
Contracts und ATCLang-Code ausfuehrt. Sie ist Teil des ShivaOS-Kernels und
operiert unterhalb des Betriebssystems.

```
SCHICHTEN-MODELL
├── Layer 0: Hardware (TPM, CPU, RAM)
├── Layer 1: ShivaOS Kernel (ATS-1000+)
│   ├── ATC-LIC: Hardware-Zertifikat-Verifikation
│   └── Secure Boot: Kernel-Signatur-Pruefung
├── Layer 2: ATVM (A-Town Virtual Machine) ← HIER
│   ├── License Gate: Code-Hash -> Registry -> Royalty
│   ├── Execution Engine: ATCLang Bytecode
│   └── Isolation: Sandbox pro Ausfuehrung
├── Layer 3: Smart Contracts (ATC-01)
│   ├── License Registry Contract
│   ├── Royalty Payment Contract
│   └── DAO Governance Contract (ATC-17)
└── Layer 4: Anwendungen (DApps, KI-Agenten, Marketplace)
```

### 4.2 License Gate: Funktionsweise

Das License Gate ist **fest in die ATVM einkompiliert** und kann nicht
umgangen, deaktiviert oder modifiziert werden (ohne Kernel-Kompilierung).

```
ATVM LICENSE GATE (Pseudocode)
┌─────────────────────────────────────────────────────┐
│ FUNCTION execute(code_hash, caller_did, input):     │
│                                                     │
│   # 1. REGISTRY LOOKUP                              │
│   entry = LicenseRegistry.get(code_hash)            │
│   IF entry == NULL:                                 │
│     RETURN execute_public(code_hash, input)         │
│     // Public domain — free execution               │
│                                                     │
│   # 2. LICENSE CHECK                                │
│   licensed = check_license(entry, caller_did)       │
│   IF NOT licensed:                                  │
│     log_blocked(code_hash, caller_did)              │
│     RETURN LICENSE_DENIED                           │
│     // Code wird NICHT ausgefuehrt                  │
│                                                     │
│   # 3. ATOMIC ROYALTY + EXECUTION                   │
│   BEGIN TRANSACTION:                                │
│     royalty = calculate_royalty(entry, caller_did)  │
│     IF royalty > 0:                                 │
│       Token.transfer(caller_did,                    │
│                       entry.developer, royalty)     │
│     result = execute_sandboxed(code_hash, input)    │
│     LicenseRegistry.record_call(code_hash,          │
│                                 caller_did, royalty)│
│     DAG.log("LICENSE_EXEC", {                       │
│       code_hash, caller_did, developer,             │
│       royalty, timestamp, result_hash               │
│     })                                              │
│   COMMIT TRANSACTION                                │
│                                                     │
│   RETURN result                                     │
└─────────────────────────────────────────────────────┘
```

### 4.3 Unabanderbarkeit

Das License Gate ist **nicht umgehbar**:

| Angriffsvektor | Abwehr |
|----------------|--------|
| Code ohne Lizenz ausfuehren | ATVM blockt physisch |
| ATVM umgehen | ATVM ist im Kernel (ATS-1000+) |
| Kernel modifizieren | Secure Boot verhindert (ATC-LIC) |
| Registry manipulieren | Smart Contract on-chain (ATC-01) |
| Royalty umleiten | Atomic Transaction (kein Zwischenzustand) |
| Ausfuehrung loggen ohne Bezahlung | Atomar — beides oder keins |
| Node ohne TPM betreiben | ATC-LIC verweigert Netzwerk-Zugang |

### 4.4 Sandbox-Isolation

Jede Code-Ausfuehrung in der ATVM erfolgt in einer isolierten Sandbox:

- **Speicher-Isolation** — Code kann nicht auf anderen Speicher zugreifen
- **Netzwerk-Isolation** — Nur explizit erlaubte Verbindungen
- **Zeit-Limit** — Maximal erlaubte Ausfuehrungszeit
- **Ressourcen-Limit** — CPU/Memory-Schranken (ATC-37)
- **Dateisystem-Isolation** — Nur ATCFS-eigener Namespace

---

## 5. License Registry Smart Contract — Spezifikation

### 5.1 Interface

```solidity
// SPDX-License-Identifier: ATC-LIC
// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems
pragma solidity ^0.8.0;

interface ILicenseRegistry {
    // === Registration ===
    function registerCode(
        bytes32 codeHash,
        address developerDid,
        LicenseType licenseType,
        uint256 basePrice
    ) external returns (uint256 licenseId);

    function updateLicense(
        uint256 licenseId,
        LicenseType newType,
        uint256 newPrice
    ) external onlyDeveloper(licenseId);

    function revokeLicense(uint256 licenseId) external onlyDeveloper(licenseId);

    // === Execution Gate ===
    function checkLicense(
        bytes32 codeHash,
        address callerDid
    ) external view returns (bool licensed, uint256 royalty);

    function recordExecution(
        bytes32 codeHash,
        address callerDid,
        uint256 royaltyPaid
    ) external onlyATVM;

    // === Queries ===
    function getLicense(bytes32 codeHash) external view returns (
        address developer,
        LicenseType licenseType,
        uint256 basePrice,
        uint256 totalCalls,
        uint256 totalRoyalties,
        bool isActive
    );

    function getDeveloperEarnings(address developerDid) external view returns (
        uint256 totalCalls,
        uint256 totalRoyalties,
        uint256 activeLicenses
    );

    // === Events ===
    event CodeRegistered(bytes32 indexed codeHash, address indexed developer, LicenseType licenseType, uint256 basePrice);
    event LicenseUpdated(uint256 indexed licenseId, LicenseType newType, uint256 newPrice);
    event LicenseRevoked(uint256 indexed licenseId);
    event ExecutionRecorded(bytes32 indexed codeHash, address indexed caller, address indexed developer, uint256 royalty);
}
```

### 5.2 License Types Enum

```solidity
enum LicenseType {
    PER_CALL,        // 0 — Pro Ausfuehrung
    SUBSCRIPTION,    // 1 — Zeitbasiert (start, end)
    PERPETUAL,       // 2 — Einmalzahlung, lebenslang
    REVENUE_SHARE,   // 3 — % des Umsatzes
    FREEMIUM,        // 4 — Basis frei, Premium kostenpflichtig
    DAO_GOVERNED     // 5 — DAO bestimmt Preis
}
```

### 5.3 Royalty Calculator

```solidity
contract RoyaltyCalculator {
    ILicenseRegistry public registry;
    IToken public atc11Token;

    function calculateRoyalty(
        bytes32 codeHash,
        address callerDid
    ) public view returns (uint256 royalty) {
        (, LicenseType licenseType, uint256 basePrice,,,) = registry.getLicense(codeHash);

        if (licenseType == LicenseType.PER_CALL) {
            return basePrice;

        } else if (licenseType == LicenseType.SUBSCRIPTION) {
            if (hasActiveSubscription(callerDid, codeHash)) return 0;
            return basePrice;

        } else if (licenseType == LicenseType.PERPETUAL) {
            if (ownsPerpetualLicense(callerDid, codeHash)) return 0;
            return basePrice;

        } else if (licenseType == LicenseType.REVENUE_SHARE) {
            uint256 callerRevenue = getCallerRevenue(callerDid);
            return (callerRevenue * basePrice) / 10000; // basis points

        } else if (licenseType == LicenseType.FREEMIUM) {
            if (isPremiumFeature(codeHash, callerDid)) return basePrice;
            return 0;

        } else if (licenseType == LicenseType.DAO_GOVERNED) {
            return getDAOResolution(codeHash);
        }
    }

    function executeWithLicense(
        bytes32 codeHash,
        address callerDid
    ) external returns (bool) {
        uint256 royalty = calculateRoyalty(codeHash, callerDid);

        if (royalty > 0) {
            require(
                atc11Token.balanceOf(callerDid) >= royalty,
                "ATVM: Insufficient balance for license"
            );
            // Atomic transfer
            atc11Token.transferFrom(callerDid, getDeveloper(codeHash), royalty);
        }

        // Record execution
        registry.recordExecution(codeHash, callerDid, royalty);
        return true;
    }
}
```

### 5.4 ATCLang Implementation (Proprietaer)

```atc
// ATCLang v0.3.0 — License Registry
// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems

namespace ATC::License::Registry {

  contract LicenseRegistry {
    state entries: Map<Hash, LicenseEntry>;
    state developerEarnings: Map<DID, Earnings>;

    fn register_code(code_hash: Hash, developer: DID,
                     license_type: LicenseType, price: Amount) -> LicenseId {
      let entry = LicenseEntry {
        developer: developer,
        license_type: license_type,
        price: price,
        registered_at: block.timestamp,
        total_calls: 0,
        total_royalties: 0,
        active: true
      };
      self.entries.insert(code_hash, entry);
      emit CodeRegistered(code_hash, developer, license_type, price);
    }

    fn check_license(code_hash: Hash, caller: DID) -> (bool, Amount) {
      let entry = self.entries.get(code_hash);
      match entry {
        None => (true, 0),  // Public domain
        Some(e) => {
          if !e.active { return (false, 0); }
          let royalty = self.calculate_royalty(e, caller);
          if royalty > 0 && !token.has_balance(caller, royalty) {
            return (false, 0);  // Blocked
          }
          (true, royalty)
        }
      }
    }

    fn execute_with_license(code_hash: Hash, caller: DID, input: Data) -> Result {
      let (licensed, royalty) = self.check_license(code_hash, caller);
      if !licensed {
        emit LicenseDenied(code_hash, caller);
        return Err(LICENSE_DENIED);
      }

      // Atomic: Transfer + Execute
      atomic {
        if royalty > 0 {
          token.transfer(caller, self.entries[code_hash].developer, royalty);
        }
        let result = atvm.execute_sandboxed(code_hash, input);
        self.entries[code_hash].total_calls += 1;
        self.entries[code_hash].total_royalties += royalty;
        dag.log("LICENSE_EXEC", {
          code_hash, caller, developer, royalty, result
        });
      }
      Ok(result)
    }
  }
}
```

---

## 6. Royalty Payment Loop

### 6.1 End-to-End Flow

```
ENTWICKLER                    ATVM                    NUTZER
   |                           |                        |
   | 1. Code schreiben         |                        |
   | 2. code_hash = SHA-256    |                        |
   | 3. register_code()        |                        |
   |--------------------------->|                        |
   |                           | Registry updated       |
   |                           |                        |
   |                           |     4. execute_request |
   |                           |<-----------------------|
   |                           | 5. check_license()     |
   |                           | 6. calculate_royalty() |
   |                           | 7. balance_check()     |
   |                           |    balance >= royalty? |
   |                           |<-----------------------|
   |                           |    YES                 |
   |                           |                        |
   |                           | 8. ATOMIC TRANSACTION  |
   |    9. royalty received    |                        |
   |<---------------------------|                        |
   |                           | 10. code executed      |
   |                           |----------------------->|
   |                           |    result              |
   |                           |                        |
   | 11. earnings visible      |                        |
   |    in Dashboard           |                        |
   |<---------------------------|                        |
```

### 6.2 Subscription Lifecycle

```
SUBSCRIPTION LIFECYCLE
├── User subscribes
│   ├── pay_subscription(base_price)
│   ├── subscription_start = block.timestamp
│   ├── subscription_end = start + duration
│   └── DAG log: "SUBSCRIPTION_STARTED"
├── During subscription
│   ├── execute_with_license() -> royalty = 0
│   └── Unlimited calls within period
├── Subscription expires
│   ├── block.timestamp > subscription_end
│   ├── execute_with_license() -> royalty = base_price (per call)
│   └── User must renew or pay per-call
└── Renewal
    ├── pay_subscription(base_price)
    └── subscription_end += duration
```

### 6.3 DAO-Governed Pricing

```
DAO-GOVERNED LICENSE FLOW
├── Developer proposes price
│   ├── dao_propose("set_price", code_hash, new_price)
│   └── Voting period starts (ATC-17)
├── DAO votes
│   ├── Validators vote (reputation-weighted, ATC-30)
│   └── Quorum must be reached
├── Resolution
│   ├── If accepted: price updated in Registry
│   └── If rejected: price unchanged
└── Execution continues with new price
```

---

## 7. BaFin-Compliance: Audit-Checkliste

### 7.1 Provisionsabwicklung

- [x] Deterministische Royalty-Berechnung (ATC-14)
- [x] Atomare Transaktion (Transfer + Execution)
- [x] Kein manueller Eingriff moeglich
- [x] Vollstaendiger Audit-Trail im DAG (ATC-04)
- [x] Echtzeit-Abfrage aller Zahlungen moeglich
- [x] Export als CSV/JSON fuer Audits

### 7.2 Urheberschutz

- [x] Code-Hash (SHA-256) als eindeutige Werks-Identifikation
- [x] Developer-DID (ATC-03) als Urheberschaft-Nachweis
- [x] ECDSA-Signatur bei Registrierung
- [x] ATVM verweigert unlizanzierte Ausfuehrung (physisch)
- [x] Keine Ausnahme moeglich (kein "Override")
- [x] Sandbox-Isolation verhindert Manipulation

### 7.3 Transparenz

- [x] Alle Lizenz-Zahlungen oeffentlich im DAG
- [x] Explorer (#5) fuer BaFin-Zugriff
- [x] IP & License Dashboard (GlobusOS) fuer Echtzeit-Monitoring
- [x] Compliance-Report-Export verfuegbar
- [x] Developer-Einnahmen einsehbar

### 7.4 Hardware-Sicherheit

- [x] TPM-Attestation fuer jeden Node (ATC-LIC)
- [x] Secure Boot verhindert Kernel-Manipulation
- [x] Tamper-Detection erkennt Hardware-Veraenderung
- [x] Nur lizenzierte Hardware im Netzwerk

### 7.5 Governance

- [x] DAO (ATC-17) kann Regeln anpassen (nur via On-Chain Voting)
- [x] Keine zentrale Instanz kann Lizenzen "ausschalten"
- [x] Alle Governance-Entscheidungen im DAG
- [x] Reputation-System (ATC-30) fuer Vertrauen

---

## 8. Incident-Response

### 8.1 Lizenz-Verletzung
Eine Lizenz-Verletzung ist **per Definition nicht moeglich**, da die ATVM
unlizanzierten Code nicht ausfuehrt. Es gibt keinen "Incident" im klassischen
Sinne.

### 8.2 Fehlerhafte Royalty-Berechnung
Sollte durch einen Smart-Contract-Bug eine falsche Royalty berechnet werden:

1. **DAO-Proposal** (ATC-17) fuer Korrektur
2. **Rueckbuchung** via DAO-genehmigter Transaktion
3. **Audit-Trail** der Korrektur im DAG
4. **BaFin-Benachrichtigung** (falls betreffend)

### 8.3 Registry-Ausfall
Die License Registry ist ein Smart Contract on-chain. Ein "Ausfall" ist nur
durch Netzwerk-Partition moeglich:

1. **ATC-02 (Liquid State)** — State-Migration bei Node-Ausfall
2. **ATC-04 (DAG)** — Konsens stellt Konsistenz sicher
3. **Graceful Degradation** — ATVM kann "public domain" Code weiterhin ausfuehren

---

## 9. Referenzen

| Dokument | Bezug |
|---------|------|
| [ATC-LIC Spezifikation](../standards/ATC-LIC-SMART_CONTRACT_LICENSE.md) | Hauptstandard |
| [ATC-LIC Spezifikation](../standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md) | Hardware-Lizenzen |
| [Compliance-Handbuch](COMPLIANCE_HANDBUCH.md) | Uebergeordnetes Handbuch |
| [Lizenz-Uebersicht](../LICENSING_OVERVIEW.md) | Zentrale Uebersicht |
| ATC-01 | Smart Contracts |
| ATC-03 | Decentralized Identity |
| ATC-04 | DAG Consensus |
| ATC-11 | Fungible Token |
| ATC-14 | Deterministic Execution |
| ATC-17 | DAO Governance |
| ATC-30 | Reputation-Scoring |
| ATC-37 | Resource Allocation |
| ATS-1000+ | ShivaOS Kernel Standards |

---

*Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.*
*Dieses Dokument ist vertraulich und Teil des A-TownChain Compliance-Handbuchs.*
*Letzte Aktualisierung: 06.07.2026 | Aurora (Superagent)*
