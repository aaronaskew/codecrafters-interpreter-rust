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

            Expr::Unary { operator, right } => {
                let right_val = self.visit(right);

                match operator {
                    crate::Operator::Minus => {
                        let Value::Number(n) = right_val else {
                            panic!()
                        };

                        Value::Number(-n)
                    }
                    crate::Operator::Not => Value::Boolean(!right_val.is_truthy()),
                    _ => {
                        panic!()
                    }
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.visit(left);
                let right_val = self.visit(right);

                match operator {
                    crate::Operator::Equivalent => todo!(),
                    crate::Operator::NotEquivalent => todo!(),
                    crate::Operator::LessThan => todo!(),
                    crate::Operator::LessThanOrEquals => todo!(),
                    crate::Operator::GreaterThan => todo!(),
                    crate::Operator::GreaterThanOrEquals => todo!(),
                    crate::Operator::Plus => match (left_val, right_val) {
                        (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                        (Value::String(left), Value::String(right)) => {
                            Value::String(format!("{}{}", left, right))
                        }
                        _ => {
                            panic!();
                        }
                    },
                    crate::Operator::Minus => {
                        let Value::Number(left) = left_val else {
                            panic!()
                        };
                        let Value::Number(right) = right_val else {
                            panic!()
                        };

                        Value::Number(left - right)
                    }
                    crate::Operator::Multiply => {
                        let Value::Number(left) = left_val else {
                            panic!()
                        };
                        let Value::Number(right) = right_val else {
                            panic!()
                        };

                        Value::Number(left * right)
                    }
                    crate::Operator::Divide => {
                        let Value::Number(left) = left_val else {
                            panic!()
                        };
                        let Value::Number(right) = right_val else {
                            panic!()
                        };

                        Value::Number(left / right)
                    }
                    crate::Operator::Not => todo!(),
                }
            }
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
impl Value {
    fn is_truthy(&self) -> bool {
        if matches!(self, Value::Nil) {
            return false;
        }

        if let Value::Boolean(b) = self {
            return *b;
        }

        true
    }
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
