use std::fmt::Display;

use crate::{
    scan::Scanner,
    token::{LiteralKind, Token, TokenKind},
};

// expression     → literal
//                | unary
//                | binary
//                | grouping ;
//
// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

#[derive(Debug)]
pub enum ExprLiteral {
    Number(LiteralKind),
    String(LiteralKind),
    True(String),
    False(String),
    Nil(String),
}

#[derive(Debug)]
pub enum Expr {
    Literal(ExprLiteral),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

impl Expr {
    fn visit(&self) -> String {
        match self {
            Expr::Literal(expr_literal) => match expr_literal {
                ExprLiteral::Number(literal_kind) => todo!(),
                ExprLiteral::String(literal_kind) => todo!(),
                ExprLiteral::True(_) => "true".into(),
                ExprLiteral::False(_) => "false".into(),
                ExprLiteral::Nil(_) => todo!(),
            },
            Expr::Unary { operator, right } => todo!(),
            Expr::Binary {
                left,
                operator,
                right,
            } => todo!(),
            Expr::Grouping { expr } => todo!(),
        }
    }

    fn parenthesize(&self) -> String {
        todo!()
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expr::Literal(expr_literal) => match expr_literal {
                    ExprLiteral::Number(literal_kind) => todo!(),
                    ExprLiteral::String(literal_kind) => todo!(),
                    ExprLiteral::True(_) => "true",
                    ExprLiteral::False(_) => "false",
                    ExprLiteral::Nil(_) => "nil",
                },
                Expr::Unary { operator, right } => todo!(),
                Expr::Binary {
                    left,
                    operator,
                    right,
                } => todo!(),
                Expr::Grouping { expr } => todo!(),
            }
        )
    }
}

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
    pub fn parse(&mut self) -> Expr {
        self.expression()
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

    fn expression(&mut self) -> Expr {
        self.equality()
    }
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&[TokenKind::EQUAL_EQUAL, TokenKind::BANG_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            {
                expr = Expr::Binary {
                    left: expr.into(),
                    operator,
                    right: right.into(),
                };
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        // TODO:

        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        expr
    }
    fn unary(&mut self) -> Expr {
        let mut expr = self.primary();
        expr
    }
    fn primary(&mut self) -> Expr {
        // eprintln!("primary(): ")

        let literal = if self.check(&TokenKind::TRUE) {
            ExprLiteral::True("true".into())
        } else if self.check(&TokenKind::FALSE) {
            ExprLiteral::False("false".into())
        } else if self.check(&TokenKind::NIL) {
            ExprLiteral::Nil("nil".into())
        } else if self.check(&TokenKind::NUMBER) {
            ExprLiteral::Number(self.previous().literal.unwrap())
        } else if self.check(&TokenKind::STRING) {
            ExprLiteral::String(self.previous().literal.unwrap())
        } else {
            todo!()
        };

        return Expr::Literal(literal);
    }
}

impl From<Scanner<'_>> for Parser {
    fn from(scanner: Scanner) -> Self {
        Self {
            tokens: scanner.tokens(),
            current: 0,
        }
    }
}
