# A-TownChain Lizenzmodell — Zentrale Uebersicht

> **Version:** 1.0.0 | **Datum:** 06.07.2026
> **Autor:** Michael Wroblewski / ShivaCore / A-TownChain-Okosystems
> **Status:** Offiziell

---

## Das Paradigma

**Monetarisiertes, autonomes Open-Source-Oekosystem.**

Die A-TownChain ersetzt das klassische Justizsystem zur Durchsetzung von
Urheberrechten durch Kryptografie und Smart Contracts. **"Code is Law"** wird
auf der Lizenzebene wortwoertlich genommen — unlizenzierter Code wird von der
A-Town Virtual Machine (ATVM) **physisch gar nicht erst ausgefuehrt**.

| Eigenschaft | Open Source | B2B Closed | **A-TownChain** |
|-------------|------------|------------|-----------------|
| Quellcode | Frei | Geschlossen | **Frei (im Oekosystem)** |
| Ausfuehrung | Frei | Lizenzschluessel | **ATVM-Gated (Smart Contract)** |
| Verguetung | Keine | Lizenzgebuehr | **Automatische Royalty** |
| Durchsetzung | Gerichtlich | Gerichtlich | **Kryptografisch (Code is Law)** |
| Innovation | Hoch | Beschraenkt | **Hoch (offen + belohnt)** |

---

## Die zwei Lizenz-Standards

### ATC-LIC — Smart Contract Licenses
> Code-Lizenzen auf Software-Ebene

| Lizenz-Typ | Beschreibung | Use Case |
|-----------|-------------|----------|
| `PER_CALL` | Pro Ausfuehrung | API-Funktionen, KI-Inferenz |
| `SUBSCRIPTION` | Zeitbasiert | DApps, Services |
| `PERPETUAL` | Einmalig | Tools, Libraries |
| `REVENUE_SHARE` | Umsatzbeteiligung | Marktplatz-Integration |
| `FREEMIUM` | Basis frei, Premium kostenpflichtig | Endnutzer-Apps |
| `DAO_GOVERNED` | DAO bestimmt Preis | Systemkritische Module |

**Durchsetzung:** ATVM License Gate — unlizenzierter Code wird nicht ausgefuehrt.
**Zahlungsmittel:** ATC-11 Token — atomarer Transfer bei jeder Ausfuehrung.

### ATC-LIC — System & Hardware Licenses
> Hardware-Zertifikate auf physikalischer Ebene

| Lizenz-Typ | Beschreibung |
|-----------|-------------|
| `VALIDATOR` | Vollstaendige Node-Teilnahme (Consensus) |
| `COMPUTE` | Nur Compute-Node (KI-Inferenz) |
| `STORAGE` | Nur Storage-Node (ATCFS) |
| `GATEWAY` | Nur Gateway-Node (API) |
| `FULL` | Alle Funktionen |

**Durchsetzung:** ShivaOS Kernel (ATS-1000+) — TPM, Secure Boot, Tamper-Detection.
**Voraussetzung:** Node ohne TPM-Zertifikat kann nicht am Netzwerk teilnehmen.

---

## Wie es funktioniert (End-to-End)

```
1. Entwickler schreibt Code
2. Entwickler registriert Code-Hash in License Registry (Smart Contract)
3. Nutzer ruft Code auf
4. ATVM prueft License Registry vor Ausfuehrung
5. ATVM berechnet Royalty (deterministisch, ATC-14)
6. ATVM prueft: Nutzer hat genug ATC-11 Token?
   ├── NEIN -> Code wird BLOCKIERT (physisch, nicht juristisch)
   └── JA  -> Atomare Transaktion:
       ├── ATC-11 Token -> Developer
       ├── Code wird ausgefuehrt (sandboxed)
       └── Ausfuehrung im DAG (ATC-04) protokolliert
7. Developer sieht Einnahmen in Echtzeit (GlobusOS Dashboard)
8. BaFin kann jede Zahlung im Explorer auditieren
```

---

## IP & License Registry Dashboard (GlobusOS)

Zentrales Dashboard innerhalb von GlobusOS fuer Lizenzen, Patente und
Hardware-Zertifikate:

