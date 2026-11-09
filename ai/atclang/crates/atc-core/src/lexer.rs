// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Lexer: Tokens fuer die ATCLang-Teilmenge (SCR-0128 Stufe 1: if/else/while
//! und Vergleichsoperatoren ==, !=, <, >, <=, >=; Booleans kodiert als i64 0/1).
//! Token-Modell am Python-Referenz-Lexer (src/atclang/frontend/lexer/lexer.py)
//! ausgerichtet (SCR-0084/0085). Hinweis: '=' ist im Referenz-Modell EQ und dient
//! im let-Kontext als Zuweisung; ':' ist COLON; '->' ist ARROW (Return-Type).
//! '%' wird geparst, gehoert aber NICHT zum Operator-Subset des Referenz-Parsers.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Int(u128),
    Ident(String),
    Let,
    Const,
    Return,
    Fn,
    If,
    Else,
    While,
    Contract,
    State,
    Function,
    Capability,
    Policy,
    Require,
    Use,
    /// `module` — Programmkopf laut SPEC 3.1.
    Module,
    /// `import` — Datei- oder Pfadimport (Korpusform).
    Import,
    /// `as` — Alias eines Dateiimports.
    As,
    /// `pub` — Sichtbarkeitsmarker aus dem Korpus. ATCLang kennt keine
    /// Sichtbarkeitsebenen; der Parser nimmt das Wort an und verwirft es.
    Pub,
    Struct,
    Enum,
    Pipe,
    Ampersand,
    Assign,
    Eq,
    NotEq,
    Bang,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Plus,
    Minus,
    Arrow,
    Star,
    Slash,
    Percent,
    Colon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semi,
    Eof,
    At,
    Dot,
    /// `..` — SPEC 2.1: DOTDOT.
    DotDot,
    /// `..=` — SPEC 2.1: DOTDOTEQ.
    DotDotEq,
    Question,
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub pos: usize,
    pub ch: char,
}

/// Zeile und Spalte eines Tokens, 1-basiert.
///
/// Der Parser braucht die Spalte fuer die eingerueckten Rumpfformen
/// (`struct X:` / `enum X:`): ohne Spalte ist nicht entscheidbar, wo ein
/// eingerueckter Block endet.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Pos {
    pub line: u32,
    pub col: u32,
}

