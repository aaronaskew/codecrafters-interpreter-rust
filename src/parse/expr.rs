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

use std::fmt::Display;

use crate::token::{LiteralKind, Token};

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

// #[derive(Debug)]
// pub enum Operator {
//     Equivalent,
//     NotEquivalent,
//     LessThan,
//     LessThanOrEquals,
//     GreaterThan,
//     GreaterThanOrEquals,
//     Plus,
//     Minus,
//     Multiply,
//     Divide,
//     Not,
// }
//
// impl From<Token> for Operator {
//     fn from(token: Token) -> Self {
//         match token.kind {
//             TokenKind::EQUAL_EQUAL => Self::Equivalent,
//             TokenKind::BANG_EQUAL => Self::NotEquivalent,
//             TokenKind::LESS => Self::LessThan,
//             TokenKind::LESS_EQUAL => Self::LessThanOrEquals,
//             TokenKind::GREATER => Self::GreaterThan,
//             TokenKind::GREATER_EQUAL => Self::GreaterThanOrEquals,
//             TokenKind::PLUS => Self::Plus,
//             TokenKind::MINUS => Self::Minus,
//             TokenKind::STAR => Self::Multiply,
//             TokenKind::SLASH => Self::Divide,
//             TokenKind::BANG => Self::Not,
//             _ => unreachable!("Not an operator"),
//         }
//     }
// }

// impl Display for Operator {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Token{kind: TokenKind::} => "==",
//                 Token{kind: TokenKind::} => "!=",
//                 Token{kind: TokenKind::} => "<",
//                 Token{kind: TokenKind::} => "<=",
//                 Token{kind: TokenKind::} => ">",
//                 Token{kind: TokenKind::} => ">=",
//                 Token{kind: TokenKind::} => "+",
//                 Token{kind: TokenKind::} => "-",
//                 Token{kind: TokenKind::} => "*",
//                 Token{kind: TokenKind::} => "/",
//                 Token{kind: TokenKind::} => "!",
//             }
//         )
//     }
// }

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
    pub fn print_ast(&self) {
        println!("{self}");
    }

    fn parenthesize(&self) -> String {
        (match self {
            Expr::Literal(expr_literal) => expr_literal.to_string(),
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.parenthesize())
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.parenthesize(),
                right.parenthesize()
            ),
            Expr::Grouping { expr } => format!("(group {})", expr.parenthesize()),
        })
        .to_string()
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expr::Literal(literal) => literal.to_string(),
                Expr::Unary {
                    operator: _,
                    right: _,
                } => {
                    self.parenthesize()
                }
                Expr::Binary {
                    left: _,
                    operator: _,
                    right: _,
                } => self.parenthesize(),
                Expr::Grouping { expr: _ } => self.parenthesize(),
            }
        )
    }
}
