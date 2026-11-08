# 🏛️ ATVM (ATCLang Virtual Machine) Architecture Specification

> **Repository:** [atc-vm](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-vm)  
> **Stand:** 2026-08-05  

---

## 📌 Architektur-Konzept

Die **ATVM** ist eine Stack-basierte, isolierte virtuelle Maschine. Sie verarbeitet deterministischen Bytecode, der aus ATCLang-Programmen kompiliert wurde.

---

## 🧠 Kernkomponenten

```
+---------------------------------------------------------------+
|                       ATVM Runtime State                      |
+---------------------------------------------------------------+
|                                                               |
|  [ Instruction Pointer (IP) ]   [ Frame Pointer (FP) ]        |
|                                                               |
|  [ Operand Stack ]              [ Call Stack / Frames ]       |
|  - Values, Local Variables      - Function Context, Returns   |
|                                                               |
|  [ Memory / Heap Registry ]     [ Gas Counter & Limits ]      |
|  - Dynamic Structures           - Instruction Cost Tracking   |
|                                                               |
+---------------------------------------------------------------+
                               |
                               v
+---------------------------------------------------------------+
|                   Opcode Dispatcher Loop                      |
| (Fetches Opcode, Decodes, Executes State Mutation, Costs Gas) |
+---------------------------------------------------------------+
```

---

## 🔒 Security & ATC-99 Policy Enforcement

1. **Sandboxing:** Keine ungeprüften Systemaufrufe. Alle I/O-, Netzwerk- oder Chain-Interaktionen laufen über abgesicherte Syscalls.
2. **Gas Metering:** Jede Bytecode-Instruktion verbraucht eine definierte Menge an Gas. Bei Unterschreitung bricht die Ausführung deterministisch ab (`OUT_OF_GAS`).
3. **ATC-99 License Verification:** Vor der ersten Instruktion verifiziert ATVM die Gültigkeit der Smart Contract-Lizenz.
