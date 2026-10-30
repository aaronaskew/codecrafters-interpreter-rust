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
    True,
    False,
    Nil,
}

impl Display for ExprLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExprLiteral::Number(literal_kind) | ExprLiteral::String(literal_kind) =>
                    literal_kind.to_string(),
                ExprLiteral::True => true.to_string(),
                ExprLiteral::False => false.to_string(),
                ExprLiteral::Nil => "nil".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub enum Operator {
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<Token> for Operator {
    fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::EQUAL_EQUAL => Self::Equals,
            TokenKind::BANG_EQUAL => Self::NotEquals,
            TokenKind::LESS => Self::LessThan,
            TokenKind::LESS_EQUAL => Self::LessThanOrEquals,
            TokenKind::GREATER => Self::GreaterThan,
            TokenKind::GREATER_EQUAL => Self::GreaterThanOrEquals,
            TokenKind::PLUS => Self::Add,
            TokenKind::MINUS => Self::Subtract,
            TokenKind::STAR => Self::Multiply,
            TokenKind::SLASH => Self::Divide,
            _ => unreachable!("Not an operator"),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Equals => "==",
                Operator::NotEquals => "!=",
                Operator::LessThan => "<",
                Operator::LessThanOrEquals => "<=",
                Operator::GreaterThan => ">",
                Operator::GreaterThanOrEquals => ">=",
                Operator::Add => "+",
                Operator::Subtract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
            }
        )
    }
}

#[derive(Debug)]
pub enum Expr {
    Literal(ExprLiteral),
    Unary {
        operator: Operator,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

impl Expr {
    pub fn print_ast(&self) {
        println!("{self}");
    }

    fn parenthesize(&self) -> String {
        let output = format!(
            "({}",
            match self {
                Expr::Literal(expr_literal) => expr_literal.to_string(),
                Expr::Unary { operator, right } => format!("{} {}", operator, right.parenthesize()),
                Expr::Binary {
                    left,
                    operator,
                    right,
                } => format!(
                    "{} {} {}",
                    operator,
                    left.parenthesize(),
                    right.parenthesize()
                ),
                Expr::Grouping { expr } => format!("group {}", expr.parenthesize()),
            }
        );

        let output = format!("{})", output);

        output
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expr::Literal(literal) => literal.to_string(),
                Expr::Unary { operator, right } => {
                    self.parenthesize()
                }
                Expr::Binary {
                    left,
                    operator,
                    right,
                } => self.parenthesize(),
                Expr::Grouping { expr } => self.parenthesize(),
            }
        )
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
        eprintln!("begin advance() {:#?}", self);
        if !self.is_at_end() {
            self.current += 1;
        }

        dbg!(self.previous())
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

    fn consume(&self, right_paren: TokenKind, arg: &str) -> Token {
        todo!()
    }

    fn expression(&mut self) -> Expr {
        eprintln!("==expression== current token_kind: {:?}", self.peek().kind);
        self.equality()
    }
    fn equality(&mut self) -> Expr {
        eprintln!("==equality== current token_kind: {:?}", self.peek().kind);
        let mut expr = self.comparison();

        while self.match_token(&[TokenKind::EQUAL_EQUAL, TokenKind::BANG_EQUAL]) {
            let operator = Operator::from(self.previous());
            let right = self.comparison();
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        eprintln!("==comparision== current token_kind: {:?}", self.peek().kind);
        let mut expr = self.term();

        while self.match_token(&[
            TokenKind::LESS,
            TokenKind::LESS_EQUAL,
            TokenKind::GREATER,
            TokenKind::GREATER_EQUAL,
        ]) {
            let operator = Operator::from(self.previous());
            let right = self.term();
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            };
        }

        expr
    }
    fn term(&mut self) -> Expr {
        eprintln!("==term== current token_kind: {:?}", self.peek().kind);
        let mut expr = self.factor();

        while self.match_token(&[TokenKind::PLUS, TokenKind::MINUS]) {
            let operator = Operator::from(self.previous());
            let right = self.factor();
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            };
        }
        expr
    }
    fn factor(&mut self) -> Expr {
        eprintln!("==factor== current token_kind: {:?}", self.peek().kind);
        let mut expr = self.unary();

        while self.match_token(&[TokenKind::SLASH, TokenKind::STAR]) {
            let operator = Operator::from(self.previous());
            let right = self.unary();
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: right.into(),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        eprintln!("==unary== current token_kind: {:?}", self.peek().kind);

        if self.match_token(&[TokenKind::BANG, TokenKind::MINUS]) {
            let operator = Operator::from(self.previous());
            let right = self.unary();

            Expr::Unary {
                operator,
                right: right.into(),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        eprintln!("==primary== current token_kind: {:?}", self.peek().kind);

        if self.match_token(&[TokenKind::TRUE]) {
            return Expr::Literal(ExprLiteral::True);
        } else if self.match_token(&[TokenKind::FALSE]) {
            return Expr::Literal(ExprLiteral::False);
        } else if self.match_token(&[TokenKind::NIL]) {
            return Expr::Literal(ExprLiteral::Nil);
        } else if self.match_token(&[TokenKind::NUMBER]) {
            return Expr::Literal(ExprLiteral::Number(self.previous().literal.unwrap()));
        } else if self.match_token(&[TokenKind::STRING]) {
            return Expr::Literal(ExprLiteral::String(self.previous().literal.unwrap()));
        }

        if self.match_token(&[TokenKind::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenKind::RIGHT_PAREN, "Expect ')' after expression.");

            return Expr::Grouping { expr: expr.into() };
        }

        unreachable!("primary should not reach this point")
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
