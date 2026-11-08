# ATC-LIC — Smart Contract License Protocol
> **Status:** DRAFT — Spezifikation fuer Compliance-Handbuch (BaFin-konform) | **Version:** 1.0.0 | **Datum:** 05.07.2026 23:18
> **Autor:** Michael Wroblewski / ShivaCore, Aurora (Superagent)
> **Standard-ID:** ATC-LIC
> **Tier:** Lizenz-Layer (uebergreifend)
> **Referenzen:** ATC-01 (Smart Contracts), ATC-03 (Identity), ATC-04 (DAG), ATC-11 (Token), ATC-17 (DAO), ATC-29 (Marketplace), ATC-30 (Reputation), ATC-37 (Resource Allocation)

---

## Abstract

ATC-LIC definiert das Smart Contract License Protocol fuer das A-TownChain-
Oekosystem. Es ersetzt das klassische Justizsystem zur Durchsetzung von
Urheberrechten durch Kryptografie und Smart Contracts.

**"Code is Law" wird hier auf der Lizenzebene wortwoertlich genommen:**
Unlizenzierter Code wird von der A-Town Virtual Machine (ATVM) physisch gar
nicht erst ausgefuehrt.

> **ATC-LIC = Open Source + automatische Royalty-Durchsetzung durch die ATVM.**
> Der Quellcode ist fuer das Oekosystem zugaenglich, aber jede Ausfuehrung
> loest automatisch und unaufhaltsam eine Provisionszahlung an den Entwickler aus.

---

## 1. Paradigma: Monetarisiertes Autonomes Open-Source-Oekosystem

Das A-TownChain-Oekosystem strebt weder eine traditionell geschlossene B2B-
Software noch ein unentgeltliches Open-Source-Modell an. Stattdessen etabliert
es ein **monetarisiertes, autonomes Open-Source-Oekosystem**.

| Eigenschaft | Klassisches Open Source | Klassisches B2B | A-TownChain (ATC-LIC) |
|-------------|------------------------|-----------------|----------------------|
| Quellcode | Frei verfuegbar | Geschlossen | Frei verfuegbar (im Oekosystem) |
| Ausfuehrung | Frei | Lizenzschluessel | ATVM-Gated (Smart Contract) |
| Verguetung | Keine / Spenden | Lizenzgebuehr | Automatische Royalty (ATC-11) |
| Durchsetzung | Gerichtlich | Gerichtlich | Kryptografisch (Code is Law) |
| Skalierbarkeit | Hoch | Beschraenkt | Hoch (Open Source + Monetarisierung) |
| Innovation | Hoch | Beschraenkt | Hoch (Offen + belohnt) |

---

## 2. Kernkonzepte

### 2.1 ATVM License Gate

Die A-Town Virtual Machine (ATVM) ist die Ausfuehrungsumgebung fuer Smart
Contracts und ATCLang-Code. Vor jeder Ausfuehrung eines geschuetzten Code-Moduls
prueft die ATVM:

1. **License Registry Lookup** — Ist der Code-Hash in der ATC-LIC Registry?
2. **Caller Identity** — Wer ruft auf? (ATC-03 DID)
3. **License Type** — Welche Lizenzart? (Per-call, Subscription, Perpetual)
4. **Royalty Calculation** — Wie viel ATC-11 Token faellig?
5. **Payment Verification** — Hat der Caller ausreichend Token?
6. **Execution Gate** — Ja: Ausfuehren + Royalty transfer. Nein: ABLEHNEN.

```
ATVM EXECUTION GATE
├── Code-Hash -> License Registry Lookup
├── If NOT in Registry -> FREE EXECUTION (public domain)
├── If IN Registry:
│   ├── Check Caller DID (ATC-03)
│   ├── Check License Type:
│   │   ├── PER_CALL: Royalty = price_per_execution
│   │   ├── SUBSCRIPTION: Check active subscription (time-based)
│   │   └── PERPETUAL: Check if DID owns perpetual license
│   ├── Check Payment:
│   │   ├── Caller balance >= Royalty?
│   │   ├── If YES: Execute + ATC-11 Transfer to Developer
│   │   └── If NO: REJECT (unlicensed execution blocked)
│   └── Log execution in DAG (ATC-04)
└── Result: Code executed OR blocked (kryptografisch erzwungen)
```

> **Physische Durchsetzung:** Unlizenzierter Code wird nicht "illegal" — er wird
> **unmoeglich**. Die ATVM kann ihn schlicht nicht ausfuehren.

### 2.2 License Types

| Typ | Beschreibung | Royalty | Use Case |
|-----|-------------|---------|----------|
| `PER_CALL` | Pro Ausfuehrung | Fix pro Call | API-Funktionen, Inferenz |
| `SUBSCRIPTION` | Zeitbasiert | Periodisch | DApps, Services |
| `PERPETUAL` | Einmalig | Einmalzahlung | Tools, Libraries |
| `REVENUE_SHARE` | Umsatzbeteiligung | % des Umsatzes | Marktplatz-Integration |
| `FREEMIUM` | Basis frei, Premium kostenpflichtig | Premium-Features | Endnutzer-Apps |
| `DAO_GOVERNED` | Von DAO bestimmt | Variabel | Systemkritische Module |

