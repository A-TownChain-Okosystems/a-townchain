# 📋 File Register — atc-contracts module mirror

This directory is a synchronized ATCLang mirror of the canonical `atc-contracts` sources.

## Production-language policy

- Production contract implementations: **ATCLang only**
- Production `.py` contract implementations: **0**
- Python contract runtime dependency: **removed**
- Python package setup/dependencies: **removed**

The canonical source of truth is the repository root. This mirror must not reintroduce Python contract implementations.

## Verification

The root repository validates all `.atc` files with the ATCLang consensus SecurityGate and semantic compiler, and validates executable behavior through the native Rust ATVM EXEC-CHAIN gate.
