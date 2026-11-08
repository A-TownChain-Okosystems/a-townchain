# ATC-99 — ATCLang Universal Mandate

> **Standard-ID:** ATC-99  
> **Kategorie:** Language Policy  
> **Tier:** 0 — Foundational  
> **Status:** 📐 FINAL — Spezifikation vollständig, verbindlich ab sofort  
> **Version:** 1.0.0  
> **Autor:** Michael Wroblewski  
> **Sprint:** Übergreifend (gilt für alle Sprints 2.1–4.2d)  
> **Wiki-Ref:** Kapitel 69, Kapitel 66  
> **Abhängigkeiten:** ATC-92 (ATCLang Spec), ATC-93 (ATCLang VM), ATC-94 (ATCLang Stdlib)  
> **Entscheidung:** AD-006 (RESOLVED — ATCLang First)  

---

## 1. Zweck und Geltungsbereich

ATC-99 ist der verbindliche Standard, der **ATCLang als einzige Programmiersprache** für das gesamte A-TownChain OS / KAI-OS Ökosystem festlegt. Keine andere Programmiersprache ist zulässig — weder für Smart Contracts, noch für Systemcode, Kernel-Module, Backend-Services, Gateway-Komponenten oder Tests.

Dieser Standard formalisiert die Architektur-Entscheidung AD-006 (ATCLang First) und Regel 0 der AGENT_MASTERRULES.md.

### 1.1 Geltungsbereich

ATC-99 gilt verbindlich für:

| Bereich | Betroffene Komponenten |
|---------|----------------------|
| Smart Contracts | `bridge.atc`, `dao.atc`, `dex.atc`, `governance.atc`, `marketplace.atc`, `shivamon.atc`, `wallet.atc`, `registry.atc`, `revenue.atc`, `token.atc` |
| Kernel & OS | `kernel.atc`, `consensus.atc`, `atcfs.atc`, `atcnet.atc`, `atcos_main.atc` |
| Gateway & Backend | `gateway.atc`, `event_bus.atc` |
| Compiler & VM | ATCLang Compiler (ATC-92), ATCLang VM (ATC-93), ATCLang Stdlib (ATC-94) |
| Tests | Alle Tests müssen in ATCLang geschrieben werden |
| Infrastructure Scripts | CI/CD-Skripte, Deploy-Scripts (Ausnahme: Shell-Skripte für Bootstrapping) |

### 1.2 Ausgeschlossene Sprachen

| Sprache | Status | Begründung |
|---------|--------|-----------|
| Python | ❌ Verboten (als Endzustand) | Temporärer Stub, Migration geplant |
| Solidity | ❌ Verboten | Non-EVM Architektur (ATC-81, AD-004) |
| Rust | ❌ Verboten | Nicht Teil der Architektur |
| JavaScript/TypeScript | ❌ Verboten (als Backend) | Nur Frontend-UI erlaubt (nicht Teil des OS) |
| C/C++ | ❌ Verboten | Hardware-Treiber durch ATCLang FFI |
| Go | ❌ Verboten | Nicht Teil der Architektur |

---

## 2. Migrations-Policy

### 2.1 STUB-Regel

Alle aktuellen Python-Dateien sind als **temporäre Stubs** markiert und müssen gemäß folgendem Migrations-Plan durch ATCLang-Implementierungen ersetzt werden:

| Sprint | Migration | Dateien |
|--------|-----------|---------|
| 2.1 | ATCLang Compiler, VM, Lexer, Parser, Stdlib, ECDSA, Keygen | 6+ Kernkomponenten |
| 2.2 | Bootstrap, Discovery | 2 P2P-Komponenten |
| 2.3 | PoH, PoS, PoW, Hybrid Consensus, Fork Resolution, Gas, AMM, Token | 8 Konsens+Contract |
| 2.5 | Marketplace, Shivamon Contract | 2 NFT/Marketplace |
| 2.6 | Bridge, Governance, DAO, Multisig | 4 Governance+Bridge |
| 3.0 | Server, Gateway, KAI-CLI | 3 Backend+Gateway |
| 4.0 | Mainnet Config | 1 Mainnet-Konfiguration |

**Jeder Python-Stub muss folgenden Header tragen:**
```
# STUB: Temporärer Python-Stub — wird in Sprint X.X durch ATCLang ersetzt (ATCLang First Policy, AD-006, ATC-99)
```

### 2.2 Migrations-Reihenfolge

Die Migration erfolgt streng nach der Roadmap v2.0:

```
ATCLang Core (Sprint 2.1) → Consensus (2.3) → Smart Contracts (2.3-2.6) → Kernel (2.4) → Backend (3.0) → Mainnet (4.0)
```

Kein Sprint darf übersprungen werden. Jede Migration erfordert:
1. ATCLang-Implementierung der Funktionalität
2. Test-Suite in ATCLang (ATC-98)
3. Verifikation der ATC-Standard-Konformität
4. Entfernung des Python-Stubs
5. Wiki- und Dokumentations-Update

