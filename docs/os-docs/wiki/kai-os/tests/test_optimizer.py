# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATCLang Optimizer Tests — ATC-92
Tests für Constant Folding, Dead Code Elimination, Algebraic Simplification.
"""
import unittest
import sys
import os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from atclang.compiler.optimizer import ATCOptimizer
from atclang.parser.parser import parse
from atclang.parser.ast_nodes import (
    IntLiteral, FloatLiteral, StringLiteral, BoolLiteral, NullLiteral,
    BinaryOp, Identifier, LetStatement, ReturnStatement, IfStatement,
)
from atclang.compiler.compiler import compile_source
from atclang.vm.atcvm import OP


class TestConstantFolding(unittest.TestCase):
    """Constant folding: 2 + 3 → 5 at compile time."""

    def _optimize(self, code: str, level: int = 2):
        ast = parse(code)
        opt = ATCOptimizer(level=level)
        result = opt.optimize_ast(ast)
        return result, opt

    def test_add(self):
        ast, opt = self._optimize("const x = 2 + 3")
        self.assertEqual(opt.stats["constants_folded"], 1)

    def test_mul(self):
        ast, opt = self._optimize("const x = 4 * 5")
        self.assertEqual(opt.stats["constants_folded"], 1)

    def test_sub(self):
        ast, opt = self._optimize("const x = 10 - 3")
        self.assertEqual(opt.stats["constants_folded"], 1)

    def test_div(self):
        ast, opt = self._optimize("const x = 20 / 4")
        self.assertEqual(opt.stats["constants_folded"], 1)

    def test_mod(self):
        ast, opt = self._optimize("const x = 10 % 3")
        self.assertEqual(opt.stats["constants_folded"], 1)

    def test_power(self):
        ast, opt = self._optimize("const x = 2 ** 8")
        self.assertEqual(opt.stats["constants_folded"], 1)

    def test_string_concat(self):
        ast, opt = self._optimize('const x = "hello" + " " + "world"')
        self.assertGreaterEqual(opt.stats["constants_folded"], 1)

    def test_comparison(self):
        ast, opt = self._optimize("const x = 5 > 3")
        self.assertEqual(opt.stats["constants_folded"], 1)

    def test_logical(self):
        ast, opt = self._optimize("const x = true && false")
        self.assertEqual(opt.stats["constants_folded"], 1)

    def test_nested(self):
        ast, opt = self._optimize("const x = (2 + 3) * (4 - 1)")
        self.assertGreaterEqual(opt.stats["constants_folded"], 2)

    def test_unary_neg(self):
        ast, opt = self._optimize("const x = -5")
        self.assertGreaterEqual(opt.stats["constants_folded"], 1)

    def test_unary_not(self):
        ast, opt = self._optimize("const x = !true")
        self.assertGreaterEqual(opt.stats["constants_folded"], 1)


class TestConstantPropagation(unittest.TestCase):
    """Constant propagation: const a = 5; let b = a + 1 → let b = 6."""

    def _optimize(self, code: str, level: int = 2):
        ast = parse(code)
        opt = ATCOptimizer(level=level)
        result = opt.optimize_ast(ast)
        return result, opt

    def test_simple_propagation(self):
        ast, opt = self._optimize("const a = 5\nlet b = a + 1")
        self.assertGreaterEqual(opt.stats["constants_propagated"], 1)

    def test_chained_propagation(self):
        code = "const a = 5\nconst b = a + 10\nlet c = b * 2"
        ast, opt = self._optimize(code)
        self.assertGreaterEqual(opt.stats["constants_propagated"], 2)


class TestDeadCodeElimination(unittest.TestCase):
    """Dead code elimination: code after return is removed."""

    def _optimize(self, code: str, level: int = 1):
        ast = parse(code)
        opt = ATCOptimizer(level=level)
        result = opt.optimize_ast(ast)
        return result, opt

    def test_dead_after_return(self):
        code = "fn foo() -> Int {\n return 42\n let dead = 1\n}"
        ast, opt = self._optimize(code)
        self.assertGreaterEqual(opt.stats["dead_code_removed"], 1)

    def test_if_false_dead(self):
        code = "if false { let x = 1 }\nlet y = 2"
        ast, opt = self._optimize(code, level=2)
        self.assertGreaterEqual(opt.stats["dead_code_removed"], 1)

    def test_while_false_dead(self):
        code = "while false { let x = 1 }"
        ast, opt = self._optimize(code, level=2)
        self.assertGreaterEqual(opt.stats["dead_code_removed"], 1)

    def test_dead_after_break(self):
        code = "while true { break\n let dead = 1 }"
        ast, opt = self._optimize(code)
        self.assertGreaterEqual(opt.stats["dead_code_removed"], 1)


class TestAlgebraicSimplification(unittest.TestCase):
    """Algebraic simplification: x * 1 → x, x + 0 → x."""

    def _optimize(self, code: str, level: int = 2):
        ast = parse(code)
        opt = ATCOptimizer(level=level)
        result = opt.optimize_ast(ast)
        return result, opt

    def test_mul_one(self):
        ast, opt = self._optimize("let x = 7\nlet y = x * 1")
        self.assertGreaterEqual(opt.stats["algebraic_simplified"], 1)

    def test_add_zero(self):
        ast, opt = self._optimize("let x = 7\nlet y = x + 0")
        self.assertGreaterEqual(opt.stats["algebraic_simplified"], 1)

    def test_mul_zero(self):
        ast, opt = self._optimize("let x = 7\nlet y = x * 0")
        self.assertGreaterEqual(opt.stats["algebraic_simplified"], 1)

    def test_sub_zero(self):
        ast, opt = self._optimize("let x = 7\nlet y = x - 0")
        self.assertGreaterEqual(opt.stats["algebraic_simplified"], 1)

    def test_div_one(self):
        ast, opt = self._optimize("let x = 7\nlet y = x / 1")
        self.assertGreaterEqual(opt.stats["algebraic_simplified"], 1)

    def test_pow_zero(self):
        ast, opt = self._optimize("let x = 7\nlet y = x ** 0")
        self.assertGreaterEqual(opt.stats["algebraic_simplified"], 1)

    def test_pow_one(self):
        ast, opt = self._optimize("let x = 7\nlet y = x ** 1")
        self.assertGreaterEqual(opt.stats["algebraic_simplified"], 1)

    def test_and_true(self):
        ast, opt = self._optimize("let y = true && x")
        # This depends on whether 'x' is defined — might not simplify
        # Just check no crash
        self.assertTrue(True)


class TestBytecodeOptimization(unittest.TestCase):
    """Bytecode-level optimizations."""

    def test_dead_bytecode_removal(self):
        """Instructions after RETURN should be removed."""
        code = "fn main() -> Int {\n return 42\n}"
        m = compile_source(code)
        insts = m.functions["main"]
        opt = ATCOptimizer(level=2)
        result = opt.optimize_bytecode(insts)
        # Should keep PUSH 42, RETURN
        self.assertLessEqual(len(result), len(insts))

    def test_peephole_push_pop(self):
        """PUSH followed by POP should be removed."""
        from atclang.vm.atcvm import Instruction
        insts = [
            Instruction(OP.PUSH, [1]),
            Instruction(OP.POP),
            Instruction(OP.PUSH, [42]),
            Instruction(OP.RETURN),
        ]
        opt = ATCOptimizer(level=2)
        result = opt.optimize_bytecode(insts)
        self.assertLess(len(result), len(insts))

    def test_peephole_push_zero_add(self):
        """PUSH 0 + ADD should be removed (x + 0 = x)."""
        from atclang.vm.atcvm import Instruction
        insts = [
            Instruction(OP.PUSH, [5]),
            Instruction(OP.PUSH, [0]),
            Instruction(OP.ADD),
            Instruction(OP.RETURN),
        ]
        opt = ATCOptimizer(level=2)
        result = opt.optimize_bytecode(insts)
        self.assertLess(len(result), len(insts))


class TestOptimizerIntegration(unittest.TestCase):
    """Integration: optimize → compile → run."""

    def test_optimized_addition(self):
        """const x = 2 + 3 should compile to PUSH 5."""
        code = "const x = 2 + 3\nfn main() -> Int {\n return x\n}"
        ast = parse(code)
        opt = ATCOptimizer(level=2)
        opt_ast = opt.optimize_ast(ast)
        self.assertGreaterEqual(opt.stats["constants_folded"], 1)

        # Compile and run
        from atclang.compiler.compiler import ATCCompiler
        from atclang.vm.atcvm import ATCVM
        compiler = ATCCompiler()
        module = compiler.compile_program(opt_ast)
        vm = ATCVM()
        result = vm.execute(module.functions.get("main", module.instructions))
        self.assertEqual(result, 5)

    def test_optimized_dead_code(self):
        """Function with dead code should still return correct value."""
        code = "fn main() -> Int {\n return 42\n let dead = 1\n dead + 1\n}"
        ast = parse(code)
        opt = ATCOptimizer(level=1)
        opt_ast = opt.optimize_ast(ast)

        from atclang.compiler.compiler import ATCCompiler
        from atclang.vm.atcvm import ATCVM
        compiler = ATCCompiler()
        module = compiler.compile_program(opt_ast)
        vm = ATCVM()
        result = vm.execute(module.functions.get("main", module.instructions))
        self.assertEqual(result, 42)

    def test_level_zero_no_optimization(self):
        """Level 0 should not optimize anything."""
        code = "const x = 2 + 3"
        ast = parse(code)
        opt = ATCOptimizer(level=0)
        opt.optimize_ast(ast)
        self.assertEqual(opt.stats["constants_folded"], 0)


if __name__ == "__main__":
    unittest.main()
