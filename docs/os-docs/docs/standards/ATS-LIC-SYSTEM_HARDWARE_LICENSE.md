# ATC-LIC — System & Hardware License Protocol
> **Status:** DRAFT — Spezifikation fuer Compliance-Handbuch | **Version:** 1.0.0 | **Datum:** 05.07.2026 23:18
> **Autor:** Michael Wroblewski / ShivaCore, Aurora (Superagent)
> **Standard-ID:** ATC-LIC
> **Tier:** Lizenz-Layer (Hardware & System)
> **Referenzen:** ATC-LIC (Smart Contract Licenses), ATC-01 (Smart Contracts), ATC-03 (Identity), ATC-22 (HAL), ATS-1000+ (ShivaOS Standards)

---

## Abstract

ATC-LIC definiert das System & Hardware License Protocol. Es sichert die
physikalische Unantastbarkeit des Netzwerks ab, indem es verlangt, dass die
Blockchain kryptografische Hardware-Zertifikate (wie TPM) abfragt.

Waehrend ATC-LIC die Software-Lizenzierung ueber Smart Contracts regelt, sorgt
ATC-LIC dafuer, dass die **Hardware-Ebene** ebenfalls lizenziert und
verifiziert ist.

> **ATC-LIC = Hardware-Zertifikate als Voraussetzung fuer Node-Teilnahme.**
> ATC-LIC = Code-Lizenzen (Software). ATC-LIC = Hardware-Lizenzen (Physik).

---

## 1. Kernkonzepte

### 1.1 Hardware-Zertifikate (TPM)
Jeder Node muss ein gueltiges Hardware-Zertifikat vorweisen, um am Netzwerk
teilzunehmen. Das Trusted Platform Module (TPM) generiert kryptografische
Schluessel, die an die Hardware gebunden sind.

- **TPM-Attestation** — Node beweist, dass er auf echter Hardware laeuft
- **Hardware-Identity** — Jeder Node hat eine eindeutige Hardware-ID
- **Tamper-Detection** — Veraenderungen an der Hardware invalidieren das Zertifikat
- **Secure Boot** — Nur signierte Betriebssystem-Komponenten werden geladen

### 1.2 Node-Lizenzierung
Node-Betreiber muessen eine ATC-LIC-Lizenz erwerben, um ihren Node im
A-TownChain-Netzwerk zu betreiben:

| Lizenz-Typ | Beschreibung | Kosten |
|-----------|-------------|--------|
| `VALIDATOR` | Vollstaendige Node-Teilnahme (Consensus) | Jaehrlich |
| `COMPUTE` | Nur Compute-Node (KI-Inferenz) | Pro-Stunde |
| `STORAGE` | Nur Storage-Node (ATCFS) | Pro-GB |
| `GATEWAY` | Nur Gateway-Node (API) | Jaehrlich |
| `FULL` | Alle Funktionen | Jaehrlich (Premium) |

### 1.3 License Enforcement auf Kernel-Ebene
ATC-LIC wird im ShivaOS-Kernel erzwungen (ATS-1000+):

```
SHIVAOS KERNEL BOOT
├── 1. TPM-Attestation
│   ├── Hardware-Zertifikat gueltig?
│   ├── If NO -> BOOT ABGEBROCHEN
│   └── If YES -> continue
├── 2. ATC-LIC License Check
│   ├── Node-Lizenz in Registry?
│   ├── If NO -> NETWORK ACCESS DENIED
│   └── If YES -> continue
├── 3. Secure Boot Verification
│   ├── Kernel-Signatur gueltig?
│   ├── If NO -> BOOT ABGEBROCHEN
│   └── If YES -> continue
├── 4. Network Join
│   ├── ATC-01 Handshake mit License-Proof
│   └── Node aktiv im Netzwerk
```

---

## 2. IP & License Registry Dashboard

Innerhalb von GlobusOS gibt es ein dediziertes Dashboard, das Lizenzen, Patente
und Hardware-Zertifikate sichtbar macht.

### Dashboard-Features:
- **Lizenz-Verwaltung** — ATC-LIC und ATC-LIC Lizenzen an einem Ort
- **Hardware-Status** — TPM-Verifikation, Secure Boot, Tamper-Status
- **Patent-Registry** — IP-Referenzen mit DAG-Verankerung
- **Echtzeit-Einnahmen** — Royalty-Stream von ATC-LIC + ATC-LIC
- **Compliance-Report** — BaFin-konformer Audit-Export
- **Node-Verwaltung** — Lizenz-Typ, Status, Hardware-Zertifikat

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-LIC (Smart Contract Licenses)
ATC-LIC sichert die Hardware. ATC-LIC sichert die Software. Beide zusammen
bilden das vollstaendige Lizenzmodell.

### 3.2 ATC-22 (Hardware Abstraction Layer)
ATC-LIC nutzt ATC-22 fuer Hardware-Erkennung und TPM-Kommunikation.

### 3.3 ATC-03 (Decentralized Identity)
Hardware-Zertifikate werden an Node-DIDs gebunden.

### 3.4 ATS-1000+ (ShivaOS Standards)
ATC-LIC wird im ShivaOS-Kernel erzwungen (ATS-1000 Boot, ATS-1003 Security).

---

## 4. BaFin-Compliance: Hardware-Zertifikate

- **TPM als kryptografischer Beweis** — Hardware ist echt, nicht virtualisiert
- **Secure Boot als Integritaetsnachweis** — Kernel ist unveraendert
- **Tamper-Detection als Manipulationsschutz** — Hardware-Veraenderung erkannt
- **Node-Lizenz als Teilnahmevoraussetzung** — Nur lizenzierte Hardware im Netzwerk
- **Auditierbar** — Hardware-Zertifikate im DAG (ATC-04) verankert

---

*Dieses Dokument ist Teil des A-TownChain Compliance-Handbuchs.*
*Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. Apache-2.0 lizenziert — siehe LICENSE*
*Letzte Aktualisierung: 05.07.2026 23:18 | Aurora (Superagent)*
