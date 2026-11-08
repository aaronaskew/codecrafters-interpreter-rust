use crate::token::{LiteralKind, Token};
use std::fmt::Display;

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
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
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
            Expr::Variable { .. } => todo!(),
            Expr::Assign { .. } => todo!(),
            Expr::Logical { .. } => todo!(),
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
                Expr::Unary { .. } => {
                    self.parenthesize()
                }
                Expr::Binary { .. } => self.parenthesize(),
                Expr::Grouping { .. } => self.parenthesize(),
                Expr::Variable { .. } => todo!(),
                Expr::Assign { .. } => todo!(),
                Expr::Logical { .. } => todo!(),
            }
        )
    }
}

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
