# 🏛️ Architektur — atc-kernel

> **Repo:** [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel)
> **Layer:** L2 | **Titel:** Microkernel
> **Stand:** 2026-08-06 | **Version:** v1.0.0

---

## Übersicht

ATCLang-basierter Microkernel mit ATCFS Dateisystem und Syscall-Interface.

## Komponenten

### ATCLang Module (.atc)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `consensus/consensus.atc` | 144 | Consensus |
| `fs/atcfs.atc` | 142 | Atcfs |
| `kernel/kernel.atc` | 148 | Kernel |
| `net/atcnet.atc` | 135 | Atcnet |

### Python Module (.py)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `consensus/poh_integration.py` | 29 | Poh Integration |
| `consensus/shiva_consensus.py` | 641 | Shiva Consensus |
| `fs/atcfs.py` | 331 | Atcfs |
| `ipc/ipc_bus.py` | 94 | Ipc Bus |
| `kernel.py` | 106 | Kernel |
| `kernel/kernel.py` | 382 | Kernel |
| `net/atcnet.py` | 17 | Atcnet |

## Statistik

| Metrik | Wert |
|--------|------|
| .atc | 4 |
| .py | 7 |
| .rs | 0 |
| .ts | 0 |
| Total Zeilen | 2,169 |

---

*Auto-generiert 2026-08-06 · Aurora*
