# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCLang — Die Programmiersprache des A-TownChain Ökosystems
Version: 0.3.0-alpha
"""
ATCLANG_VERSION = "0.3.0"

from atclang.compiler.compiler import (  # noqa: E402 — sys.path-Setup vor Import
    ATCCompiler,
    compile_source,
    disassemble,
)
from atclang.lexer.lexer import (  # noqa: E402 — sys.path-Setup vor Import
    TT,
    ATCLexer,
    Token,
    tokenize,
)
from atclang.parser.parser import ATCParser, parse  # noqa: E402 — sys.path-Setup vor Import
from atclang.vm.atcvm import ATCVM, OP, Instruction  # noqa: E402 — sys.path-Setup vor Import