### 2.3 Verbotene Muster

- ❌ Neuer Python-Code (außer STUBs mit Sprint-Marker)
- ❌ Neuer Solidity-Code
- ❌ Neuer JavaScript-Code für Backend/OS
- ❌ Mixed-Language-Dateien
- ❌ Python-Dateien ohne STUB-Marker

---

## 3. ATCLang Sprachspezifikation

ATCLang ist definiert durch:

| Standard | Titel | Status |
|----------|-------|--------|
| ATC-92 | ATCLang Language Specification | DRAFT |
| ATC-93 | ATCLang Virtual Machine (Bytecode) | DRAFT |
| ATC-94 | ATCLang Standard Library | ACCEPTED |
| ATC-95 | ATCLang Test Framework | DRAFT |

### 3.1 Kern-Features

- **Typsystem:** Statisch, mit Type-Inference
- **Kryptografie:** SHA-256 (ATC-86), ECDSA secp256k1 (ATC-86)
- **Chain-ID:** 9000 (AD-004, Non-EVM)
- **Ausführung:** ATCLang VM (ATC-93) — Bytecode-basiert
- **Standardbibliothek:** 6 Module (crypto, collections, io, math, encoding, primitives)
- **Gas-Modell:** Jede Operation hat definierte Gas-Kosten (ATC-87)

### 3.2 Context-Isolation

| Context | Berechtigungen |
|---------|---------------|
| Node | Vollzugriff (File I/O, Network, Crypto) |
| Contract | Eingeschränkt (State, Storage, Crypto, kein File I/O) |
| Test | Mock-Environment (keine echten Nebenwirkungen) |

---

## 4. Verifikation und Audit

### 4.1 Konformitäts-Check

Die ATC-99-Konformität wird durch folgende Prüfungen sichergestellt:

1. **Datei-Extension-Check:** Alle Programmdateien haben `.atc`-Endung
2. **STUB-Marker-Check:** Alle Python-Dateien haben `# STUB:`-Header
3. **Non-EVM-Check:** Keine `.sol`-Dateien, keine Solana/Substrate-Abhängigkeiten
4. **Hash-Algorithm-Check:** Kein Keccak-256, kein SHA-3, nur SHA-256
5. **Chain-ID-Check:** Chain-ID 9000 in allen Konfigurationen

### 4.2 Automatisierung

Die Konformitäts-Prüfung läuft täglich um 08:00 Uhr (Europe/Berlin) als Teil der KAI-OS Daily Full Sync Automation (ID: `6a2a84debb58cc332fc9f9fb`).

### 4.3 Verletzungs-Management

| Severity | Konsequenz |
|----------|-----------|
| Neue Python-Datei ohne STUB | Release gesperrt (Regel 8) |
| Solidity-Datei | Release gesperrt, sofortige Löschung |
| Falscher Hash-Algorithmus | Release gesperrt (ATC-86) |
| Falsche Chain-ID | Release gesperrt (AD-004) |

---

## 5. Querverweise

| Referenz | Dokument |
|----------|---------|
| AD-006 | ATCLang First Entscheidung (RESOLVED) |
| Regel 0 | AGENT_MASTERRULES.md — "Alles ist ATCLang" |
| ATC-92 | ATCLang Language Specification |
| ATC-93 | ATCLang Virtual Machine |
| ATC-94 | ATCLang Standard Library |
| ATC-95 | ATCLang Test Framework |
| ATC-81 | Non-EVM Architecture |
| ATC-86 | ECDSA & SHA-256 Standard |
| ATC-98 | Testing Standard |
| Kapitel 66 | ATCLang Standard Library Referenz |
| Kapitel 69 | Konsistenz-Prüfung & Konformitäts-Audit |

---

## 6. Gas-Cost-Tabelle

ATC-99 selbst ist ein Meta-Standard und definiert keine eigenen Gas-Kosten. Die Gas-Kosten für ATCLang-Operationen sind in ATC-87 (Gas Fee Standard) spezifiziert.

---

## 7. Testing-Strategie

| Test | Beschreibung |
|------|-------------|
| Extension-Test | Alle `.py`-Dateien haben STUB-Marker, alle `.sol`-Dateien sind 0 |
| Hash-Test | Kein `sha3` oder `keccak` in der Codebasis |
| Chain-ID-Test | `9000` in allen Konfigurationsdateien |
| ATC-ID-Test | Keine alten ATC-IDs (1000er, KIP, AIP, ATS) in ATCLang-Dateien |
| STUB-Sprint-Test | Jeder STUB hat eine gültige Sprint-Zuweisung |

**Mindestens 5 Konformitäts-Tests**, automatisiert im CI/CD-Pipeline.

---

> **Dies ist der ultimative Standard. ATC-99 schließt den Kreis: Alles ist ATCLang. Keine Ausnahmen.**
