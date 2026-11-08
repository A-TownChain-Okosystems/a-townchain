# A-TownChain Agent Guidelines

This file dictates how AI Agents interact with the A-TownChain core system and the ATS layer.

## System Integration
* All agents must communicate via the BCL (Blockchain Command Language).
* Agents function in an isolated Ring 3 Userspace (WASM Sandbox).
* The AI Agent Governor regulates resource consumption.

## Security Constraints
* No direct hardware access.
* Zero-Knowledge Proofs must be generated for autonomous task verification.
* Any changes to the consensus layer are strictly forbidden.
