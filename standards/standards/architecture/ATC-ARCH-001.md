---
standard:
  id: ATC-ARCH-001
  title: "KAI-OS System Architecture Standard"
  version: "1.0.0"
  status: approved
  category: architecture
  authority: "A-TownChain Ecosystems"
  owner: "Michael Wroblewski (Owner-Direktive) / Standards Governance"
  created: "2026-09-12"
  updated: "2026-09-12"
  normative: true
  applies_to: "Gesamtes A-TownChain-Ökosystem, alle Repositories, alle Agenten"
  mandate: "Owner-Direktive 12.09.2026 (KAI-OS-Begriffsklärung)"
  related: [ATC-STD-000, ATC-STD-100, AD-008, KAI-CORE-RUNTIME-001]
---

# ATC-ARCH-001 — KAI-OS System Architecture Standard (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Direktive 12.09.2026. **Scope:** Begriffs- und
> Architekturdefinition des gesamten Ökosystems. **Governance:** ATC-STD-000.

## 1. Normative Definition

> **KAI-OS = Cryptographic AI Operating System Architecture**

KAI-OS ist der **übergeordnete Architekturbegriff** für den gesamten
kryptografisch abgesicherten AI-OS-Stack.

**KAI-OS ist:**
- eine Architekturbezeichnung (Systemplattform),
- der Systembegriff, unter dem die sechs Kerntechnologien zusammengehören.

**KAI-OS ist NICHT:**
- ein separates Betriebssystem (neben GlobusOS),
- ein Repository,
- ein einzelnes Produkt.

## 2. Systemarchitektur

```
KAI-OS — Kryptografisches AI Operating System
        │
        ┌────────────────┼────────────────┐
        │                │                │
     Security          Execution          AI
        │                │                │
   ShivaCore          ATC-VM          Aurora OS
     Kernel              │                │
        │             ATCLang             │
        │                │                │
        └────────── A-TownChain ──────────┘
                         │
                    GlobusOS
                  Betriebssystem
```

## 3. Kernkomponenten (normative Rollen)

| Komponente | Rolle innerhalb KAI-OS | Repo |
|---|---|---|
| **ATCLang** | Native/strategische Programmiersprache | `atclang` |
| **ShivaCore** | Sicherheitskritischer Capability-/Security-Kernel | `atc-shivacore` |
| **ATC-VM** | Deterministische Ausführungsumgebung (Virtual Machine) | `atc-vm` |
| **A-TownChain** | Dezentrales Trust-/Consensus-System (Blockchain) | `a-townchain` |
| **Aurora OS** | Dezentrale AI-/Agentenplattform | `aurora-ai` |
| **GlobusOS** | Betriebssystem (Desktop-/Systemplattform) | `globus-os` |

**Integration:** `a-townchain-os` — Build-, Workspace-, CI/CD-, Deployment- und
Integrationsschicht des Ökosystems (L7 — Integration/Orchestrierung).

**Governance:** `atc-standards` — Standardisierung, Registry, SCR-Prozess.

## 4. Architektur-Reihenfolge (Kanonische Nummerierung)

```
KAI-OS
├── 1. ATCLang       — Programmiersprache
├── 2. ShivaCore     — Capability-/Security-Kernel
├── 3. ATC-VM        — deterministische Virtual Machine
├── 4. A-TownChain   — dezentrales Trust-/Consensus-System
├── 5. Aurora OS      — dezentrale AI / KI-Assistenten / Agenten
└── 6. GlobusOS      — Betriebssystem
```

## 5. Rollen-Trennung (§-verbindlich: KEINE Rollen-Vermischung)

> **Keine Komponente darf ihre Rolle mit einer anderen Komponente vermischen.**

| Verbot | Begründung |
|---|---|
| GlobusOS ≠ Kernel | GlobusOS ist OS, Kernel ist ShivaCore |
| ShivaCore ≠ Betriebssystem | ShivaCore ist Kernel, nicht OS |
| A-TownChain ≠ Betriebssystem | Blockchain ist Trust-Schicht, nicht OS |
| ATC-VM ≠ Blockchain | VM ist Execution, nicht Konsens |
| ATCLang ≠ VM | Sprache ≠ Laufzeitumgebung |
| Aurora OS ≠ Kernel | AI-Plattform ≠ Kernel |
| **KAI-OS ≠ weiteres Betriebssystem** | KAI-OS ist Architektur, kein Produkt |
| **a-townchain-os ≠ KAI-OS** | Integration-Repo ≠ Architekturbegriff |

## 6. Vertikale Abhängigkeit (korrigierte Formulierung)

KAI-OS beschreibt die **Gesamtarchitektur**; GlobusOS ist das eigentliche
Betriebssystem **innerhalb** dieser Architektur. Es gibt genau EIN Betriebssystem
(GlobusOS), EINEN Kernel (ShivaCore), EINE VM (ATC-VM), EINE Blockchain
(A-TownChain), EINE AI-Plattform (Aurora OS) und EINE Sprache (ATCLang).

## 7. Layer-Konsistenz

Die bestehenden Layer-Zuordnungen bleiben unverändert korrekt:

- `globus-os` = **L4 — Betriebssystem** ✅
- `a-townchain-os` = **L7 — Integration / Orchestrierung** ✅

## 8. Verhältnis zu verwandten Standards

- **ATC-STD-100** (Language & Technology Stack): Sprach- und Technologieauswahl
  je Repository — ATC-ARCH-001 definiert die Systemrollen darüber.
- **AD-008** (Language & Execution Boundary): ATCLang on-chain, Rust native,
  ATVM als Ausführungsgrenze — gültig innerhalb der Komponenten 1, 3 und 4.
- **KAI-CORE-RUNTIME-001** (a-townchain-os): Core Runtime Track S01–S26 ist die
  Implementierung der KAI-OS-Systemplattform im Integrations-Repo — der Begriff
  `kai-os/` dort bezeichnet die Core-Runtime-Implementierung, nicht KAI-OS selbst.

## 9. Konsequenzen

1. Dokumentation, READMEs, Wiki und Standards verwenden „KAI-OS" ausschließlich
   als Architekturbegriff („die KAI-OS-Architektur"), nie als OS- oder Produktnamen.
2. „Betriebssystem" ohne Qualifikation bezeichnet GlobusOS.
3. Core-Runtime-Arbeit im Integrations-Repo wird als „KAI-OS Core Runtime" /
   „KAI-OS-Architektur, Komponenten-Implementierung" bezeichnet.
4. Widersprüche zu dieser Definition sind INVALID und via SCR zu beheben.
