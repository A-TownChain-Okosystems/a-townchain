// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Recursive-Descent-Parser — Canonical Core (SCR-0085, Fortschreibung SCR-0128
//! Stufe 1). Programm-Ebene = let/const/fn; Bloecke = let/return/if/else/while/
//! ExprStatement. Praezedenz: Vergleich (< == >) unter Addition, Addition unter
//! Multiplikation (linksassoziativ, keine Kurzschluss-Booleans).
//! Referenz-Semantiken bewusst uebernommen: Parameter MIT Pflicht-Typ,
//! 'return' nur ohne Semikolon vor '}' (return; wirft wie Referenz Fehler),
//! nackte Ausdruecke werden als ExprStatement gewrappt.

use crate::ast::{Expr, FunctionDef, LetStmt, Param, Program, Stmt, TypeHint};
use crate::lexer::{tokenize_pos, Pos, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
}

/// Text eines Namens- oder Feldpfads (`self.owner`, `A::b`).
fn path_text(e: &Expr) -> Option<String> {
    match e {
        Expr::Ident(name) => Some(name.clone()),
        Expr::Field { base, name } => Some(format!("{}.{}", path_text(base)?, name)),
        _ => None,
    }
}

/// Quelltext eines Schluesselwort-Tokens, falls es eines ist.
///
/// Wird gebraucht, wo ein Name stehen muss, der Korpus dort aber ein
/// Schluesselwort schreibt (`struct S { state: u64 }`).
fn keyword_text(t: &Token) -> Option<&'static str> {
    Some(match t {
        Token::Let => "let",
        Token::Const => "const",
        Token::Return => "return",
        Token::Fn => "fn",
        Token::If => "if",
        Token::Else => "else",
        Token::While => "while",
        Token::Contract => "contract",
        Token::State => "state",
        Token::Function => "function",
        Token::Capability => "capability",
        Token::Policy => "policy",
        Token::Require => "require",
        Token::Use => "use",
        Token::Module => "module",
        Token::Import => "import",
        Token::As => "as",
        Token::Pub => "pub",
        Token::Struct => "struct",
        Token::Enum => "enum",
        _ => return None,
    })
}

pub fn parse_program(src: &str) -> Result<Program, ParseError> {
    let (toks, cols) = tokenize_pos(src).map_err(|e| ParseError {
        message: format!("Lex-Fehler bei Position {}: {:?}", e.pos, e.ch),
    })?;
    let mut p = Parser {
        toks,
        pos: 0,
        cols,
        errors: Vec::new(),
        collect: false,
    };
    p.program()
}

/// Wie [`parse_program`], aber sammelt **alle** Fehler einer Datei.
///
/// Nach einem Fehler springt der Parser bis zur naechsten Deklaration und
/// macht weiter. Das Programm ist damit nur ein Teilstand und darf nicht
/// kompiliert werden; die Fehlerliste sagt dagegen, welche Formen in dieser
/// Datei wirklich fehlen. Einzelne Fehlerdaten sind ungenauer als bei
/// [`parse_program`], weil der Kontext nach dem Sprung verloren geht.
pub fn parse_program_collecting(src: &str) -> (Option<Program>, Vec<ParseError>) {
    let tokens = tokenize_pos(src);
    let (toks, cols) = match tokens {
        Ok(pair) => pair,
        Err(e) => {
            return (
                None,
                vec![ParseError {
                    message: format!("Lex-Fehler bei Position {}: {:?}", e.pos, e.ch),
                }],
            )
        }
    };
    let mut p = Parser {
        toks,
        pos: 0,
        cols,
        errors: Vec::new(),
        collect: true,
    };
    let program = p.program();
    let errors = std::mem::take(&mut p.errors);
    match program {
        Ok(program) => (Some(program), errors),
        Err(e) => {
            let mut errors = errors;
            errors.push(e);
            (None, errors)
        }
    }
}

struct Parser {
    toks: Vec<Token>,
    pos: usize,
    /// Zeile/Spalte je Token. Leer, wenn ohne Positionen geparst wird; dann
    /// stehen die eingerueckten Rumpfformen nicht zur Verfuegung.
    cols: Vec<Pos>,
    /// Gesammelte Fehler im Sammelmodus (siehe [`parse_program_collecting`]).
    errors: Vec<ParseError>,
    /// Im Sammelmodus wird nach einem Fehler bis zur naechsten Deklaration
    /// weitergesprungen, statt abzubrechen. Im Normalbetrieb `false`, dann
    /// verhaelt sich der Parser exakt wie zuvor.
    collect: bool,
}

impl Parser {
    /// Fehler verbuchen und weitergehen, oder im Normalbetrieb durchreichen.
    ///
    /// Im Sammelmodus wird der Fehler abgelegt und bis zur naechsten
    /// Deklaration gesprungen; sonst verlaesst der Fehler den Parser.
    fn failed(&mut self, error: ParseError) -> Result<(), ParseError> {
        if !self.collect {
            // Normalbetrieb: der Fehler verlaesst den Parser wie bisher.
            return Err(error);
        }
        self.errors.push(error);
        self.recover();
        Ok(())
    }

    /// Bis zur naechsten Deklaration springen (mindestens ein Token weit).
    ///
    /// Anker sind die Token, die eine Deklaration beginnen koennen. Eine
    /// schliessende Klammer beendet den Sprung, damit ein Fehler im Rumpf
    /// nicht in den naechsten Block laeuft.
    fn recover(&mut self) {
        if self.cur() != Token::Eof {
            self.pos += 1;
        }
        while self.cur() != Token::Eof {
            match self.cur() {
                Token::RBrace => return,
                Token::Let
                | Token::Const
                | Token::Fn
                | Token::Use
                | Token::Import
                | Token::Struct
                | Token::Enum
                | Token::Contract
                | Token::State
                | Token::Module
                | Token::Pub
                | Token::At => return,
                Token::Ident(name) if name == "event" || name == "type" => return,
                _ => self.pos += 1,
            }
        }
    }

    /// Spalte des aktuellen Tokens, falls Positionen bekannt sind.
    fn col(&self) -> Option<u32> {
        self.cols.get(self.pos).map(|p| p.col)
    }

    /// Steht das aktuelle Token in einem eingerueckten Rumpf?
    ///
    /// Eingerueckt heisst: weiter rechts als die Deklaration, die den Rumpf
    /// aufmacht. Genau das ist die DEDENT-Regel aus SPEC 3.2, auf Spalten
    /// abgebildet; ohne Spalten ist die Frage nicht beantwortbar und die
    /// Antwort bleibt `false`.
    fn at_indented_line(&self, base: Option<u32>) -> bool {
        match (self.col(), base) {
            (Some(col), Some(base)) => col > base && self.cur() != Token::Eof,
            _ => false,
        }
    }

    fn cur(&self) -> Token {
        self.toks.get(self.pos).cloned().unwrap_or(Token::Eof)
    }

