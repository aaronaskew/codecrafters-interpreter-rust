use std::fmt::Display;

use crate::{Expr, token::LiteralKind};

#[derive(Debug)]
pub struct Interpreter {
    expr: Expr,
}

impl Interpreter {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn evaluate(&self) -> Value {
        self.visit(&self.expr)
    }

    fn visit(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(expr_literal) => match expr_literal {
                crate::ExprLiteral::Number(literal_kind) => {
                    let LiteralKind::Number(n) = literal_kind else {
                        panic!("must be number")
                    };

                    Value::Number(*n)
                }
                crate::ExprLiteral::String(literal_kind) => {
                    let LiteralKind::String(s) = literal_kind else {
                        panic!("must be string");
                    };

                    Value::String(s.into())
                }
                crate::ExprLiteral::True => Value::Boolean(true),
                crate::ExprLiteral::False => Value::Boolean(false),
                crate::ExprLiteral::Nil => Value::Nil,
            },

            Expr::Unary { operator, right } => todo!(),
            Expr::Binary {
                left,
                operator,
                right,
            } => todo!(),
            Expr::Grouping { expr } => self.visit(expr),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Boolean(b) => b.to_string(),
                Value::Nil => "nil".to_string(),
                Value::Number(n) => n.to_string(),
                Value::String(s) => s.into(),
            }
        )
    }
}
