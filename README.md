# atc-vm

> **A-TownChain Virtual Machine (ATVM)** — verifizierte Bytecode-Ausfuehrung fuer ATCLang-Vertraege.

**Prioritaet:** P0 (Kern-Laufzeit, AD-043) | **Chain-ID:** 658467 (AD-004) | **Org:** [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems)

> ## Fuer KI-Agenten - Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo [`a-townchain-os-docs`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs):
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md)
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md)
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) - insb. AD-021 (Rust-first), AD-019/022 (Vertrags-Engine), AD-043 (dieses Repo), AD-023 (kein Mainnet-Termin)

---

## Rolle im Oekosystem

```
atclang (Compiler: Lexer/Parser/AST/Semantics/ATC-IR/Optimizer/Codegen)
        |
        v  ATC-Bytecode (verifiziertes Artefakt)
   +------------------+
   |      ATVM        |   <- dieses Repo
   +------------------+
   |  Bytecode-Verifier (kein unverifizierter Code wird je ausgefuehrt)
   |  Interpreter / Execution Engine (deterministisch, gas-limitiert)
   |  Runtime + Host-Interface (Syscalls, Storage, Chain-Kontext)
   |  Sandbox + Security-Boundary (ATC-STD-NET-001/002/003 Trust Boundary)
   |  License-Gate-Interface (ATC-LIC-Enforcement)
   +------------------+
        |
        v  Contract-Kontext (AD-019/022)
   a-townchain (Blockchain) - atc-contracts (.atc-Referenzvertraege)
```

## Abgrenzung (AD-043)

| Zustaendigkeit | Repo |
|---|---|
| Sprache, Compiler, Bytecode-Erzeugung | `atclang` |
| **Bytecode-Verifikation + Ausfuehrung + Runtime** | **`atc-vm` (dieses Repo)** |
| Vertrags-Standards + .atc-Referenzvertraege | `atc-contracts` |
| Chain-Konsens, Block-Execution-Orchestrierung | `a-townchain` |
| Kernel (Prozess-/Speicher-Isolation) | `atc-shivacore` |

Das bisherige `atc-vm`-Modul in `atclang` (Python-Referenz) wird als
Referenz-Implementierung erhalten; die kanonische Rust-Implementierung
entsteht hier (AD-021 Rust-first). Die Modul-Migration von atclang nach
atc-vm ist als offener Punkt im DECISIONS_REGISTER (AD-043) gefuehrt.

## Status (AD-020-Rebuild-Aera)

- **Stand:** R1-Skeleton (ATC-STD-201) — Struktur + Governance stehen,
  Implementierung folgt im qualitaetsgetriebenen Rebuild (AD-023,
  G0-G19-Gates).
- **Naechste Schritte:** (1) ATC-IR/Bytecode-Format-Spezifikation aus
  atclang ueberfuehren, (2) Bytecode-Verifier (Rust), (3) deterministischer
  Interpreter + Gas-Modell, (4) Fuzzing/Harness gegen die Python-Referenz.
- **Qualitaets-Gates:** ATVM ist Trust Boundary (Sicherheit S4-Klasse) —
  G18 Security-Audit vor jedem Freeze (AD-023).

## Lizenz

Proprietaer - All Rights Reserved (ATC-LIC). Siehe [LICENSE](LICENSE).
