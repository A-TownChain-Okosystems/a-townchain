// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! AST des Canonical Cores mit kanonischer JSON-Serialisierung.
//! Die JSON ist BYTGLEICH mit der Python-Referenz-Ausgabe aus
//! tools/differential/dump_reference.py (json.dumps, sort_keys=True,
//! compact separators) — das ist der Differential-Kontrakt (SCR-0084/0085).
//! Stage 3: FunctionDef/Parameter/ReturnStatement/ExprStatement (SCR-0085).

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    /// Exact unsigned literals beyond the VM's current i64 execution subset.
    BigInt(u128),
    String(String),
    Ident(String),
    Unary {
        op: String,
        operand: Box<Expr>,
    },
    Binary {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        target: Box<Expr>,
        args: Vec<Expr>,
    },
    /// Feldzugriff: `self.balance` oder `a.b`.
    Field {
        base: Box<Expr>,
        name: String,
    },
    /// Indexzugriff: `m["k"]` oder `xs[0]`.
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    /// Bereich `a..b`, `a..=b`, `a..` — SPEC 3.4: RangeExpr.
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        inclusive: bool,
    },
    /// Index-Bereich `xs[a..b]`, `xs[a..]` — SPEC 3.4: SliceExpr.
    Slice {
        base: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        inclusive: bool,
    },
    /// Array-Literal: `[]`, `[1, 2]`.
    Array(Vec<Expr>),
    /// Tupel: `(a, b)`.
    Tuple(Vec<Expr>),
    /// Map-Literal: `{}`, `{"a": 1}`.
    MapLiteral(Vec<(Expr, Expr)>),
    /// Struct-Literal: `Proposal { id: 1, status: Active }`.
    StructLiteral {
        name: String,
        fields: Vec<(String, Expr)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeHint {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt {
    pub name: String,
    pub is_const: bool,
    pub type_hint: Option<TypeHint>,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_hint: TypeHint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeHint>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let(LetStmt),
    Fn(FunctionDef),
    /// Native function syntax preserved when it exceeds the executable subset.
    NativeFunction {
        name: String,
        signature_tokens: Vec<String>,
        body_tokens: Vec<String>,
    },
    /// Import path from a native `use A::B` declaration.
    Use {
        path: String,
    },
    /// `module NAME` — Programmkopf (SPEC 3.1). Metadatum, keine Ausfuehrung.
    Module {
        name: String,
    },
    /// `import "std/crypto.atc" as Crypto` — Dateiimport mit optionalem Alias.
    Import {
        source: String,
        alias: Option<String>,
    },
    /// `use A::B::{X, Y}` — Gruppenimport.
    UseGroup {
        path: String,
        names: Vec<String>,
    },
    /// Native data declaration; generic field types are preserved textually.
    Struct {
        name: String,
        fields: Vec<(String, TypeHint)>,
    },
    /// Native enum declaration and its variant names.
    Enum {
        name: String,
        variants: Vec<String>,
    },
    /// Contract mit geparstem Koerper (Items: state/event/fn/const/struct/enum/use).
    Contract {
        name: String,
        bases: Vec<String>,
        body: Vec<Stmt>,
    },
    /// Persistenter Zustand im Contract: `state owner: Address [= default]`.
    State {
        name: String,
        ty: TypeHint,
        default: Option<Expr>,
    },
    /// Blockform `state { a: T = v; ... }`.
    StateBlock {
        fields: Vec<(String, TypeHint, Option<Expr>)>,
    },
    /// Event-Deklaration: `event ProposalCreated(proposal_id: Hash, ...)`.
    Event {
        name: String,
        fields: Vec<(String, TypeHint)>,
    },
    /// `emit Event(args)`.
    Emit {
        event: String,
        args: Vec<Expr>,
    },
    /// `require(cond[, "meldung"])`.
    Require {
        cond: Expr,
        message: Option<Expr>,
    },
    /// Zuweisung; op = None bedeutet `=`, sonst `+=`, `-=`, `*=` (Operator ohne '=').
    Assign {
        target: Expr,
        op: Option<String>,
        value: Expr,
    },
    Return {
        value: Option<Expr>,
    },
    /// Nackter Ausdruck — Referenz wrappt als ExprStatement.
    Expr(Expr),
    /// if/else (SCR-0128 Stufe 1); else_body None = kein else, Some = else-Zweig
    /// (else-if wird als verschachteltes If im else_body abgebildet).
    If {
        cond: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    /// while (SCR-0128 Stufe 1).
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    /// for-Schleife (SPEC 3.2: for_stmt), `iter` ist ein Bereich.
    For {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

fn esc(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => out.push(c),
        }
    }
    out
}

fn expr_json(e: &Expr, out: &mut String) {
    match e {
        Expr::Int(v) => {
            out.push_str("{\"kind\":\"IntLiteral\",\"value\":");
            out.push_str(&v.to_string());
            out.push('}');
        }
        Expr::BigInt(v) => {
            out.push_str("{\"kind\":\"IntLiteral\",\"value\":");
            out.push_str(&v.to_string());
            out.push('}');
        }
        Expr::String(value) => {
            out.push_str("{\"kind\":\"StringLiteral\",\"value\":\"");
            out.push_str(&esc(value));
            out.push_str("\"}");
        }
        Expr::Ident(name) => {
            out.push_str("{\"kind\":\"Identifier\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\"}");
        }
        Expr::Field { base, name } => {
            out.push_str("{\"kind\":\"FieldAccess\",\"base\":");
            expr_json(base, out);
            out.push_str(",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\"}");
        }
        Expr::Index { base, index } => {
            out.push_str("{\"index\":");
            expr_json(index, out);
            out.push_str(",\"kind\":\"IndexAccess\",\"base\":");
            expr_json(base, out);
            out.push('}');
        }
        Expr::Range {
            start,
            end,
            inclusive,
        } => {
            out.push_str("{\"end\":");
            opt_expr_json(end.as_deref(), out);
            out.push_str(",\"inclusive\":");
            out.push_str(if *inclusive { "true" } else { "false" });
            out.push_str(",\"kind\":\"RangeExpr\",\"start\":");
            opt_expr_json(start.as_deref(), out);
            out.push('}');
        }
        Expr::Slice {
            base,
            start,
            end,
            inclusive,
        } => {
            out.push_str("{\"base\":");
            expr_json(base, out);
            out.push_str(",\"end\":");
            opt_expr_json(end.as_deref(), out);
            out.push_str(",\"inclusive\":");
            out.push_str(if *inclusive { "true" } else { "false" });
            out.push_str(",\"kind\":\"SliceExpr\",\"start\":");
            opt_expr_json(start.as_deref(), out);
            out.push('}');
        }
        Expr::Array(items) => {
            out.push_str("{\"kind\":\"ArrayLiteral\",\"elements\":[");
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                expr_json(item, out);
            }
            out.push_str("]}");
        }
        Expr::StructLiteral { name, fields } => {
            out.push_str("{\"fields\":[");
            for (i, (field, value)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str("{\"kind\":\"FieldAssignment\",\"name\":\"");
                out.push_str(&esc(field));
                out.push_str("\",\"value\":");
                expr_json(value, out);
                out.push('}');
            }
            out.push_str("],\"kind\":\"StructLiteral\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\"}");
        }
        Expr::MapLiteral(entries) => {
            out.push_str("{\"entries\":[");
            for (i, (key, value)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str("{\"key\":");
                expr_json(key, out);
                out.push_str(",\"value\":");
                expr_json(value, out);
                out.push('}');
            }
            out.push_str("],\"kind\":\"MapLiteral\"}");
        }
        Expr::Tuple(items) => {
            out.push_str("{\"kind\":\"TupleLiteral\",\"elements\":[");
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                expr_json(item, out);
            }
            out.push_str("]}");
        }
        Expr::Unary { op, operand } => {
            out.push_str("{\"kind\":\"UnaryOp\",\"op\":\"");
            out.push_str(&esc(op));
            out.push_str("\",\"operand\":");
            expr_json(operand, out);
            out.push('}');
        }
        Expr::Binary { op, left, right } => {
            out.push_str("{\"kind\":\"BinaryOp\",\"left\":");
            expr_json(left, out);
            out.push_str(",\"op\":\"");
            out.push_str(&esc(op));
            out.push_str("\",\"right\":");
            expr_json(right, out);
            out.push('}');
        }
        Expr::Call { target, args } => {
            out.push_str("{\"args\":[");
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                expr_json(a, out);
            }
            out.push_str("],\"kind\":\"FunctionCall\",\"target\":");
            expr_json(target, out);
            out.push('}');
        }
    }
}

