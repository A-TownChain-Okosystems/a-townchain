# 🏛️ Package Manager (`atcpkg`) Architecture

> **Repository:** [atc-atcpkg](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atcpkg)  
> **Stand:** 2026-08-05  

---

## 📌 Übersicht & Kernel-Anbindung

`atcpkg` steuert die Verteilung, Versionierung und Modul-Registrierung im A-TownChain Ökosystem. Über Protokolle wie **ATC-96 (Kernel Interface Protocol)** und **ATC-24 (Agent Scheduling)** kommuniziert `atcpkg` direkt mit dem KAI-OS Microkernel.

```
 +------------------+     +-------------------+
 |  atcpkg CLI      | --> | Dependency Tree   |
 +------------------+     +-------------------+
          |                         |
          v                         v
 +------------------+     +-------------------+
 | Registry Client  |     | Kernel IPC (ATC96)|
 +------------------+     +-------------------+
```

---

## 🛠️ Modul-Manager

- **`kernel/manager.atc`**: Kernel-nahe Paket-Registrierung, Memory-Map Registrierung und Modul-Lifecycle Management.
- **`tools/manager.atc`**: CLI-Verarbeitung, Verzeichnis-Scans, Dependency Tree Unwrapping.
