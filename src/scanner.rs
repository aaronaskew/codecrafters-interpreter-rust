use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum LiteralKind {
    _String(String),
    _Number(f64),
}

impl Display for LiteralKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal_str = match self {
            LiteralKind::_String(s) => s,
            LiteralKind::_Number(n) => &n.to_string(),
        };
        write!(f, "{}", literal_str)
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum TokenKind {
    EOF,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,
}

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: Option<LiteralKind>,
}

impl Token {
    fn eof() -> Self {
        Self {
            kind: TokenKind::EOF,
            lexeme: String::new(),
            literal: None,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.kind,
            self.lexeme,
            self.literal
                .as_ref()
                .map_or("null".to_string(), |l| l.to_string())
        )
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    pub has_error: bool,
    location: (usize, usize),
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let mut display_string = String::new();
        for token in &self.tokens {
            let _ = writeln!(f, "{}", token);
        }

        Ok(())
    }
}

impl Scanner {
    fn advance_char(&mut self) -> Option<char> {
        if self.source.is_empty() {
            return None;
        }

        while let Some(c) = self.source.pop() {
            self.location.0 += 1;
            if c == '\n' {
                self.location.0 = 1;
                self.location.1 += 1;
            } else {
                return Some(c);
            }
        }

        None
    }

    pub(crate) fn from_str(source: &str) -> Self {
        let mut scanner = Self {
            source: source.chars().rev().collect(),
            tokens: vec![],
            has_error: false,
            location: (1, 1),
        };

        while let Some(c) = scanner.advance_char() {
            let token_kind_res = match c {
                '(' => Ok(TokenKind::LEFT_PAREN),
                ')' => Ok(TokenKind::RIGHT_PAREN),
                '{' => Ok(TokenKind::LEFT_BRACE),
                '}' => Ok(TokenKind::RIGHT_BRACE),
                ',' => Ok(TokenKind::COMMA),
                '.' => Ok(TokenKind::DOT),
                '-' => Ok(TokenKind::MINUS),
                '+' => Ok(TokenKind::PLUS),
                ';' => Ok(TokenKind::SEMICOLON),
                '*' => Ok(TokenKind::STAR),

                unknown_char => {
                    eprintln!(
                        "[line {}] Error: Unexpected character: {}",
                        scanner.location.1, unknown_char
                    );
                    scanner.has_error = true;
                    Err(())
                }
            };

            let lexeme = c.to_string();

            let literal = None;

            if let Ok(token_kind) = token_kind_res {
                scanner.tokens.push(Token {
                    kind: token_kind,
                    lexeme,
                    literal,
                });
            }
        }

        scanner.tokens.push(Token::eof());

        scanner
    }
}
