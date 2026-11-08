import { Program, Statement, Expression, BlockStatement, FunctionLiteral, Identifier } from './types';

export class Environment {
  private store: Record<string, any> = {};
  private outer: Environment | null = null;

  constructor(outer: Environment | null = null) {
    this.outer = outer;
  }

  public get(name: string): any {
    if (name in this.store) {
      return this.store[name];
    } else if (this.outer) {
      return this.outer.get(name);
    }
    return undefined;
  }

  public set(name: string, value: any): any {
    this.store[name] = value;
    return value;
  }

  public assign(name: string, value: any): any {
    if (name in this.store) {
      this.store[name] = value;
      return value;
    } else if (this.outer && this.outer.get(name) !== undefined) {
      return this.outer.assign(name, value);
    }
    return new Error(`undefined variable: ${name}`);
  }

  public getStore(): Record<string, any> {
    return { ...this.store };
  }
}

class ReturnValue {
  value: any;
  constructor(value: any) { this.value = value; }
}

class FunctionObj {
  parameters: Identifier[];
  body: BlockStatement;
  env: Environment;
  constructor(parameters: Identifier[], body: BlockStatement, env: Environment) {
    this.parameters = parameters;
    this.body = body;
    this.env = env;
  }
}

class BuiltinObj {
  fn: (...args: any[]) => any;
  constructor(fn: (...args: any[]) => any) { this.fn = fn; }
}

function setupBuiltins(env: Environment) {
  env.set('len', new BuiltinObj((arg: any) => {
    if (typeof arg === 'string') return arg.length;
    if (Array.isArray(arg)) return arg.length;
    return new Error(`argument to 'len' not supported, got ${typeof arg}`);
  }));
  env.set('push', new BuiltinObj((arr: any, el: any) => {
    if (!Array.isArray(arr)) return new Error(`argument to 'push' must be array`);
    arr.push(el);
    return arr;
  }));
  env.set('random', new BuiltinObj(() => Math.random()));
  env.set('round', new BuiltinObj((arg: any) => Math.round(arg)));
  env.set('floor', new BuiltinObj((arg: any) => Math.floor(arg)));
  env.set('ceil', new BuiltinObj((arg: any) => Math.ceil(arg)));
  env.set('abs', new BuiltinObj((arg: any) => Math.abs(arg)));
  env.set('input', new BuiltinObj((promptMsg: any) => {
    const msg = promptMsg ? String(promptMsg) : '';
    return window.prompt(msg) || '';
  }));
  env.set('toNumber', new BuiltinObj((arg: any) => Number(arg)));
  env.set('toString', new BuiltinObj((arg: any) => String(arg)));
}

export class Evaluator {
  private output: string[] = [];
  private env: Environment;

  constructor(env: Environment) {
    this.env = env;
    setupBuiltins(this.env);
  }

  public getOutput(): string[] {
    return this.output;
  }

  public eval(node: Program | Statement | Expression, env: Environment = this.env): any {
    if ('statements' in node && (!('type' in node) || node.type !== 'BlockStatement')) {
      return this.evalProgram(node as Program, env);
    }

    switch ((node as any).type) {
      case 'BlockStatement':
        return this.evalBlockStatement(node as BlockStatement, env);
        
      case 'LetStatement':
        const letVal = this.eval((node as any).value, env);
        if (this.isError(letVal)) return letVal;
        env.set((node as any).name.value, letVal);
        return letVal;
      
      case 'PrintStatement':
        const printVal = this.eval((node as any).value, env);
        if (this.isError(printVal)) return printVal;
        
        let outputStr = String(printVal);
        if (printVal instanceof FunctionObj) outputStr = `[Function]`;
        
        this.output.push(outputStr);
        return printVal;

      case 'ReturnStatement':
        const returnVal = this.eval((node as any).returnValue, env);
        if (this.isError(returnVal)) return returnVal;
        return new ReturnValue(returnVal);
        
      case 'IfStatement':
        const condition = this.eval((node as any).condition, env);
        if (this.isError(condition)) return condition;
        if (this.isTruthy(condition)) {
          return this.eval((node as any).consequence, env);
        } else if ((node as any).alternative) {
          return this.eval((node as any).alternative, env);
        }
        return null;

      case 'WhileStatement':
        while (true) {
          const cond = this.eval((node as any).condition, env);
          if (this.isError(cond)) return cond;
          if (!this.isTruthy(cond)) break;
          const bodyResult = this.eval((node as any).body, env);
          if (this.isError(bodyResult)) return bodyResult;
          if (bodyResult instanceof ReturnValue) return bodyResult;
        }
        return null;

      case 'ExpressionStatement':
        return this.eval((node as any).expression, env);

      case 'AssignmentExpression':
        const assignVal = this.eval((node as any).value, env);
        if (this.isError(assignVal)) return assignVal;
        const assignRes = env.assign((node as any).left.value, assignVal);
        return assignRes;

      case 'Identifier':
        const val = env.get((node as any).value);
        if (val === undefined) {
          return new Error(`identifier not found: ${(node as any).value}`);
        }
        return val;

      case 'NumberLiteral':
        return (node as any).value;

      case 'StringLiteral':
        return (node as any).value;
        
      case 'BooleanLiteral':
        return (node as any).value;
        
      case 'ArrayLiteral':
        const elements = (node as any).elements.map((el: any) => this.eval(el, env));
        return elements;

      case 'HashLiteral':
        const hash: Record<string, any> = {};
        for (const pair of (node as any).pairs) {
          const key = this.eval(pair.key, env);
          if (this.isError(key)) return key;
          const value = this.eval(pair.value, env);
          if (this.isError(value)) return value;
          hash[String(key)] = value;
        }
        return hash;

      case 'IndexExpression':
        const leftExpr = this.eval((node as any).left, env);
        if (this.isError(leftExpr)) return leftExpr;
        const indexExpr = this.eval((node as any).index, env);
        if (this.isError(indexExpr)) return indexExpr;
        
        if (Array.isArray(leftExpr) && typeof indexExpr === 'number') {
          return leftExpr[indexExpr];
        }
        if (typeof leftExpr === 'object' && leftExpr !== null) {
          return leftExpr[String(indexExpr)];
        }
        return new Error(`index operator not supported: ${typeof leftExpr}`);

      case 'FunctionLiteral':
        const params = (node as any).parameters;
        const body = (node as any).body;
        return new FunctionObj(params, body, env);

      case 'CallExpression':
        const func = this.eval((node as any).func, env);
        if (this.isError(func)) return func;
        
        const args = (node as any).args.map((arg: any) => this.eval(arg, env));
        if (args.length === 1 && this.isError(args[0])) return args[0];
        
        return this.applyFunction(func, args);

      case 'PrefixExpression':
        const rightExpr = this.eval((node as any).right, env);
        if (this.isError(rightExpr)) return rightExpr;
        return this.evalPrefixExpression((node as any).operator, rightExpr);

      case 'BinaryExpression':
        const operator = (node as any).operator;
        if (operator === '&&' || operator === 'and') {
          const leftBool = this.eval((node as any).left, env);
          if (this.isError(leftBool)) return leftBool;
          if (!this.isTruthy(leftBool)) return leftBool; // short-circuit
          return this.eval((node as any).right, env);
        }
        if (operator === '||' || operator === 'or') {
          const leftBool = this.eval((node as any).left, env);
          if (this.isError(leftBool)) return leftBool;
          if (this.isTruthy(leftBool)) return leftBool; // short-circuit
          return this.eval((node as any).right, env);
        }
        const left = this.eval((node as any).left, env);
        if (this.isError(left)) return left;
        const right = this.eval((node as any).right, env);
        if (this.isError(right)) return right;
        return this.evalBinaryExpression(operator, left, right);
    }

    return null;
  }

