---
document_id: ATC-DOC-ARC-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# Architecture Specification — atc-vm

## Übersicht

`atc-vm` (A-TownChain Virtual Machine, ATVM) ist die Kern-Laufzeitumgebung (Layer L3) zur verifizierten Ausführung von ATCLang-Smart-Contracts.

## Subsysteme

1. **Bytecode Verifier:** Prüfung von Opcode-Gültigkeit, Sprungzielen, Stack-Grenzen und Speichergrenzen vor der Ausführung.
2. **Execution Engine:** Deterministische Interpretation der Instruktionen mit exakter Gas-Abrechnung.
3. **Host Interface:** Abstraktionsschicht für Blockchain-Syscalls (State Read/Write, Event Emission, Balance Transfer).
4. **License Enforcement:** Überprüfung der Ausführungslizenzen (ATC-LIC).
