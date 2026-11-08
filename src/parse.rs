use crate::SYNTAX_ERROR_EXIT_CODE;
use crate::grammar::*;
use crate::token::{Token, TokenKind};
use std::{fmt::Display, process::exit};
use thiserror::Error;

pub(crate) mod grammar;

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

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        match self.program() {
            Ok(statements) => statements,
            Err(e) => {
                eprintln!("{}", e);
                exit(SYNTAX_ERROR_EXIT_CODE);
            }
        }
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

    // ============== Grammar Parsing ======================

    // Lox Grammar
    // ===========
    // program        → declaration* EOF ;
    // declaration    → varDecl
    //                | statement ;
    //
    // varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
    //
    // statement      → exprStmt
    //                | ifStmt
    //                | printStmt
    //                | block ;
    //
    // exprStmt       → expression ";" ;
    // ifStmt         → "if" "(" expression ")" statement
    //                ( "else" statement )? ;
    //
    // printStmt      → "print" expression ";" ;
    // block          → "{" declaration* "}" ;
    //
    // expression     → assignment ;
    //
    // assignment     → IDENTIFIER "=" assignment
    //                | logic_or ;
    //
    // logic_or       → logic_and ( "or" logic_and )* ;
    // logic_and      → equality ( "and" equality )* ;
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")"
    //                | IDENTIFIER ;

    fn program(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenKind::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenKind::IDENTIFIER, "Expect variable name.")?;

        let mut initializer = None;
        if self.match_token(&[TokenKind::EQUAL]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenKind::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenKind::PRINT]) {
            self.print_statement()
        } else if self.match_token(&[TokenKind::LEFT_BRACE]) {
            self.block()
        } else if self.match_token(&[TokenKind::IF]) {
            self.if_statement()
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = vec![];

        while !self.check(&TokenKind::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenKind::RIGHT_BRACE, "Expect '}' after block.")?;

        Ok(Stmt::Block { statements })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenKind::LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenKind::RIGHT_PAREN, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch_opt = None;
        if self.match_token(&[TokenKind::ELSE]) {
            else_branch_opt = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch_opt,
        })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenKind::SEMICOLON, "Expect ';' after expression.")?;

        Ok(Stmt::Expression { expr })
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;

        self.consume(TokenKind::SEMICOLON, "Expect ';' after value.")?;

        Ok(Stmt::Print { expr: value })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_token(&[TokenKind::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(Self::error(equals, "Invalid assignment target.".into()));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenKind::OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenKind::AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
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
        } else if self.match_token(&[TokenKind::IDENTIFIER]) {
            return Ok(Expr::Variable {
                name: self.previous(),
            });
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
