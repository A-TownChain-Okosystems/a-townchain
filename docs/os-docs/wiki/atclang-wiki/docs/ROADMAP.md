# 🗺️ ATCLang Modular System Roadmap

> **Repository:** [atc-atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atclang)  
> **Stand:** 2026-08-05  

---

## 🎯 Phase 1: v0.3.0 Stabilization (Aktuell)
- [x] Integration des statischen TypeCheckers
- [x] AST-Optimizer mit Constant Folding
- [x] Erweiterung der Standardbibliothek (`crypto_ext`, `collections_ext`, `io_ext`)
- [x] Durchsetzung der ATC-99 First Policy in allen Modulen

## 🎯 Phase 2: Native Code Emission (Q3/2026)
- [ ] LLVM-IR / WebAssembly Compiler Target Backend
- [ ] JIT-Compilation (Just-In-Time) für hochfrequente ATVM Loops
- [ ] Verbesserte Debugging-Symbole und Source-Mapping Support

## 🎯 Phase 3: Ecosystem Consolidation (Q4/2026)
- [ ] Nahtlose Integration mit `atc-atcpkg` Paket-Registrierung
- [ ] Automatisierte Benchmarking Suite für Gas- & Speicherverbrauch
- [ ] Vollsyntaktisches LSP (Language Server Protocol) für ATCLang IDEs