fn type_json(t: &TypeHint, out: &mut String) {
    out.push_str("{\"kind\":\"TypeAnnotation\",\"name\":\"");
    out.push_str(&esc(&t.name));
    out.push_str("\",\"params\":[]}");
}

fn let_json(s: &LetStmt, out: &mut String) {
    out.push_str("{\"is_const\":");
    out.push_str(if s.is_const { "true" } else { "false" });
    out.push_str(",\"kind\":\"LetStatement\",\"name\":\"");
    out.push_str(&esc(&s.name));
    out.push_str("\",\"type_hint\":");
    match &s.type_hint {
        None => out.push_str("null"),
        Some(t) => type_json(t, out),
    }
    out.push_str(",\"value\":");
    match &s.value {
        None => out.push_str("null"),
        Some(v) => expr_json(v, out),
    }
    out.push('}');
}

fn param_json(p: &Param, out: &mut String) {
    out.push_str("{\"kind\":\"Parameter\",\"name\":\"");
    out.push_str(&esc(&p.name));
    out.push_str("\",\"type_hint\":");
    type_json(&p.type_hint, out);
    out.push('}');
}

fn fn_json(f: &FunctionDef, out: &mut String) {
    out.push_str("{\"body\":[");
    for (i, s) in f.body.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        stmt_json(s, out);
    }
    out.push_str("],\"decorators\":[],\"is_pub\":false,\"kind\":\"FunctionDef\",\"name\":\"");
    out.push_str(&esc(&f.name));
    out.push_str("\",\"params\":[");
    for (i, p) in f.params.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        param_json(p, out);
    }
    out.push_str("],\"return_type\":");
    match &f.return_type {
        None => out.push_str("null"),
        Some(t) => type_json(t, out),
    }
    out.push('}');
}

