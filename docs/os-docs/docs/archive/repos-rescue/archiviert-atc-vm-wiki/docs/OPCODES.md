# 📜 ATVM Opcode Specification Table

> **Repository:** [atc-vm](https://github.com/A-TownChain-Okosystems/atc-vm)  
> **Stand:** 2026-08-05  

---

## 📊 Opcode-Übersicht

| Opcode Hex | Mnemonic | Stack In | Stack Out | Beschreibung |
|------------|----------|----------|-----------|--------------|
| `0x00` | `HALT` | - | - | Beendet die Ausführung erfolgreich |
| `0x01` | `PUSH <val>` | - | `val` | Legt einen Wert auf den Operand Stack |
| `0x02` | `POP` | `val` | - | Entfernt den obersten Stack-Wert |
| `0x03` | `DUP` | `val` | `val, val` | Dupliziert das oberste Stack-Element |
| `0x04` | `SWAP` | `a, b` | `b, a` | Vertauscht die obersten zwei Stack-Elemente |
| `0x05` | `ADD` | `a, b` | `a + b` | Addition |
| `0x06` | `SUB` | `a, b` | `a - b` | Subtraktion |
| `0x07` | `MUL` | `a, b` | `a * b` | Multiplikation |
| `0x08` | `DIV` | `a, b` | `a / b` | Division |
| `0x10` | `JMP <addr>` | - | - | Unbedingter Sprung zur Zieladresse |
| `0x11` | `JZ <addr>` | `cond` | - | Sprung wenn oberstes Element zero/false |
| `0x12` | `CALL <addr>` | `args...` | - | Funktionsaufruf, pusht neuen Call Frame |
| `0x13` | `RET` | `ret_val` | `ret_val` | Rücksprung aus Funktion |
| `0x20` | `SYSCALL <id>` | `params` | `result` | Kernel / Chain Systemaufruf |
| `0x99` | `ATC99_CHECK` | `lic_key` | `status` | Verifiziert ATC-99 Compliance |