  private evalProgram(program: Program, env: Environment) {
    let result: any;
    for (const statement of program.statements) {
      result = this.eval(statement, env);
      
      if (result instanceof ReturnValue) {
        return result.value;
      }
      
      if (this.isError(result)) {
        this.output.push(`Error: ${(result as Error).message}`);
        return result;
      }
    }
    return result;
  }

  private evalBlockStatement(block: BlockStatement, env: Environment) {
    let result: any;
    for (const statement of block.statements) {
      result = this.eval(statement, env);
      if (result !== null && (result instanceof ReturnValue || this.isError(result))) {
        return result;
      }
    }
    return result;
  }

  private evalPrefixExpression(operator: string, right: any): any {
    switch (operator) {
      case '!':
      case 'not':
        return !this.isTruthy(right);
      case '-':
        if (typeof right !== 'number') {
          return new Error(`unknown operator: -${typeof right}`);
        }
        return -right;
      default:
        return new Error(`unknown operator: ${operator}${typeof right}`);
    }
  }

  private evalBinaryExpression(operator: string, left: any, right: any): any {
    if (typeof left === 'number' && typeof right === 'number') {
      switch (operator) {
        case '+': return left + right;
        case '-': return left - right;
        case '*': return left * right;
        case '/': 
          if (right === 0) return new Error('Division by zero');
          return left / right;
        case '<': return left < right;
        case '>': return left > right;
        case '<=': return left <= right;
        case '>=': return left >= right;
        case '==': return left === right;
        case '!=': return left !== right;
        default: return new Error(`unknown operator: ${operator}`);
      }
    }
    
    if (typeof left === 'string' || typeof right === 'string') {
      if (operator === '+') {
        return String(left) + String(right);
      }
      if (typeof left === 'string' && typeof right === 'string') {
        if (operator === '==') return left === right;
        if (operator === '!=') return left !== right;
      }
    }
    
    if (typeof left === 'boolean' && typeof right === 'boolean') {
      if (operator === '==') return left === right;
      if (operator === '!=') return left !== right;
    }

    return new Error(`type mismatch: ${typeof left} ${operator} ${typeof right}`);
  }

  private applyFunction(fn: any, args: any[]): any {
    if (fn instanceof BuiltinObj) {
      return fn.fn(...args);
    }
    if (!(fn instanceof FunctionObj)) {
      return new Error(`not a function: ${typeof fn}`);
    }
    
    const extendedEnv = new Environment(fn.env);
    
    for (let i = 0; i < fn.parameters.length; i++) {
        extendedEnv.set(fn.parameters[i].value, args[i]);
    }
    
    const evaluated = this.eval(fn.body, extendedEnv);
    return this.unwrapReturnValue(evaluated);
  }
  
  private unwrapReturnValue(obj: any): any {
    if (obj instanceof ReturnValue) return obj.value;
    return obj;
  }

  private isTruthy(obj: any): boolean {
    if (obj === false || obj === null || obj === 0 || obj === "") return false;
    return true;
  }

  private isError(obj: any): boolean {
    return obj instanceof Error;
  }
}
