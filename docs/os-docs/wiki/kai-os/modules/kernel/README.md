# kernel — L2: ShivaOS Microkernel — IPC Bus, ATCFS, Consensus, Net

Part of [A-TownChain OS Monorepo](../../README.md)

## Inhalt

```
modules/kernel/
├── ARCHITECTURE.md
├── CHANGELOG.md
├── SECURITY.md
├── __init__.py
├── consensus/__init__.py
├── consensus/consensus.atc
├── consensus/poh_integration.py
├── consensus/shiva_consensus.py
├── docs/ATS_STANDARDS.md
├── fs/__init__.py
├── fs/atcfs.atc
├── fs/atcfs.py
├── ipc/ipc_bus.py
├── kernel.py
├── kernel/__init__.py
├── kernel/kernel.atc
├── kernel/kernel.py
├── net/__init__.py
├── net/atcnet.atc
├── net/atcnet.py
```

## Starten

```bash
cd modules/kernel
pip install -r requirements.txt  # falls vorhanden
python -m kernel  # oder spezifischer Einstiegspunkt
```

## Layer-Kontext

| Layer | Modul | Ports | Abhängigkeiten |
|-------|-------|-------|----------------|
| L2 | kernel | — | core/, blockchain/ |

---
*Teil des A-TownChain OS Monorepo — Apache 2.0*
