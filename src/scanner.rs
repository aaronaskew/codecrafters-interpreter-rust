use std::{
    fmt::{Debug, Display},
    iter::Peekable,
    str::Chars,
};

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
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
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

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    pub has_error: bool,
    location: (usize, usize),
}

impl<'a> Display for Scanner<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let mut display_string = String::new();
        for token in &self.tokens {
            let _ = writeln!(f, "{}", token);
        }

        Ok(())
    }
}

impl<'a> Scanner<'a> {
    fn advance_char(&mut self) -> Option<char> {
        self.source.peek()?;

        while let Some(c) = self.source.next() {
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

    fn advance_char_if_match(&mut self, test_ch: char) -> bool {
        if let Some(ch) = self.source.peek()
            && *ch == test_ch
        {
            self.source.next();
            return true;
        }

        false
    }

    pub(crate) fn from_str(source: &'a str) -> Self {
        let source = source.chars().peekable();

        let mut scanner = Self {
            source,
            tokens: vec![],
            has_error: false,
            location: (1, 1),
        };

        while let Some(c) = scanner.advance_char() {
            let mut lexeme = String::new();

            let token_kind_res = match c {
                // Single-character
                '(' => {
                    lexeme.push(c);
                    Ok(TokenKind::LEFT_PAREN)
                }
                ')' => {
                    lexeme.push(c);
                    Ok(TokenKind::RIGHT_PAREN)
                }
                '{' => {
                    lexeme.push(c);
                    Ok(TokenKind::LEFT_BRACE)
                }
                '}' => {
                    lexeme.push(c);
                    Ok(TokenKind::RIGHT_BRACE)
                }
                ',' => {
                    lexeme.push(c);
                    Ok(TokenKind::COMMA)
                }
                '.' => {
                    lexeme.push(c);
                    Ok(TokenKind::DOT)
                }
                '-' => {
                    lexeme.push(c);
                    Ok(TokenKind::MINUS)
                }
                '+' => {
                    lexeme.push(c);
                    Ok(TokenKind::PLUS)
                }
                ';' => {
                    lexeme.push(c);
                    Ok(TokenKind::SEMICOLON)
                }
                '*' => {
                    lexeme.push(c);
                    Ok(TokenKind::STAR)
                }

                // One or two character
                '=' => {
                    lexeme.push(c);
                    if scanner.advance_char_if_match('=') {
                        lexeme.push('=');
                        Ok(TokenKind::EQUAL_EQUAL)
                    } else {
                        Ok(TokenKind::EQUAL)
                    }
                }
                '!' => {
                    lexeme.push(c);
                    if scanner.advance_char_if_match('=') {
                        lexeme.push('=');
                        Ok(TokenKind::BANG_EQUAL)
                    } else {
                        Ok(TokenKind::BANG)
                    }
                }

                unknown_char => {
                    eprintln!(
                        "[line {}] Error: Unexpected character: {}",
                        scanner.location.1, unknown_char
                    );
                    scanner.has_error = true;
                    Err(())
                }
            };

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
