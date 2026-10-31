use std::fmt::Display;

use crate::Expr;

#[derive(Debug)]
pub struct Interpreter {
    expr: Expr,
}

impl Interpreter {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn evaluate(&self) -> Value {
        match &self.expr {
            Expr::Literal(expr_literal) => match expr_literal {
                crate::ExprLiteral::Number(literal_kind) => todo!(),
                crate::ExprLiteral::String(literal_kind) => todo!(),
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
            Expr::Grouping { expr } => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Boolean(b) => b.to_string(),
                Value::Nil => "nil".to_string(),
            }
        )
    }
}
