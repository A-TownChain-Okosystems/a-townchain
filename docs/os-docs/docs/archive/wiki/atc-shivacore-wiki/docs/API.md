# 🔌 API Reference — atc-shivacore

> **Repo:** [atc-shivacore](https://github.com/A-TownChain-Okosystems/atc-shivacore)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `main()` | () | `boot/src/main.rs` | Rust |
| 2 | `spawn()` | Pid | `kernel/src/ats1000.rs` | Rust |
| 3 | `kill()` | bool | `kernel/src/ats1000.rs` | Rust |
| 4 | `wait()` | ExitCode | `kernel/src/ats1000.rs` | Rust |
| 5 | `list_processes()` | () | `kernel/src/ats1000.rs` | Rust |
| 6 | `alloc()` | Option | `kernel/src/ats1000.rs` | Rust |
| 7 | `free()` | bool | `kernel/src/ats1000.rs` | Rust |
| 8 | `mmap()` | Option | `kernel/src/ats1000.rs` | Rust |
| 9 | `open()` | Option | `kernel/src/ats1000.rs` | Rust |
| 10 | `read()` | u64 | `kernel/src/ats1000.rs` | Rust |
| 11 | `write()` | u64 | `kernel/src/ats1000.rs` | Rust |
| 12 | `close()` | bool | `kernel/src/ats1000.rs` | Rust |
| 13 | `connect()` | Option | `kernel/src/ats1000.rs` | Rust |
| 14 | `send()` | bool | `kernel/src/ats1000.rs` | Rust |
| 15 | `recv()` | u64 | `kernel/src/ats1000.rs` | Rust |
| 16 | `init()` | OffsetPageTable | `kernel/src/memory.rs` | Rust |
| 17 | `active_level_4_table()` | () | `kernel/src/memory.rs` | Rust |
| 18 | `init()` | Self | `kernel/src/memory.rs` | Rust |
| 19 | `usable_frames()` | impl | `kernel/src/memory.rs` | Rust |
| 20 | `allocate_frame()` | Option | `kernel/src/memory.rs` | Rust |
| 21 | `is_kernel()` | bool | `kernel/src/userspace.rs` | Rust |
| 22 | `is_user()` | bool | `kernel/src/userspace.rs` | Rust |
| 23 | `dpl()` | u8 | `kernel/src/userspace.rs` | Rust |
| 24 | `default()` | Self | `kernel/src/userspace.rs` | Rust |
| 25 | `contains()` | bool | `kernel/src/userspace.rs` | Rust |
| 26 | `in_code()` | bool | `kernel/src/userspace.rs` | Rust |
| 27 | `in_data()` | bool | `kernel/src/userspace.rs` | Rust |
| 28 | `in_stack()` | bool | `kernel/src/userspace.rs` | Rust |
| 29 | `in_heap()` | bool | `kernel/src/userspace.rs` | Rust |
| 30 | `initial_rsp()` | u64 | `kernel/src/userspace.rs` | Rust |
| 31 | `hello_world()` | Self | `kernel/src/userspace.rs` | Rust |
| 32 | `from_bytes()` | Self | `kernel/src/userspace.rs` | Rust |
| 33 | `code_len()` | usize | `kernel/src/userspace.rs` | Rust |
| 34 | `data_len()` | usize | `kernel/src/userspace.rs` | Rust |
| 35 | `new()` | Self | `kernel/src/userspace.rs` | Rust |
| 36 | `is_user_mode()` | bool | `kernel/src/userspace.rs` | Rust |
| 37 | `valid_address()` | bool | `kernel/src/userspace.rs` | Rust |
| 38 | `set_return()` | () | `kernel/src/userspace.rs` | Rust |
| 39 | `exit()` | () | `kernel/src/userspace.rs` | Rust |
| 40 | `is_exited()` | bool | `kernel/src/userspace.rs` | Rust |
| 41 | `default()` | Self | `kernel/src/userspace.rs` | Rust |
| 42 | `verify()` | bool | `kernel/src/userspace.rs` | Rust |
| 43 | `fmt()` | core | `kernel/src/userspace.rs` | Rust |
| 44 | `default()` | Self | `kernel/src/userspace.rs` | Rust |
| 45 | `new()` | Self | `kernel/src/userspace.rs` | Rust |
| 46 | `load_binary()` | Result | `kernel/src/userspace.rs` | Rust |
| 47 | `get_context()` | Option | `kernel/src/userspace.rs` | Rust |
| 48 | `get_context_mut()` | Option | `kernel/src/userspace.rs` | Rust |
| 49 | `enter_userspace()` | Result | `kernel/src/userspace.rs` | Rust |
| 50 | `handle_syscall()` | Result | `kernel/src/userspace.rs` | Rust |

*+4756 weitere Funktionen*

**Total: 4806 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
