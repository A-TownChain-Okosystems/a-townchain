# IP & License Registry Dashboard — GlobusOS
## Spezifikation fuer das Lizenz-Management-Interface

> **Version:** 1.0.0 | **Datum:** 06.07.2026
> **Autor:** Michael Wroblewski / ShivaCore / A-TownChain-Okosystems
> **Status:** TECHNICAL SPEC — Implementation geplant
> **Referenz-Standard:** ATC-LIC v1.0, ATC-LIC v1.0

---

## 1. Uebersicht

Das IP & License Registry Dashboard ist ein zentrales Interface innerhalb von
GlobusOS, das Lizenzen, Patente und Hardware-Zertifikate sichtbar macht.
Entwickler und Node-Betreiber koennen ihre Rechte verwalten und in Echtzeit
sehen, welche Einnahmen ihr geschuetzter Code generiert.

## 2. Benutzerrollen

| Rolle | Berechtigungen |
|-------|---------------|
| **Developer** | Code registrieren, Lizenzen verwalten, Einnahmen einsehen |
| **Node Operator** | Node-Lizenz verwalten, Hardware-Status pruefen |
| **Caller/User** | Aktive Lizenzen einsehen, Ausfuehrungshistorie |
| **DAO Member** | Governance-Proposals fuer Lizenzregeln |
| **Auditor (BaFin)** | Read-only Zugriff auf alle Compliance-Daten |
| **Admin** | Vollzugriff (DAO-genehmigt) |

## 3. Dashboard-Module

### 3.1 License Management
```
LICENSE MANAGEMENT
├── Code Registration
│   ├── Upload Code -> SHA-256 Hash
│   ├── Select License Type (PER_CALL, SUBSCRIPTION, ...)
│   ├── Set Price (ATC-11 Token)
│   ├── Developer DID (ATC-03)
│   └── Submit -> Registry Contract
├── License Overview
│   ├── Active Licenses (table)
│   ├── Total Earnings (real-time)
│   ├── Call History per License
│   └── License Status (active / revoked / expired)
└── License Actions
    ├── Update Price
    ├── Change License Type
    ├── Revoke License
    └── Transfer Ownership
```

### 3.2 Royalty Monitor
```
ROYALTY MONITOR (Echtzeit)
├── Live Earnings Feed
│   ├── Incoming royalties (websocket)
│   ├── Per-module breakdown
│   ├── Hourly/Daily/Monthly charts
│   └── Total all-time earnings
├── Top-Earning Modules
│   ├── Ranked by revenue
│   ├── Call count
│   └── Average royalty per call
└── Royalty History
    ├── Timeline view
    ├── Filter by module/date
    └── Export as CSV/JSON
```

### 3.3 Hardware & Node Status (ATC-LIC)
```
HARDWARE & NODE STATUS
├── Node Overview
│   ├── Node type (Validator / Compute / Storage / Gateway)
│   ├── License status (active / expired)
│   ├── TPM status (verified / warning / failed)
│   └── Secure Boot status
├── Hardware Certificates
│   ├── TPM certificate details
│   ├── Issue date / expiry
│   └── Tamper history
└── Network Participation
    ├── Connected since
    ├── Blocks validated
    └── Reputation score (ATC-30)
```

### 3.4 Compliance Report (BaFin)
```
COMPLIANCE REPORT
├── Audit Export
│   ├── Date range selector
│   ├── All royalty transactions (CSV/JSON)
│   ├── License registry snapshot
│   └── Execution logs (filtered)
├── Compliance Checklist
│   ├── All items green / red
│   └── Automated verification
└── Incident Log
    ├── Blocked execution attempts
    ├── License violations (none expected)
    └── Governance changes
```

### 3.5 Patent & IP Registry
```
PATENT & IP REGISTRY
├── Registered Patents
│   ├── Patent ID
│   ├── Description
│   ├── DAG reference (ATC-04)
│   └── Filing date
├── Code Ownership
│   ├── Code hash -> Developer mapping
│   ├── Registration date
│   └── Transfer history
└── IP Disputes
    ├── DAO governance cases
    └── Resolution history
```

## 4. UI/UX Spezifikation

### Design-Prinzipien
- **Neon/Dark Theme** — A-TownChain Corporate Identity
- ** Echtzeit-Updates** — WebSocket fuer Live-Royalties
- **Responsive** — Desktop + Tablet + Mobile
- **Accessible** — Standard HTML/Web, keine proprietären Plugins

### Key Views
| View | Beschreibung | User |
|------|-------------|------|
| Dashboard Home | Einnahmen-Overview, Quick Stats | Developer |
| License Manager | CRUD fuer Code-Lizenzen | Developer |
| Royalty Feed | Live-Einnahmen-Stream | Developer |
| Node Status | Hardware-Zertifikate | Operator |
| Compliance | Audit-Export, Checkliste | Auditor |
| IP Registry | Patente, Code-Ownership | Developer, Admin |

### GlobusOS Integration
Das Dashboard ist ein Module innerhalb von GlobusOS und nutzt:
- GlobusOS Authentication (ATC-03 DID)
- GlobusOS Theme System
- GlobusOS Notification Center
- GlobusOS API Gateway (Port 4000)

## 5. API Endpoints

| Endpoint | Method | Beschreibung |
|---------|--------|-------------|
| `/api/licenses` | GET | Alle Lizenzen des Users |
| `/api/licenses` | POST | Neue Lizenz registrieren |
| `/api/licenses/:id` | PUT | Lizenz aktualisieren |
| `/api/licenses/:id` | DELETE | Lizenz widerrufen |
| `/api/royalties/feed` | WS | Echtzeit Royalty-Stream |
| `/api/royalties/history` | GET | Royalty-Historie |
| `/api/earnings/:did` | GET | Gesamteinnahmen pro Developer |
| `/api/nodes/:did` | GET | Node-Status und Hardware |
| `/api/compliance/export` | GET | BaFin-Audit-Export |
| `/api/compliance/checklist` | GET | Compliance-Checkliste |
| `/api/patents` | GET | Patent-Registry |
| `/api/patents` | POST | Patent registrieren |

> Alle Endpunkte gehen durch den API Gateway (Port 4000, ATC-22).

## 6. Datenmodell

```json
{
  "License": {
    "code_hash": "sha256...",
    "developer_did": "ATC-DID:0x...",
    "license_type": "PER_CALL",
    "base_price": 0.5,
    "registered_at": "2026-07-06T11:00:00Z",
    "total_calls": 0,
    "total_royalties": 0,
    "is_active": true,
    "version": "1.0.0"
  },
  "Execution": {
    "code_hash": "sha256...",
    "caller_did": "ATC-DID:0x...",
    "developer_did": "ATC-DID:0x...",
    "royalty": 0.5,
    "timestamp": "2026-07-06T11:05:00Z",
    "result_hash": "sha256...",
    "dag_tx_id": "atc-tx-..."
  },
  "Node": {
    "node_did": "ATC-DID:0x...",
    "node_type": "VALIDATOR",
    "license_status": "active",
    "tpm_verified": true,
    "secure_boot": true,
    "connected_since": "2026-07-01T00:00:00Z",
    "reputation_score": 0.85
  }
}
```

---

*Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. Apache-2.0 lizenziert — siehe LICENSE*
*Letzte Aktualisierung: 06.07.2026 | Aurora (Superagent)*
