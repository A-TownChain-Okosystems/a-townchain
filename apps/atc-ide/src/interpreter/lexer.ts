import { Token, TokenType } from './types';

const Keywords: Record<string, TokenType> = {
  let: TokenType.LET,
  print: TokenType.PRINT,
  if: TokenType.IF,
  else: TokenType.ELSE,
  while: TokenType.WHILE,
  fn: TokenType.FN,
  return: TokenType.RETURN,
  true: TokenType.TRUE,
  false: TokenType.FALSE,
  and: TokenType.AND,
  or: TokenType.OR,
  not: TokenType.BANG,
};

export class Lexer {
  private input: string;
  private position: number = 0;
  private readPosition: number = 0;
  private ch: string | null = null;
  private line: number = 1;

  constructor(input: string) {
    this.input = input;
    this.readChar();
  }

  private readChar() {
    if (this.readPosition >= this.input.length) {
      this.ch = null;
    } else {
      this.ch = this.input[this.readPosition];
    }
    this.position = this.readPosition;
    this.readPosition++;
  }

  private peekChar(): string | null {
    if (this.readPosition >= this.input.length) {
      return null;
    } else {
      return this.input[this.readPosition];
    }
  }

  public nextToken(): Token {
    this.skipWhitespace();

    let tok: Token;

    switch (this.ch) {
      case '=':
        if (this.peekChar() === '=') {
          this.readChar();
          tok = { type: TokenType.EQ, literal: '==', line: this.line };
        } else {
          tok = { type: TokenType.ASSIGN, literal: '=', line: this.line };
        }
        break;
      case '!':
        if (this.peekChar() === '=') {
          this.readChar();
          tok = { type: TokenType.NEQ, literal: '!=', line: this.line };
        } else {
          tok = { type: TokenType.BANG, literal: this.ch, line: this.line };
        }
        break;
      case '&':
        if (this.peekChar() === '&') {
          this.readChar();
          tok = { type: TokenType.AND, literal: '&&', line: this.line };
        } else {
          tok = { type: TokenType.ILLEGAL, literal: this.ch, line: this.line };
        }
        break;
      case '|':
        if (this.peekChar() === '|') {
          this.readChar();
          tok = { type: TokenType.OR, literal: '||', line: this.line };
        } else {
          tok = { type: TokenType.ILLEGAL, literal: this.ch, line: this.line };
        }
        break;
      case '<':
        if (this.peekChar() === '=') {
          this.readChar();
          tok = { type: TokenType.LTE, literal: '<=', line: this.line };
        } else {
          tok = { type: TokenType.LT, literal: '<', line: this.line };
        }
        break;
      case '>':
        if (this.peekChar() === '=') {
          this.readChar();
          tok = { type: TokenType.GTE, literal: '>=', line: this.line };
        } else {
          tok = { type: TokenType.GT, literal: '>', line: this.line };
        }
        break;
      case '+':
        tok = { type: TokenType.PLUS, literal: this.ch, line: this.line };
        break;
      case '-':
        tok = { type: TokenType.MINUS, literal: this.ch, line: this.line };
        break;
      case '*':
        tok = { type: TokenType.MULTIPLY, literal: this.ch, line: this.line };
        break;
      case '/':
        tok = { type: TokenType.DIVIDE, literal: this.ch, line: this.line };
        break;
      case '(':
        tok = { type: TokenType.LPAREN, literal: this.ch, line: this.line };
        break;
      case ')':
        tok = { type: TokenType.RPAREN, literal: this.ch, line: this.line };
        break;
      case '{':
        tok = { type: TokenType.LBRACE, literal: this.ch, line: this.line };
        break;
      case '}':
        tok = { type: TokenType.RBRACE, literal: this.ch, line: this.line };
        break;
      case '[':
        tok = { type: TokenType.LBRACKET, literal: this.ch, line: this.line };
        break;
      case ']':
        tok = { type: TokenType.RBRACKET, literal: this.ch, line: this.line };
        break;
      case ':':
        tok = { type: TokenType.COLON, literal: this.ch, line: this.line };
        break;
      case ',':
        tok = { type: TokenType.COMMA, literal: this.ch, line: this.line };
        break;
      case '"':
        tok = { type: TokenType.STRING, literal: this.readString(), line: this.line };
        break;
      case null:
        tok = { type: TokenType.EOF, literal: '', line: this.line };
        break;
      default:
        if (this.isLetter(this.ch)) {
          const literal = this.readIdentifier();
          const type = Keywords[literal] || TokenType.IDENTIFIER;
          return { type, literal, line: this.line };
        } else if (this.isDigit(this.ch)) {
          return { type: TokenType.NUMBER, literal: this.readNumber(), line: this.line };
        } else {
          tok = { type: TokenType.ILLEGAL, literal: this.ch, line: this.line };
        }
    }

    this.readChar();
    return tok;
  }

  private skipWhitespace() {
    while (this.ch === ' ' || this.ch === '\t' || this.ch === '\n' || this.ch === '\r') {
      if (this.ch === '\n') {
        this.line++;
      }
      this.readChar();
    }
  }

  private isLetter(ch: string): boolean {
    return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch === '_';
  }

  private isDigit(ch: string): boolean {
    return ch >= '0' && ch <= '9';
  }

  private readIdentifier(): string {
    const position = this.position;
    while (this.ch !== null && (this.isLetter(this.ch) || this.isDigit(this.ch))) {
      this.readChar();
    }
    return this.input.slice(position, this.position);
  }

  private readNumber(): string {
    const position = this.position;
    while (this.ch !== null && (this.isDigit(this.ch) || this.ch === '.')) {
      this.readChar();
    }
    return this.input.slice(position, this.position);
  }

  private readString(): string {
    const position = this.position + 1;
    this.readChar();
    while (this.ch !== '"' && this.ch !== null) {
      if (this.ch === '\\' && this.peekChar() === '"') {
        this.readChar(); // Consume the backslash
      }
      this.readChar();
    }
    const str = this.input.slice(position, this.position).replace(/\\"/g, '"');
    return str;
  }
}
