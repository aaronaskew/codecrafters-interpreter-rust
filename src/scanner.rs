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
    tokens: Vec<Token>,
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
    pub(crate) fn from_str(input: &str) -> Self {
        let mut scanner = Self { tokens: vec![] };

        for c in input.chars() {
            let token_kind = match c {
                '(' => TokenKind::LEFT_PAREN,
                ')' => TokenKind::RIGHT_PAREN,
                '{' => TokenKind::LEFT_BRACE,
                '}' => TokenKind::RIGHT_BRACE,
                ',' => TokenKind::COMMA,
                '.' => TokenKind::DOT,
                '-' => TokenKind::MINUS,
                '+' => TokenKind::PLUS,
                ';' => TokenKind::SEMICOLON,
                '*' => TokenKind::STAR,

                _ => todo!(),
            };

            let lexeme = c.to_string();

            let literal = None;

            scanner.tokens.push(Token {
                kind: token_kind,
                lexeme,
                literal,
            });
        }

        scanner.tokens.push(Token::eof());

        scanner
    }
}
