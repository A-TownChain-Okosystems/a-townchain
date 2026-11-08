import { Lexer } from './lexer';
import {
  Token, TokenType, Program, Statement, LetStatement, PrintStatement,
  ExpressionStatement, BlockStatement, IfStatement, WhileStatement, ReturnStatement,
  Expression, Identifier, NumberLiteral, StringLiteral, BooleanLiteral,
  BinaryExpression, AssignmentExpression, FunctionLiteral, CallExpression
} from './types';

enum Precedence {
  LOWEST = 1,
  OR,          // || or 'or'
  AND,         // && or 'and'
  EQUALS,      // ==
  LESSGREATER, // > or <
  SUM,         // +
  PRODUCT,     // *
  PREFIX,      // -X or !X
  CALL,        // myFunction(X)
  INDEX        // array[index]
}

const precedences: Record<string, Precedence> = {
  [TokenType.OR]: Precedence.OR,
  [TokenType.AND]: Precedence.AND,
  [TokenType.EQ]: Precedence.EQUALS,
  [TokenType.NEQ]: Precedence.EQUALS,
  [TokenType.LT]: Precedence.LESSGREATER,
  [TokenType.GT]: Precedence.LESSGREATER,
  [TokenType.LTE]: Precedence.LESSGREATER,
  [TokenType.GTE]: Precedence.LESSGREATER,
  [TokenType.PLUS]: Precedence.SUM,
  [TokenType.MINUS]: Precedence.SUM,
  [TokenType.MULTIPLY]: Precedence.PRODUCT,
  [TokenType.DIVIDE]: Precedence.PRODUCT,
  [TokenType.LPAREN]: Precedence.CALL,
  [TokenType.LBRACKET]: Precedence.INDEX,
};

export class Parser {
  private lexer: Lexer;
  private currentToken!: Token;
  private peekToken!: Token;
  public errors: string[] = [];

  constructor(lexer: Lexer) {
    this.lexer = lexer;
    this.nextToken();
    this.nextToken();
  }

  private nextToken() {
    this.currentToken = this.peekToken;
    this.peekToken = this.lexer.nextToken();
  }

  public parseProgram(): Program {
    const program: Program = { statements: [] };

    while (this.currentToken.type !== TokenType.EOF) {
      const statement = this.parseStatement();
      if (statement) {
        program.statements.push(statement);
      }
      this.nextToken();
    }

    return program;
  }

  private parseStatement(): Statement | null {
    switch (this.currentToken.type) {
      case TokenType.LET: return this.parseLetStatement();
      case TokenType.PRINT: return this.parsePrintStatement();
      case TokenType.IF: return this.parseIfStatement();
      case TokenType.WHILE: return this.parseWhileStatement();
      case TokenType.RETURN: return this.parseReturnStatement();
      default: return this.parseExpressionStatement();
    }
  }

  private parseLetStatement(): LetStatement | null {
    const line = this.currentToken.line;
    if (!this.expectPeek(TokenType.IDENTIFIER)) return null;

    const name: Identifier = { type: 'Identifier', value: this.currentToken.literal };

    if (!this.expectPeek(TokenType.ASSIGN)) return null;

    this.nextToken();
    const value = this.parseExpression(Precedence.LOWEST);

    return { type: 'LetStatement', name, value: value!, line };
  }

  private parsePrintStatement(): PrintStatement | null {
    const line = this.currentToken.line;
    this.nextToken();
    const value = this.parseExpression(Precedence.LOWEST);
    return { type: 'PrintStatement', value: value!, line };
  }

  private parseReturnStatement(): ReturnStatement | null {
    const line = this.currentToken.line;
    this.nextToken();
    const returnValue = this.parseExpression(Precedence.LOWEST);
    return { type: 'ReturnStatement', returnValue: returnValue!, line };
  }

  private parseIfStatement(): IfStatement | null {
    const line = this.currentToken.line;
    this.nextToken(); // consume 'if'
    const condition = this.parseExpression(Precedence.LOWEST);
    
    if (!this.expectPeek(TokenType.LBRACE)) return null;
    
    const consequence = this.parseBlockStatement();
    let alternative: BlockStatement | undefined;

    if (this.peekToken.type === TokenType.ELSE) {
      this.nextToken();
      if (!this.expectPeek(TokenType.LBRACE)) return null;
      alternative = this.parseBlockStatement();
    }

    return { type: 'IfStatement', condition: condition!, consequence, alternative, line };
  }

  private parseWhileStatement(): WhileStatement | null {
    const line = this.currentToken.line;
    this.nextToken(); // consume 'while'
    const condition = this.parseExpression(Precedence.LOWEST);

    if (!this.expectPeek(TokenType.LBRACE)) return null;

    const body = this.parseBlockStatement();

    return { type: 'WhileStatement', condition: condition!, body, line };
  }

  private parseBlockStatement(): BlockStatement {
    const block: BlockStatement = { type: 'BlockStatement', statements: [], line: this.currentToken.line };
    this.nextToken();

    while (this.currentToken.type !== TokenType.RBRACE && this.currentToken.type !== TokenType.EOF) {
      const stmt = this.parseStatement();
      if (stmt) block.statements.push(stmt);
      this.nextToken();
    }

    return block;
  }

  private parseExpressionStatement(): ExpressionStatement | null {
    const line = this.currentToken.line;
    const expression = this.parseExpression(Precedence.LOWEST);
    if (!expression) return null;
    
    // Support assignment: ident = expr
    if (expression.type === 'Identifier' && this.peekToken.type === TokenType.ASSIGN) {
      this.nextToken(); // consume ident
      this.nextToken(); // consume =
      const value = this.parseExpression(Precedence.LOWEST);
      return { type: 'ExpressionStatement', expression: { type: 'AssignmentExpression', left: expression, value: value! }, line };
    }

    return { type: 'ExpressionStatement', expression, line };
  }

