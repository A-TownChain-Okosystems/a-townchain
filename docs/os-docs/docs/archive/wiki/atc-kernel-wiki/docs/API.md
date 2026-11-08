# 🔌 API Reference — atc-kernel

> **Repo:** [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `start()` | — | `kernel/kernel.atc` | ATCLang |
| 2 | `stop()` | — | `kernel/kernel.atc` | ATCLang |
| 3 | `spawn()` | u64 | `kernel/kernel.atc` | ATCLang |
| 4 | `kill()` | bool | `kernel/kernel.atc` | ATCLang |
| 5 | `get_process()` | Process | `kernel/kernel.atc` | ATCLang |
| 6 | `list_processes()` | Vec | `kernel/kernel.atc` | ATCLang |
| 7 | `status()` | Map | `kernel/kernel.atc` | ATCLang |
| 8 | `alloc()` | bool | `kernel/kernel.atc` | ATCLang |
| 9 | `uptime()` | u64 | `kernel/kernel.atc` | ATCLang |
| 10 | `_kill_process()` | — | `kernel/kernel.atc` | ATCLang |
| 11 | `init()` | — | `fs/atcfs.atc` | ATCLang |
| 12 | `open()` | u64 | `fs/atcfs.atc` | ATCLang |
| 13 | `write()` | u64 | `fs/atcfs.atc` | ATCLang |
| 14 | `read()` | bytes | `fs/atcfs.atc` | ATCLang |
| 15 | `close()` | — | `fs/atcfs.atc` | ATCLang |
| 16 | `stat()` | FileStat | `fs/atcfs.atc` | ATCLang |
| 17 | `rm()` | bool | `fs/atcfs.atc` | ATCLang |
| 18 | `hash_file()` | bytes32 | `fs/atcfs.atc` | ATCLang |
| 19 | `write_encrypted()` | u64 | `fs/atcfs.atc` | ATCLang |
| 20 | `mkdir()` | bool | `fs/atcfs.atc` | ATCLang |
| 21 | `ls()` | Vec | `fs/atcfs.atc` | ATCLang |
| 22 | `generate_poh()` | bytes32 | `consensus/consensus.atc` | ATCLang |
| 23 | `verify_poh()` | bool | `consensus/consensus.atc` | ATCLang |
| 24 | `register_validator()` | — | `consensus/consensus.atc` | ATCLang |
| 25 | `select_validator()` | Address | `consensus/consensus.atc` | ATCLang |
| 26 | `mine_block()` | Map | `consensus/consensus.atc` | ATCLang |
| 27 | `create_block()` | Block | `consensus/consensus.atc` | ATCLang |
| 28 | `validate_chain()` | bool | `consensus/consensus.atc` | ATCLang |
| 29 | `get_height()` | u64 | `consensus/consensus.atc` | ATCLang |
| 30 | `get_block()` | Block | `consensus/consensus.atc` | ATCLang |
| 31 | `get_state()` | ConsensusState | `consensus/consensus.atc` | ATCLang |
| 32 | `init()` | — | `net/atcnet.atc` | ATCLang |
| 33 | `start()` | — | `net/atcnet.atc` | ATCLang |
| 34 | `stop()` | — | `net/atcnet.atc` | ATCLang |
| 35 | `connect()` | bool | `net/atcnet.atc` | ATCLang |
| 36 | `disconnect()` | — | `net/atcnet.atc` | ATCLang |
| 37 | `broadcast()` | u64 | `net/atcnet.atc` | ATCLang |
| 38 | `propagate_block()` | u64 | `net/atcnet.atc` | ATCLang |
| 39 | `propagate_tx()` | u64 | `net/atcnet.atc` | ATCLang |
| 40 | `peer_count()` | u64 | `net/atcnet.atc` | ATCLang |
| 41 | `is_running()` | bool | `net/atcnet.atc` | ATCLang |
| 42 | `get_node_id()` | bytes32 | `net/atcnet.atc` | ATCLang |
| 43 | `read()` | bytes | `kernel.py` | Python |
| 44 | `write()` | — | `kernel.py` | Python |
| 45 | `send()` | bool | `kernel.py` | Python |
| 46 | `read()` | bytes | `kernel/kernel.py` | Python |
| 47 | `write()` | — | `kernel/kernel.py` | Python |
| 48 | `send()` | bool | `kernel/kernel.py` | Python |
| 49 | `recv()` | Optional | `kernel/kernel.py` | Python |
| 50 | `peek()` | int | `kernel/kernel.py` | Python |
| 51 | `__init__()` | — | `kernel/kernel.py` | Python |
| 52 | `_boot()` | — | `kernel/kernel.py` | Python |
| 53 | `_spawn_system()` | int | `kernel/kernel.py` | Python |
| 54 | `_next_pid()` | int | `kernel/kernel.py` | Python |
| 55 | `_next_chan()` | int | `kernel/kernel.py` | Python |
| 56 | `_register_builtins()` | — | `kernel/kernel.py` | Python |
| 57 | `spawn()` | int | `kernel/kernel.py` | Python |
| 58 | `_run_process()` | — | `kernel/kernel.py` | Python |
| 59 | `_target()` | — | `kernel/kernel.py` | Python |
| 60 | `kill()` | bool | `kernel/kernel.py` | Python |

*+94 weitere*

**Total: 154 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
