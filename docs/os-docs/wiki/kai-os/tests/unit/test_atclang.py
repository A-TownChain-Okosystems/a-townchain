# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCLang — Vollständige Test-Suite
Testet Lexer, Parser, Compiler und VM
"""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import unittest
from atclang.lexer.lexer import ATCLexer, TT, Token
from atclang.parser.parser import ATCParser
from atclang.compiler.compiler import ATCCompiler, compile_source, disassemble
from atclang.vm.atcvm import ATCVM, ATCVMError, RequireError, OP, Instruction, ATCFunction


# ═══════════════════════════════════════════════════════════
# LEXER TESTS
# ═══════════════════════════════════════════════════════════

class TestATCLexer(unittest.TestCase):

    def tokenize(self, src):
        return ATCLexer(src).tokenize()

    def test_integer_literal(self):
        tokens = self.tokenize("42")
        self.assertEqual(tokens[0].type, TT.INT)
        self.assertEqual(tokens[0].value, 42)

    def test_integer_with_underscore(self):
        tokens = self.tokenize("1_000_000")
        self.assertEqual(tokens[0].type, TT.INT)
        self.assertEqual(tokens[0].value, 1000000)

    def test_float_literal(self):
        tokens = self.tokenize("3.14")
        self.assertEqual(tokens[0].type, TT.FLOAT)
        self.assertAlmostEqual(tokens[0].value, 3.14)

    def test_string_literal(self):
        tokens = self.tokenize('"hallo welt"')
        self.assertEqual(tokens[0].type, TT.STRING)
        self.assertEqual(tokens[0].value, "hallo welt")

    def test_string_escape(self):
        tokens = self.tokenize('"hallo\\nwelt"')
        self.assertEqual(tokens[0].value, "hallo\nwelt")

    def test_bool_true(self):
        tokens = self.tokenize("true")
        self.assertEqual(tokens[0].type, TT.BOOL)
        self.assertEqual(tokens[0].value, True)

    def test_bool_false(self):
        tokens = self.tokenize("false")
        self.assertEqual(tokens[0].type, TT.BOOL)
        self.assertEqual(tokens[0].value, False)

    def test_keyword_fn(self):
        tokens = self.tokenize("fn")
        self.assertEqual(tokens[0].type, TT.KEYWORD)
        self.assertEqual(tokens[0].value, "fn")

    def test_keyword_contract(self):
        tokens = self.tokenize("contract")
        self.assertEqual(tokens[0].type, TT.KEYWORD)

    def test_all_keywords(self):
        from atclang.lexer.lexer import KEYWORDS
        for kw in ["wallet", "contract", "fn", "state", "emit", "require", "return",
                   "if", "else", "for", "while", "let", "const", "mint", "burn"]:
            self.assertIn(kw, KEYWORDS, f"Keyword '{kw}' fehlt")

    def test_type_tokens(self):
        for typ in ["UInt256", "Address", "Bool", "String", "Map"]:
            tokens = self.tokenize(typ)
            self.assertEqual(tokens[0].type, TT.TYPE, f"Typ '{typ}' nicht erkannt")

    def test_operators(self):
        cases = [
            ("==", TT.EQEQ), ("!=", TT.NEQ), ("->", TT.ARROW),
            ("::", TT.DCOLON), (":=", TT.ASSIGN), (">=", TT.GTE), ("<=", TT.LTE),
        ]
        for src, expected_type in cases:
            tokens = self.tokenize(src)
            self.assertEqual(tokens[0].type, expected_type, f"Operator '{src}' falsch erkannt")

    def test_delimiter_tokens(self):
        cases = [("(", TT.LPAREN), (")", TT.RPAREN), ("{", TT.LBRACE),
                 ("}", TT.RBRACE), (",", TT.COMMA), (".", TT.DOT), ("@", TT.AT)]
        for src, expected in cases:
            tokens = self.tokenize(src)
            self.assertEqual(tokens[0].type, expected)

    def test_line_comment_ignored(self):
        tokens = self.tokenize("42 // das ist ein kommentar\n100")
        values = [t.value for t in tokens if t.type == TT.INT]
        self.assertIn(42, values)
        self.assertIn(100, values)

    def test_block_comment_ignored(self):
        tokens = self.tokenize("42 /* block kommentar */ 100")
        values = [t.value for t in tokens if t.type == TT.INT]
        self.assertEqual(values, [42, 100])

    def test_identifier(self):
        tokens = self.tokenize("meine_variable")
        self.assertEqual(tokens[0].type, TT.IDENT)
        self.assertEqual(tokens[0].value, "meine_variable")

    def test_line_col_tracking(self):
        tokens = self.tokenize("a\nb")
        self.assertEqual(tokens[0].line, 1)
        idents = [t for t in tokens if t.type == TT.IDENT]
        self.assertEqual(idents[1].line, 2)

    def test_full_transfer_snippet(self):
        src = """
