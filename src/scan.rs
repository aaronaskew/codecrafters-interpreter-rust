use crate::token::LiteralKind;
use crate::token::Token;
use crate::token::TokenKind;
use std::{fmt::Display, str::Chars};

pub struct Scanner<'a> {
    source: itertools::PeekNth<Chars<'a>>,
    tokens: Vec<Token>,
    pub has_error: bool,
    location: (usize, usize),
}

impl<'a> Scanner<'a> {
    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }

    fn advance_char(&mut self) -> Option<char> {
        if let Some(c) = self.source.next() {
            self.location.0 += 1;
            return Some(c);
        }

        None
    }

    fn advance_if_next_char_match(&mut self, test_ch: char) -> bool {
        if let Some(ch) = self.source.peek()
            && *ch == test_ch
        {
            self.source.next();
            return true;
        }

        false
    }

    fn advance_if<F>(&mut self, f: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        if let Some(&peeked_ch) = self.source.peek()
            && f(peeked_ch)
        {
            return self.advance_char();
        }

        None
    }
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

impl<'a> From<&'a str> for Scanner<'a> {
    fn from(source: &'a str) -> Self {
        let source = itertools::peek_nth(source.chars());

        let mut scanner = Self {
            source,
            tokens: vec![],
            has_error: false,
            location: (1, 1),
        };

        while let Some(c) = scanner.advance_char() {
            let mut literal = None;

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
                    if scanner.advance_if_next_char_match('=') {
                        lexeme.push('=');
                        Ok(TokenKind::EQUAL_EQUAL)
                    } else {
                        Ok(TokenKind::EQUAL)
                    }
                }
                '!' => {
                    lexeme.push(c);
                    if scanner.advance_if_next_char_match('=') {
                        lexeme.push('=');
                        Ok(TokenKind::BANG_EQUAL)
                    } else {
                        Ok(TokenKind::BANG)
                    }
                }
                '<' => {
                    lexeme.push(c);
                    if scanner.advance_if_next_char_match('=') {
                        lexeme.push('=');
                        Ok(TokenKind::LESS_EQUAL)
                    } else {
                        Ok(TokenKind::LESS)
                    }
                }
                '>' => {
                    lexeme.push(c);
                    if scanner.advance_if_next_char_match('=') {
                        lexeme.push('=');
                        Ok(TokenKind::GREATER_EQUAL)
                    } else {
                        Ok(TokenKind::GREATER)
                    }
                }
                '/' => {
                    if scanner.advance_if_next_char_match('/') {
                        while let Some(ch) = scanner.source.peek()
                            && *ch != '\n'
                        {
                            scanner.advance_char();
                        }
                        continue;
                    } else {
                        lexeme.push(c);
                        Ok(TokenKind::SLASH)
                    }
                }
                ' ' | '\r' | '\t' => {
                    continue;
                }
                '\n' => {
                    scanner.location.0 = 0;
                    scanner.location.1 += 1;
                    continue;
                }
                // String literal
                '"' => {
                    lexeme.push('"');
                    let mut string_literal = String::new();
                    let mut terminated = false;
                    while let Some(ch) = scanner.advance_char() {
                        if ch != '"' {
                            string_literal.push(ch);
                            lexeme.push(ch);
                        } else {
                            lexeme.push('"');
                            terminated = true;
                            break;
                        }
                    }

                    if terminated {
                        literal = Some(LiteralKind::String(string_literal));
                        Ok(TokenKind::STRING)
                    } else {
                        eprintln!("[line {}] Error: Unterminated string.", scanner.location.1);
                        scanner.has_error = true;
                        Err(())
                    }
                }
                // Number literal
                '0'..='9' => {
                    lexeme.push(c);
                    let mut num_string = String::from(c);

                    // handle integer part
                    while let Some(&ch) = scanner.source.peek()
                        && ch.is_ascii_digit()
                    {
                        lexeme.push(ch);
                        num_string.push(scanner.advance_char().expect("must be a digit"));
                    }

                    // handle possible fractional part
                    if let Some(&ch) = scanner.source.peek()
                        && let Some(ch2) = scanner.source.peek_nth(1)
                        && ch == '.'
                        && ch2.is_ascii_digit()
                    {
                        lexeme.push('.');
                        num_string.push(scanner.advance_char().expect("is dot"));

                        while let Some(&ch) = scanner.source.peek()
                            && ch.is_ascii_digit()
                        {
                            lexeme.push(ch);
                            num_string.push(scanner.advance_char().expect("must be a digit"));
                        }
                    }

                    literal = Some(LiteralKind::Number(
                        num_string.parse().expect("should be parsable as f64"),
                    ));

                    Ok(TokenKind::NUMBER)
                }
                // identifier
                '_' | 'a'..='z' | 'A'..='Z' => {
                    lexeme.push(c);
                    let mut identifier = String::from(c);
                    while let Some(ch) =
                        scanner.advance_if(|c| c.is_alphanumeric() || matches!(c, '_' | '-'))
                    {
                        lexeme.push(ch);
                        identifier.push(ch);
                    }

                    Ok(match identifier.as_str() {
                        "and" => TokenKind::AND,
                        "class" => TokenKind::CLASS,
                        "else" => TokenKind::ELSE,
                        "false" => TokenKind::FALSE,
                        "fun" => TokenKind::FUN,
                        "for" => TokenKind::FOR,
                        "if" => TokenKind::IF,
                        "nil" => TokenKind::NIL,
                        "or" => TokenKind::OR,
                        "print" => TokenKind::PRINT,
                        "return" => TokenKind::RETURN,
                        "super" => TokenKind::SUPER,
                        "this" => TokenKind::THIS,
                        "true" => TokenKind::TRUE,
                        "var" => TokenKind::VAR,
                        "while" => TokenKind::WHILE,
                        _ => TokenKind::IDENTIFIER,
                    })
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

            if let Ok(token_kind) = token_kind_res {
                let token = Token {
                    kind: token_kind,
                    lexeme,
                    literal,
                    location: scanner.location,
                };

                // eprintln!("token: {}", token);

                scanner.tokens.push(token);
            }
        }

        scanner.tokens.push(Token::eof(scanner.location));

        scanner
    }
}