- **License Management** — Code registrieren, Preise setzen, Lizenzen verwalten
- **Royalty Monitor** — Echtzeit-Einnahmen pro Code-Modul (WebSocket)
- **Node Status** — TPM-Verifikation, Secure Boot, Hardware-Zertifikate
- **Compliance Report** — BaFin-Audit-Export (CSV/JSON)
- **Patent Registry** — IP-Referenzen mit DAG-Verankerung

---

## Compliance-Handbuch (BaFin)

- **Kryptografische Durchsetzung** > Gerichtliche Durchsetzung
- **Deterministisch** — Kein Ermessensspielraum (ATC-14)
- **Atomar** — Transfer + Ausfuehrung in einer Transaktion
- **Transparent** — Alle Zahlungen im oeffentlichen DAG (ATC-04)
- **Auditierbar** — Vollstaendige Historie im Explorer
- **Compliance by Design** — ATVM-Gate im Kernel, nicht nachtraeglich

---

## Implementations-Status

| Komponente | Status |
|-----------|--------|
| Proprietary LICENSE (24 Repos) | ✅ Abgeschlossen |
| Copyright-Header (760+ Dateien) | ✅ Abgeschlossen |
| ATC-LIC Spezifikation | ✅ DRAFT dokumentiert |
| ATC-LIC Spezifikation | ✅ DRAFT dokumentiert |
| Compliance-Handbuch | ✅ DRAFT dokumentiert |
| Smart-Contract-Richtlinie (BaFin) | ✅ DRAFT dokumentiert |
| ATVM License Gate Spec | ✅ DRAFT dokumentiert |
| IP & License Dashboard Spec | ✅ DRAFT dokumentiert |
| ATVM License Gate (Implementation) | 📐 Geplant |
| License Registry Smart Contract | 📐 Geplant |
| Royalty Payment Loop | 📐 Geplant |
| IP & License Dashboard (GlobusOS) | 📐 Geplant |
| BaFin-Compliance-Audit | 📐 Geplant |

---

## Detail-Dokumente

| Dokument | Beschreibung |
|---------|-------------|
| [ATC-LIC Spezifikation](standards/ATC-LIC-SMART_CONTRACT_LICENSE.md) | Hauptstandard: ATVM Gate, Lizenz-Typen, Code-Beispiele |
| [ATC-LIC Spezifikation](standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md) | Hardware-Lizenzen: TPM, Node-Lizenzierung, Secure Boot |
| [Compliance-Handbuch](compliance/COMPLIANCE_HANDBUCH.md) | BaFin-konforme Gesamtdokumentation |
| [BaFin-Konformitaetsbericht](compliance/BAFIN_KONFORMITAETSBERICHT.md) | Formeller Bericht BAFIN-ATC-LIC-2026-001 zur BaFin-Einreichung |
| [Smart-Contract-Richtlinie](compliance/SMART_CONTRACT_RICHTLINIE.md) | BaFin-Policy: Provisionsabwicklung, Urheberschutz, Audit |
| [ATVM License Gate Spec](compliance/ATVM_LICENSE_GATE_SPEC.md) | Technische Spec: State Machine, API, Security |
| [IP & License Dashboard](compliance/IP_LICENSE_DASHBOARD_SPEC.md) | GlobusOS: Module, API, Datenmodell |

---

## Querverweise

| Standard | Bezug zum Lizenzmodell |
|---------|----------------------|
| ATC-01 | Smart Contracts (License Registry) |
| ATC-03 | Identity (Developer & Caller DID) |
| ATC-04 | DAG (Audit-Trail) |
| ATC-11 | Token (Royalty-Zahlungsmittel) |
| ATC-14 | Deterministic Execution (Royalty-Berechnung) |
| ATC-17 | DAO Governance (Lizenzregeln) |
| ATC-22 | HAL (TPM-Kommunikation) |
| ATC-29 | Marketplace (Lizenz-Handel) |
| ATC-30 | Reputation (Developer-Vertrauen) |
| ATC-37 | Resource Allocation (Lizenzierte Module priorisiert) |
| ATS-1000+ | ShivaOS Kernel (Hardware-Enforcement) |

---

*Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.*
