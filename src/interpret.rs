use std::fmt::Display;

use crate::{Expr, Operator, token::LiteralKind};

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
                    Operator::Minus => {
                        let Value::Number(n) = right_val else {
                            panic!()
                        };

                        Value::Number(-n)
                    }
                    Operator::Not => Value::Boolean(!right_val.is_truthy()),
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
                    Operator::Equivalent => todo!(),
                    Operator::NotEquivalent => todo!(),
                    Operator::LessThan
                    | Operator::LessThanOrEquals
                    | Operator::GreaterThan
                    | Operator::GreaterThanOrEquals => {
                        let Value::Number(left) = left_val else {
                            panic!()
                        };
                        let Value::Number(right) = right_val else {
                            panic!()
                        };

                        Value::Boolean(match operator {
                            Operator::LessThan => left < right,
                            Operator::LessThanOrEquals => left <= right,
                            Operator::GreaterThan => left > right,
                            Operator::GreaterThanOrEquals => left >= right,
                            _ => {
                                panic!()
                            }
                        })
                    }
                    Operator::Plus => match (left_val, right_val) {
                        (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                        (Value::String(left), Value::String(right)) => {
                            Value::String(format!("{}{}", left, right))
                        }
                        _ => {
                            panic!();
                        }
                    },
                    Operator::Minus => {
                        let Value::Number(left) = left_val else {
                            panic!()
                        };
                        let Value::Number(right) = right_val else {
                            panic!()
                        };

                        Value::Number(left - right)
                    }
                    Operator::Multiply => {
                        let Value::Number(left) = left_val else {
                            panic!()
                        };
                        let Value::Number(right) = right_val else {
                            panic!()
                        };

                        Value::Number(left * right)
                    }
                    Operator::Divide => {
                        let Value::Number(left) = left_val else {
                            panic!()
                        };
                        let Value::Number(right) = right_val else {
                            panic!()
                        };

                        Value::Number(left / right)
                    }
                    Operator::Not => todo!(),
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