fn stmt_json(s: &Stmt, out: &mut String) {
    match s {
        Stmt::Let(l) => let_json(l, out),
        Stmt::Fn(f) => fn_json(f, out),
        Stmt::NativeFunction {
            name,
            signature_tokens,
            body_tokens,
        } => {
            out.push_str("{\"body_tokens\":[");
            for (i, token) in body_tokens.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push('"');
                out.push_str(&esc(token));
                out.push('"');
            }
            out.push_str("],\"kind\":\"NativeFunctionDef\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\",\"signature_tokens\":[");
            for (i, token) in signature_tokens.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push('"');
                out.push_str(&esc(token));
                out.push('"');
            }
            out.push_str("]}");
        }
        Stmt::Use { path } => {
            out.push_str("{\"kind\":\"UseDeclaration\",\"path\":\"");
            out.push_str(&esc(path));
            out.push_str("\"}");
        }
        Stmt::Module { name } => {
            out.push_str("{\"kind\":\"ModuleDeclaration\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\"}");
        }
        Stmt::Import { source, alias } => {
            out.push_str("{\"kind\":\"FileImportDeclaration\",\"source\":\"");
            out.push_str(&esc(source));
            out.push_str("\",\"alias\":");
            match alias {
                Some(a) => {
                    out.push('"');
                    out.push_str(&esc(a));
                    out.push('"');
                }
                None => out.push_str("null"),
            }
            out.push('}');
        }
        Stmt::UseGroup { path, names } => {
            out.push_str("{\"kind\":\"UseGroupDeclaration\",\"path\":\"");
            out.push_str(&esc(path));
            out.push_str("\",\"names\":[");
            for (i, name) in names.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push('"');
                out.push_str(&esc(name));
                out.push('"');
            }
            out.push_str("]}");
        }
        Stmt::Struct { name, fields } => {
            out.push_str("{\"fields\":[");
            for (i, (field, ty)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str("{\"kind\":\"FieldDeclaration\",\"name\":\"");
                out.push_str(&esc(field));
                out.push_str("\",\"type_hint\":");
                type_json(ty, out);
                out.push('}');
            }
            out.push_str("],\"kind\":\"StructDeclaration\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\"}");
        }
        Stmt::Enum { name, variants } => {
            out.push_str("{\"kind\":\"EnumDeclaration\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\",\"variants\":[");
            for (i, variant) in variants.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push('"');
                out.push_str(&esc(variant));
                out.push('"');
            }
            out.push_str("]}");
        }
        Stmt::Contract { name, bases, body } => {
            out.push_str("{\"bases\":[");
            for (i, base) in bases.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push('"');
                out.push_str(&esc(base));
                out.push('"');
            }
            out.push_str("],\"body\":[");
            for (i, item) in body.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                stmt_json(item, out);
            }
            out.push_str("],\"kind\":\"ContractDeclaration\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\"}");
        }
        Stmt::State { name, ty, default } => {
            out.push_str("{\"default\":");
            match default {
                None => out.push_str("null"),
                Some(v) => expr_json(v, out),
            }
            out.push_str(",\"kind\":\"StateDeclaration\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\",\"type_hint\":");
            type_json(ty, out);
            out.push('}');
        }
        Stmt::StateBlock { fields } => {
            out.push_str("{\"fields\":[");
            for (i, (name, ty, default)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str("{\"default\":");
                match default {
                    None => out.push_str("null"),
                    Some(v) => expr_json(v, out),
                }
                out.push_str(",\"kind\":\"StateDeclaration\",\"name\":\"");
                out.push_str(&esc(name));
                out.push_str("\",\"type_hint\":");
                type_json(ty, out);
                out.push('}');
            }
            out.push_str("],\"kind\":\"StateBlockDeclaration\"}");
        }
        Stmt::Event { name, fields } => {
            out.push_str("{\"fields\":[");
            for (i, (field, ty)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str("{\"kind\":\"Parameter\",\"name\":\"");
                out.push_str(&esc(field));
                out.push_str("\",\"type_hint\":");
                type_json(ty, out);
                out.push('}');
            }
            out.push_str("],\"kind\":\"EventDeclaration\",\"name\":\"");
            out.push_str(&esc(name));
            out.push_str("\"}");
        }
        Stmt::Emit { event, args } => {
            out.push_str("{\"args\":[");
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                expr_json(arg, out);
            }
            out.push_str("],\"kind\":\"EmitStatement\",\"event\":\"");
            out.push_str(&esc(event));
            out.push_str("\"}");
        }
        Stmt::Require { cond, message } => {
            out.push_str("{\"condition\":");
            expr_json(cond, out);
            out.push_str(",\"kind\":\"RequireStatement\",\"message\":");
            match message {
                None => out.push_str("null"),
                Some(m) => expr_json(m, out),
            }
            out.push('}');
        }
        Stmt::Assign { target, op, value } => {
            out.push_str("{\"kind\":\"AssignmentStatement\",\"operator\":\"");
            out.push_str(op.as_deref().unwrap_or("="));
            out.push_str("\",\"target\":");
            expr_json(target, out);
            out.push_str(",\"value\":");
            expr_json(value, out);
            out.push('}');
        }
        Stmt::Return { value } => {
            out.push_str("{\"kind\":\"ReturnStatement\",\"value\":");
            match value {
                None => out.push_str("null"),
                Some(v) => expr_json(v, out),
            }
            out.push('}');
        }
        Stmt::Expr(e) => {
            out.push_str("{\"expr\":");
            expr_json(e, out);
            out.push_str(",\"kind\":\"ExprStatement\"}");
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            out.push_str("{\"cond\":");
            expr_json(cond, out);
            out.push_str(",\"else_body\":");
            match else_body {
                None => out.push_str("null"),
                Some(body) => stmt_list_json(body, out),
            }
            out.push_str(",\"kind\":\"IfStatement\",\"then_body\":");
            stmt_list_json(then_body, out);
            out.push('}');
        }
        Stmt::While { cond, body } => {
            out.push_str("{\"body\":");
            stmt_list_json(body, out);
            out.push_str(",\"cond\":");
            expr_json(cond, out);
            out.push_str(",\"kind\":\"WhileStatement\"}");
        }
        Stmt::For { var, iter, body } => {
            out.push_str("{\"body\":");
            stmt_list_json(body, out);
            out.push_str(",\"iter\":");
            expr_json(iter, out);
            out.push_str(",\"kind\":\"ForStatement\",\"var\":\"");
            out.push_str(&esc(var));
            out.push_str("\"}");
        }
    }
}

/// Serialisiert einen optionalen Teilausdruck (`null` bei offenem Bereichsende).
fn opt_expr_json(expr: Option<&Expr>, out: &mut String) {
    match expr {
        None => out.push_str("null"),
        Some(e) => expr_json(e, out),
    }
}

/// Gemeinsame Statement-Listen-Serialisierung (alphabetische Keys, bytstabil).
fn stmt_list_json(stmts: &[Stmt], out: &mut String) {
    out.push('[');
    for (i, s) in stmts.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        stmt_json(s, out);
    }
    out.push(']');
}

impl Program {
    /// Kanonisches JSON — bytgleich zur Python-Referenz (Differential-Kontrakt).
    pub fn to_json(&self) -> String {
        let mut out = String::from("{\"kind\":\"Program\",\"statements\":[");
        for (i, s) in self.statements.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            stmt_json(s, &mut out);
        }
        out.push_str("]}");
        out
    }
}