### 2.3 Developer Registration

Entwickler registrieren ihren Code in der License Registry:

```python
# ATC-LIC Developer Registration (Smart Contract)
class LicenseRegistry:
    def register_code(self, code_hash: str, developer_did: str,
                      license_type: str, price: float):
        """
        Registriert Code in der ATC-LIC Registry.
        Ab diesem Moment wird jede ATVM-Ausfuehrung royalty-pflichtig.
        """
        self.registry[code_hash] = {
            "developer": developer_did,       # ATC-03 DID
            "license_type": license_type,      # PER_CALL / SUBSCRIPTION / ...
            "price": price,                    # ATC-11 Token amount
            "registered_at": block_timestamp,
            "total_calls": 0,
            "total_royalties": 0,
            "version": "1.0.0"
        }
    
    def execute_with_license(self, code_hash: str, caller_did: str):
        """ATVM ruft dies vor jeder Code-Ausfuehrung auf."""
        entry = self.registry.get(code_hash)
        if not entry:
            return True  # Public domain, free execution
        
        royalty = self._calculate_royalty(entry, caller_did)
        if royalty > 0:
            if not self._check_balance(caller_did, royalty):
                return False  # BLOCKED — unlicensed execution
        
        # Atomic: Transfer royalty + execute
        self._transfer_royalty(caller_did, entry["developer"], royalty)
        entry["total_calls"] += 1
        entry["total_royalties"] += royalty
        return True  # Licensed — execute
```

### 2.4 IP & License Registry Dashboard

Innerhalb von GlobusOS gibt es ein dediziertes Dashboard, das Lizenzen, Patente
und Hardware-Zertifikate sichtbar macht. Entwickler und Node-Betreiber koennen:

- **Lizenzen verwalten** — Registrierung, Preis-Aenderung, Lizenz-Typ
- **Einnahmen einsehen** — Echtzeit-Royalty-Stream pro Code-Modul
- **Ausfuehrungen tracken** — Wer hat wann welchen Code ausgefuehrt
- **Patente verwalten** — IP-Referenzen im DAG
- **Hardware-Zertifikate** — ATC-LIC TPM-Verifikation

---

## 3. Smart-Contract-Richtlinie fuer BaFin-Compliance

### 3.1 Automatisierte Provisionsabwicklung

Die ATC-LIC Provisionsabwicklung ist vollstaendig automatisiert und
deterministisch (ATC-14). Es gibt keine manuelle Abrechnung:

1. **Deterministische Berechnung** — Royalty = f(License Type, Price, Caller)
2. **Atomare Ausfuehrung** — Transfer + Code-Execution in einer Transaktion
3. **Unabanderbarkeit** — Smart Contract kann nicht umgangen werden
4. **Transparenz** — Jede Royalty-Zahlung ist im DAG (ATC-04) verankert
5. **Auditierbarkeit** — Vollstaendige Historie aller Lizenz-Zahlungen

### 3.2 Urheberschutz auf Kernel-Ebene (ATVM)

Der Urheberschutz wird nicht durch Gerichte, sondern durch die ATVM
kryptografisch erzwungen:

| Schutz-Ebene | Mechanismus | Durchsetzung |
|-------------|------------|-------------|
| Code-Hash | SHA-256 des geschuetzten Codes | Registry-Lookup vor Ausfuehrung |
| Developer-DID | ATC-03 Identity des Urhebers | ECDSA-Signatur bei Registrierung |
| Execution-Gate | ATVM verweigert unlizanzierte Ausfuehrung | Physisch (Code kann nicht laufen) |
| Royalty-Transfer | ATC-11 Token automatisch | Atomic mit Ausfuehrung |
| Audit-Trail | ATC-04 DAG pro Ausfuehrung | Unveraenderlich |
| DAO-Governance | ATC-17 kann Lizenzregeln anpassen | On-Chain Voting |

### 3.3 BaFin-Relevante Aspekte

- **Kryptografische Durchsetzung > Gerichtliche Durchsetzung** — Kein
  Diskretionsspielraum, keine "Versehentlich unlizanzierte Ausfuehrung"