fn transfer(to: Address, amount: UInt256) -> Bool {
    require(balance >= amount)
    return true
}
"""
        tokens = self.tokenize(src)
        kws = [t.value for t in tokens if t.type == TT.KEYWORD]
        self.assertIn("fn", kws)
        self.assertIn("require", kws)
        self.assertIn("return", kws)


# ═══════════════════════════════════════════════════════════
# VM TESTS
# ═══════════════════════════════════════════════════════════

class TestATCVM(unittest.TestCase):

    def vm(self):
        return ATCVM()

    def run_vm(self, instructions):
        return self.vm().execute(instructions)

    def I(self, op, *args):
        return Instruction(op, list(args))

    def test_push_pop(self):
        result = self.run_vm([
            self.I(OP.PUSH, 42),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 42)

    def test_addition(self):
        result = self.run_vm([
            self.I(OP.PUSH, 10),
            self.I(OP.PUSH, 32),
            self.I(OP.ADD),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 42)

    def test_subtraction(self):
        result = self.run_vm([
            self.I(OP.PUSH, 100),
            self.I(OP.PUSH, 58),
            self.I(OP.SUB),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 42)

    def test_multiplication(self):
        result = self.run_vm([
            self.I(OP.PUSH, 6),
            self.I(OP.PUSH, 7),
            self.I(OP.MUL),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 42)

    def test_division(self):
        result = self.run_vm([
            self.I(OP.PUSH, 84),
            self.I(OP.PUSH, 2),
            self.I(OP.DIV),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 42)

    def test_division_by_zero(self):
        vm = self.vm()
        with self.assertRaises(ATCVMError):
            vm.execute([
                self.I(OP.PUSH, 10),
                self.I(OP.PUSH, 0),
                self.I(OP.DIV),
            ])

    def test_modulo(self):
        result = self.run_vm([
            self.I(OP.PUSH, 10),
            self.I(OP.PUSH, 3),
            self.I(OP.MOD),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 1)

    def test_comparison_eq(self):
        result = self.run_vm([
            self.I(OP.PUSH, 42),
            self.I(OP.PUSH, 42),
            self.I(OP.EQ),
            self.I(OP.RETURN),
        ])
        self.assertTrue(result)

    def test_comparison_neq(self):
        result = self.run_vm([
            self.I(OP.PUSH, 1),
            self.I(OP.PUSH, 2),
            self.I(OP.NEQ),
            self.I(OP.RETURN),
        ])
        self.assertTrue(result)

    def test_comparison_lt(self):
        result = self.run_vm([
            self.I(OP.PUSH, 5),
            self.I(OP.PUSH, 10),
            self.I(OP.LT),
            self.I(OP.RETURN),
        ])
        self.assertTrue(result)

    def test_logical_and(self):
        result = self.run_vm([
            self.I(OP.PUSH, True),
            self.I(OP.PUSH, True),
            self.I(OP.AND),
            self.I(OP.RETURN),
        ])
        self.assertTrue(result)

    def test_logical_or(self):
        result = self.run_vm([
            self.I(OP.PUSH, False),
            self.I(OP.PUSH, True),
            self.I(OP.OR),
            self.I(OP.RETURN),
        ])
        self.assertTrue(result)

    def test_logical_not(self):
        result = self.run_vm([
            self.I(OP.PUSH, False),
            self.I(OP.NOT),
            self.I(OP.RETURN),
        ])
        self.assertTrue(result)

    def test_store_load(self):
        vm = self.vm()
        result = vm.execute([
            self.I(OP.PUSH, 99),
            self.I(OP.STORE, "x"),
            self.I(OP.LOAD, "x"),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 99)

    def test_dup(self):
        result = self.run_vm([
            self.I(OP.PUSH, 5),
            self.I(OP.DUP),
            self.I(OP.ADD),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 10)

    def test_jump_unconditional(self):
        result = self.run_vm([
            self.I(OP.PUSH, 1),
            self.I(OP.JUMP, 3),    # -> Zeile 3
            self.I(OP.PUSH, 999),  # wird übersprungen
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 1)

    def test_jump_if_true(self):
        result = self.run_vm([
            self.I(OP.PUSH, True),
            self.I(OP.JUMP_IF, 3),
            self.I(OP.PUSH, 999),  # übersprungen
            self.I(OP.PUSH, 42),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 42)

    def test_jump_not_false(self):
        result = self.run_vm([
            self.I(OP.PUSH, False),
            self.I(OP.JUMP_NOT, 3),
            self.I(OP.PUSH, 999),
            self.I(OP.PUSH, 1),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 1)

    def test_require_pass(self):
        result = self.run_vm([
            self.I(OP.PUSH, True),
            self.I(OP.REQUIRE, "Fehler"),
            self.I(OP.PUSH, 1),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, 1)

    def test_require_fail(self):
        with self.assertRaises(RequireError):
            self.run_vm([
                self.I(OP.PUSH, False),
                self.I(OP.REQUIRE, "Test-Fehler"),
            ])

    def test_emit_event(self):
        vm = self.vm()
        vm.execute([
            self.I(OP.PUSH, "ShivaCore"),
            self.I(OP.PUSH, 100),
            self.I(OP.EMIT, "Transfer", 2),
            self.I(OP.PUSH, 1),
            self.I(OP.RETURN),
        ])
        events = vm.get_events()
        self.assertEqual(len(events), 1)
        self.assertEqual(events[0]['event'], "Transfer")

    def test_new_map(self):
        result = self.run_vm([
            self.I(OP.NEW_MAP),
            self.I(OP.RETURN),
        ])
        self.assertIsInstance(result, dict)

    def test_new_list(self):
        result = self.run_vm([
            self.I(OP.NEW_LIST),
            self.I(OP.RETURN),
        ])
        self.assertIsInstance(result, list)

    def test_gas_metering(self):
        vm = self.vm()
        vm.execute([
            self.I(OP.PUSH, 1),
            self.I(OP.PUSH, 2),
            self.I(OP.ADD),
            self.I(OP.RETURN),
        ])
        self.assertGreater(vm.gas_used, 0)

    def test_gas_limit_exceeded(self):
        vm = self.vm()
        vm.gas_limit = 5  # sehr niedriges Limit
        with self.assertRaises(ATCVMError):
            vm.execute([self.I(OP.PUSH, i) for i in range(100)] + [self.I(OP.RETURN)])

    def test_stack_underflow(self):
        with self.assertRaises(ATCVMError):
            self.run_vm([self.I(OP.POP)])

    def test_string_concat(self):
        result = self.run_vm([
            self.I(OP.PUSH, "hallo "),
            self.I(OP.PUSH, "welt"),
            self.I(OP.ADD),
            self.I(OP.RETURN),
        ])
        self.assertEqual(result, "hallo welt")


# ═══════════════════════════════════════════════════════════
# COMPILER + VM INTEGRATION TESTS
# ═══════════════════════════════════════════════════════════

class TestATCLangIntegration(unittest.TestCase):

    def run_source(self, source):
        module = compile_source(source)
        vm = ATCVM()
        for fname, instrs in module.functions.items():
            params = module.function_params.get(fname, [])
            fn = ATCFunction(name=fname, params=params, instructions=instrs)
            vm.register_function(fn)
        main_instrs = module.instructions if len(module.instructions) > 1 else (
            list(module.functions.values())[0] if module.functions else module.instructions
        )
        return vm.execute(main_instrs), vm

    def test_simple_addition(self):
        result, _ = self.run_source("let x = 10\nlet y = 32\nx + y")
        self.assertEqual(result, 42)

    def test_let_declaration(self):
        result, vm = self.run_source("let answer = 42\nanswer")
        self.assertEqual(result, 42)

    def test_if_true(self):
        result, _ = self.run_source("""
let x = 10
if x > 5 {
    100
} else {
    0
}
""")
        self.assertEqual(result, 100)

    def test_if_false(self):
        result, _ = self.run_source("""
let x = 3
if x > 5 {
    100
} else {
    0
}
""")
        self.assertEqual(result, 0)

    def test_function_call(self):
        result, _ = self.run_source("""
fn double(n: UInt256) -> UInt256 {
    return n * 2
}
double(21)
""")
        self.assertEqual(result, 42)

    def test_recursive_function(self):
        result, _ = self.run_source("""
fn fib(n: UInt256) -> UInt256 {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
fib(10)
""")
        self.assertEqual(result, 55)

    def test_compile_source_creates_module(self):
        module = compile_source("let x = 1")
        self.assertIsNotNone(module)

    def test_disassemble(self):
        module = compile_source("let x = 42")
        asm = disassemble(module)
        self.assertIsInstance(asm, str)
        self.assertGreater(len(asm), 0)


if __name__ == '__main__':
    unittest.main(verbosity=2)
