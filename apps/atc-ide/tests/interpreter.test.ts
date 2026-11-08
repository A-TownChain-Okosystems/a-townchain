import test from "node:test";
import assert from "node:assert/strict";
import { Lexer } from "../src/interpreter/lexer.ts";
import { Parser } from "../src/interpreter/parser.ts";
import { Evaluator, Environment } from "../src/interpreter/evaluator.ts";
import { TokenType } from "../src/interpreter/types.ts";

function parse(source: string) {
  const parser = new Parser(new Lexer(source));
  const program = parser.parseProgram();
  assert.deepEqual(parser.errors, [], `parser errors: ${parser.errors.join("; ")}`);
  return program;
}

test("lexer recognizes keywords, operators, and EOF", () => {
  const lexer = new Lexer("let total = 2 + 3 print total");
  const actual = [];
  for (let token = lexer.nextToken(); token.type !== TokenType.EOF; token = lexer.nextToken()) {
    actual.push([token.type, token.literal]);
  }
  assert.deepEqual(actual.slice(0, 5), [
    [TokenType.LET, "let"],
    [TokenType.IDENTIFIER, "total"],
    [TokenType.ASSIGN, "="],
    [TokenType.NUMBER, "2"],
    [TokenType.PLUS, "+"],
  ]);
  assert.ok(actual.length >= 5);
});

test("parser builds a program for variable assignment and print", () => {
  const program = parse("let total = 2 + 3 print total");
  assert.equal(program.statements.length, 2);
  assert.equal(program.statements[0].type, "LetStatement");
  assert.equal(program.statements[1].type, "PrintStatement");
});

test("evaluator computes arithmetic and emits expected output", () => {
  const env = new Environment();
  const evaluator = new Evaluator(env);
  const result = evaluator.eval(parse("let total = 2 + 3 * 4 print total"));
  assert.equal(result, 14);
  assert.deepEqual(evaluator.getOutput(), ["14"]);
});

test("evaluator supports functions and return values", () => {
  const env = new Environment();
  const evaluator = new Evaluator(env);
  const result = evaluator.eval(parse("let double = fn(x) { return x * 2 } double(6)"));
  assert.equal(result, 12);
});

test("evaluator rejects division by zero", () => {
  const env = new Environment();
  const evaluator = new Evaluator(env);
  const result = evaluator.eval(parse("10 / 0"));
  assert.ok(result instanceof Error);
  assert.match(result.message, /Division by zero/);
});

test("parser reports malformed syntax instead of silently accepting it", () => {
  const parser = new Parser(new Lexer("let ="));
  parser.parseProgram();
  assert.ok(parser.errors.length > 0);
});
