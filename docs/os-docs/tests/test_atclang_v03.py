# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Tests: ATCLang v0.3.0 Features (Fix #35)"""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from atclang.v03.atclang_v03_features import (
    ClosureFactory, TypeInference, ModuleRegistry, ATCModule,
    atc_format, ErrorReporter, GenericType, ATCLANG_V03_FEATURES
)

def test_closures():
    c = ClosureFactory.parse("|x: u128| safe_mul(x, 2)")
    assert c is not None
    result = c.call(21)
    assert result == 42
    print("  ✅ Closures")

def test_type_inference():
    assert TypeInference.infer(42)    == "u8"
    assert TypeInference.infer(True)  == "bool"
    assert TypeInference.infer("hi")  == "string"
    assert TypeInference.infer_from_literal("true") == "bool"
    assert TypeInference.infer_from_literal('"hello"') == "string"
    print("  ✅ Type Inference")

def test_module_system():
    m = ATCModule("ATC::Gaming::Shivamon")
    m.export("breed", lambda a, b: f"offspring({a},{b})")
    ModuleRegistry.register("ATC::Gaming::Shivamon", m)
    resolved = ModuleRegistry.resolve("ATC::Gaming::Shivamon")
    assert resolved is not None
    assert resolved.get("breed") is not None
    print("  ✅ Module-System")

def test_string_interpolation():
    result = atc_format("Balance: {bal} ATC", {"bal": 1000})
    assert "1000" in result
    print("  ✅ String-Interpolation")

def test_error_reporter():
    er = ErrorReporter()
    er.error("ATC-PARSE-001", "UnexpectedToken", 5, 12,
             "let x = \nfn broken(")
    assert er.has_errors()
    formatted = er.format_all()
    assert "Zeile 5" in formatted
    print("  ✅ Error-Reporter (Zeile+Spalte)")

def test_generics():
    g = GenericType("Vec", ["T"])
    resolved = g.resolve({"T": "u128"})
    assert "u128" in resolved
    print("  ✅ Generics")

def test_version():
    assert ATCLANG_V03_FEATURES["version"] == "0.3.0"
    assert len(ATCLANG_V03_FEATURES["features"]) >= 7
    print("  ✅ Version 0.3.0 deklariert")

if __name__ == "__main__":
    print("=== Tests: ATCLang v0.3.0 ===")
    test_closures()
    test_type_inference()
    test_module_system()
    test_string_interpolation()
    test_error_reporter()
    test_generics()
    test_version()
    print("✅ Alle ATCLang v0.3.0 Tests bestanden")
