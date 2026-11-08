# ATCLang — Roadmap

> Stand: 10. Juni 2026

## v0.2.0 — ABGESCHLOSSEN ✅
- Compiler Label-Cache O(1) — PERF-002 Fix
- Security Analyzer v1.0.0 (15 Regeln, ATC-SEC-0001)
- MAX_CALL_DEPTH=128, MAX_SOURCE_SIZE=1MB
- REPL ANSI-Fix (SEC-001), Stdlib safe_add/sub/mul
- ATCFS Syscall-Integration

## v0.3.0 — IN PLANUNG 🔄
- [ ] async/await (non-blocking Calls)
- [ ] Generics (`fn foo<T>(x: T)`)
- [ ] Closures / Lambda
- [ ] Pattern Matching vollständig
- [ ] Module-System (import/export)
- [ ] Type Inference
- [ ] Besserer Error-Reporter (Zeile+Spalte+Kontext)
- [ ] ATCLang → WASM (experimentell)

## Langfristig
- [ ] Formale Verifikation
- [ ] ZK-SNARK Integration
- [ ] Cross-Chain Contract Calls
