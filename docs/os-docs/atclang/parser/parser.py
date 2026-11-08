# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# STUB: Temporärer Python-Stub — wird in Sprint 2.1 durch ATCLang ersetzt (ATCLang First Policy, AD-006)
"""
ATCLang Parser — Recursive Descent Parser
Wandelt Token-Liste in einen AST um
Version: 0.1.0-alpha
"""

from typing import List, Optional
from .ast_nodes import *
from ..lexer.lexer import ATCLexer, Token, TT


class ATCParser:
    """
    Recursive Descent Parser für ATCLang.
    Produziert einen vollständigen AST.
    """

    def __init__(self, tokens: List[Token]):
        self.tokens  = [t for t in tokens if t.type not in (TT.NEWLINE, TT.COMMENT)]
        self.pos     = 0

    def error(self, msg: str):
        tok = self.current()
        raise SyntaxError(f"[ATCLang Parser] {msg} @ Zeile {tok.line}:{tok.col} (bekam: {tok.type.name} = {tok.value!r})")

    def current(self) -> Token:
        return self.tokens[self.pos] if self.pos < len(self.tokens) else self.tokens[-1]

    def peek(self, offset=1) -> Token:
        idx = self.pos + offset
        return self.tokens[idx] if idx < len(self.tokens) else self.tokens[-1]

    def advance(self) -> Token:
        tok = self.current()
        self.pos += 1
        return tok

    def check(self, ttype: TT, value=None) -> bool:
        tok = self.current()
        if tok.type != ttype:
            return False
        if value is not None and tok.value != value:
            return False
        return True

    def expect(self, ttype: TT, value=None) -> Token:
        if not self.check(ttype, value):
            exp = f"{ttype.name}" + (f"('{value}')" if value else "")
            self.error(f"Erwartet {exp}")
        return self.advance()

    def match(self, ttype: TT, value=None) -> Optional[Token]:
        if self.check(ttype, value):
            return self.advance()
        return None

    # ── Typ-Annotation ────────────────────────────────────
    def parse_type(self) -> TypeAnnotation:
        # Accept TYPE tokens (Int, UInt256, Address, ...) and IDENT fallback (custom types)
        if self.check(TT.TYPE):
            tok = self.advance()
        elif self.check(TT.IDENT):
            tok = self.advance()
        else:
            tok = self.expect(TT.TYPE)  # Will produce error
        node = TypeAnnotation(tok.value, [], tok.line, tok.col)
        # Generic with <>: Map<String, Int> or nested Map<String, Map<String, Int>>
        if self.match(TT.LT):
            while not self.check(TT.GT) and not (self.current().type == TT.RSHIFT):
                node.params.append(self.parse_type())
                # After parsing a param, check if inner consumed >> (closing both generics)
                if getattr(self, '_skip_outer_gt', False):
                    self._skip_outer_gt = False
                    return node  # Our > was already consumed by inner >>
                if not self.match(TT.COMMA):
                    break
            # Handle >>: it closes two nested generics at once
            if self.current().type == TT.RSHIFT:
                self.advance()
                self._skip_outer_gt = True
            else:
                self.expect(TT.GT)
        # Generic with []: List[Int], Map[String, Int]
        if self.match(TT.LBRACKET):
            while not self.check(TT.RBRACKET):
                node.params.append(self.parse_type())
                if not self.match(TT.COMMA):
                    break
            self.expect(TT.RBRACKET)
        return node

    # ── Expressions (Full Operator Precedence) ───────────
    # Precedence (low → high):
    #   || → && → | → ^ → & → == != < > <= >= → + - → * / % → ** → unary → postfix

    def parse_expr(self) -> ASTNode:
        return self.parse_logical_or()

    def parse_logical_or(self) -> ASTNode:
        left = self.parse_logical_and()
        while self.current().type == TT.OR:
            op = self.advance().value
            right = self.parse_logical_and()
            left = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_logical_and(self) -> ASTNode:
        left = self.parse_bitwise_or()
        while self.current().type == TT.AND:
            op = self.advance().value
            right = self.parse_bitwise_or()
            left = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_bitwise_or(self) -> ASTNode:
        left = self.parse_bitwise_xor()
        while self.current().type == TT.PIPE:
            op = self.advance().value
            right = self.parse_bitwise_xor()
            left = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_bitwise_xor(self) -> ASTNode:
        left = self.parse_bitwise_and()
        while self.current().type == TT.CARET:
            op = self.advance().value
            right = self.parse_bitwise_and()
            left = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_bitwise_and(self) -> ASTNode:
        left = self.parse_shift()
        while self.current().type == TT.AMP:
            op = self.advance().value
            right = self.parse_shift()
            left = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_shift(self) -> ASTNode:
        left = self.parse_comparison()
        while self.current().type in (TT.LSHIFT, TT.RSHIFT):
            op = '<<' if self.current().type == TT.LSHIFT else '>>'
            self.advance()
            right = self.parse_comparison()
            left = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_comparison(self) -> ASTNode:
        left = self.parse_addition()
        while self.current().type in (TT.EQEQ, TT.NEQ, TT.LT, TT.GT, TT.LTE, TT.GTE):
            op  = self.advance().value
            right = self.parse_addition()
            left = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_addition(self) -> ASTNode:
        left = self.parse_multiplication()
        while self.current().type in (TT.PLUS, TT.MINUS):
            if self.current().type == TT.PLUS and self.peek().type == TT.PLUS:
                self.advance(); self.advance()  # consume both +
                right = self.parse_multiplication()
                left = BinaryOp(left, '++', right, left.line, left.col)
            else:
                op    = self.advance().value
                right = self.parse_multiplication()
                left  = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_multiplication(self) -> ASTNode:
        left = self.parse_power()
        while self.current().type in (TT.STAR, TT.SLASH, TT.PERCENT):
            op    = self.advance().value
            right = self.parse_power()
            left  = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_power(self) -> ASTNode:
        left = self.parse_unary()
        if self.current().type == TT.STARSTAR:
            op = self.advance().value
            right = self.parse_power()  # right-associative
            left = BinaryOp(left, op, right, left.line, left.col)
        return left

    def parse_unary(self) -> ASTNode:
        if self.current().type == TT.MINUS:
            tok = self.advance()
            return UnaryOp('-', self.parse_unary(), tok.line, tok.col)
        if self.current().type == TT.TILDE:
            tok = self.advance()
            return UnaryOp('~', self.parse_unary(), tok.line, tok.col)
        if self.current().type == TT.AND and self.peek().type == TT.KEYWORD and self.peek().value == 'not':
            # 'and not' — skip, handled by logical_and
            pass
        if self.current().type == TT.NOT:
            tok = self.advance()
            return UnaryOp('!', self.parse_unary(), tok.line, tok.col)
        if self.check(TT.KEYWORD, 'not'):
            tok = self.advance()
            return UnaryOp('!', self.parse_unary(), tok.line, tok.col)
        if self.check(TT.KEYWORD, 'true') or self.check(TT.KEYWORD, 'false'):
            tok = self.advance()
            return BoolLiteral(tok.value == 'true', tok.line, tok.col)
        return self.parse_postfix()

    def parse_postfix(self) -> ASTNode:
        node = self.parse_primary()
        while True:
            if self.match(TT.LBRACKET):
                # Handle slice: [start:end], [:end], [start:]
                from atclang.parser.ast_nodes import SliceExpr
                if self.check(TT.COLON) or self.check(TT.DOTDOT):
                    self.advance()
                    if self.check(TT.RBRACKET):
                        self.advance()
                        idx = SliceExpr(None, None)
                    else:
                        end = self.parse_expr()
                        self.expect(TT.RBRACKET)
                        idx = SliceExpr(None, end)
                else:
                    idx = self.parse_expr()
                    if self.check(TT.COLON) or self.check(TT.DOTDOT):
                        self.advance()
                        if self.check(TT.RBRACKET):
                            self.advance()
                            idx = SliceExpr(idx, None)
                        else:
                            end = self.parse_expr()
                            self.expect(TT.RBRACKET)
                            idx = SliceExpr(idx, end)
                    else:
                        self.expect(TT.RBRACKET)
                node = IndexAccess(node, idx, node.line, node.col)
            elif self.match(TT.DOT):
                # Accept both IDENT and KEYWORD after dot (obj.stake, obj.transfer)
                if self.current().type == TT.IDENT or self.current().type == TT.KEYWORD:
                    field_tok = self.advance()
                else:
                    field_tok = self.expect(TT.IDENT)
                node = DotAccess(node, field_tok.value, node.line, node.col)
            elif self.check(TT.LPAREN):
                self.advance()
                args = []
                while not self.check(TT.RPAREN):
                    args.append(self.parse_expr())
                    if not self.match(TT.COMMA):
                        break
                self.expect(TT.RPAREN)
                node = FunctionCall(node, args, node.line, node.col)
            else:
                break
        return node

    def parse_primary(self) -> ASTNode:
        tok = self.current()

        if self.check(TT.KEYWORD, 'match'):
            return self.parse_match()

        if tok.type in (TT.INT, TT.HEX_INT, TT.OCTAL_INT, TT.BIN_INT):
            self.advance()
            if tok.type == TT.HEX_INT:
                return IntLiteral(tok.value, tok.line, tok.col)
            return IntLiteral(tok.value, tok.line, tok.col)

        if tok.type == TT.FLOAT:
            self.advance()
            return FloatLiteral(tok.value, tok.line, tok.col)

        if tok.type == TT.STRING:
            self.advance()
            return StringLiteral(tok.value, tok.line, tok.col)

        if tok.type == TT.BOOL:
            self.advance()
            return BoolLiteral(tok.value, tok.line, tok.col)

        if self.check(TT.KEYWORD, 'null'):
            self.advance()
            return NullLiteral(tok.line, tok.col)

        # Namespace: ATC::Wallet::new
        # TYPE tokens used as identifiers (e.g., Process, FileHandle in expressions)
        if tok.type == TT.TYPE:
            self.advance()
            # Namespace: Vec::new(), Type::method()
            if self.check(TT.DCOLON):
                parts = [tok.value]
                while self.check(TT.DCOLON):
                    self.advance()
                    if self.current().type == TT.KEYWORD and self.current().value in ('new', 'delete', 'deploy', 'call'):
                        parts.append(self.advance().value)
                    elif self.current().type in (TT.IDENT, TT.TYPE):
                        parts.append(self.advance().value)
                    else:
                        parts.append(self.expect(TT.IDENT).value)
                return NamespaceAccess(parts, tok.line, tok.col)
            # Check for struct literal: TypeName { field: value }
            if self.check(TT.LBRACE):
                self.advance()
                fields = []
                while not self.check(TT.RBRACE):
                    if self.check(TT.EOF): break
                    fname = self.current()
                    if fname.type in (TT.IDENT, TT.KEYWORD, TT.TYPE):
                        self.advance()
                    else:
                        self.expect(TT.IDENT)
                    self.expect(TT.COLON)
                    fval = self.parse_expr()
                    fields.append((fname.value, fval))
                    if not self.match(TT.COMMA): break
                self.expect(TT.RBRACE)
                return StructLiteral(struct_name=tok.value, fields=fields, line=tok.line, col=tok.col)
            return Identifier(tok.value, tok.line, tok.col)

        if tok.type == TT.IDENT:
            parts = [tok.value]
            self.advance()
            while self.check(TT.DCOLON):
                self.advance()
                if self.current().type in (TT.IDENT, TT.KEYWORD, TT.TYPE):
                    parts.append(self.advance().value)
                else:
                    parts.append(self.expect(TT.IDENT).value)
            if len(parts) > 1:
                node = NamespaceAccess(parts, tok.line, tok.col)
            else:
                node = Identifier(parts[0], tok.line, tok.col)
            # Struct literal: Foo { field: value, ... } — only for PascalCase names
            if (self.check(TT.LBRACE) and len(parts) == 1 
                    and parts[0][0].isupper()
                    and not parts[0].isupper()):  # PascalCase only, not SCREAMING_SNAKE_CASE
                self.advance()
                fields = []
                while not self.check(TT.RBRACE):
                    if self.check(TT.EOF): break
                    fname = self.current()
                    if fname.type in (TT.IDENT, TT.KEYWORD):
                        self.advance()
                    else:
                        self.expect(TT.IDENT)
                    self.expect(TT.COLON)
                    fval = self.parse_expr()
                    fields.append((fname.value, fval))
                    if not self.match(TT.COMMA): break
                self.expect(TT.RBRACE)
                return StructLiteral(struct_name=parts[0], fields=fields, line=tok.line, col=tok.col)
            return node

        if tok.type == TT.LPAREN:
            self.advance()
            expr = self.parse_expr()
            # Tuple expression: (a, b, c)
            if self.check(TT.COMMA):
                values = [expr]
                while self.match(TT.COMMA):
                    values.append(self.parse_expr())
                self.expect(TT.RPAREN)
                from atclang.parser.ast_nodes import TupleExpr
                return TupleExpr(values)
            self.expect(TT.RPAREN)
            return expr

        # ATC Stdlib namespace: ATC::Crypto::sha256(...)
        if tok.type == TT.ATC_STD:
            self.advance()
            parts = tok.value.split("::")
            return NamespaceAccess(parts, tok.line, tok.col)

        # Kontextuelle Keywords als Identifier: most keywords can be variable names
        # EXCEPT control-flow keywords that start statements
        _reserved_as_stmt = {'let', 'const', 'if', 'else', 'elif', 'for', 'while',
                             'return', 'break', 'continue', 'fn', 'struct', 'enum',
                             'contract', 'trait', 'emit', 'require', 'import',
                             'use', 'match'}
        if tok.type == TT.KEYWORD and tok.value not in _reserved_as_stmt:
            self.advance()
            return Identifier(tok.value, tok.line, tok.col)

        # List literal: [1, 2, 3] or []
        if tok.type == TT.LBRACKET:
            self.advance()
            elements = []
            while not self.check(TT.RBRACKET):
                if self.check(TT.EOF): break
                elements.append(self.parse_expr())
                if not self.match(TT.COMMA): break
            self.expect(TT.RBRACKET)
            return ListLiteral(elements, tok.line, tok.col)

        # Map literal: {"key": "value"} or {}
        if tok.type == TT.LBRACE:
            self.advance()
            pairs = []
            while not self.check(TT.RBRACE):
                if self.check(TT.EOF): break
                key = self.parse_expr()
                self.expect(TT.COLON)
                val = self.parse_expr()
                pairs.append((key, val))
                if not self.match(TT.COMMA): break
            self.expect(TT.RBRACE)
            return MapLiteral(pairs, tok.line, tok.col)

        # If-expression: if cond { val } else { val }
        if tok.type == TT.KEYWORD and tok.value == 'if':
            return self.parse_if()
        
        self.error(f"Unerwartetes Token in Ausdruck: {tok.type.name}='{tok.value}'"  )


    def parse_match(self) -> 'MatchStatement':
        tok = self.advance()  # 'match'
        subject = self.parse_expr()
        self.advance()  # consume LBRACE
        arms = []
        while not self.check(TT.RBRACE) and not self.check(TT.EOF):
            # Pattern: can be INT, STRING, IDENT, TYPE, or underscore (_)
            if self.check(TT.IDENT) and self.current().value == '_':
                self.advance()
                pattern = None  # wildcard
            elif self.current().type in (TT.INT, TT.STRING, TT.IDENT, TT.TYPE, TT.KEYWORD):
                pattern = self.advance().value
            else:
                pattern = self.advance().value
            self.expect(TT.FATARROW)
            # Arm body: single expression or block
            if self.check(TT.LBRACE):
                self.advance()
                body = self.parse_block()
            else:
                body = [self.parse_statement()]
            arms.append((pattern, body))
            if self.check(TT.COMMA): self.advance()
        self.expect(TT.RBRACE)
        from atclang.parser.ast_nodes import MatchStatement
        return MatchStatement(subject, arms, tok.line, tok.col)


    def parse_match(self):
        tok = self.advance()  # 'match'
        subject = self.parse_expr()
        self.advance()  # consume LBRACE
        arms = []
        while not self.check(TT.RBRACE) and not self.check(TT.EOF):
            # Pattern: INT, STRING, IDENT, TYPE, KEYWORD, or wildcard '_'
            if self.current().type == TT.IDENT and self.current().value == '_':
                self.advance()
                pattern = None
            else:
                # Pattern can be: literal, Enum::Variant, or identifier
                pattern_parts = [self.advance().value]
                while self.check(TT.DCOLON):
                    self.advance()
                    pattern_parts.append(self.advance().value)
                pattern = '::'.join(pattern_parts)
            self.expect(TT.FAT_ARROW)
            if self.check(TT.LBRACE):
                self.advance()
                body = self.parse_block()
            else:
                body = [self.parse_statement()]
            arms.append((pattern, body))
            if self.check(TT.COMMA): self.advance()
        self.expect(TT.RBRACE)
        from atclang.parser.ast_nodes import MatchStatement
        return MatchStatement(subject, arms, tok.line, tok.col)

    # ── Statements ────────────────────────────────────────
    def parse_statement(self) -> ASTNode:
        tok = self.current()

        if self.check(TT.KEYWORD, 'let') or self.check(TT.KEYWORD, 'const'):
            return self.parse_let()
        if self.check(TT.KEYWORD, 'return'):
            return self.parse_return()
        if self.check(TT.KEYWORD, 'emit'):
            return self.parse_emit()
        if self.check(TT.KEYWORD, 'require'):
            return self.parse_require()
        if self.check(TT.KEYWORD, 'if'):
            return self.parse_if()
        if self.check(TT.KEYWORD, 'for'):
            return self.parse_for()
        if self.check(TT.KEYWORD, 'while'):
            return self.parse_while()
        if self.check(TT.KEYWORD, 'match'):
            return self.parse_match()
        if self.check(TT.KEYWORD, 'break'):
            self.advance(); return BreakStatement(tok.line, tok.col)
        if self.check(TT.KEYWORD, 'continue'):
            self.advance(); return ContinueStatement(tok.line, tok.col)

        # Compound assignment (x += y, x -= y, x *= y, x /= y)
        if self.peek().type in (TT.PLUSEQ, TT.MINUSEQ, TT.STAREQ, TT.SLASHEQ):
            target_tok = self.advance()
            op_tok = self.advance()  # +=, -=, *=, /=
            val = self.parse_expr()
            from atclang.parser.ast_nodes import Identifier
            target_id = Identifier(target_tok.value, target_tok.line, target_tok.col)
            compound = BinaryOp(target_id, op_tok.value[0], val, target_tok.line, target_tok.col)
            return Assignment(target=target_id, value=compound, line=target_tok.line, col=target_tok.col)
        # Zuweisung oder Ausdruck
        expr = self.parse_expr()
        if self.match(TT.EQ):
            value = self.parse_expr()
            self.match(TT.SEMICOLON)
            return Assignment(expr, value, expr.line, expr.col)
        # Compound assignment: x += y, x -= y, etc.
        if self.current().type in (TT.PLUSEQ, TT.MINUSEQ, TT.STAREQ, TT.SLASHEQ):
            op_tok = self.advance()
            val = self.parse_expr()
            compound = BinaryOp(expr, op_tok.value[0], val, expr.line, expr.col)
            return Assignment(expr, compound, expr.line, expr.col)
        self.match(TT.SEMICOLON)
        return ExprStatement(expr, expr.line, expr.col)

    def parse_let(self) -> LetStatement:
        tok      = self.advance()
        is_const = tok.value == 'const'
        # Handle tuple destructuring: let (a, b) = expr
        if self.check(TT.LPAREN):
            self.advance()
            names = []
            while not self.check(TT.RPAREN):
                if self.current().type in (TT.IDENT, TT.KEYWORD):
                    names.append(self.advance().value)
                else:
                    names.append(self.expect(TT.IDENT).value)
                if not self.match(TT.COMMA): break
            self.expect(TT.RPAREN)
            # No type hint for tuple destructuring
            value = None
            if self.match(TT.EQ):
                value = self.parse_expr()
            self.match(TT.SEMICOLON)
            # Return with tuple names joined
            from atclang.parser.ast_nodes import TupleExpr
            name = "(" + ", ".join(names) + ")"
            return LetStatement(name, None, value, is_const, tok.line, tok.col)
        if self.current().type in (TT.IDENT, TT.KEYWORD):
            name = self.advance().value
        else:
            name = self.expect(TT.IDENT).value
        type_hint = None
        if self.match(TT.COLON):
            type_hint = self.parse_type()
        value = None
        if self.match(TT.EQ):
            value = self.parse_expr()
        self.match(TT.SEMICOLON)
        return LetStatement(name, type_hint, value, is_const, tok.line, tok.col)

    def parse_return(self) -> ReturnStatement:
        tok = self.advance()
        if self.check(TT.RBRACE) or self.check(TT.EOF):
            return ReturnStatement(None, tok.line, tok.col)
        # parse_expr handles both tuple (a, b) and grouped (a) / b naturally
        value = self.parse_expr()
        self.match(TT.SEMICOLON)
        return ReturnStatement(value, tok.line, tok.col)

    def parse_emit(self) -> EmitStatement:
        tok   = self.advance()
        event = self.expect(TT.IDENT).value
        args  = []
        if self.match(TT.LPAREN):
            while not self.check(TT.RPAREN):
                args.append(self.parse_expr())
                if not self.match(TT.COMMA): break
            self.expect(TT.RPAREN)
        return EmitStatement(event, args, tok.line, tok.col)

    def parse_require(self) -> RequireStatement:
        tok  = self.advance()
        if self.match(TT.LPAREN):
            cond = self.parse_expr()
            msg  = None
            if self.match(TT.COMMA):
                msg = self.parse_expr()
            self.expect(TT.RPAREN)
        else:
            cond = self.parse_expr()
            msg  = None
        return RequireStatement(cond, msg, tok.line, tok.col)


    def parse_if(self) -> IfStatement:
        tok  = self.advance()
        cond = self.parse_expr()
        self.advance()  # consume LBRACE
        then = self.parse_block()
        elif_blocks = []
        else_block  = None
        while self.check(TT.KEYWORD, 'elif'):
            self.advance()
            ec = self.parse_expr()
            self.advance()  # consume LBRACE
            eb = self.parse_block()
            elif_blocks.append((ec, eb))
        if self.check(TT.KEYWORD, 'else'):
            self.advance()
            if self.check(TT.KEYWORD, 'if'):
                # "else if" → treat as nested elif
                inner = self.parse_if()  # returns IfStatement
                elif_blocks.append((inner.condition, inner.then_block))
                for ec, eb in inner.elif_blocks:
                    elif_blocks.append((ec, eb))
                else_block = inner.else_block
            else:
                self.advance()  # consume LBRACE
                else_block = self.parse_block()
        return IfStatement(condition=cond, then_block=then, elif_blocks=elif_blocks, else_block=else_block, line=tok.line, col=tok.col)

    def parse_for(self) -> ForStatement:
        tok = self.advance()
        if self.current().type in (TT.IDENT, TT.KEYWORD):
            var = self.advance().value
        else:
            var = self.expect(TT.IDENT).value
        if self.check(TT.KEYWORD, 'in'):
            self.advance()
        elif self.check(TT.IDENT) and self.current().value == 'in':
            self.advance()
        else:
            self.expect(TT.KEYWORD, 'in')
        iterable = self.parse_expr()
        # Range: 1..N
        if self.current().type == TT.DOTDOT:
            self.advance()
            end = self.parse_expr()
            from atclang.parser.ast_nodes import RangeExpr
            iterable = RangeExpr(iterable, end)
        self.advance()  # consume LBRACE
        body = self.parse_block()
        return ForStatement(var, iterable, body, tok.line, tok.col)

    def parse_while(self) -> WhileStatement:
        tok  = self.advance()
        cond = self.parse_expr()
        self.advance()  # consume LBRACE
        body = self.parse_block()
        return WhileStatement(cond, body, tok.line, tok.col)

    def parse_block(self) -> List[ASTNode]:
        stmts = []
        while not self.check(TT.RBRACE) and not self.check(TT.EOF):
            stmts.append(self.parse_statement())
        self.expect(TT.RBRACE)
        return stmts

    def parse_param(self) -> Parameter:
        if self.current().type in (TT.IDENT, TT.KEYWORD, TT.TYPE):
            name = self.advance().value
        else:
            self.error(f"Erwartet IDENT für Parametername")
        self.expect(TT.COLON)
        typ  = self.parse_type()
        return Parameter(name, typ)

    def parse_function(self) -> FunctionDef:
        tok    = self.advance()  # 'fn'
        # Function name can be IDENT or KEYWORD (transfer, mint, burn, stake, etc.)
        if self.current().type in (TT.IDENT, TT.KEYWORD):
            name = self.advance().value
        else:
            self.error(f"Erwartet IDENT für Funktionsname")
        self.expect(TT.LPAREN)
        params = []
        while not self.check(TT.RPAREN):
            params.append(self.parse_param())
            if not self.match(TT.COMMA): break
        self.expect(TT.RPAREN)
        ret_type = None
        if self.match(TT.ARROW):
            # Handle tuple return type: (A, B)
            if self.check(TT.LPAREN):
                self.advance()
                types = [self.parse_type()]
                while self.match(TT.COMMA):
                    types.append(self.parse_type())
                self.expect(TT.RPAREN)
                ret_type = TypeAnnotation('Tuple', types)
            else:
                ret_type = self.parse_type()
        if self.check(TT.LBRACE):
            self.advance()  # consume LBRACE
            body = self.parse_block()
        else:
            body = []  # Abstract/trait method — no body
        return FunctionDef(name, params, ret_type, body, False, [], tok.line, tok.col)

    def parse_contract(self, keyword_type: str = 'contract') -> ContractDef:
        tok  = self.advance()  # 'contract' or 'trait'
        # Accept both IDENT and TYPE as contract names
        if self.current().type in (TT.IDENT, TT.TYPE):
            name = self.advance().value
        else:
            name = self.expect(TT.IDENT).value
        standards = []
        if self.match(TT.COLON):
            std_parts = []
            while not self.check(TT.LBRACE) and not self.check(TT.EOF):
                std_parts.append(str(self.advance().value))
            if std_parts:
                standards.append("".join(std_parts))
        self.expect(TT.LBRACE)
        states, events, errors, functions = [], [], [], []
        while not self.check(TT.RBRACE) and not self.check(TT.EOF):
            if self.check(TT.KEYWORD, 'state'):
                self.advance()
                if self.current().type in (TT.IDENT, TT.KEYWORD):
                    fname = self.advance().value
                else:
                    fname = self.expect(TT.IDENT).value
                self.expect(TT.COLON)
                ftype = self.parse_type()
                val = None
                if self.match(TT.EQ):
                    val = self.parse_expr()
                states.append(StateField(fname, ftype, val))
            elif self.check(TT.KEYWORD, 'event'):
                self.advance()
                ename = self.expect(TT.IDENT).value
                params = []
                if self.match(TT.LPAREN):
                    while not self.check(TT.RPAREN):
                        # Event params can be type-only: event Transfer(String, String, Int)
                        # or named: event Transfer(from: String, to: String, amount: Int)
                        if self.current().type in (TT.TYPE, TT.IDENT, TT.KEYWORD) and self.peek() and self.peek().type == TT.COLON:
                            # Named param: name: Type
                            params.append(self.parse_param())
                        else:
                            # Type-only param
                            typ = self.parse_type()
                            params.append(Parameter("", typ))
                        if not self.match(TT.COMMA): break
                    self.expect(TT.RPAREN)
                events.append(EventDef(ename, params))
            elif self.check(TT.KEYWORD, 'error'):
                self.advance()
                ename = self.expect(TT.IDENT).value
                errors.append(ErrorDef(ename))
            elif self.check(TT.KEYWORD, 'fn'):
                functions.append(self.parse_function())
            else:
                self.advance()  # skip unknown
        self.expect(TT.RBRACE)
        return ContractDef(name, standards, states, events, errors, functions, tok.line, tok.col)

    def parse_struct(self) -> StructDef:
        tok = self.advance()  # 'struct'
        # Accept both IDENT and TYPE as struct names (PascalCase types)
        if self.current().type in (TT.IDENT, TT.TYPE):
            name = self.advance().value
        else:
            name = self.expect(TT.IDENT).value
        self.expect(TT.LBRACE)
        fields = []
        while not self.check(TT.RBRACE):
            if self.check(TT.EOF): break
            # Accept both IDENT and KEYWORD as field names (stake, transfer, etc.)
            if self.current().type == TT.IDENT or self.current().type == TT.KEYWORD:
                field_name = self.advance().value
            else:
                field_name = self.expect(TT.IDENT).value
            self.expect(TT.COLON)
            field_type = self.parse_type()
            fields.append((field_name, field_type))
            if self.check(TT.COMMA): self.advance()
        self.expect(TT.RBRACE)
        return StructDef(name, fields, tok.line, tok.col)

    def parse_enum(self) -> EnumDef:
        tok = self.advance()  # 'enum'
        # Accept both IDENT and TYPE as enum names
        if self.current().type in (TT.IDENT, TT.TYPE):
            name = self.advance().value
        else:
            name = self.expect(TT.IDENT).value
        self.expect(TT.LBRACE)
        variants = []
        while not self.check(TT.RBRACE):
            if self.check(TT.EOF): break
            variants.append(self.expect(TT.IDENT).value)
            if self.check(TT.COMMA): self.advance()
        self.expect(TT.RBRACE)
        return EnumDef(name, variants, tok.line, tok.col)

    # ── Top-Level ─────────────────────────────────────────
    def parse_program(self) -> Program:
        prog = Program([], 1, 1)
        while not self.check(TT.EOF):
            if self.check(TT.KEYWORD, 'contract') or self.check(TT.KEYWORD, 'trait'):
                prog.statements.append(self.parse_contract())
            elif self.check(TT.KEYWORD, 'wallet'):
                tok  = self.advance()
                name = self.expect(TT.IDENT).value
                self.expect(TT.EQ)
                val  = self.parse_expr()
                prog.statements.append(WalletDef(name, val, tok.line, tok.col))
            elif self.check(TT.KEYWORD, 'fn'):
                prog.statements.append(self.parse_function())
            elif self.check(TT.KEYWORD, 'struct'):
                prog.statements.append(self.parse_struct())
            elif self.check(TT.KEYWORD, 'event'):
                self.advance()
                ename = self.expect(TT.IDENT).value
                params = []
                if self.match(TT.LPAREN):
                    while not self.check(TT.RPAREN):
                        if self.check(TT.EOF): break
                        pname = self.current()
                        if pname.type in (TT.IDENT, TT.KEYWORD, TT.TYPE):
                            self.advance()
                        else:
                            self.expect(TT.IDENT)
                        self.expect(TT.COLON)
                        typ = self.parse_type()
                        params.append(Parameter(pname.value, typ))
                        if not self.match(TT.COMMA): break
                    self.expect(TT.RPAREN)
                elif self.match(TT.LBRACE):
                    while not self.check(TT.RBRACE):
                        if self.check(TT.EOF): break
                        pname = self.current()
                        if pname.type in (TT.IDENT, TT.KEYWORD, TT.TYPE):
                            self.advance()
                        else:
                            self.expect(TT.IDENT)
                        self.expect(TT.COLON)
                        typ = self.parse_type()
                        params.append(Parameter(pname.value, typ))
                        if not self.match(TT.COMMA): break
                    self.expect(TT.RBRACE)
                prog.statements.append(EventDef(ename, params))
            elif self.check(TT.KEYWORD, 'enum'):
                prog.statements.append(self.parse_enum())
            elif self.check(TT.KEYWORD, 'let') or self.check(TT.KEYWORD, 'const'):
                prog.statements.append(self.parse_let())
            elif self.check(TT.KEYWORD, 'import') or self.check(TT.KEYWORD, 'use'):
                tok  = self.advance()
                # Accept both IDENT and ATC_STD (e.g., "import ATC::Crypto")
                if self.current().type == TT.ATC_STD:
                    parts = [self.advance().value]
                else:
                    parts = [self.expect(TT.IDENT).value]
                while self.match(TT.DCOLON):
                    if self.current().type == TT.KEYWORD and self.current().value in ('new', 'delete', 'deploy', 'call'):
                        parts.append(self.advance().value)
                    elif self.current().type == TT.ATC_STD:
                        parts.append(self.advance().value)
                    else:
                        parts.append(self.expect(TT.IDENT).value)
                # Selective import: use ATC::Crypto::{ sha256, hmac }
                imported_names = []
                if self.check(TT.LBRACE):
                    self.advance()
                    while not self.check(TT.RBRACE):
                        if self.check(TT.EOF): break
                        if self.current().type in (TT.IDENT, TT.TYPE, TT.KEYWORD):
                            imported_names.append(self.advance().value)
                        else:
                            imported_names.append(self.expect(TT.IDENT).value)
                        if not self.match(TT.COMMA): break
                    self.expect(TT.RBRACE)
                alias = None
                if self.check(TT.KEYWORD, 'as'):
                    self.advance()
                    alias = self.expect(TT.IDENT).value
                prog.statements.append(ImportStatement(parts, alias, tok.line, tok.col))
            else:
                prog.statements.append(self.parse_statement())
        return prog


    # Alias für Kompatibilität
    parse = parse_program


def parse(source: str) -> Program:
    """Hilfsfunktion: Quellcode → AST"""
    lexer  = ATCLexer(source)
    tokens = lexer.tokenize()
    parser = ATCParser(tokens)
    return parser.parse_program()
