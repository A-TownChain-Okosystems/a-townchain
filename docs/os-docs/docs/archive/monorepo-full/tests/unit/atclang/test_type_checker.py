# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATCLang Type Checker Tests — ATC-92
Tests für statische Typ-Prüfung zur Compile-Zeit.
"""
import unittest
import sys
import os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from atclang.compiler.type_checker import ATCTypeChecker, ATCType
from atclang.parser.parser import parse


class TestTypeInference(unittest.TestCase):
    """Type inference for literals and expressions."""

    def _check(self, code: str):
        ast = parse(code)
        checker = ATCTypeChecker()
        return checker.check(ast)

    def test_int_literal(self):
        errors = self._check("let x = 42")
        self.assertEqual(len(errors), 0)

    def test_float_literal(self):
        errors = self._check("let x = 3.14")
        self.assertEqual(len(errors), 0)

    def test_string_literal(self):
        errors = self._check('let x = "hello"')
        self.assertEqual(len(errors), 0)

    def test_bool_literal(self):
        errors = self._check("let x = true")
        self.assertEqual(len(errors), 0)

    def test_string_concat(self):
        errors = self._check('let s = "hello" + " world"')
        self.assertEqual(len(errors), 0)

    def test_arithmetic_int(self):
        errors = self._check("let x = 10 + 32")
        self.assertEqual(len(errors), 0)

    def test_arithmetic_float(self):
        errors = self._check("let x = 3.14 * 2.0")
        self.assertEqual(len(errors), 0)

    def test_mixed_arithmetic(self):
        errors = self._check("let x = 10 + 3.14")
        self.assertEqual(len(errors), 0)

    def test_comparison(self):
        errors = self._check("let x = 10 > 5")
        self.assertEqual(len(errors), 0)

    def test_logical_and(self):
        errors = self._check("let x = true && false")
        self.assertEqual(len(errors), 0)

    def test_logical_or(self):
        errors = self._check("let x = true || false")
        self.assertEqual(len(errors), 0)

    def test_bitwise_and(self):
        errors = self._check("let x = 255 & 0xFF")
        self.assertEqual(len(errors), 0)


class TestTypeErrors(unittest.TestCase):
    """Type error detection."""

    def _check(self, code: str):
        ast = parse(code)
        checker = ATCTypeChecker()
        return checker.check(ast)

    def test_type_mismatch_let(self):
        errors = self._check('let x: Int = "hello"')
        self.assertEqual(len(errors), 1)
        self.assertIn("expected", errors[0].message)

    def test_non_bool_if_condition(self):
        errors = self._check("if 42 { let x = 1 }")
        self.assertEqual(len(errors), 1)
        self.assertIn("Bool", errors[0].message)

    def test_non_bool_while_condition(self):
        errors = self._check("while 42 { let x = 1 }")
        self.assertEqual(len(errors), 1)

    def test_undefined_variable(self):
        errors = self._check("x + 1")
        self.assertEqual(len(errors), 1)
        self.assertIn("undefined", errors[0].message)

    def test_non_bool_require(self):
        errors = self._check('require "hello"')
        self.assertEqual(len(errors), 1)

    def test_logical_on_non_bool(self):
        errors = self._check('let x = "hello" && true')
        self.assertEqual(len(errors), 1)

    def test_bitwise_on_string(self):
        errors = self._check('let x = "hello" & "world"')
        self.assertEqual(len(errors), 1)


class TestFunctions(unittest.TestCase):
    """Function type checking."""

    def _check(self, code: str):
        ast = parse(code)
        checker = ATCTypeChecker()
        return checker.check(ast)

    def test_function_definition(self):
        code = "fn add(a: Int, b: Int) -> Int {\n return a + b\n}"
        errors = self._check(code)
        self.assertEqual(len(errors), 0)

    def test_function_call_correct(self):
        code = "fn add(a: Int, b: Int) -> Int {\n return a + b\n}\nlet result = add(1, 2)"
        errors = self._check(code)
        self.assertEqual(len(errors), 0)

    def test_function_call_wrong_arg_count(self):
        code = "fn add(a: Int, b: Int) -> Int {\n return a + b\n}\nlet result = add(1)"
        errors = self._check(code)
        self.assertEqual(len(errors), 1)

    def test_function_void_return(self):
        code = "fn doNothing() {\n let x = 1\n}"
        errors = self._check(code)
        self.assertEqual(len(errors), 0)

    def test_recursive_function(self):
        code = "fn fib(n: Int) -> Int {\n if n <= 1 { return n }\n return fib(n - 1) + fib(n - 2)\n}"
        errors = self._check(code)
        self.assertEqual(len(errors), 0)


class TestControlFlow(unittest.TestCase):
    """Control flow type checking."""

    def _check(self, code: str):
        ast = parse(code)
        checker = ATCTypeChecker()
        return checker.check(ast)

    def test_if_else(self):
        code = "let x = 10\nif x > 5 { let y = 1 } else { let y = 2 }"
        errors = self._check(code)
        self.assertEqual(len(errors), 0)

    def test_while_loop(self):
        code = "let x = 0\nwhile x < 10 { x = x + 1 }"
        errors = self._check(code)
        self.assertEqual(len(errors), 0)

    def test_for_loop(self):
        code = "for i in range(10) { let x = i }"
        errors = self._check(code)
        self.assertEqual(len(errors), 0)

    def test_break_continue(self):
        code = "while true { break }"
        errors = self._check(code)
        self.assertEqual(len(errors), 0)


class TestContracts(unittest.TestCase):
    """Contract type checking."""

    def _check(self, code: str):
        ast = parse(code)
        checker = ATCTypeChecker()
        return checker.check(ast)

    def test_simple_contract(self):
        code = '''contract Token : ATC-89 {
    state balance: Map
    state total_supply: UInt256
    
    fn transfer(to: Address, amount: UInt256) -> Bool {
        return true
    }
}'''
        errors = self._check(code)
        self.assertEqual(len(errors), 0)

    def test_contract_with_require(self):
        code = '''contract Token : ATC-89 {
    state balance: Map
    
    fn withdraw(amount: UInt256) -> Bool {
        require(amount > 0, "Amount must be positive")
        return true
    }
}'''
        errors = self._check(code)
        self.assertEqual(len(errors), 0)


class TestTypeSystem(unittest.TestCase):
    """ATC type system primitives."""

    def test_type_equality(self):
        from atclang.compiler.type_checker import T_INT, T_STRING, T_BOOL
        self.assertEqual(T_INT, T_INT)
        self.assertNotEqual(T_INT, T_STRING)

    def test_type_registry(self):
        from atclang.compiler.type_checker import ATC_TYPE_REGISTRY, T_INT, T_STRING
        self.assertEqual(ATC_TYPE_REGISTRY["Int"], T_INT)
        self.assertEqual(ATC_TYPE_REGISTRY["String"], T_STRING)
        self.assertEqual(ATC_TYPE_REGISTRY["UInt256"].name, "UInt256")

    def test_type_compatibility(self):
        from atclang.compiler.type_checker import (
            ATCTypeChecker, T_INT, T_FLOAT, T_STRING, T_BOOL, T_ANY, T_UNKNOWN
        )
        tc = ATCTypeChecker()
        self.assertTrue(tc._is_compatible(T_INT, T_INT))
        self.assertTrue(tc._is_compatible(T_INT, T_FLOAT))
        self.assertTrue(tc._is_compatible(T_ANY, T_INT))
        self.assertFalse(tc._is_compatible(T_INT, T_STRING))
        self.assertFalse(tc._is_compatible(T_STRING, T_BOOL))

    def test_is_numeric(self):
        from atclang.compiler.type_checker import (
            ATCTypeChecker, T_INT, T_FLOAT, T_STRING, T_UINT256
        )
        tc = ATCTypeChecker()
        self.assertTrue(tc._is_numeric(T_INT))
        self.assertTrue(tc._is_numeric(T_FLOAT))
        self.assertTrue(tc._is_numeric(T_UINT256))
        self.assertFalse(tc._is_numeric(T_STRING))


if __name__ == "__main__":
    unittest.main()
