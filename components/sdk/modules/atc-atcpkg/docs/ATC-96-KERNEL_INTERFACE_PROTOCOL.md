# ATC-96 — Kernel Interface (KAI-OS Syscalls)

> **Standard-ID:** ATC-96  
> **Kategorie:** Kernel  
> **Tier:** 1 — Core Infrastructure  
> **Status:** 📝 DRAFT — Spezifikation in Arbeit  
> **Version:** 0.1.0  
> **Autor:** Michael Wroblewski  
> **Sprint:** 2.4  
> **Wiki-Ref:** Kapitel 37, 69  
> **Abhängigkeiten:** ATC-08, ATC-09, ATC-10, ATC-21, ATC-22  

---

## 1. Zweck

ATC-96 definiert die Syscall-Schnittstelle zwischen ATCLang-Programmen und dem KAI-OS Kernel. Es umfasst Prozessverwaltung, Speicher, I/O und Netzwerk-Operationen.

## 2. Syscall-Katalog

### 2.1 Prozessverwaltung
| Syscall | Beschreibung | Gas |
|---------|-------------|-----|
| `spawn(name)` | Neuer Prozess | 500 |
| `kill(pid)` | Prozess beenden | 100 |
| `sleep(ms)` | Pause | 10/ms |

### 2.2 Speicher
| Syscall | Beschreibung | Gas |
|---------|-------------|-----|
| `alloc(size)` | Heap-Allokation | 10/byte |
| `free(ptr)` | Freigabe | 5 |
| `memcpy(dst, src, n)` | Kopieren | 1/byte |

### 2.3 I/O
| Syscall | Beschreibung | Gas |
|---------|-------------|-----|
| `read(fd, buf, n)` | Lesen | 20 |
| `write(fd, buf, n)` | Schreiben | 20 |
| `open(path)` | Datei öffnen | 50 |

## 3. Context-Isolation

| Context | Syscalls verfügbar |
|---------|-------------------|
| Node | Alle |
| Contract | Nur alloc/free, keine I/O |
| Test | Mock-Syscalls |

## 4. Gas-Cost-Tabelle

Siehe Sektion 2 — alle Syscalls haben definierte Gas-Kosten.

## 5. Testing-Strategie

| Test | Beschreibung |
|------|-------------|
| Unit | Jeder Syscall einzeln |
| Integration | Spawn→Execute→Kill cycle |
| E2E | Multi-Prozess-Scheduling |
| Gas | Gas-Cost-Assertion pro Syscall |

## 6. Querverweise

| Referenz | Dokument |
|----------|---------|
| ATC-08 | Process Scheduling |
| ATC-09 | Memory Management |
| ATC-10 | File System (ATCFS) |
| ATC-21 | Hardware Abstraction |
| ATC-22 | Device Drivers |
| ATC-99 | ATCLang Universal Mandate |