    fn program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        while self.cur() != Token::Eof {
            // File-level annotations (for example @version and @name) are
            // metadata, not executable statements. Consume them up to the
            // next recognized declaration without changing runtime semantics.
            if self.cur() == Token::At {
                self.skip_file_annotations();
                continue;
            }
            // `module NAME { ... }` gruppiert Deklarationen; der Block selbst
            // fuehrt nichts aus, die Deklarationen darin gelten programmweit.
            if self.cur() == Token::Module {
                match self.module_block() {
                    Ok(items) => statements.extend(items),
                    Err(e) => self.failed(e)?,
                }
                continue;
            }
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => self.failed(e)?,
            }
        }
        Ok(Program { statements })
    }

    fn function_decl(&mut self) -> Result<Stmt, ParseError> {
        let start = self.pos;
        match self.function() {
            Ok(function) => Ok(Stmt::Fn(function)),
            Err(_) => {
                self.pos = start;
                self.native_function_decl()
            }
        }
    }

    fn native_function_decl(&mut self) -> Result<Stmt, ParseError> {
        let start = self.pos;
        self.pos += 1; // fn
        let name = self.expect_name("Funktionsname")?;
        let mut signature_tokens = Vec::new();
        while self.cur() != Token::LBrace {
            let token = self.cur();
            if token == Token::Eof {
                self.pos = start;
                return Err(ParseError {
                    message: format!("'{{' in Signatur von {name} erwartet"),
                });
            }
            self.pos += 1;
            signature_tokens.push(token);
        }
        // Preserve unsupported native syntax as metadata, but only for a
        // recognizable native shape. Generic signatures plus indexed access
        // cover list helpers such as cat(a: List<string>, b: List<string>)
        // without accepting arbitrary ordinary functions.
        let has_generic_signature = signature_tokens.contains(&Token::Lt);
        self.pos += 1; // function body opening brace
        let mut depth = 1usize;
        let mut body_tokens = Vec::new();
        let mut has_native_string = false;
        let mut has_index_access = false;
        let mut has_struct_literal = false;
        let mut previous_token: Option<Token> = None;
        while depth > 0 {
            let token = self.cur();
            if token == Token::Eof {
                self.pos = start;
                return Err(ParseError {
                    message: format!("Nicht geschlossener Funktionsblock {name}"),
                });
            }
            self.pos += 1;
            match token {
                Token::LBrace => {
                    if previous_token
                        .as_ref()
                        .is_some_and(|previous| matches!(previous, Token::Ident(_)))
                    {
                        has_struct_literal = true;
                    }
                    depth += 1;
                    body_tokens.push(Self::token_text(&Token::LBrace));
                }
                Token::RBrace => {
                    depth -= 1;
                    if depth > 0 {
                        body_tokens.push(Self::token_text(&Token::RBrace));
                    }
                }
                Token::String(_) => {
                    has_native_string = true;
                    body_tokens.push(Self::token_text(&token));
                }
                Token::LBracket => {
                    has_index_access = true;
                    body_tokens.push(Self::token_text(&token));
                }
                _ => body_tokens.push(Self::token_text(&token)),
            }
            previous_token = Some(token);
        }
        let has_typed_return = signature_tokens.contains(&Token::Arrow);
        if !has_native_string
            && !(has_generic_signature && has_index_access)
            && !(has_typed_return && has_struct_literal)
        {
            self.pos = start;
            return Err(ParseError {
                message: format!(
                    "Funktionsdeklaration {name} ist außerhalb des unterstützten Parsers"
                ),
            });
        }
        Ok(Stmt::NativeFunction {
            name,
            signature_tokens: signature_tokens.iter().map(Self::token_text).collect(),
            body_tokens,
        })
    }
    fn simple_type(&mut self) -> Result<TypeHint, ParseError> {
        // Tupeltyp `(A, B)` — fuer Mehrfach-Rueckgaben.
        if self.cur() == Token::LParen {
            self.pos += 1;
            let mut parts = Vec::new();
            while self.cur() != Token::RParen {
                parts.push(self.simple_type()?.name);
                if self.cur() == Token::Comma {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            if self.cur() != Token::RParen {
                return Err(ParseError {
                    message: "')' nach Tupeltyp erwartet".to_string(),
                });
            }
            self.pos += 1;
            return Ok(TypeHint {
                name: format!("({})", parts.join(", ")),
            });
        }
        let mut name = match self.cur() {
            Token::Ident(n) => {
                self.pos += 1;
                n
            }
            other => {
                return Err(ParseError {
                    message: format!(
                        "Typ erwartet, fand {:?} (Umfeld: {})",
                        other,
                        self.context_window()
                    ),
                })
            }
        };

        // Preserve generic types such as Map<Address, Map<Address, u128>>.
        if self.cur() == Token::Lt {
            name.push('<');
            self.pos += 1;
            let mut depth = 1usize;
            while depth > 0 {
                let token = self.cur();
                if token == Token::Eof {
                    return Err(ParseError {
                        message: "Unvollständiger generischer Typ".to_string(),
                    });
                }
                self.pos += 1;
                match token {
                    Token::Lt => {
                        depth += 1;
                        name.push('<');
                    }
                    Token::Gt => {
                        depth -= 1;
                        name.push('>');
                    }
                    _ => name.push_str(&Self::token_text(&token)),
                }
            }
        }
        // `data: u8[120]` — Feldtyp mit fester Laenge steht im Korpus. Die
        // Laenge ist Teil des Typnamens, nicht des Feldnamens.
        while self.cur() == Token::LBracket {
            self.pos += 1;
            let mut inner = String::from("[");
            while self.cur() != Token::RBracket && self.cur() != Token::Eof {
                inner.push_str(&Self::token_text(&self.cur()));
                self.pos += 1;
            }
            if self.cur() != Token::RBracket {
                return Err(ParseError {
                    message: "']' nach Typlaenge erwartet".to_string(),
                });
            }
            self.pos += 1;
            inner.push(']');
            name.push_str(&inner);
        }
        Ok(TypeHint { name })
    }

    /// Steht an dieser Stelle ein Feldname mit Doppelpunkt?
    ///
    /// Korpusform: Strukturfelder sind teils nur durch einen Zeilenumbruch
    /// getrennt. Ein Name, auf den direkt ':' folgt, kann nichts anderes sein
    /// als das naechste Feld.
    fn at_field_start(&self) -> bool {
        let is_name = matches!(self.cur(), Token::Ident(_)) || keyword_text(&self.cur()).is_some();
        is_name && self.toks.get(self.pos + 1) == Some(&Token::Colon)
    }

    fn token_text(token: &Token) -> String {
        match token {
            Token::Ident(value) => value.clone(),
            Token::Int(value) => value.to_string(),
            Token::String(value) => format!("{:?}", value),
            Token::Let => "let".into(),
            Token::Const => "const".into(),
            Token::Return => "return".into(),
            Token::Fn => "fn".into(),
            Token::If => "if".into(),
            Token::Else => "else".into(),
            Token::While => "while".into(),
            Token::DotDot => "..".into(),
            Token::DotDotEq => "..=".into(),
            Token::Contract => "contract".into(),
            Token::State => "state".into(),
            Token::Function => "function".into(),
            Token::Capability => "capability".into(),
            Token::Policy => "policy".into(),
            Token::Require => "require".into(),
            Token::Use => "use".into(),
            Token::Module => "module".into(),
            Token::Import => "import".into(),
            Token::As => "as".into(),
            Token::Pub => "pub".into(),
            Token::Struct => "struct".into(),
            Token::Enum => "enum".into(),
            Token::Pipe => "|".into(),
            Token::Ampersand => "&".into(),
            Token::Assign => "=".into(),
            Token::Eq => "==".into(),
            Token::NotEq => "!=".into(),
            Token::Bang => "!".into(),
            Token::Lt => "<".into(),
            Token::Gt => ">".into(),
            Token::LtEq => "<=".into(),
            Token::GtEq => ">=".into(),
            Token::Plus => "+".into(),
            Token::Minus => "-".into(),
            Token::Arrow => "->".into(),
            Token::Star => "*".into(),
            Token::Slash => "/".into(),
            Token::Percent => "%".into(),
            Token::Colon => ":".into(),
            Token::LParen => "(".into(),
            Token::RParen => ")".into(),
            Token::LBracket => "[".into(),
            Token::RBracket => "]".into(),
            Token::LBrace => "{".into(),
            Token::RBrace => "}".into(),
            Token::Comma => ", ".into(),
            Token::Semi => ";".into(),
            Token::At => "@".into(),
            Token::Dot => ".".into(),
            Token::Question => "?".into(),
            Token::Eof => String::new(),
        }
    }

    fn expect_name(&mut self, what: &str) -> Result<String, ParseError> {
        match self.cur() {
            Token::Ident(name) => {
                self.pos += 1;
                Ok(name)
            }
            other => Err(ParseError {
                message: format!("{what} erwartet, fand {:?}", other),
            }),
        }
    }

    /// Feldname, der auch ein Schluesselwort sein darf.
    ///
    /// Im Korpus heissen Strukturfelder `state`, `type` oder `const` (23
    /// Dateien). Das ist kein Konflikt: an dieser Stelle kann nur ein Name
    /// stehen, ein Schluesselwort hat dort keine andere Bedeutung.
    fn expect_field_name(&mut self, what: &str) -> Result<String, ParseError> {
        match self.cur() {
            Token::Ident(name) => {
                self.pos += 1;
                Ok(name)
            }
            other => match keyword_text(&other) {
                Some(word) => {
                    self.pos += 1;
                    Ok(word.to_string())
                }
                None => Err(ParseError {
                    message: format!("{what} erwartet, fand {:?}", other),
                }),
            },
        }
    }

    fn use_decl(&mut self) -> Result<Stmt, ParseError> {
        self.pos += 1;
        let mut path = self.expect_name("Importpfad")?;
        while self.cur() == Token::Colon {
            self.pos += 1;
            if self.cur() != Token::Colon {
                return Err(ParseError {
                    message: "':' erwartet in Importpfad '::'".into(),
                });
            }
            self.pos += 1;
            // Gruppenimport: `use ATC::Net::{TCPSocket, UDPSocket}`.
            if self.cur() == Token::LBrace {
                self.pos += 1;
                let mut names = Vec::new();
                while self.cur() != Token::RBrace && self.cur() != Token::Eof {
                    names.push(self.expect_name("Gruppenimport-Name")?);
                    if self.cur() == Token::Comma {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                if self.cur() != Token::RBrace {
                    return Err(ParseError {
                        message: "'}' nach Gruppenimport erwartet".into(),
                    });
                }
                self.pos += 1;
                if self.cur() == Token::Semi {
                    self.pos += 1;
                }
                return Ok(Stmt::UseGroup { path, names });
            }
            path.push_str("::");
            path.push_str(&self.expect_name("Importpfadsegment")?);
        }
        if self.cur() == Token::Semi {
            self.pos += 1;
        }
        Ok(Stmt::Use { path })
    }

    /// Eine Deklaration auf Programmebene oder im Modulblock.
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        // `pub fn` / `pub struct` steht in 106 Korpusdateien. ATCLang kennt
        // keine Sichtbarkeitsebenen, das Wort hat also keine Bedeutung; es
        // wird angenommen und nicht in die AST geschrieben — so bleiben die
        // kanonischen Vertragsbytes und alle ABI-Hashes unveraendert.
        if self.cur() == Token::Pub {
            self.pos += 1;
        }
        match self.cur() {
            Token::Let | Token::Const => Ok(Stmt::Let(self.let_stmt()?)),
            Token::Fn => self.function_decl(),
            Token::Use => self.use_decl(),
            Token::Import => self.import_decl(),
            Token::Struct => self.struct_decl(),
            Token::Enum => self.enum_decl(),
            Token::Contract => self.contract_decl(),
            // `event Transfer { ... }` steht in 42 Korpusdateien auf Programm-
            // ebene, nicht nur im Vertragskoerper. `event` bleibt Bezeichner
            // (kein Schluesselwort), damit Variablen dieses Namens weiter
            // erlaubt sind.
            Token::Ident(name) if name == "event" => Ok(self.event_decl()?),
            other => Err(ParseError {
                message: format!("Deklaration am Top-Level erwartet, fand {:?}", other),
            }),
        }
    }

    /// `module NAME` oder `module NAME { deklaration* }`. Der Block ist reine
    /// Gruppierung: seine Deklarationen behalten Programmebene, damit ein
    /// Modul keine zweite Namensaufloesung einfuehrt.
    fn module_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.pos += 1;
        let mut name = self.expect_name("Modulname")?;
        // `module ATC.OS.Kernel` — punktierte Namen stehen im Korpus. Der
        // Punkt ist Namensbestandteil, keine Feldauswahl.
        while self.cur() == Token::Dot {
            self.pos += 1;
            name.push('.');
            name.push_str(&self.expect_name("Modulsegment")?);
        }
        // `module foo[AD-49] {` — der Klammerzusatz ist ein Governance-Verweis
        // auf das Entscheidungsregister, kein Sprachbestandteil. Annehmen und
        // verwerfen; ein fehlendes `]` bleibt ein Fehler.
        if self.cur() == Token::LBracket {
            while self.cur() != Token::RBracket && self.cur() != Token::Eof {
                self.pos += 1;
            }
            if self.cur() != Token::RBracket {
                return Err(ParseError {
                    message: "']' nach Modul-Tag erwartet".into(),
                });
            }
            self.pos += 1;
        }
        let mut out = vec![Stmt::Module { name }];
        if self.cur() == Token::Semi {
            self.pos += 1;
        }
        if self.cur() == Token::LBrace {
            self.pos += 1;
            while self.cur() != Token::RBrace && self.cur() != Token::Eof {
                if self.cur() == Token::At {
                    self.skip_file_annotations();
                    continue;
                }
                out.push(self.declaration()?);
            }
            if self.cur() != Token::RBrace {
                return Err(ParseError {
                    message: "'}' nach module erwartet".into(),
                });
            }
            self.pos += 1;
        }
        Ok(out)
    }

    /// `import` in den beiden Korpusformen: `import "std/crypto.atc" as Crypto`
    /// und `import ATC::OS::Kernel`. Die Pfadform ist semantisch ein `use`.
    fn import_decl(&mut self) -> Result<Stmt, ParseError> {
        self.pos += 1;
        if let Token::String(source) = self.cur() {
            self.pos += 1;
            let alias = if self.cur() == Token::As {
                self.pos += 1;
                Some(self.expect_name("Import-Alias")?)
            } else {
                None
            };
            if self.cur() == Token::Semi {
                self.pos += 1;
            }
            return Ok(Stmt::Import { source, alias });
        }
        // Pfadform: use_decl erwartet, dass das Schluesselwort noch steht.
        self.pos -= 1;
        self.use_decl()
    }

    fn struct_decl(&mut self) -> Result<Stmt, ParseError> {
        let decl_col = self.col();
        self.pos += 1;
        let name = self.expect_name("Struct-Name")?;
        // Eingerueckte Form `struct X:` mit Feldern je Zeile.
        if self.cur() == Token::Colon {
            self.pos += 1;
            let mut fields = Vec::new();
            while self.at_indented_line(decl_col) {
                let field = self.expect_field_name("Struct-Feld")?;
                if self.cur() != Token::Colon {
                    return Err(ParseError {
                        message: "':' nach Struct-Feld erwartet".into(),
                    });
                }
                self.pos += 1;
                let ty = self.simple_type()?;
                fields.push((field, ty));
                match self.cur() {
                    Token::Comma | Token::Semi => self.pos += 1,
                    _ => {}
                }
            }
            if self.cur() == Token::Semi {
                self.pos += 1;
            }
            return Ok(Stmt::Struct { name, fields });
        }
        if self.cur() != Token::LBrace {
            return Err(ParseError {
                message: "'{' nach struct erwartet".into(),
            });
        }
        self.pos += 1;
        let mut fields = Vec::new();
        while self.cur() != Token::RBrace {
            if self.cur() == Token::Eof {
                return Err(ParseError {
                    message: "'}' in struct erwartet".into(),
                });
            }
            let field = self.expect_field_name("Struct-Feld")?;
            if self.cur() != Token::Colon {
                return Err(ParseError {
                    message: "':' nach Struct-Feld erwartet".into(),
                });
            }
            self.pos += 1;
            let ty = self.simple_type()?;
            fields.push((field, ty));
            match self.cur() {
                Token::Comma | Token::Semi => self.pos += 1,
                Token::RBrace => {}
                // Felder ohne Komma, nur durch Zeilenumbruch getrennt.
                _ if self.at_field_start() => {}
                other => {
                    return Err(ParseError {
                        message: format!(
                            "',' oder '}}' nach Struct-Feld erwartet, fand {:?}",
                            other
                        ),
                    })
                }
            }
        }
        self.pos += 1;
        if self.cur() == Token::Semi {
            self.pos += 1;
        }
        Ok(Stmt::Struct { name, fields })
    }

    fn enum_decl(&mut self) -> Result<Stmt, ParseError> {
        let decl_col = self.col();
        self.pos += 1;
        let name = self.expect_name("Enum-Name")?;
        // Eingerueckte Form `enum X:`, eine Variante je Zeile.
        if self.cur() == Token::Colon {
            self.pos += 1;
            let mut variants = Vec::new();
            while self.at_indented_line(decl_col) && matches!(self.cur(), Token::Ident(_)) {
                variants.push(self.expect_name("Enum-Variante")?);
                if self.cur() == Token::LParen {
                    let mut depth = 0usize;
                    while self.cur() != Token::Eof {
                        let token = self.cur();
                        self.pos += 1;
                        if token == Token::LParen {
                            depth += 1;
                        }
                        if token == Token::RParen {
                            depth = depth.saturating_sub(1);
                            if depth == 0 {
                                break;
                            }
                        }
                    }
                }
                match self.cur() {
                    Token::Comma | Token::Semi => self.pos += 1,
                    Token::Assign => {
                        self.pos += 1;
                        self.expr()?;
                    }
                    _ => {}
                }
            }
            if self.cur() == Token::Semi {
                self.pos += 1;
            }
            return Ok(Stmt::Enum { name, variants });
        }
        if self.cur() != Token::LBrace {
            return Err(ParseError {
                message: "'{' nach enum erwartet".into(),
            });
        }
        self.pos += 1;
        let mut variants = Vec::new();
        while self.cur() != Token::RBrace {
            if self.cur() == Token::Eof {
                return Err(ParseError {
                    message: "'}' in enum erwartet".into(),
                });
            }
            variants.push(self.expect_name("Enum-Variante")?);
            if self.cur() == Token::LParen {
                let mut depth = 0usize;
                while self.cur() != Token::Eof {
                    let token = self.cur();
                    self.pos += 1;
                    if token == Token::LParen {
                        depth += 1;
                    }
                    if token == Token::RParen {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            break;
                        }
                    }
                }
            }
            match self.cur() {
                Token::Comma | Token::Semi => self.pos += 1,
                Token::RBrace => {}
                other => {
                    return Err(ParseError {
                        message: format!(
                            "',' oder '}}' nach Enum-Variante erwartet, fand {:?}",
                            other
                        ),
                    })
                }
            }
        }
        self.pos += 1;
        if self.cur() == Token::Semi {
            self.pos += 1;
        }
        Ok(Stmt::Enum { name, variants })
    }

    fn contract_decl(&mut self) -> Result<Stmt, ParseError> {
        self.pos += 1;
        let name = self.expect_name("Contract-Name")?;
        let mut bases = Vec::new();
        if self.cur() == Token::Colon {
            self.pos += 1;
            loop {
                bases.push(self.expect_name("Contract-Basis")?);
                while self.cur() == Token::Colon {
                    self.pos += 1;
                    if self.cur() != Token::Colon {
                        return Err(ParseError {
                            message: "'::' erwartet im Contract-Basispfad".into(),
                        });
                    }
                    self.pos += 1;
                    let segment = self.expect_name("Contract-Basispfadsegment")?;
                    bases.last_mut().unwrap().push_str("::");
                    bases.last_mut().unwrap().push_str(&segment);
                }
                if self.cur() != Token::Comma {
                    break;
                }
                self.pos += 1;
            }
        }
        if self.cur() != Token::LBrace {
            return Err(ParseError {
                message: "'{' nach contract erwartet".into(),
            });
        }
        self.pos += 1;
        let mut body = Vec::new();
        while self.cur() != Token::RBrace {
            if self.cur() == Token::Eof {
                return Err(ParseError {
                    message: "Nicht geschlossener contract-Block".into(),
                });
            }
            match self.contract_item() {
                Ok(items) => body.extend(items),
                Err(e) => self.failed(e)?,
            }
        }
        self.pos += 1; // schliessende '}'
        Ok(Stmt::Contract { name, bases, body })
    }

    /// Item im Contract-Koerper: state/event/fn/const/struct/enum/use.
    fn contract_item(&mut self) -> Result<Vec<Stmt>, ParseError> {
        // `pub fn` auch hier — gleiche Bedeutungslosigkeit wie am Top-Level.
        if self.cur() == Token::Pub {
            self.pos += 1;
        }
        match self.cur() {
            // Im Contract gibt es keinen Native-Fallback: der Koerper MUSS parsen.
            Token::Fn | Token::Function => Ok(vec![Stmt::Fn(self.function()?)]),
            Token::Const | Token::Let => Ok(vec![Stmt::Let(self.let_stmt()?)]),
            Token::Struct => Ok(vec![self.struct_decl()?]),
            Token::Enum => Ok(vec![self.enum_decl()?]),
            Token::Use => Ok(vec![self.use_decl()?]),
            Token::State => self.state_decl(),
            Token::Ident(name) if name == "event" => Ok(vec![self.event_decl()?]),
            // Annotation wie `@capability mint` ist Metadatum vor dem Item.
            Token::At => {
                self.pos += 1;
                while self.cur() != Token::Eof {
                    match self.cur() {
                        Token::Fn
                        | Token::Function
                        | Token::State
                        | Token::Struct
                        | Token::Enum
                        | Token::Use
                        | Token::Const
                        | Token::Let
                        | Token::RBrace => break,
                        Token::Ident(name) if name == "event" => break,
                        _ => self.pos += 1,
                    }
                }
                Ok(Vec::new())
            }
            other => Err(ParseError {
                message: format!(
                    "Deklaration im contract erwartet, fand {:?} (Umfeld: {})",
                    other,
                    self.context_window()
                ),
            }),
        }
    }

    /// `state name: Typ [= default]` oder Blockform `state { a: T = v; ... }`.
    fn state_decl(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.pos += 1; // 'state'
        if self.cur() == Token::LBrace {
            self.pos += 1;
            let mut fields = Vec::new();
            while self.cur() != Token::RBrace {
                let name = self.expect_name("Feldname im state-Block")?;
                if self.cur() != Token::Colon {
                    return Err(ParseError {
                        message: format!("':' nach state-Feld {name} erwartet"),
                    });
                }
                self.pos += 1;
                let ty = self.simple_type()?;
                let default = if self.cur() == Token::Assign {
                    self.pos += 1;
                    Some(self.expr()?)
                } else {
                    None
                };
                if self.cur() == Token::Semi {
                    self.pos += 1;
                }
                fields.push((name, ty, default));
            }
            self.pos += 1; // '}'
            return Ok(vec![Stmt::StateBlock { fields }]);
        }
        let name = self.expect_name("Zustandsname nach 'state'")?;
        if self.cur() != Token::Colon {
            return Err(ParseError {
                message: format!("':' nach state {name} erwartet"),
            });
        }
        self.pos += 1;
        let ty = self.simple_type()?;
        let default = if self.cur() == Token::Assign {
            self.pos += 1;
            Some(self.expr()?)
        } else {
            None
        };
        Ok(vec![Stmt::State { name, ty, default }])
    }

    /// `event Name(feld: Typ, ...)`
    fn event_decl(&mut self) -> Result<Stmt, ParseError> {
        self.pos += 1; // 'event'
        let name = self.expect_name("Event-Name nach 'event'")?;
        let mut fields = Vec::new();
        if self.cur() == Token::LParen {
            self.pos += 1;
            while self.cur() != Token::RParen {
                let field = self.expect_name("Feldname im Event")?;
                if self.cur() != Token::Colon {
                    return Err(ParseError {
                        message: format!("':' nach Event-Feld {field} erwartet"),
                    });
                }
                self.pos += 1;
                let ty = self.simple_type()?;
                fields.push((field, ty));
                if self.cur() == Token::Comma {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            if self.cur() != Token::RParen {
                return Err(ParseError {
                    message: format!("')' nach Event {name} erwartet"),
                });
            }
            self.pos += 1;
        }
        // Blockform `event Transfer { from: Address, to: Address }` — im Korpus
        // die Regelform, waehrend die Klammerform oben die Ausnahme ist.
        if self.cur() == Token::LBrace {
            self.pos += 1;
            while self.cur() != Token::RBrace {
                if self.cur() == Token::Eof {
                    return Err(ParseError {
                        message: format!("'}}' nach Event {name} erwartet"),
                    });
                }
                let field = self.expect_field_name("Feldname im Event")?;
                if self.cur() != Token::Colon {
                    return Err(ParseError {
                        message: format!("':' nach Event-Feld {field} erwartet"),
                    });
                }
                self.pos += 1;
                let ty = self.simple_type()?;
                fields.push((field, ty));
                match self.cur() {
                    Token::Comma | Token::Semi => self.pos += 1,
                    Token::RBrace => {}
                    _ if self.at_field_start() => {}
                    other => {
                        return Err(ParseError {
                            message: format!(
                                "',' oder '}}' nach Event-Feld erwartet, fand {:?}",
                                other
                            ),
                        })
                    }
                }
            }
            self.pos += 1;
        }
        Ok(Stmt::Event { name, fields })
    }

    fn skip_file_annotations(&mut self) {
        while self.cur() != Token::Eof {
            if matches!(
                self.cur(),
                Token::Let
                    | Token::Const
                    | Token::Fn
                    | Token::Use
                    | Token::Struct
                    | Token::Enum
                    | Token::Contract
            ) {
                break;
            }
            self.pos += 1;
        }
    }

    fn let_stmt(&mut self) -> Result<LetStmt, ParseError> {
        let is_const = self.cur() == Token::Const;
        self.pos += 1;
        let name = match self.cur() {
            Token::Ident(n) => n,
            other => {
                return Err(ParseError {
                    message: format!("Name erwartet, fand {:?}", other),
                })
            }
        };
        self.pos += 1;
        let mut type_hint = None;
        if self.cur() == Token::Colon {
            self.pos += 1;
            type_hint = Some(self.simple_type()?);
        }
        let mut value = None;
        if self.cur() == Token::Assign {
            self.pos += 1;
            value = Some(self.expr()?);
        }
        if self.cur() == Token::Semi {
            self.pos += 1;
        }
        Ok(LetStmt {
            name,
            is_const,
            type_hint,
            value,
        })
    }

    fn function(&mut self) -> Result<FunctionDef, ParseError> {
        self.pos += 1; // 'fn' oder 'function'
        let name = match self.cur() {
            Token::Ident(n) => n,
            other => {
                return Err(ParseError {
                    message: format!("Fn-Name erwartet, fand {:?}", other),
                })
            }
        };
        self.pos += 1;
        if self.cur() != Token::LParen {
            return Err(ParseError {
                message: "'(' erwartet".to_string(),
            });
        }
        self.pos += 1;
        let mut params = Vec::new();
        while self.cur() != Token::RParen {
            let pname = match self.cur() {
                Token::Ident(n) => n,
                // Schluesselwoerter als Parameternamen (z. B. `capability: String`);
                // die Position rueckt erst in der naechsten Zeile weiter.
                Token::Capability => "capability".to_string(),
                Token::State => "state".to_string(),
                Token::Function => "function".to_string(),
                Token::Policy => "policy".to_string(),
                other => {
                    return Err(ParseError {
                        message: format!("Parametername erwartet, fand {:?}", other),
                    })
                }
            };
            self.pos += 1;
            if self.cur() != Token::Colon {
                return Err(ParseError {
                    message: format!(
                        "Parameter-Typ ist Pflicht (': typ'), Referenz-Regel; Token {}; Umfeld: {}",
                        self.pos,
                        self.context_window()
                    ),
                });
            }
            self.pos += 1;
            let t = self.simple_type()?;
            params.push(Param {
                name: pname,
                type_hint: t,
            });
            if self.cur() == Token::Comma {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.cur() != Token::RParen {
            return Err(ParseError {
                message: format!("\")\" erwartet (Umfeld: {})", self.context_window()),
            });
        }
        self.pos += 1;
        let mut return_type = None;
        if self.cur() == Token::Arrow {
            self.pos += 1;
            return_type = Some(self.simple_type()?);
        }
        if self.cur() != Token::LBrace {
            return Err(ParseError {
                message: "'{' erwartet".to_string(),
            });
        }
        self.pos += 1;
        let body = self.block()?;
        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while self.cur() != Token::RBrace && self.cur() != Token::Eof {
            stmts.push(self.statement()?);
        }
        if self.cur() != Token::RBrace {
            return Err(ParseError {
                message: "'}' erwartet".to_string(),
            });
        }
        self.pos += 1;
        Ok(stmts)
    }

    /// Bezeichner, der auch Schluesselwoerter akzeptiert (Namen wie `capability`).
    fn context_window(&self) -> String {
        let from = self.pos.saturating_sub(4);
        let to = (self.pos + 3).min(self.toks.len());
        self.toks[from..to]
            .iter()
            .map(Self::token_text)
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Prueft, ob nach dem aktuellen '{' ein Struct-Literal folgt.
    fn struct_literal_ahead(&self) -> bool {
        if self.cur() != Token::LBrace {
            return false;
        }
        match self.toks.get(self.pos + 1) {
            Some(Token::RBrace) => true,
            Some(Token::Ident(_)) => self.toks.get(self.pos + 2) == Some(&Token::Colon),
            _ => false,
        }
    }

    /// Argumentliste einer Call-Syntax; setzt voraus, dass '('-Token kommt.
    fn call_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        if self.cur() != Token::LParen {
            return Err(ParseError {
                message: "'(' erwartet".into(),
            });
        }
        self.pos += 1;
        let mut args = Vec::new();
        if self.cur() != Token::RParen {
            loop {
                args.push(self.expr()?);
                if self.cur() == Token::Comma {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        if self.cur() != Token::RParen {
            return Err(ParseError {
                message: format!("\")\" erwartet (Umfeld: {})", self.context_window()),
            });
        }
        self.pos += 1;
        Ok(args)
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        match self.cur() {
            Token::Let | Token::Const => Ok(Stmt::Let(self.let_stmt()?)),
            Token::If => self.if_stmt(),
            // `for` bleibt Bezeichner (wie `event`), damit Quellen mit `for`
            // als Namen nicht brechen; an Statement-Position ist es die
            // Schleife aus SPEC 3.2.
            Token::Ident(name) if name == "for" => self.for_stmt(),
            Token::While => self.while_stmt(),
            Token::Return => {
                self.pos += 1;
                let value = if self.cur() == Token::RBrace || self.cur() == Token::Eof {
                    None
                } else {
                    Some(self.expr()?)
                };
                if self.cur() == Token::Semi {
                    self.pos += 1;
                }
                Ok(Stmt::Return { value })
            }
            Token::Ident(name) if name == "emit" => {
                self.pos += 1;
                let mut event = self.expect_name("Event-Name nach 'emit'")?;
                while self.cur() == Token::Colon
                    && self.toks.get(self.pos + 1) == Some(&Token::Colon)
                {
                    self.pos += 2;
                    event.push_str("::");
                    event.push_str(&self.expect_name("Pfadsegment nach '::'")?);
                }
                let args = if self.cur() == Token::LParen {
                    self.call_args()?
                } else {
                    Vec::new()
                };
                if self.cur() == Token::Semi {
                    self.pos += 1;
                }
                Ok(Stmt::Emit { event, args })
            }
            Token::Require => {
                self.pos += 1;
                let args = self.call_args()?;
                if self.cur() == Token::Semi {
                    self.pos += 1;
                }
                let mut it = args.into_iter();
                let cond = it.next().ok_or_else(|| ParseError {
                    message: "require ohne Bedingung".into(),
                })?;
                Ok(Stmt::Require {
                    cond,
                    message: it.next(),
                })
            }
            _ => {
                let e = self.expr()?;
                let op = match self.cur() {
                    Token::Assign => Some(None),
                    Token::Plus | Token::Minus | Token::Star
                        if self.toks.get(self.pos + 1) == Some(&Token::Assign) =>
                    {
                        Some(Some(
                            match self.cur() {
                                Token::Plus => "+",
                                Token::Minus => "-",
                                _ => "*",
                            }
                            .to_string(),
                        ))
                    }
                    _ => None,
                };
                if let Some(op) = op {
                    self.pos += if op.is_some() { 2 } else { 1 };
                    let value = self.expr()?;
                    if self.cur() == Token::Semi {
                        self.pos += 1;
                    }
                    return Ok(Stmt::Assign {
                        target: e,
                        op,
                        value,
                    });
                }
                if self.cur() == Token::Semi {
                    self.pos += 1;
                }
                Ok(Stmt::Expr(e))
            }
        }
    }

    /// if <expr> { ... } [else { ... } | else if ...] — SCR-0128 Stufe 1.
    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.pos += 1; // 'if'
        let cond = self.expr()?;
        if self.cur() != Token::LBrace {
            return Err(ParseError {
                message: "'{' nach if-Bedingung erwartet".to_string(),
            });
        }
        self.pos += 1;
        let then_body = self.block()?;
        let else_body = if self.cur() == Token::Else {
            self.pos += 1;
            match self.cur() {
                Token::LBrace => {
                    self.pos += 1;
                    Some(self.block()?)
                }
                // else if = else { if ... } (Zucker, kanonisch verschachtelt)
                Token::If => Some(vec![self.if_stmt()?]),
                other => {
                    return Err(ParseError {
                        message: format!("'{{' oder 'if' nach else erwartet, fand {:?}", other),
                    })
                }
            }
        } else {
            None
        };
        Ok(Stmt::If {
            cond,
            then_body,
            else_body,
        })
    }

    /// while <expr> { ... } — SCR-0128 Stufe 1.
    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.pos += 1; // 'while'
        let cond = self.expr()?;
        if self.cur() != Token::LBrace {
            return Err(ParseError {
                message: "'{' nach while-Bedingung erwartet".to_string(),
            });
        }
        self.pos += 1;
        let body = self.block()?;
        Ok(Stmt::While { cond, body })
    }

    /// `for IDENT in <bereich> { ... }` — SPEC 3.2: for_stmt.
    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.pos += 1; // 'for'
        let var = self.expect_name("Schleifenvariable nach 'for'")?;
        // `in` bleibt ebenfalls Bezeichner (SPEC-Keywords, aber im Korpus auch
        // als Name verwendet).
        match self.cur() {
            Token::Ident(name) if name == "in" => self.pos += 1,
            _ => {
                return Err(ParseError {
                    message: "'in' nach for-Variable erwartet".into(),
                })
            }
        }
        let iter = self.expr()?;
        let iter = match iter {
            Expr::Range { .. } => iter,
            other => {
                return Err(ParseError {
                    message: format!("for erwartet einen Bereich a..b, fand {other:?}"),
                })
            }
        };
        if self.cur() != Token::LBrace {
            return Err(ParseError {
                message: "'{' nach for-Bereich erwartet".into(),
            });
        }
        self.pos += 1;
        let body = self.block()?;
        Ok(Stmt::For { var, iter, body })
    }

    fn expr(&mut self) -> Result<Expr, ParseError> {
        self.range_expr()
    }

    /// Bereich `a..b`, `a..=b`, `a..`, `..b` — SPEC 3.4: RangeExpr.
    ///
    /// Die SPEC nennt `Range` unter `parse_primary`. Fuer `0..n` muss der
    /// Bereich aber schwaecher binden als `n.len()`, sonst wird das Ende
    /// abgeschnitten; deshalb liegt die Stufe hier auf `expr`-Ebene, direkt
    /// ueber `||`. Die Knotenform selbst folgt der SPEC.
    fn range_expr(&mut self) -> Result<Expr, ParseError> {
        let starts_with_range = matches!(self.cur(), Token::DotDot | Token::DotDotEq);
        let start = if starts_with_range {
            None
        } else {
            Some(Box::new(self.logic_or()?))
        };
        let inclusive = match self.cur() {
            Token::DotDot => false,
            Token::DotDotEq => true,
            _ if starts_with_range => {
                return Err(ParseError {
                    message: "Bereich '..' ohne Start und Ende ist nicht erlaubt".into(),
                })
            }
            _ => return Ok(*start.expect("Start vorhanden")),
        };
        self.pos += 1;
        // Offenes Ende: `xs[a..]`, `for i in a..` (dort unten abgelehnt).
        let end = if matches!(
            self.cur(),
            Token::RBracket
                | Token::RParen
                | Token::RBrace
                | Token::Comma
                | Token::Semi
                | Token::LBrace
                | Token::Eof
        ) {
            None
        } else {
            Some(Box::new(self.logic_or()?))
        };
        if start.is_none() && end.is_none() {
            return Err(ParseError {
                message: "Bereich '..' ohne Start und Ende ist nicht erlaubt".into(),
            });
        }
        Ok(Expr::Range {
            start,
            end,
            inclusive,
        })
    }

    /// `||` (und einzelnes `|` als bitor) — unterster Rang, linksassoziativ.
    fn logic_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.logic_and()?;
        loop {
            let pipe = self.cur() == Token::Pipe;
            if !pipe {
                break;
            }
            // `||` und `|` werden beide als Pipe-Token(s) gelesen.
            self.pos += 1;
            let double = self.cur() == Token::Pipe;
            if double {
                self.pos += 1;
            }
            let right = self.logic_and()?;
            left = Expr::Binary {
                op: if double { "||" } else { "|" }.to_string(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// `&&` (und einzelnes `&` als bitand).
    fn logic_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.comparison()?;
        loop {
            if self.cur() != Token::Ampersand {
                break;
            }
            self.pos += 1;
            let double = self.cur() == Token::Ampersand;
            if double {
                self.pos += 1;
            }
            let right = self.comparison()?;
            left = Expr::Binary {
                op: if double { "&&" } else { "&" }.to_string(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Vergleiche: linksassoziativ, unter der Addition (SCR-0128 Stufe 1).
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.addition()?;
        loop {
            let op = match self.cur() {
                Token::Eq => "==",
                Token::NotEq => "!=",
                Token::Lt => "<",
                Token::Gt => ">",
                Token::LtEq => "<=",
                Token::GtEq => ">=",
                _ => break,
            };
            self.pos += 1;
            let right = self.addition()?;
            left = Expr::Binary {
                op: op.to_string(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn addition(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.multiplication()?;
        loop {
            // `a += 1` ist eine Zuweisung, kein Binaerausdruck.
            if self.toks.get(self.pos + 1) == Some(&Token::Assign) {
                break;
            }
            let op = match self.cur() {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => break,
            };
            self.pos += 1;
            let right = self.multiplication()?;
            left = Expr::Binary {
                op: op.to_string(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn multiplication(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.unary()?;
        loop {
            let op = match self.cur() {
                Token::Star => "*",
                Token::Slash => "/",
                _ => break,
            };
            self.pos += 1;
            let right = self.unary()?;
            left = Expr::Binary {
                op: op.to_string(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.cur() == Token::Bang {
            self.pos += 1;
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                op: "!".to_string(),
                operand: Box::new(operand),
            });
        }
        if self.cur() == Token::Minus {
            self.pos += 1;
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                op: "-".to_string(),
                operand: Box::new(operand),
            });
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, ParseError> {
        let mut node = self.primary()?;
        loop {
            match self.cur() {
                // Pfad `A::b` — wird als ein Name "A::b" gefuehrt.
                Token::Colon
                    if self
                        .toks
                        .get(self.pos + 1)
                        .is_some_and(|t| *t == Token::Colon) =>
                {
                    let Some(base) = path_text(&node) else {
                        return Err(ParseError {
                            message: "Pfadoperator '::' nur nach einem Namen erlaubt".into(),
                        });
                    };
                    self.pos += 2;
                    let segment = self.expect_name("Pfadsegment nach '::'")?;
                    node = Expr::Ident(format!("{base}::{segment}"));
                }
                Token::Dot => {
                    self.pos += 1;
                    let name = self.expect_field_name("Feldname nach '.'")?;
                    node = Expr::Field {
                        base: Box::new(node),
                        name,
                    };
                }
                Token::LBracket => {
                    self.pos += 1;
                    let index = self.expr()?;
                    if self.cur() != Token::RBracket {
                        return Err(ParseError {
                            message: "']' nach Index erwartet".into(),
                        });
                    }
                    self.pos += 1;
                    // SPEC 3.4: ein Bereich im Index ist eine SliceExpr,
                    // kein IndexAccess mit Bereichsindex.
                    node = match index {
                        Expr::Range {
                            start,
                            end,
                            inclusive,
                        } => Expr::Slice {
                            base: Box::new(node),
                            start,
                            end,
                            inclusive,
                        },
                        index => Expr::Index {
                            base: Box::new(node),
                            index: Box::new(index),
                        },
                    };
                }
                // Struct-Literal `Name { feld: wert }` — nur wenn der Lookahead
                // eindeutig ist (sonst waere `if name {` ein Block).
                Token::LBrace if path_text(&node).is_some() && self.struct_literal_ahead() => {
                    let Some(name) = path_text(&node) else {
                        unreachable!("Lookahead garantiert einen Namen");
                    };
                    self.pos += 1;
                    let mut fields = Vec::new();
                    while self.cur() != Token::RBrace {
                        let field = self.expect_field_name("Feldname im Struct-Literal")?;
                        if self.cur() != Token::Colon {
                            return Err(ParseError {
                                message: format!(
                                    "':' nach Feld {field} im Struct-Literal erwartet"
                                ),
                            });
                        }
                        self.pos += 1;
                        let value = self.expr()?;
                        fields.push((field, value));
                        if self.cur() == Token::Comma {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    if self.cur() != Token::RBrace {
                        return Err(ParseError {
                            message: "}' nach Struct-Literal erwartet".into(),
                        });
                    }
                    self.pos += 1;
                    node = Expr::StructLiteral { name, fields };
                }
                Token::LParen => {
                    let args = self.call_args()?;
                    node = Expr::Call {
                        target: Box::new(node),
                        args,
                    };
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.cur() {
            Token::Int(v) => {
                self.pos += 1;
                if v <= i64::MAX as u128 {
                    Ok(Expr::Int(v as i64))
                } else {
                    Ok(Expr::BigInt(v))
                }
            }
            Token::String(value) => {
                self.pos += 1;
                Ok(Expr::String(value))
            }
            Token::Ident(n) => {
                self.pos += 1;
                Ok(Expr::Ident(n))
            }
            // Schluesselwoerter als Wert (z. B. `xs[i] == capability`).
            Token::Capability => {
                self.pos += 1;
                Ok(Expr::Ident("capability".to_string()))
            }
            Token::State => {
                self.pos += 1;
                Ok(Expr::Ident("state".to_string()))
            }
            Token::Policy => {
                self.pos += 1;
                Ok(Expr::Ident("policy".to_string()))
            }
            Token::LBrace => {
                self.pos += 1;
                let mut entries = Vec::new();
                while self.cur() != Token::RBrace {
                    let key = self.expr()?;
                    if self.cur() != Token::Colon {
                        return Err(ParseError {
                            message: "':' in Map-Literal erwartet".to_string(),
                        });
                    }
                    self.pos += 1;
                    let value = self.expr()?;
                    entries.push((key, value));
                    if self.cur() == Token::Comma {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                if self.cur() != Token::RBrace {
                    return Err(ParseError {
                        message: "}' nach Map-Literal erwartet".to_string(),
                    });
                }
                self.pos += 1;
                Ok(Expr::MapLiteral(entries))
            }
            Token::LBracket => {
                self.pos += 1;
                let mut items = Vec::new();
                if self.cur() != Token::RBracket {
                    loop {
                        items.push(self.expr()?);
                        if self.cur() == Token::Comma {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                }
                if self.cur() != Token::RBracket {
                    return Err(ParseError {
                        message: "']' nach Array-Literal erwartet".to_string(),
                    });
                }
                self.pos += 1;
                Ok(Expr::Array(items))
            }
            Token::LParen => {
                self.pos += 1;
                let e = self.expr()?;
                if self.cur() == Token::Comma {
                    let mut items = vec![e];
                    while self.cur() == Token::Comma {
                        self.pos += 1;
                        if self.cur() == Token::RParen {
                            break;
                        }
                        items.push(self.expr()?);
                    }
                    if self.cur() != Token::RParen {
                        return Err(ParseError {
                            message: format!(
                                "\")\" nach Tupel erwartet (Umfeld: {})",
                                self.context_window()
                            ),
                        });
                    }
                    self.pos += 1;
                    return Ok(Expr::Tuple(items));
                }
                if self.cur() != Token::RParen {
                    return Err(ParseError {
                        message: format!("\")\" erwartet (Umfeld: {})", self.context_window()),
                    });
                }
                self.pos += 1;
                Ok(e)
            }
            other => Err(ParseError {
                message: format!(
                    "Ausdruck erwartet, fand {:?} (Token {}; Umfeld: {})",
                    other,
                    self.pos,
                    self.context_window()
                ),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn praecedenz_und_assoziativitaet() {
        let p = parse_program("let r = 1 + 2 * 3 - 4 / 5;").unwrap();
        let j = p.to_json();
        assert!(j.contains("\"op\":\"*\""));
        assert!(j.contains("\"op\":\"-\""));
    }

    #[test]
    fn fn_definition_mit_return() {
        let p = parse_program("fn add(a: u64, b: u64) -> u64 { return a + b; }").unwrap();
        let j = p.to_json();
        assert!(j.contains("\"kind\":\"FunctionDef\""));
        assert!(j.contains("\"kind\":\"ReturnStatement\""));
        assert!(j.contains("\"kind\":\"Parameter\""));
        assert!(j.contains("\"return_type\":{\"kind\":\"TypeAnnotation\",\"name\":\"u64\""));
    }

    #[test]
    fn fn_body_let_expr_bare_return() {
        let p = parse_program("fn f(x: u64) { let y = x; add(y, 1); return }").unwrap();
        let j = p.to_json();
        assert!(j.contains("\"kind\":\"LetStatement\""));
        assert!(j.contains("\"kind\":\"ExprStatement\""));
        assert!(j.contains("\"value\":null"));
    }

    #[test]
    fn referenz_semantik_return_mit_semi_vor_klammer() {
        // Referenz: 'return;' vor '}' wirft Fehler (parse_expr auf ';')
        assert!(parse_program("fn f() { return; }").is_err());
    }

    #[test]
    fn if_else_und_while_json() {
        let p = parse_program(
            "fn f(x: u64) { if x < 1 { return 1; } else { while x < 2 { let x = x + 1; } } return 0; }",
        )
        .unwrap();
        let j = p.to_json();
        assert!(j.contains("\"kind\":\"IfStatement\""));
        assert!(j.contains("\"kind\":\"WhileStatement\""));
        assert!(j.contains("\"op\":\"<\""));
        // Alphabetische Keys (kanonisch): cond vor else_body vor kind vor then_body
        let idx = j.find("\"kind\":\"IfStatement\"").unwrap();
        assert!(j[..idx].contains("\"cond\":"));
        let idx2 = j.find("\"kind\":\"WhileStatement\"").unwrap();
        assert!(j[..idx2].rfind("\"cond\":").unwrap() > j[..idx2].find("\"body\":").unwrap());
    }

    #[test]
    fn else_if_zucker_verschachtelt() {
        let p = parse_program(
            "fn f(x: u64) { if x < 1 { return 1; } else if x < 2 { return 2; } return 0; }",
        )
        .unwrap();
        let j = p.to_json();
        // Genau EIN else_body mit genau EINEM IfStatement darin
        assert_eq!(j.matches("\"kind\":\"IfStatement\"").count(), 2);
        assert!(j.contains("\"else_body\":[{\"cond\":"));
    }

    #[test]
    fn vergleichs_praezedenz_unter_addition() {
        let p = parse_program("fn f(a: u64) { return a + 1 < 2; }").unwrap();
        let j = p.to_json();
        assert!(j.contains("\"op\":\"<\""));
        assert!(j.find("\"op\":\"<\"").unwrap() > j.find("\"op\":\"+\"").unwrap());
    }

    #[test]
    fn native_import_struct_enum_and_contract_ast() {
        let source = r#"
            use ATC::Types::Address
            enum Status { Open, Closed }
            struct Entry { owner: Address, balances: Map<Address, u128> }
            contract Vault : BaseVault {
                state balances: Map<Address, u128>
                fn transfer(to: Address, amount: u128) -> bool {
                    if amount > 0 { self.balances[to] += amount }
                    return true
                }
            }
        "#;
        let program = parse_program(source).expect("native declarations must parse");
        let json = program.to_json();
        assert!(json.contains("\"kind\":\"UseDeclaration\",\"path\":\"ATC::Types::Address\""));
        assert!(json.contains("\"kind\":\"EnumDeclaration\",\"name\":\"Status\""));
        assert!(json.contains("\"kind\":\"StructDeclaration\",\"name\":\"Entry\""));
        assert!(json.contains("\"name\":\"Map<Address, u128>\""));
        assert!(json.contains("\"kind\":\"ContractDeclaration\",\"name\":\"Vault\""));
        assert!(json.contains("\"bases\":[\"BaseVault\"]"));
    }

    #[test]
    fn funktionskoerper_mit_string_literal_parst_als_ast() {
        // Frueanlegung: dieser Koerper landete frueher als Token-Liste im
        // NativeFunction-Fallback. Heute muss er echt geparst werden.
        let source = r#"
            fn tokenize(s: string) -> ScanResult {
                let tokens: List<string> = []
                if s.len() > 0 && s.slice(0, 1) == "0" {
                    tokens.push("zero")
                }
                return tokens
            }
        "#;
        let program = parse_program(source).expect("Funktionskoerper muss parsen");
        let json = program.to_json();
        assert!(json.contains("\"kind\":\"FunctionDef\""), "{json}");
        assert!(!json.contains("NativeFunctionDef"), "{json}");
        assert!(json.contains("\"&&\""), "Logikoperator fehlt: {json}");
    }

    #[test]
    fn string_literal_is_preserved_in_ast_json() {
        let program = parse_program("const LABEL: string = \"A-Town\"").unwrap();
        assert!(program
            .to_json()
            .contains("\"kind\":\"StringLiteral\",\"value\":\"A-Town\""));
    }

    #[test]
    fn large_contract_integer_is_preserved_without_i64_wraparound() {
        let program = parse_program("const MAX_SUPPLY: u128 = 100_000_000_000_000_000_000_000_000")
            .expect("u128 contract literal must parse");
        assert!(program
            .to_json()
            .contains("\"value\":100000000000000000000000000"));
    }

    #[test]
    fn fehlerfaelle() {
        assert!(parse_program("let $ = 1;").is_err());
        assert!(parse_program("let x = ;").is_err());
        assert!(parse_program("return 42;").is_err()); // Subset: nur let/const/fn am Top-Level
        assert!(parse_program("fn f(a) { }").is_err()); // Parameter-Typ ist Pflicht
    }

    #[test]
    fn hash_kommentar_vor_contract() {
        let p = parse_program("# ATC AD-86\ncontract T { fn f() -> int { return 1; } }").unwrap();
        assert!(p.to_json().contains("\"kind\":\"ContractDeclaration\""));
    }

    #[test]
    fn module_kopf_ohne_block() {
        let j = parse_program("module architect_agent\nfn f() -> int { return 1; }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"ModuleDeclaration\""));
        assert!(j.contains("\"name\":\"architect_agent\""));
        assert!(j.contains("\"kind\":\"FunctionDef\""));
    }

    #[test]
    fn module_block_gruppiert_deklarationen() {
        // Der Block ist reine Gruppierung: seine Deklarationen gelten
        // programmweit, es entsteht kein zweiter Namensraum.
        let j = parse_program("module m {\nstruct S { a: u64 }\nfn f() -> int { return 1; }\n}")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"ModuleDeclaration\""));
        assert!(j.contains("\"kind\":\"StructDeclaration\""));
        assert!(j.contains("\"kind\":\"FunctionDef\""));
    }

    #[test]
    fn module_block_verlangt_schliessende_klammer() {
        assert!(parse_program("module m { fn f() -> int { return 1; }").is_err());
    }

    #[test]
    fn dateiimport_mit_alias() {
        let j = parse_program("import \"std/crypto.atc\" as Crypto")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"FileImportDeclaration\""));
        assert!(j.contains("\"source\":\"std/crypto.atc\""));
        assert!(j.contains("\"alias\":\"Crypto\""));
    }

    #[test]
    fn dateiimport_ohne_alias() {
        let j = parse_program("import \"std/chain.atc\";")
            .unwrap()
            .to_json();
        assert!(j.contains("\"alias\":null"));
    }

    #[test]
    fn import_pfadform_ist_semantisch_use() {
        let j = parse_program("import ATC::OS::Kernel").unwrap().to_json();
        assert!(j.contains("\"kind\":\"UseDeclaration\""));
        assert!(j.contains("\"path\":\"ATC::OS::Kernel\""));
    }

    #[test]
    fn gruppenimport_mit_mehreren_namen() {
        let j = parse_program("use ATC::Net::{ TCPSocket, UDPSocket }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"UseGroupDeclaration\""));
        assert!(j.contains("\"path\":\"ATC::Net\""));
        assert!(j.contains("\"names\":[\"TCPSocket\",\"UDPSocket\"]"));
    }

    #[test]
    fn gruppenimport_mit_einem_namen_ohne_komma() {
        let j = parse_program("use ATC::X::{A}").unwrap().to_json();
        assert!(j.contains("\"names\":[\"A\"]"));
    }

    #[test]
    fn gruppenimport_verlangt_schliessende_klammer() {
        assert!(parse_program("use ATC::X::{A, B").is_err());
    }

    #[test]
    fn pub_ist_bedeutungslos_und_landet_nicht_in_der_ast() {
        // `pub` steht in 106 Korpusdateien. Es gibt keine Sichtbarkeitsebenen,
        // also darf es die AST (und damit jeden ABI-Hash) nicht veraendern.
        let j = parse_program("pub fn f() -> int { return 1; }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"FunctionDef\""));
        // Das AST-Feld `is_pub` ist im Compiler fest auf false gesetzt; der
        // Korpus-Marker darf es nicht kippen, sonst verschoeben sich die
        // ABI-Hashes jeder Datei, die `pub` schreibt.
        assert!(j.contains("\"is_pub\":false"), "{j}");
        assert!(!j.contains("\"name\":\"pub\""));
        assert!(parse_program("pub struct S { a: u64 }").is_ok());
        assert!(parse_program("pub const X: u64 = 1;").is_ok());
    }

    #[test]
    fn pub_im_contract_koerper() {
        let j = parse_program("contract C { pub fn f() -> int { return 1; } }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"FunctionDef\""));
    }

    #[test]
    fn modulname_darf_punktiert_sein() {
        let j = parse_program("module ATC.OS.Kernel\nfn f() -> int { return 1; }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"name\":\"ATC.OS.Kernel\""), "{j}");
    }

    #[test]
    fn modul_tag_wird_verworfen() {
        let j = parse_program("module aistudio[AD-49] {\nfn f() -> int { return 1; }\n}")
            .unwrap()
            .to_json();
        assert!(j.contains("\"name\":\"aistudio\""), "{j}");
        assert!(!j.contains("AD-49"));
    }

    #[test]
    fn modul_tag_ohne_klammer_ist_ein_fehler() {
        assert!(parse_program("module a[AD-49").is_err());
    }

    #[test]
    fn schluesselwort_als_strukt_feld() {
        let j = parse_program("struct S { state: u64, return: int }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"name\":\"state\""), "{j}");
        assert!(j.contains("\"name\":\"return\""), "{j}");
    }

    #[test]
    fn schluesselwort_als_feldzugriff() {
        assert!(parse_program("fn f(x: S) -> int { return x.state; }").is_ok());
    }

    #[test]
    fn schluesselwort_im_strukt_literal() {
        assert!(parse_program("fn f() -> int { let s = S { state: 1 }; return 0; }").is_ok());
    }

    #[test]
    fn event_auf_programmebene() {
        // 42 Korpusdateien deklarieren Events ausserhalb eines Contracts.
        let j = parse_program("event Transfer { from: Address, to: Address }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"EventDeclaration\""), "{j}");
        assert!(j.contains("\"name\":\"Transfer\""), "{j}");
    }

    #[test]
    fn event_bleibt_als_variablenname_erlaubt() {
        // `event` ist kein Schluesselwort geworden, damit dieser Code gilt.
        assert!(parse_program("fn f() -> int { let event = 1; return event; }").is_ok());
    }

    #[test]
    fn feldtyp_mit_fester_laenge() {
        let j = parse_program("struct S { data: u8[120] }")
            .unwrap()
            .to_json();
        assert!(j.contains("u8[120]"), "{j}");
    }

    #[test]
    fn typlaenge_ohne_klammer_ist_ein_fehler() {
        assert!(parse_program("struct S { data: u8[120 }").is_err());
    }

    #[test]
    fn strukt_felder_ohne_komma() {
        let j = parse_program("struct S { a: u64\n b: u64 }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"name\":\"a\""), "{j}");
        assert!(j.contains("\"name\":\"b\""), "{j}");
    }

    #[test]
    fn eingeruecktes_struct() {
        let j = parse_program("struct Stat:\n    base: u64\n    growth: u64")
            .unwrap()
            .to_json();
        assert!(j.contains("\"name\":\"base\""), "{j}");
        assert!(j.contains("\"name\":\"growth\""), "{j}");
    }

    #[test]
    fn eingeruecktes_enum() {
        let j = parse_program("enum Rarity:\n    COMMON\n    RARE\n")
            .unwrap()
            .to_json();
        assert!(j.contains("\"COMMON\""), "{j}");
        assert!(j.contains("\"RARE\""), "{j}");
    }

    #[test]
    fn eingerueckter_rumpf_endet_an_der_dedent() {
        // Der Rumpf endet, wo die Einrueckung zurueckgeht; `fn` gehoert dem
        // umgebenden Contract, nicht dem Struct.
        let j = parse_program(
            "contract C {\n  struct S:\n    a: u64\n  fn f() -> int { return 1; }\n}\n",
        )
        .unwrap()
        .to_json();
        assert!(j.contains("\"name\":\"a\""), "{j}");
        assert!(j.contains("\"kind\":\"FunctionDef\""), "{j}");
    }

    #[test]
    fn eingerueckter_rumpf_ohne_positionen_ist_ein_fehler() {
        // Ohne Spalten ist die Blockausdehnung nicht bestimmbar. Der Parser
        // darf dann nicht raten, sondern muss ablehnen.
        let toks = crate::lexer::tokenize("struct S:\n    a: u64").unwrap();
        let mut p = Parser {
            toks,
            pos: 0,
            cols: Vec::new(),
            errors: Vec::new(),
            collect: false,
        };
        assert!(p.program().is_err());
    }

    #[test]
    fn func_ist_ein_fn() {
        let j = parse_program("func f() -> int { return 1; }")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"FunctionDef\""), "{j}");
    }

    #[test]
    fn sammelmodus_meldet_mehrere_blocker() {
        // Zwei kaputte Deklarationen, dazwischen eine gesunde.
        let src = "struct A:\n    a u64\nfn ok() -> int { return 1; }\nenum B:\n    X =\n";
        let (program, errors) = parse_program_collecting(src);
        assert!(errors.len() >= 2, "nur {} Fehler: {errors:?}", errors.len());
        // Der Sammelmodus liefert einen Teilstand, keine Ausnahme.
        assert!(program.is_some());
    }

    #[test]
    fn normalbetrieb_bricht_beim_ersten_fehler_ab() {
        let src = "struct A:\n    a u64\nfn ok() -> int { return 1; }";
        let (_, errors) = parse_program_collecting(src);
        assert!(!errors.is_empty());
        assert!(parse_program(src).is_err());
    }

    #[test]
    fn sammelmodus_auf_gesunder_quelle_ist_leer() {
        let (program, errors) = parse_program_collecting("fn f() -> int { return 1; }");
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(program.unwrap().statements.len(), 1);
    }

    #[test]
    fn for_ueber_bereich() {
        let j = parse_program(
            "fn f() -> int {\n    for i in 0..3 {\n        return i\n    }\n    return 0\n}",
        )
        .unwrap()
        .to_json();
        assert!(j.contains("\"kind\":\"ForStatement\""), "{j}");
        assert!(j.contains("\"kind\":\"RangeExpr\""), "{j}");
        assert!(j.contains("\"inclusive\":false"), "{j}");
    }

    #[test]
    fn inklusiver_bereich() {
        let j = parse_program(
            "fn f() -> int {\n    for i in 0..=3 {\n        return i\n    }\n    return 0\n}",
        )
        .unwrap()
        .to_json();
        assert!(j.contains("\"inclusive\":true"), "{j}");
    }

    #[test]
    fn index_bereich_ist_slice() {
        let j = parse_program("fn f(xs: List<u8>) -> int {\n    let a = xs[0..2]\n    return 1\n}")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"SliceExpr\""), "{j}");
        // Einzelindex bleibt IndexAccess — keine Sonderbehandlung von Punkten.
        let j = parse_program("fn f(xs: List<u8>) -> int {\n    let a = xs[1]\n    return 1\n}")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"IndexAccess\""), "{j}");
        assert!(!j.contains("SliceExpr"), "{j}");
    }

    #[test]
    fn offenes_bereichsende_parst() {
        let j = parse_program("fn f(xs: List<u8>) -> int {\n    let a = xs[1..]\n    return 1\n}")
            .unwrap()
            .to_json();
        assert!(j.contains("\"kind\":\"SliceExpr\""), "{j}");
        assert!(j.contains("\"start\":{\"kind\":\"IntLiteral\""), "{j}");
        assert!(j.contains("\"end\":null"), "{j}");
    }

    #[test]
    fn for_ohne_in_ist_ein_fehler() {
        assert!(parse_program(
            "fn f() -> int {\n    for i 0..3 {\n        return i\n    }\n    return 0\n}"
        )
        .is_err());
    }

    #[test]
    fn for_ohne_bereich_ist_ein_fehler() {
        // Im Contract gibt es keinen Native-Fallback: der Koerper MUSS parsen.
        let src = "contract C {\n    fn f() -> int {\n        for i in xs {\n            return i\n        }\n        return 0\n    }\n}";
        let err = parse_program(src).expect_err("for ohne Bereich");
        assert!(err.message.contains("Bereich"), "{err:?}");
    }
}
