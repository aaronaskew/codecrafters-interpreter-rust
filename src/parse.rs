use std::{fmt::Display, process::exit};

use thiserror::Error;

use crate::SYNTAX_ERROR_EXIT_CODE;
pub(crate) use crate::expr::*;
pub(crate) use crate::stmt::*;

use crate::token::{Token, TokenKind};

pub(crate) mod expr;
pub(crate) mod stmt;

#[derive(Debug, Error)]
pub struct ParseError {
    msg: String,
    token: Token,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.token.kind == TokenKind::EOF {
            write!(
                f,
                "[line {}] Error at end: {}.",
                self.token.line(),
                self.msg
            )
        } else {
            write!(
                f,
                "[line {}] Error at '{}': {}.",
                self.token.line(),
                self.token.lexeme,
                self.msg
            )
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// Lox Grammar
// ===========
// program        → statement* EOF ;
//
// statement      → exprStmt
//                | printStmt ;
//
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(SYNTAX_ERROR_EXIT_CODE);
                }
            }
        }

        statements
    }

    pub fn parse_expr(&mut self) -> Expr {
        match self.expression() {
            Ok(expr) => expr,
            Err(e) => {
                eprintln!("{}", e);
                exit(SYNTAX_ERROR_EXIT_CODE);
            }
        }
    }

    /// If current token matches any of `kinds`, return true and advance current token. Else, return
    /// false.
    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Checks if current token matches kind without advancing current token.
    fn check(&mut self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().kind == *kind
    }

    /// Advances current token by one.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    /// Returns current token. Does not advance current token.
    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .expect("bounds are checked")
            .clone()
    }

    /// Returns previous token. Useful after a `match_token()` or `advance()` call.
    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("bounds are checked")
            .clone()
    }

    /// Consumes the current token and advances it by one. Returns Ok<Token> is matches `kind`, else
    /// returns Err<String> with the provided `err_msg`.
    fn consume(&mut self, kind: TokenKind, msg: &str) -> Result<Token, ParseError> {
        if self.check(&kind) {
            return Ok(self.advance());
        }

        Err(ParseError {
            msg: msg.into(),
            token: self.peek(),
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenKind::PRINT]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;

        self.consume(TokenKind::SEMICOLON, "Expect ';' after value.")?;

        Ok(Stmt::Print { expr: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenKind::SEMICOLON, "Expect ';' after expression.")?;

        Ok(Stmt::Expression { expr })
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
            self.consume(TokenKind::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expr: expr.into() });
        }

        Err(Self::error(self.peek(), "Expect expression".into()))
    }

    fn error(token: Token, msg: String) -> ParseError {
        ParseError { msg, token }
    }
}