- **Vollstaendige Transparenz** — Alle Lizenz-Zahlungen im oeffentlichen DAG
- **Deterministische Abrechnung** — Keine manuelle Nachberechnung noetig
- **Auditierbarkeit** — BaFin kann jede Royalty-Zahlung im Explorer (#5) verfolgen
- **Dezentral** — Keine zentrale Instanz kann Lizenzen "ausschalten"
- **Compliance by Design** — ATVM-Gate ist Teil des Kernels, nicht nachtraeglich

---

## 4. Zusammenhang mit anderen Standards

### 4.1 ATC-01 (Smart Contracts)
ATC-LIC nutzt Smart Contracts fuer License-Registry und Royalty-Transfer.

### 4.2 ATC-03 (Decentralized Identity)
Developer-DID als Urheberschaft-Nachweis. Caller-DID fuer Lizenz-Pruefung.

### 4.3 ATC-11 (Fungible Token)
ATC-11 Token als Royalty-Zahlungsmittel. Atomarer Transfer bei Ausfuehrung.

### 4.4 ATC-17 (DAO Governance)
DAO kann Lizenzregeln anpassen (Preise, Typen, Ausnahmen).

### 4.5 ATC-29 (Marketplace)
Lizenzierte Code-Module koennen im Marketplace gehandelt werden.

### 4.6 ATC-30 (Reputation)
Entwickler mit hoeherer Reputation koennen hoehere Royalties verlangen.

### 4.7 ATC-37 (Resource Allocation)
ATC-LIC kann als Faktor in der Ressourcen-Allokation einfliesen (lizenzierte
Module bekommen Prioritaet).

### 4.8 ATC-LIC (System & Hardware Licenses)
ATC-LIC sichert die physikalische Ebene ab (Hardware-Zertifikate).

---

## 5. Implementations-Status

| Komponente | Status | Beschreibung |
|-----------|--------|-------------|
| ATVM (Execution Gate) | GEPLANT | Virtuelle Maschine mit License-Gate |
| License Registry (Smart Contract) | GEPLANT | On-Chain Registry fuer Code-Hashes |
| Royalty Transfer (ATC-11) | BASIS DA | ATC-11 Token implementiert, Royalty-Logic geplant |
| Developer Registration | GEPLANT | DID-basierte Code-Registrierung |
| License Types | GEPLANT | PER_CALL, SUBSCRIPTION, PERPETUAL, etc. |
| IP & License Dashboard | GEPLANT | GlobusOS Dashboard fuer Lizenzen |
| Audit-Trail (ATC-04 DAG) | BASIS DA | DAG vorhanden, License-Events geplant |
| DAO Governance (ATC-17) | BASIS DA | governance_contract.py vorhanden |
| BaFin-Compliance-Doku | DRAFT | Dieses Dokument |

---

## 6. Roadmap

| Meilenstein | Beschreibung | Status |
|------------|-------------|--------|
| ATC-LIC Spezifikation | Dieses Dokument | DRAFT |
| ATVM License Gate | Virtuelle Maschine mit Gate | GEPLANT |
| License Registry Contract | On-Chain Registry | GEPLANT |
| Royalty Payment Loop | ATC-11 automatischer Transfer | GEPLANT |
| Developer Registration UI | Dashboard fuer Entwickler | GEPLANT |
| IP & License Dashboard | GlobusOS Integration | GEPLANT |
| BaFin Compliance Audit | Rechtliche Pruefung | GEPLANT |
| ATC-LIC Integration | Hardware-Zertifikate | GEPLANT |

---

## 7. Code-Beispiel: Vollstaendiger License-Flow

```python
# === ATC-LIC: Vollstaendiger License-Execution-Flow ===

# 1. Developer registriert Code
license_registry.register_code(
    code_hash="sha256(atc_neural_inference_module)",
    developer_did="ATC-DID:0xabc...developer",
    license_type="PER_CALL",
    price=0.5  # 0.5 ATC-11 Token pro Ausfuehrung
)

# 2. User moechte Code ausfuehren
user_did = "ATC-DID:0xdef...user"
code_hash = "sha256(atc_neural_inference_module)"

# 3. ATVM prueft Lizenz VOR Ausfuehrung
licensed = license_registry.execute_with_license(code_hash, user_did)
# -> Intern:
#    a) Registry lookup: code_hash gefunden
#    b) License type: PER_CALL, price: 0.5 ATC-11
#    c) Balance check: user_did has 10 ATC-11 -> OK
#    d) Atomic transfer: 0.5 ATC-11 -> developer_did
#    e) Log in DAG (ATC-04)
#    f) Return True -> ATVM fuehrt Code aus

if licensed:
    result = atvm.execute(code_hash, input_data)
    # Code wird ausgefuehrt
    # Developer hat bereits 0.5 ATC-11 erhalten
else:
    # Ausfuehrung BLOCKIERT
    # "Unlicensed execution attempt" -> logged in DAG
    raise LicenseError("Insufficient license: Code execution blocked by ATVM")
```

### Unabanderbarkeit:
```
Angreifer versucht, Code ohne Lizenz auszufuehren:
  -> ATVM prueft License Registry
  -> Code-Hash gefunden, PER_CALL, 0.5 ATC-11
  -> Caller balance: 0 ATC-11
  -> ATVM: BLOCKED
  -> Code wird PHYSISCH nicht ausgefuehrt
  -> Keine Gerichtsverhandlung noetig
  -> Kein "Versehentlich" moeglich
  -> Code is Law. Punkt.
```

---

*Dieses Dokument ist Teil des A-TownChain Compliance-Handbuchs.*
*Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. Apache-2.0 lizenziert — siehe LICENSE*
*Letzte Aktualisierung: 05.07.2026 23:18 | Aurora (Superagent)*