/// Zeile/Spalte je Zeichenposition des Quelltextes.
fn line_col_table(src: &str) -> Vec<Pos> {
    let mut table = Vec::with_capacity(src.len());
    let mut line = 1u32;
    let mut col = 1u32;
    for ch in src.chars() {
        table.push(Pos { line, col });
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    table
}

/// Merkt sich das Token und die Zeichenposition, an der es beginnt.
fn push(tokens: &mut Vec<Token>, marks: &mut Vec<usize>, pos: usize, token: Token) {
    tokens.push(token);
    marks.push(pos);
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    Ok(scan(src)?.0)
}

/// Wie [`tokenize`], zusaetzlich die Position jedes Tokens.
pub fn tokenize_pos(src: &str) -> Result<(Vec<Token>, Vec<Pos>), LexError> {
    let (tokens, marks) = scan(src)?;
    let table = line_col_table(src);
    let positions = marks
        .iter()
        .map(|&i| table.get(i).copied().unwrap_or_default())
        .collect();
    Ok((tokens, positions))
}

fn scan(src: &str) -> Result<(Vec<Token>, Vec<usize>), LexError> {
    let mut tokens = Vec::new();
    let mut marks: Vec<usize> = Vec::new();
    let mut chars = src.chars().enumerate().peekable();
    while let Some((pos, ch)) = chars.next() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {}
            '=' => {
                if chars.peek().is_some_and(|&(_, c)| c == '=') {
                    chars.next();
                    push(&mut tokens, &mut marks, pos, Token::Eq);
                } else {
                    push(&mut tokens, &mut marks, pos, Token::Assign);
                }
            }
            '!' => {
                if chars.peek().is_some_and(|&(_, c)| c == '=') {
                    chars.next();
                    push(&mut tokens, &mut marks, pos, Token::NotEq);
                } else {
                    push(&mut tokens, &mut marks, pos, Token::Bang);
                }
            }
            '<' => {
                if chars.peek().is_some_and(|&(_, c)| c == '=') {
                    chars.next();
                    push(&mut tokens, &mut marks, pos, Token::LtEq);
                } else {
                    push(&mut tokens, &mut marks, pos, Token::Lt);
                }
            }
            '>' => {
                if chars.peek().is_some_and(|&(_, c)| c == '=') {
                    chars.next();
                    push(&mut tokens, &mut marks, pos, Token::GtEq);
                } else {
                    push(&mut tokens, &mut marks, pos, Token::Gt);
                }
            }
            '+' => push(&mut tokens, &mut marks, pos, Token::Plus),
            '-' => {
                if chars.peek().is_some_and(|&(_, c)| c == '>') {
                    chars.next();
                    push(&mut tokens, &mut marks, pos, Token::Arrow);
                } else {
                    push(&mut tokens, &mut marks, pos, Token::Minus);
                }
            }
            '*' => push(&mut tokens, &mut marks, pos, Token::Star),
            '/' => {
                if chars.peek().is_some_and(|&(_, c)| c == '/') {
                    // Line comments are lexical trivia. Consume the complete
                    // line before tokenizing, including arbitrary Unicode text.
                    chars.next();
                    for (_, comment_ch) in chars.by_ref() {
                        if comment_ch == '\n' {
                            break;
                        }
                    }
                } else {
                    push(&mut tokens, &mut marks, pos, Token::Slash);
                }
            }
            // SPEC 2.6: '#' bis Zeilenende ist ein Kommentar. Der Rust-Lexer
            // kannte bisher nur '//', weshalb jede Quelle mit '#' scheiterte.
            '#' => {
                for (_, comment_ch) in chars.by_ref() {
                    if comment_ch == '\n' {
                        break;
                    }
                }
            }
            '%' => push(&mut tokens, &mut marks, pos, Token::Percent),
            '|' => push(&mut tokens, &mut marks, pos, Token::Pipe),
            '&' => push(&mut tokens, &mut marks, pos, Token::Ampersand),
            ':' => push(&mut tokens, &mut marks, pos, Token::Colon),
            '@' => push(&mut tokens, &mut marks, pos, Token::At),
            '.' => {
                // SPEC 2.1: `..` und `..=` sind eigene Token, kein Sonderfall
                // von zwei Punkten im Parser.
                if chars.peek().is_some_and(|&(_, c)| c == '.') {
                    chars.next();
                    if chars.peek().is_some_and(|&(_, c)| c == '=') {
                        chars.next();
                        push(&mut tokens, &mut marks, pos, Token::DotDotEq);
                    } else {
                        push(&mut tokens, &mut marks, pos, Token::DotDot);
                    }
                } else {
                    push(&mut tokens, &mut marks, pos, Token::Dot);
                }
            }
            '?' => push(&mut tokens, &mut marks, pos, Token::Question),
            '(' => push(&mut tokens, &mut marks, pos, Token::LParen),
            ')' => push(&mut tokens, &mut marks, pos, Token::RParen),
            '[' => push(&mut tokens, &mut marks, pos, Token::LBracket),
            ']' => push(&mut tokens, &mut marks, pos, Token::RBracket),
            '{' => push(&mut tokens, &mut marks, pos, Token::LBrace),
            '}' => push(&mut tokens, &mut marks, pos, Token::RBrace),
            ',' => push(&mut tokens, &mut marks, pos, Token::Comma),
            ';' => push(&mut tokens, &mut marks, pos, Token::Semi),
            '"' => {
                let mut value = String::new();
                loop {
                    match chars.next() {
                        Some((_, '"')) => break,
                        Some((pos, '\\')) => match chars.next() {
                            Some((_, 'n')) => value.push('\n'),
                            Some((_, 'r')) => value.push('\r'),
                            Some((_, 't')) => value.push('\t'),
                            Some((_, '"')) => value.push('"'),
                            Some((_, '\\')) => value.push('\\'),
                            Some((_, ch)) => return Err(LexError { pos, ch }),
                            None => return Err(LexError { pos, ch: '\\' }),
                        },
                        Some((_pos, ch)) => value.push(ch),
                        None => {
                            return Err(LexError {
                                pos: src.len(),
                                ch: '"',
                            })
                        }
                    }
                }
                push(&mut tokens, &mut marks, pos, Token::String(value));
            }
            '0'..='9' => {
                let mut n: u128 = (ch as u128) - ('0' as u128);
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_ascii_digit() {
                        let digit_pos = chars.peek().map(|(p, _)| *p).unwrap_or(pos);
                        n = n
                            .checked_mul(10)
                            .and_then(|value| value.checked_add((c as u128) - ('0' as u128)))
                            .ok_or(LexError {
                                pos: digit_pos,
                                ch: c,
                            })?;
                        chars.next();
                    } else if c == '_' {
                        let underscore_pos = chars.peek().map(|(p, _)| *p).unwrap_or(pos);
                        chars.next();
                        match chars.peek() {
                            Some((_, next)) if next.is_ascii_digit() => {}
                            _ => {
                                return Err(LexError {
                                    pos: underscore_pos,
                                    ch: '_',
                                })
                            }
                        }
                    } else {
                        break;
                    }
                }
                push(&mut tokens, &mut marks, pos, Token::Int(n));
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut ident = String::new();
                ident.push(ch);
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "let" => push(&mut tokens, &mut marks, pos, Token::Let),
                    "const" => push(&mut tokens, &mut marks, pos, Token::Const),
                    "return" => push(&mut tokens, &mut marks, pos, Token::Return),
                    "fn" => push(&mut tokens, &mut marks, pos, Token::Fn),
                    // `func` ist im Korpus die zweite Schreibweise fuer `fn`
                    // (32 Dateien). Dieselbe Deklaration, kein eigener Knoten.
                    "func" => push(&mut tokens, &mut marks, pos, Token::Fn),
                    "if" => push(&mut tokens, &mut marks, pos, Token::If),
                    "else" => push(&mut tokens, &mut marks, pos, Token::Else),
                    "while" => push(&mut tokens, &mut marks, pos, Token::While),
                    "contract" => push(&mut tokens, &mut marks, pos, Token::Contract),
                    "state" => push(&mut tokens, &mut marks, pos, Token::State),
                    "function" => push(&mut tokens, &mut marks, pos, Token::Function),
                    "capability" => push(&mut tokens, &mut marks, pos, Token::Capability),
                    "policy" => push(&mut tokens, &mut marks, pos, Token::Policy),
                    "require" => push(&mut tokens, &mut marks, pos, Token::Require),
                    "use" => push(&mut tokens, &mut marks, pos, Token::Use),
                    "module" => push(&mut tokens, &mut marks, pos, Token::Module),
                    "import" => push(&mut tokens, &mut marks, pos, Token::Import),
                    "as" => push(&mut tokens, &mut marks, pos, Token::As),
                    "pub" => push(&mut tokens, &mut marks, pos, Token::Pub),
                    "struct" => push(&mut tokens, &mut marks, pos, Token::Struct),
                    "enum" => push(&mut tokens, &mut marks, pos, Token::Enum),
                    _ => push(&mut tokens, &mut marks, pos, Token::Ident(ident)),
                }
            }
            _ => return Err(LexError { pos, ch }),
        }
    }
    push(&mut tokens, &mut marks, src.chars().count(), Token::Eof);
    Ok((tokens, marks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_zuweisung() {
        let ts = tokenize("let x = 42;").unwrap();
        assert_eq!(
            ts,
            vec![
                Token::Let,
                Token::Ident("x".to_string()),
                Token::Assign,
                Token::Int(42),
                Token::Semi,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn fn_signatur_mit_arrow() {
        let ts = tokenize("fn add(a: u64) -> u64 { }").unwrap();
        assert_eq!(
            ts,
            vec![
                Token::Fn,
                Token::Ident("add".to_string()),
                Token::LParen,
                Token::Ident("a".to_string()),
                Token::Colon,
                Token::Ident("u64".to_string()),
                Token::RParen,
                Token::Arrow,
                Token::Ident("u64".to_string()),
                Token::LBrace,
                Token::RBrace,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn minus_nicht_arrow() {
        let ts = tokenize("let n = -5;").unwrap();
        assert_eq!(ts[3], Token::Minus);
        assert_eq!(ts[4], Token::Int(5));
    }

    #[test]
    fn native_tokens() {
        let ts = tokenize("@version 1.0 @name \"ATC\" contract C { @capability mint function f(to: Address) -> TokenId { require(true, \"ok\"); } }").unwrap();
        assert!(ts.contains(&Token::At));
        assert!(ts.contains(&Token::Dot));
        assert!(ts.contains(&Token::String("ATC".into())));
        assert!(ts.contains(&Token::Contract));
        assert!(ts.contains(&Token::Capability));
        assert!(ts.contains(&Token::Function));
        assert!(ts.contains(&Token::Require));
    }

    #[test]
    fn line_comments_ignore_unicode_text() {
        let tokens = tokenize("// ───────────── Unicode comment\nlet x = 1;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Ident("x".to_string()),
                Token::Assign,
                Token::Int(1),
                Token::Semi,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn square_brackets_tokenize_list_literals_and_index_access() {
        let tokens = tokenize("let xs: List<Hash> = []; xs[addr] = entry;").unwrap();

        assert_eq!(
            tokens
                .iter()
                .filter(|token| **token == Token::LBracket)
                .count(),
            2
        );
        assert_eq!(
            tokens
                .iter()
                .filter(|token| **token == Token::RBracket)
                .count(),
            2
        );
        assert!(tokens.contains(&Token::Ident("xs".to_string())));
        assert!(tokens.contains(&Token::Ident("addr".to_string())));
    }

    #[test]
    fn native_contract_operators_and_declarations() {
        let tokens = tokenize("use ATC::Types::Address; struct Entry { value: u128 } enum Status { Open, Closed } let mask = a | b & c;").unwrap();
        assert!(tokens.contains(&Token::Use));
        assert!(tokens.contains(&Token::Struct));
        assert!(tokens.contains(&Token::Enum));
        assert!(tokens.contains(&Token::Pipe));
        assert!(tokens.contains(&Token::Ampersand));
    }

    #[test]
    fn integer_literals_allow_digit_separators() {
        assert!(tokenize("let supply = 21_000_000;")
            .unwrap()
            .contains(&Token::Int(21000000)));
        assert_eq!(
            tokenize("let bad = 1_;"),
            Err(LexError { pos: 11, ch: '_' })
        );
    }

    #[test]
    fn square_brackets_support_list_literals_and_indexing() {
        let tokens = tokenize("let xs: List<Hash> = []; xs[addr] = entry;").unwrap();
        assert!(tokens.contains(&Token::LBracket));
        assert!(tokens.contains(&Token::RBracket));
        assert_eq!(
            tokens
                .iter()
                .filter(|token| **token == Token::LBracket)
                .count(),
            2
        );
        assert_eq!(
            tokens
                .iter()
                .filter(|token| **token == Token::RBracket)
                .count(),
            2
        );
    }

    #[test]
    fn ungültiges_zeichen() {
        assert_eq!(tokenize("let $ = 1"), Err(LexError { pos: 4, ch: '$' }));
    }

    #[test]
    fn hash_kommentar_ist_trivia() {
        // SPEC 2.6: '#' bis Zeilenende ist ein Kommentar.
        let ts = tokenize("# Modulkopf\nlet x = 1; # nachgestellt\n").unwrap();
        assert!(ts.contains(&Token::Let));
        assert!(ts.contains(&Token::Ident("x".into())));
        assert!(ts.contains(&Token::Int(1)));
        assert_eq!(ts.last(), Some(&Token::Eof));
    }

    #[test]
    fn hash_kommentar_schliesst_zeichen_aus() {
        // Ein '#' in einem String ist kein Kommentar.
        let ts = tokenize("let s = \"a#b\";").unwrap();
        assert!(ts.contains(&Token::String("a#b".into())));
    }

    #[test]
    fn modul_import_und_alias_sind_schluesselwoerter() {
        let ts = tokenize("module m import \"p\" as A").unwrap();
        assert!(ts.contains(&Token::Module));
        assert!(ts.contains(&Token::Import));
        assert!(ts.contains(&Token::As));
    }

    #[test]
    fn pub_ist_ein_schluesselwort() {
        let ts = tokenize("pub fn f() -> int { return 1; }").unwrap();
        assert_eq!(ts.first(), Some(&Token::Pub));
        assert!(ts.contains(&Token::Fn));
    }

    #[test]
    fn bereichs_token_sind_unterscheidbar() {
        // SPEC 2.1: DOTDOT und DOTDOTEQ sind eigene Token, kein Sonderfall
        // von zwei Punkten im Parser.
        let ts = tokenize("0..n").unwrap();
        assert_eq!(
            ts,
            vec![
                Token::Int(0),
                Token::DotDot,
                Token::Ident("n".to_string()),
                Token::Eof,
            ]
        );
        let ts = tokenize("0..=n").unwrap();
        assert_eq!(ts[1], Token::DotDotEq);
        // Ein einzelner Punkt bleibt Feldzugriff.
        let ts = tokenize("a.b").unwrap();
        assert_eq!(ts[1], Token::Dot);
    }
}
