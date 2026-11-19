# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems.
# All Rights Reserved.

"""
ATCLang Compiler Package
========================

Public compiler API for ATCLang.

Pipeline:

    Source
      ↓
    Lexer
      ↓
    Parser
      ↓
    AST
      ↓
    TypeChecker
      ↓
    Compiler
      ↓
    ATC Bytecode
      ↓
    ATC VM

This package exposes compiler-level APIs only.
Lexer, parser, VM and runtime components remain
in their respective ATCLang packages.
"""

from .compiler import (
    ATCCompiler,
    CompiledModule,
    compile_source,
    disassemble,
)
from .errors import (
    CompileError,
)
from .optimizer import (
    ATCOptimizer,
    OptimizationStats,
    OptimizerConfig,
)
from .symbols import (
    Symbol,
    SymbolTable,
)
from .type_checker import (
    ATCGenericType,
    ATCType,
    ATCTypeChecker,
    TypeEnvironment,
)
from .type_checker import (
    TypeError as ATCTypeError,
)

__all__ = [
    # Compiler
    "ATCCompiler",
    "CompiledModule",
    "compile_source",
    "disassemble",

    # Symbols
    "Symbol",
    "SymbolTable",

    # Type system
    "ATCTypeChecker",
    "ATCType",
    "ATCGenericType",
    "TypeEnvironment",
    "ATCTypeError",

    # Optimizer
    "ATCOptimizer",
    "OptimizerConfig",
    "OptimizationStats",

    # Errors
    "CompileError",
]

__version__ = "0.3.0"
