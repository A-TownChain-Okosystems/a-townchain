# ATC-95 — ATCLang Test Framework

> **Standard-ID:** ATC-95
> **Status:** ✅ ACCEPTED — Spezifikation vollständig, 113 Tests aktiv
> **Sprint:** 2.1 (vorzeitig aus 2.7) | **Wiki:** Kap. 66, 69
> **Autor:** Aurora (MasterBrain · Base44) + Michael Wroblewski
> **Stand:** 05.07.2026 | Version 0.3.0
> **Kategorie:** Testing | **Tier:** Tier 0 — Foundational
> **Abhängigkeiten:** ATC-92, ATC-93, ATC-94, ATC-98

---

## 1. Zweck

ATC-95 spezifiziert das offizielle Test-Framework für ATCLang. Es definiert Syntax für Test-Definitionen, Assertions, Mock-Infrastruktur und Coverage-Messung.

### Aktuelle Implementierung

```
tests/
├── test_atclang.py        — 53 Tests (Lexer, VM, Integration) ✅
├── test_atclang_v03.py    — 7 Tests (v0.3 Features) ✅
├── unit/
│   └── test_atclang.py    — 53 Tests (Unit Tests) ✅
└── conftest.py            — pytest Konfiguration
```

**113/113 Tests GRÜN** ✅

---

## 2. Test-Syntax (geplant für ATCLang-native Tests)

```atc
test "transfer_sufficient_balance" {
    setup {
        let token = ShivaToken::new()
        token.mint(alice, 1000)
    }
    
    assert_eq(token.balance(alice), 1000, "Balance should be 1000")
    assert_true(token.transfer(alice, bob, 500), "Transfer should succeed")
    assert_eq(token.balance(alice), 500, "Alice balance after transfer")
    assert_eq(token.balance(bob), 500, "Bob balance after transfer")
    
    teardown {
        token.destroy()
    }
}
```

### 2.1 Aktuelle Python-basierte Tests (Übergang)

Aktuell werden Tests in Python mit pytest geschrieben. Die Migration zu ATCLang-native Tests erfolgt in Sprint 2.7.

```python
class TestATCLangIntegration(unittest.TestCase):
    def run_source(self, source):
        module = compile_source(source)
        vm = ATCVM()
        for fname, instrs in module.functions.items():
            params = module.function_params.get(fname, [])
            fn = ATCFunction(name=fname, params=params, instructions=instrs)
            vm.register_function(fn)
        main_instrs = list(module.instructions)
        if main_instrs and main_instrs[-1].op == OP.HALT:
            main_instrs[-1] = Instruction(OP.RETURN)
        return vm.execute(main_instrs), vm
    
    def test_simple_addition(self):
        result, _ = self.run_source("let x = 10\nlet y = 32\nx + y")
        self.assertEqual(result, 42)
```

---

## 3. Test-Typen

| Typ | Beschreibung | Beispiel |
|------|-------------|---------|
| **Unit-Test** | Einzelne Funktionen/Module | Lexer tokenizes correctly |
| **Integration-Test** | Modul-übergreifend | Lexer → Parser → Compiler → VM |
| **E2E-Test** | Vollständige Node-Operationen | Contract deploy + call |
| **Gas-Test** | Gas-Cost-Verifikation | Each opcode has correct gas cost |
| **Regression-Test** | Bug-Fix Verifikation | v03 SyntaxError fix |

---

## 4. Assertion-Types

| Assertion | Beschreibung | Gas |
|-----------|-------------|-----|
| `assert_eq(a, b, msg)` | Gleichheit prüfen | 10 |
| `assert_true(cond, msg)` | Wahrheitswert prüfen | 5 |
| `assert_false(cond, msg)` | Falschheitswert prüfen | 5 |
| `assert_throws(block, error_type)` | Exception prüfen | 15 |
| `assert_neq(a, b, msg)` | Ungleichheit prüfen | 10 |
| `assert_contains(coll, item, msg)` | Enthaltensein prüfen | 10 |
| `assert_gas(fn, max_gas)` | Gas-Verbrauch prüfen | 5 |

---