  private parseExpression(precedence: Precedence): Expression | null {
    let leftExp = this.parsePrefix();

    if (!leftExp) {
      this.errors.push(`No prefix parse function for ${this.currentToken.type} (${this.currentToken.literal}) found at line ${this.currentToken.line}`);
      return null;
    }

    while (this.peekToken.type !== TokenType.EOF && precedence < this.peekPrecedence()) {
      this.nextToken();
      leftExp = this.parseInfix(leftExp);
    }

    return leftExp;
  }

  private parsePrefix(): Expression | null {
    switch (this.currentToken.type) {
      case TokenType.IDENTIFIER: return { type: 'Identifier', value: this.currentToken.literal };
      case TokenType.NUMBER: return { type: 'NumberLiteral', value: parseFloat(this.currentToken.literal) };
      case TokenType.STRING: return { type: 'StringLiteral', value: this.currentToken.literal };
      case TokenType.TRUE: return { type: 'BooleanLiteral', value: true };
      case TokenType.FALSE: return { type: 'BooleanLiteral', value: false };
      case TokenType.BANG:
      case TokenType.MINUS: {
        const operator = this.currentToken.literal;
        this.nextToken();
        const right = this.parseExpression(Precedence.PREFIX);
        if (!right) return null;
        return { type: 'PrefixExpression', operator, right };
      }
      case TokenType.LPAREN:
        this.nextToken();
        const exp = this.parseExpression(Precedence.LOWEST);
        if (!this.expectPeek(TokenType.RPAREN)) return null;
        return exp;
      case TokenType.LBRACKET: return this.parseArrayLiteral();
      case TokenType.LBRACE: return this.parseHashLiteral();
      case TokenType.FN: return this.parseFunctionLiteral();
      default: return null;
    }
  }

  private parseInfix(left: Expression): Expression {
    if (this.currentToken.type === TokenType.LPAREN) {
      return this.parseCallExpression(left);
    }
    if (this.currentToken.type === TokenType.LBRACKET) {
      return this.parseIndexExpression(left);
    }

    const operator = this.currentToken.literal;
    const precedence = this.currentPrecedence();
    this.nextToken();
    const right = this.parseExpression(precedence);

    return { type: 'BinaryExpression', left, operator, right: right! };
  }

  private parseFunctionLiteral(): FunctionLiteral | null {
    if (!this.expectPeek(TokenType.LPAREN)) return null;

    const parameters = this.parseFunctionParameters();

    if (!this.expectPeek(TokenType.LBRACE)) return null;

    const body = this.parseBlockStatement();

    return { type: 'FunctionLiteral', parameters, body };
  }

  private parseFunctionParameters(): Identifier[] {
    const identifiers: Identifier[] = [];

    if (this.peekToken.type === TokenType.RPAREN) {
      this.nextToken();
      return identifiers;
    }

    this.nextToken();
    identifiers.push({ type: 'Identifier', value: this.currentToken.literal });

    while (this.peekToken.type === TokenType.COMMA) {
      this.nextToken();
      this.nextToken();
      identifiers.push({ type: 'Identifier', value: this.currentToken.literal });
    }

    if (!this.expectPeek(TokenType.RPAREN)) return [];

    return identifiers;
  }

  private parseCallExpression(func: Expression): CallExpression {
    const args = this.parseExpressionList(TokenType.RPAREN);
    return { type: 'CallExpression', func, args };
  }

  private parseIndexExpression(left: Expression): Expression {
    this.nextToken(); // value expression
    const index = this.parseExpression(Precedence.LOWEST);
    this.expectPeek(TokenType.RBRACKET);
    return { type: 'IndexExpression', left, index: index! };
  }

  private parseArrayLiteral(): Expression | null {
    const elements = this.parseExpressionList(TokenType.RBRACKET);
    return { type: 'ArrayLiteral', elements };
  }

  private parseHashLiteral(): Expression | null {
    const pairs: { key: Expression; value: Expression }[] = [];

    while (this.peekToken.type !== TokenType.RBRACE) {
      this.nextToken();
      
      const key = this.parseExpression(Precedence.LOWEST);
      if (!key) return null;

      if (!this.expectPeek(TokenType.COLON)) return null;

      this.nextToken();
      
      const value = this.parseExpression(Precedence.LOWEST);
      if (!value) return null;

      pairs.push({ key, value });

      if ((this.peekToken.type as TokenType) !== TokenType.RBRACE && !this.expectPeek(TokenType.COMMA)) {
        return null;
      }
    }

    if (!this.expectPeek(TokenType.RBRACE)) return null;

    return { type: 'HashLiteral', pairs };
  }

  private parseExpressionList(end: TokenType): Expression[] {
    const args: Expression[] = [];

    if (this.peekToken.type === end) {
      this.nextToken();
      return args;
    }

    this.nextToken();
    args.push(this.parseExpression(Precedence.LOWEST)!);

    while (this.peekToken.type === TokenType.COMMA) {
      this.nextToken();
      this.nextToken();
      args.push(this.parseExpression(Precedence.LOWEST)!);
    }

    if (!this.expectPeek(end)) return [];

    return args;
  }

  private peekPrecedence(): Precedence {
    return precedences[this.peekToken.type] || Precedence.LOWEST;
  }

  private currentPrecedence(): Precedence {
    return precedences[this.currentToken.type] || Precedence.LOWEST;
  }

  private expectPeek(type: TokenType): boolean {
    if (this.peekToken.type === type) {
      this.nextToken();
      return true;
    } else {
      this.errors.push(`Expected next token to be ${type}, got ${this.peekToken.type} at line ${this.peekToken.line}`);
      return false;
    }
  }
}
