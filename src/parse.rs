use std::{fmt::Display, process::exit};

use thiserror::Error;

pub(crate) use crate::expr::*;

use crate::token::{Token, TokenKind};

pub(crate) mod expr;

#[derive(Debug, Error)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError",)
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expression()?;

        Ok(expr)
    }

    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().kind == *kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .expect("bounds are checked")
            .clone()
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("bounds are checked")
            .clone()
    }

    fn consume(&mut self, kind: TokenKind, err_msg: &str) -> Result<Token, String> {
        if self.check(&kind) {
            return Ok(self.advance());
        }

        Err(err_msg.into())
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenKind::EQUAL_EQUAL, TokenKind::BANG_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenKind::LESS,
            TokenKind::LESS_EQUAL,
            TokenKind::GREATER,
            TokenKind::GREATER_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            };
        }

        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenKind::PLUS, TokenKind::MINUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            };
        }
        Ok(expr)
    }
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenKind::SLASH, TokenKind::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenKind::BANG, TokenKind::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;

            Ok(Expr::Unary {
                operator,
                right: right.into(),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenKind::TRUE]) {
            return Ok(Expr::Literal(ExprLiteral::True));
        } else if self.match_token(&[TokenKind::FALSE]) {
            return Ok(Expr::Literal(ExprLiteral::False));
        } else if self.match_token(&[TokenKind::NIL]) {
            return Ok(Expr::Literal(ExprLiteral::Nil));
        } else if self.match_token(&[TokenKind::NUMBER]) {
            return Ok(Expr::Literal(ExprLiteral::Number(
                self.previous().literal.unwrap(),
            )));
        } else if self.match_token(&[TokenKind::STRING]) {
            return Ok(Expr::Literal(ExprLiteral::String(
                self.previous().literal.unwrap(),
            )));
        }

        if self.match_token(&[TokenKind::LEFT_PAREN]) {
            let expr = self.expression()?;
            match self.consume(TokenKind::RIGHT_PAREN, "Expect ')' after expression.") {
                Ok(_) => {
                    return Ok(Expr::Grouping { expr: expr.into() });
                }
                Err(msg) => {
                    eprintln!("{}", msg);
                    exit(65);
                }
            }
        }

        Err(Self::error(self.peek(), "Expect expression".into()))
    }

    fn error(token: Token, msg: String) -> ParseError {
        if token.kind == TokenKind::EOF {
            eprintln!("[line {}] Error at end: {}.", token.line(), msg);
        } else {
            eprintln!(
                "[line {}] Error at '{}': {}.",
                token.line(),
                token.lexeme,
                msg
            );
        }

        ParseError
    }
}
