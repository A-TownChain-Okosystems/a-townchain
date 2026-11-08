export enum TokenType {
  LET = 'LET',
  PRINT = 'PRINT',
  IF = 'IF',
  ELSE = 'ELSE',
  WHILE = 'WHILE',
  FN = 'FN',
  RETURN = 'RETURN',
  TRUE = 'TRUE',
  FALSE = 'FALSE',
  IDENTIFIER = 'IDENTIFIER',
  NUMBER = 'NUMBER',
  STRING = 'STRING',
  ASSIGN = 'ASSIGN',
  PLUS = 'PLUS',
  MINUS = 'MINUS',
  MULTIPLY = 'MULTIPLY',
  DIVIDE = 'DIVIDE',
  LT = 'LT',
  GT = 'GT',
  LTE = 'LTE',
  GTE = 'GTE',
  EQ = 'EQ',
  NEQ = 'NEQ',
  LPAREN = 'LPAREN',
  RPAREN = 'RPAREN',
  LBRACE = 'LBRACE',
  RBRACE = 'RBRACE',
  LBRACKET = 'LBRACKET',
  RBRACKET = 'RBRACKET',
  COLON = 'COLON',
  COMMA = 'COMMA',
  AND = 'AND',
  OR = 'OR',
  BANG = 'BANG',
  EOF = 'EOF',
  ILLEGAL = 'ILLEGAL',
}

export interface Token {
  type: TokenType;
  literal: string;
  line: number;
}

export type Statement = LetStatement | PrintStatement | ExpressionStatement | BlockStatement | IfStatement | WhileStatement | ReturnStatement;

export interface LetStatement {
  type: 'LetStatement';
  name: Identifier;
  value: Expression;
  line: number;
}

export interface PrintStatement {
  type: 'PrintStatement';
  value: Expression;
  line: number;
}

export interface ExpressionStatement {
  type: 'ExpressionStatement';
  expression: Expression;
  line: number;
}

export interface BlockStatement {
  type: 'BlockStatement';
  statements: Statement[];
  line: number;
}

export interface IfStatement {
  type: 'IfStatement';
  condition: Expression;
  consequence: BlockStatement;
  alternative?: BlockStatement;
  line: number;
}

export interface WhileStatement {
  type: 'WhileStatement';
  condition: Expression;
  body: BlockStatement;
  line: number;
}

export interface ReturnStatement {
  type: 'ReturnStatement';
  returnValue: Expression;
  line: number;
}

export type Expression = Identifier | NumberLiteral | StringLiteral | BooleanLiteral | BinaryExpression | AssignmentExpression | FunctionLiteral | CallExpression | ArrayLiteral | HashLiteral | IndexExpression | PrefixExpression;

export interface Identifier {
  type: 'Identifier';
  value: string;
}

export interface NumberLiteral {
  type: 'NumberLiteral';
  value: number;
}

export interface StringLiteral {
  type: 'StringLiteral';
  value: string;
}

export interface BooleanLiteral {
  type: 'BooleanLiteral';
  value: boolean;
}

export interface BinaryExpression {
  type: 'BinaryExpression';
  left: Expression;
  operator: string;
  right: Expression;
}

export interface AssignmentExpression {
  type: 'AssignmentExpression';
  left: Identifier;
  value: Expression;
}

export interface FunctionLiteral {
  type: 'FunctionLiteral';
  parameters: Identifier[];
  body: BlockStatement;
}

export interface CallExpression {
  type: 'CallExpression';
  func: Expression;
  args: Expression[];
}

export interface ArrayLiteral {
  type: 'ArrayLiteral';
  elements: Expression[];
}

export interface HashLiteral {
  type: 'HashLiteral';
  pairs: { key: Expression; value: Expression }[];
}

export interface IndexExpression {
  type: 'IndexExpression';
  left: Expression;
  index: Expression;
}

export interface PrefixExpression {
  type: 'PrefixExpression';
  operator: string;
  right: Expression;
}

export interface Program {
  statements: Statement[];
}
