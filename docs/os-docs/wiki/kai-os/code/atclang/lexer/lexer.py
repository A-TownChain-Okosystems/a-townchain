# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCLang Lexer — Tokenizer
Eigene Programmiersprache für das A-TownChain Ökosystem
Version: 0.1.0-alpha | Komplett selbst geschrieben
"""

from enum import Enum, auto
from dataclasses import dataclass
from typing import List, Optional


# ── Token-Typen ──────────────────────────────────────────
class TT(Enum):
    # Literale
    INT        = auto()
    FLOAT      = auto()
    STRING     = auto()
    BOOL       = auto()

    # Bezeichner & Keywords
    IDENT      = auto()
    KEYWORD    = auto()

    # Typen
    TYPE       = auto()

    # Operatoren
    PLUS       = auto()
    MINUS      = auto()
    STAR       = auto()
    SLASH      = auto()
    EQ         = auto()   # =
    EQEQ       = auto()   # ==
    NEQ        = auto()   # !=
    LT         = auto()   # <
    GT         = auto()   # >
    LTE        = auto()   # <=
    GTE        = auto()   # >=
    ARROW      = auto()   # ->
    DCOLON     = auto()   # ::
    ASSIGN     = auto()   # :=

    # Delimiters
    LPAREN     = auto()   # (
    RPAREN     = auto()   # )
    LBRACE     = auto()   # {
    RBRACE     = auto()   # }
    LBRACKET   = auto()   # [
    RBRACKET   = auto()   # ]
    COMMA      = auto()   # ,
    COLON      = auto()   # :
    SEMICOLON  = auto()   # ;
    DOT        = auto()   # .
    AT         = auto()   # @

    # Sonstiges
    NEWLINE    = auto()
    EOF        = auto()
    COMMENT    = auto()


# ── Keywords der Sprache ─────────────────────────────────
KEYWORDS = {
    "wallet", "contract", "fn", "state", "emit", "require",
    "return", "if", "else", "elif", "for", "while", "in",
    "import", "from", "as", "let", "const", "pub", "priv",
    "self", "caller", "block", "tx", "deploy", "call",
    "event", "error", "true", "false", "null", "break",
    "continue", "struct", "enum", "impl", "trait", "new",
    "delete", "async", "await", "node", "consensus",
    "genesis", "mint", "burn", "stake", "unstake", "vote",
}

# ── Typen ────────────────────────────────────────────────
TYPES = {
    "UInt8", "UInt16", "UInt32", "UInt64", "UInt256",
    "Int8",  "Int16",  "Int32",  "Int64",
    "Bool", "String", "Address", "Hash256", "Bytes",
    "Map", "List", "Set", "Option", "Result",
    "ATC8300", "ATC9000", "ATCContract", "ATCWallet",
}


@dataclass
class Token:
    type: TT
    value: object
    line: int
    col: int

    def __repr__(self):
        return f"Token({self.type.name}, {self.value!r}, {self.line}:{self.col})"


# ── Lexer ────────────────────────────────────────────────
class ATCLexer:
    """
    Tokenizer für ATCLang.
    Liest Quellcode und gibt eine Liste von Tokens zurück.
    """

    def __init__(self, source: str):
        self.src   = source
        self.pos   = 0
        self.line  = 1
        self.col   = 1
        self.tokens: List[Token] = []

    def error(self, msg: str):
        raise SyntaxError(f"[ATCLang Lexer] {msg} @ Zeile {self.line}:{self.col}")

    def peek(self, offset=0) -> str:
        idx = self.pos + offset
        return self.src[idx] if idx < len(self.src) else '\0'

    def advance(self) -> str:
        ch = self.src[self.pos]
        self.pos += 1
        if ch == '\n':
            self.line += 1
            self.col = 1
        else:
            self.col += 1
        return ch

    def match(self, expected: str) -> bool:
        if self.pos < len(self.src) and self.src[self.pos] == expected:
            self.advance()
            return True
        return False

    def skip_whitespace(self):
        while self.pos < len(self.src) and self.peek() in ' \t\r':
            self.advance()

    def read_string(self) -> Token:
        line, col = self.line, self.col
        self.advance()  # öffnendes "
        buf = []
        while self.pos < len(self.src) and self.peek() != '"':
            ch = self.advance()
            if ch == '\\':
                esc = self.advance()
                buf.append({'n': '\n', 't': '\t', '\\': '\\', '"': '"'}.get(esc, esc))
            else:
                buf.append(ch)
        if self.pos >= len(self.src):
            self.error("Nicht geschlossener String")
        self.advance()  # schließendes "
        return Token(TT.STRING, ''.join(buf), line, col)

    def read_number(self) -> Token:
        line, col = self.line, self.col
        buf = []
        is_float = False
        while self.pos < len(self.src) and (self.peek().isdigit() or self.peek() == '_'):
            ch = self.advance()
            if ch != '_':
                buf.append(ch)
        if self.peek() == '.' and self.peek(1).isdigit():
            is_float = True
            buf.append(self.advance())
            while self.pos < len(self.src) and self.peek().isdigit():
                buf.append(self.advance())
        val = float(''.join(buf)) if is_float else int(''.join(buf))
        return Token(TT.FLOAT if is_float else TT.INT, val, line, col)

    def read_ident(self) -> Token:
        line, col = self.line, self.col
        buf = []
        while self.pos < len(self.src) and (self.peek().isalnum() or self.peek() == '_'):
            buf.append(self.advance())
        word = ''.join(buf)
        if word in KEYWORDS:
            if word in ('true', 'false'):
                return Token(TT.BOOL, word == 'true', line, col)
            return Token(TT.KEYWORD, word, line, col)
        if word in TYPES:
            return Token(TT.TYPE, word, line, col)
        return Token(TT.IDENT, word, line, col)

    def read_comment(self) -> Optional[Token]:
        line, col = self.line, self.col
        self.advance(); self.advance()  # //
        buf = []
        while self.pos < len(self.src) and self.peek() != '\n':
            buf.append(self.advance())
        return None  # Kommentare ignorieren

    def tokenize(self) -> List[Token]:
        while self.pos < len(self.src):
            self.skip_whitespace()
            if self.pos >= len(self.src):
                break

            line, col = self.line, self.col
            ch = self.peek()

            # Newline
            if ch == '\n':
                self.advance()
                self.tokens.append(Token(TT.NEWLINE, '\n', line, col))
                continue

            # Kommentar
            if ch == '/' and self.peek(1) == '/':
                self.read_comment()
                continue

            # Block-Kommentar
            if ch == '/' and self.peek(1) == '*':
                self.advance(); self.advance()
                while self.pos < len(self.src):
                    if self.peek() == '*' and self.peek(1) == '/':
                        self.advance(); self.advance()
                        break
                    self.advance()
                continue

            # String
            if ch == '"':
                self.tokens.append(self.read_string())
                continue

            # Zahl
            if ch.isdigit():
                self.tokens.append(self.read_number())
                continue

            # Bezeichner / Keyword
            if ch.isalpha() or ch == '_':
                self.tokens.append(self.read_ident())
                continue

            # Operatoren & Delimiters
            self.advance()
            if   ch == '+': self.tokens.append(Token(TT.PLUS,      ch,  line, col))
            elif ch == '*': self.tokens.append(Token(TT.STAR,      ch,  line, col))
            elif ch == '(': self.tokens.append(Token(TT.LPAREN,    ch,  line, col))
            elif ch == ')': self.tokens.append(Token(TT.RPAREN,    ch,  line, col))
            elif ch == '{': self.tokens.append(Token(TT.LBRACE,    ch,  line, col))
            elif ch == '}': self.tokens.append(Token(TT.RBRACE,    ch,  line, col))
            elif ch == '[': self.tokens.append(Token(TT.LBRACKET,  ch,  line, col))
            elif ch == ']': self.tokens.append(Token(TT.RBRACKET,  ch,  line, col))
            elif ch == ',': self.tokens.append(Token(TT.COMMA,     ch,  line, col))
            elif ch == ';': self.tokens.append(Token(TT.SEMICOLON, ch,  line, col))
            elif ch == '.': self.tokens.append(Token(TT.DOT,       ch,  line, col))
            elif ch == '@': self.tokens.append(Token(TT.AT,        ch,  line, col))
            elif ch == '-':
                if self.match('>'):
                    self.tokens.append(Token(TT.ARROW, '->', line, col))
                else:
                    self.tokens.append(Token(TT.MINUS, '-', line, col))
            elif ch == ':':
                if self.match(':'):
                    self.tokens.append(Token(TT.DCOLON, '::', line, col))
                elif self.match('='):
                    self.tokens.append(Token(TT.ASSIGN, ':=', line, col))
                else:
                    self.tokens.append(Token(TT.COLON, ':', line, col))
            elif ch == '=':
                if self.match('='):
                    self.tokens.append(Token(TT.EQEQ, '==', line, col))
                else:
                    self.tokens.append(Token(TT.EQ, '=', line, col))
            elif ch == '!':
                if self.match('='):
                    self.tokens.append(Token(TT.NEQ, '!=', line, col))
                else:
                    self.error(f"Unbekanntes Zeichen: '!'")
            elif ch == '<':
                if self.match('='):
                    self.tokens.append(Token(TT.LTE, '<=', line, col))
                else:
                    self.tokens.append(Token(TT.LT, '<', line, col))
            elif ch == '>':
                if self.match('='):
                    self.tokens.append(Token(TT.GTE, '>=', line, col))
                else:
                    self.tokens.append(Token(TT.GT, '>', line, col))
            elif ch == '/':
                self.tokens.append(Token(TT.SLASH, ch, line, col))
            else:
                self.error(f"Unbekanntes Zeichen: '{ch}'")

        self.tokens.append(Token(TT.EOF, None, self.line, self.col))
        return self.tokens


# ── CLI Test ─────────────────────────────────────────────
if __name__ == "__main__":
    sample = """
// ATCLang Test-Programm
contract ShivaToken : ATC8300 {
    state balance: Map<Address, UInt256>
    
    fn transfer(to: Address, amount: UInt256) -> Bool {
        require(balance[caller] >= amount)
        balance[caller] -= amount
        balance[to] += amount
        emit Transfer(caller, to, amount)
        return true
    }
}

wallet myWallet = ATC::Wallet::new("ShivaCore")
let total: UInt256 = 1_000_000
"""
    lexer = ATCLexer(sample)
    tokens = lexer.tokenize()
    print(f"✅ ATCLang Lexer — {len(tokens)} Tokens erzeugt\n")
    for tok in tokens:
        if tok.type not in (TT.NEWLINE, TT.EOF):
            print(f"  {tok}")
