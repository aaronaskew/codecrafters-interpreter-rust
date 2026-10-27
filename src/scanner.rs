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

    pub(crate) fn from_str(source: &'a str) -> Self {
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
                    while let Some(&ch) = scanner.source.peek() {
                        if ch.is_ascii_digit() {
                            lexeme.push(ch);
                            num_string.push(scanner.advance_char().expect("must be a digit"));
                        } else {
                            if ch == '.' {
                                // there may be a fractional part
                                if let Some(ch2) = scanner.source.peek_nth(2)
                                    && ch2.is_ascii_digit()
                                {
                                    // there is a fractional part
                                    lexeme.push('.');
                                    num_string.push(scanner.advance_char().expect("must be dot"));
                                    while let Some(&ch) = scanner.source.peek()
                                        && ch.is_ascii_digit()
                                    {
                                        lexeme.push(ch);
                                        num_string
                                            .push(scanner.advance_char().expect("must be digit"));
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    literal = Some(LiteralKind::Number(
                        num_string.parse().expect("should be parsable as f64"),
                    ));
                    Ok(TokenKind::NUMBER)
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
                };

                // eprintln!("token: {}", token);

                scanner.tokens.push(token);
            }
        }

        scanner.tokens.push(Token::eof());

        scanner
    }
}
