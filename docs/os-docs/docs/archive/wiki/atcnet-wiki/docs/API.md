# 🔌 API Reference — atcnet

> **Repo:** [atcnet](https://github.com/A-TownChain-Okosystems/atcnet)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `init()` | void | `atcnet.atc` | ATCLang |
| 2 | `start()` | void | `atcnet.atc` | ATCLang |
| 3 | `stop()` | void | `atcnet.atc` | ATCLang |
| 4 | `connect()` | bool | `atcnet.atc` | ATCLang |
| 5 | `disconnect()` | void | `atcnet.atc` | ATCLang |
| 6 | `broadcast()` | u64 | `atcnet.atc` | ATCLang |
| 7 | `propagate_block()` | u64 | `atcnet.atc` | ATCLang |
| 8 | `propagate_tx()` | u64 | `atcnet.atc` | ATCLang |
| 9 | `peer_count()` | u64 | `atcnet.atc` | ATCLang |
| 10 | `is_running()` | bool | `atcnet.atc` | ATCLang |
| 11 | `get_node_id()` | bytes32 | `atcnet.atc` | ATCLang |
| 12 | `__init__()` | Any | `bootstrap_client.py` | Python |
| 13 | `announce()` | List | `bootstrap_client.py` | Python |
| 14 | `ping()` | bool | `bootstrap_client.py` | Python |
| 15 | `get_peers()` | List | `bootstrap_client.py` | Python |
| 16 | `__post_init__()` | Any | `atcnet.py` | Python |
| 17 | `encode()` | bytes | `atcnet.py` | Python |
| 18 | `decode()` | Optional | `atcnet.py` | Python |
| 19 | `distance()` | int | `atcnet.py` | Python |
| 20 | `is_alive()` | bool | `atcnet.py` | Python |
| 21 | `generate_node_id()` | str | `atcnet.py` | Python |
| 22 | `__init__()` | Any | `atcnet.py` | Python |
| 23 | `_bucket_index()` | int | `atcnet.py` | Python |
| 24 | `add_node()` | bool | `atcnet.py` | Python |
| 25 | `remove_node()` | Any | `atcnet.py` | Python |
| 26 | `find_closest()` | List | `atcnet.py` | Python |
| 27 | `get_all_nodes()` | List | `atcnet.py` | Python |
| 28 | `size()` | int | `atcnet.py` | Python |
| 29 | `__init__()` | Any | `atcnet.py` | Python |
| 30 | `_setup_handlers()` | Any | `atcnet.py` | Python |
| 31 | `on()` | Any | `atcnet.py` | Python |
| 32 | `start()` | bool | `atcnet.py` | Python |
| 33 | `stop()` | Any | `atcnet.py` | Python |
| 34 | `send()` | bool | `atcnet.py` | Python |
| 35 | `broadcast()` | Any | `atcnet.py` | Python |
| 36 | `send_to()` | bool | `atcnet.py` | Python |
| 37 | `_recv_loop()` | Any | `atcnet.py` | Python |
| 38 | `_dispatch()` | Any | `atcnet.py` | Python |
| 39 | `_forward()` | Any | `atcnet.py` | Python |
| 40 | `_handle_ping()` | Any | `atcnet.py` | Python |
| 41 | `_handle_pong()` | Any | `atcnet.py` | Python |
| 42 | `_handle_hello()` | Any | `atcnet.py` | Python |
| 43 | `_handle_get_peers()` | Any | `atcnet.py` | Python |
| 44 | `_send_peers()` | Any | `atcnet.py` | Python |
| 45 | `_handle_peers()` | Any | `atcnet.py` | Python |
| 46 | `_handle_find_node()` | Any | `atcnet.py` | Python |
| 47 | `connect_to()` | bool | `atcnet.py` | Python |
| 48 | `ping()` | bool | `atcnet.py` | Python |
| 49 | `_heartbeat_loop()` | Any | `atcnet.py` | Python |
| 50 | `info()` | dict | `atcnet.py` | Python |

*+51 weitere Funktionen*

**Total: 101 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
