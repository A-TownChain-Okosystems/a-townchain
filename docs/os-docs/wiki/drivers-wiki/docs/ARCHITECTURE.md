# 🏗️ Treiber-Architektur — atc-drivers

---

## 1. Hardware Abstraction Layer (HAL)

Der ShivaOS Microkernel verwendet ein isoliertes Treibermodell. Treiber laufen mit eingeschränkten Rechten und kommunizieren über MMIO (Memory-Mapped I/O) und DMA Buffer.

```
[ Kernel Ring 0 ] <──> [ HAL Interface ] <──> [ Driver Ring 1/3 ] <──> [ Physical HW ]
```