## 5. Mock-Infrastruktur

| Mock | Beschreibung |
|------|-------------|
| **Context: test** | Keine echten Nebenwirkungen |
| **Network** | Mock-Peers, vordefinierte Responses |
| **Storage** | In-Memory, isoliert pro Test |
| **Time** | Kontrollierbare Timestamps |
| **Random** | Seeded RNG für deterministische Tests |

---

## 6. Coverage-Anforderungen

| Metrik | Minimum | Ziel |
|--------|---------|------|
| **Line Coverage** | 85% | 95% |
| **Branch Coverage** | 70% | 90% |
| **Function Coverage** | 90% | 100% |
| **Gas-Cost Coverage** | 80% | 100% |

### Aktuelle Coverage

| Modul | Zeilen | Tests | Coverage |
|-------|--------|-------|----------|
| lexer/lexer.py | 563 | 20 | ~95% |
| parser/parser.py | 399 | 7 (integration) | ~80% |
| parser/ast_nodes.py | 264 | — (via parser) | ~90% |
| compiler/compiler.py | 540+ | 7 (integration) | ~85% |
| vm/atcvm.py | 887 | 26 | ~90% |
| v03/atclang_v03_features.py | 300 | 7 | ~85% |
| **Overall** | ~2.950 | 113 | **~87%** |

---

## 7. CI/CD-Integration

```bash
# Test-Runner
atc test                    # Alle Tests
atc test --unit             # Nur Unit-Tests
atc test --integration      # Nur Integration-Tests
atc test --gas              # Gas-Cost-Tests
atc test --ci               # CI-Modus (keine Stdin, XML-Output)
atc test --coverage         # Mit Coverage-Report
```

### GitHub Actions Integration

```yaml
- name: ATCLang Tests
  run: |
    pip install pytest
    pytest tests/ -v --tb=short --junitxml=test-results.xml
```

---

## 8. Test-Datei-Struktur

```
tests/
├── conftest.py                    — pytest Konfiguration, sys.path
├── test_atclang.py                — Haupt-Test-Suite (53 Tests)
├── test_atclang_v03.py            — v0.3 Feature Tests (7 Tests)
├── unit/
│   └── test_atclang.py            — Unit Tests (53 Tests)
├── integration/                    — (geplant)
│   ├── test_contract_deploy.py
│   ├── test_multinode.py
│   └── test_gas_metering.py
└── e2e/                            — (geplant)
    ├── test_block_validation.py
    └── test_consensus_round.py
```

---

## 9. Test-Helpers

### run_source(source: str) → (result, vm)

Kompiliert ATCLang-Source-Code, registriert Functions und führt aus.
- Top-Level Expressions: Ergebnis bleibt auf Stack
- Contract Functions: Erste Function wird ausgeführt
- Function Params: Korrekt aus CompiledModule extrahiert

### I(op, *args) → Instruction

Erstellt eine VM-Instruction für direkte VM-Tests.

### vm() → ATCVM

Erstellt eine frische VM-Instanz mit Default-Gas-Limit.

---

## 10. Sprint-Zuweisung

- **Sprint 2.1** — Vorzeitig aktiviert (Pipeline-Reparatur)
- **Sprint 2.7** — ATCLang-native Test-Syntax Implementation
- **Status:** 113 Tests aktiv, Framework spezifiziert
- **Nächste Schritte:** Integration-Tests für Contracts, E2E-Tests für Nodes

---

## Referenzen

- **ATC-92:** ATCLang Language Specification
- **ATC-93:** ATCLang Virtual Machine (105 Opcodes)
- **ATC-94:** ATCLang Standard Library (6 Module)
- **ATC-98:** Testing Standard (allgemeine Vorgaben)
- **ATC-99:** ATCLang Universal Mandate (AD-006)
- **Code:** `tests/` Verzeichnis (113 Tests, alle GRÜN)
- **Wiki:** Kap. 66 (Stdlib), Kap. 69 (Konsistenz-Audit)

---

*Spezifikation: Aurora (MasterBrain · Base44) + Michael Wroblewski · 05.07.2026 · ATC-95 v0.3.0*
