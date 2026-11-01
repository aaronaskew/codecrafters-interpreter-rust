use std::{fmt::Display, process::exit};

use thiserror::Error;

use crate::{
    Expr,
    token::{LiteralKind, Token, TokenKind},
};

#[derive(Debug)]
pub struct Interpreter {
    expr: Expr,
}

impl Interpreter {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn evaluate(&self) -> Value {
        match Self::visit(&self.expr) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                exit(70);
            }
        }
    }

    fn is_truthy(expr: &Expr) -> bool {
        match Self::visit(expr) {
            Ok(v) => v.is_truthy(),
            Err(_) => false,
        }
    }

    fn check_number_operand(operator: &Token, expr: &Expr) -> Result<f64, RuntimeError> {
        if let Value::Number(num) = Self::visit(expr)? {
            Ok(num)
        } else {
            Err(RuntimeError::new("Operand must be number.", operator))
        }
    }

    fn check_number_operands(
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<(f64, f64), RuntimeError> {
        if let Ok(Value::Number(left)) = Self::visit(left)
            && let Ok(Value::Number(right)) = Self::visit(right)
        {
            Ok((left, right))
        } else {
            Err(RuntimeError::new("Operands must be numbers.", operator))
        }
    }

    fn visit(expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(expr_literal) => match expr_literal {
                crate::ExprLiteral::Number(literal_kind) => {
                    let LiteralKind::Number(n) = literal_kind else {
                        panic!("must be number")
                    };

                    Ok(Value::Number(*n))
                }
                crate::ExprLiteral::String(literal_kind) => {
                    let LiteralKind::String(s) = literal_kind else {
                        panic!("must be string");
                    };

                    Ok(Value::String(s.into()))
                }
                crate::ExprLiteral::True => Ok(Value::Boolean(true)),
                crate::ExprLiteral::False => Ok(Value::Boolean(false)),
                crate::ExprLiteral::Nil => Ok(Value::Nil),
            },

            Expr::Unary { operator, right } => match operator {
                Token {
                    kind: TokenKind::MINUS,
                    ..
                } => Ok(Value::Number(-Self::check_number_operand(operator, right)?)),
                Token {
                    kind: TokenKind::BANG,
                    ..
                } => Ok(Value::Boolean(!Self::is_truthy(right))),
                _ => {
                    panic!()
                }
            },
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = Self::visit(left)?;
                let right_val = Self::visit(right)?;

                match operator {
                    Token {
                        kind: TokenKind::EQUAL_EQUAL,
                        ..
                    } => Ok(Value::Boolean(left_val == right_val)),
                    Token {
                        kind: TokenKind::BANG_EQUAL,
                        ..
                    } => Ok(Value::Boolean(left_val != right_val)),
                    Token {
                        kind: TokenKind::LESS,
                        ..
                    } => Ok(Value::Boolean(left_val < right_val)),
                    Token {
                        kind: TokenKind::LESS_EQUAL,
                        ..
                    } => Ok(Value::Boolean(left_val <= right_val)),
                    Token {
                        kind: TokenKind::GREATER,
                        ..
                    } => Ok(Value::Boolean(left_val > right_val)),
                    Token {
                        kind: TokenKind::GREATER_EQUAL,
                        ..
                    } => Ok(Value::Boolean(left_val >= right_val)),
                    Token {
                        kind: TokenKind::PLUS,
                        ..
                    } => match (left_val, right_val) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok(Value::Number(left + right))
                        }
                        (Value::String(left), Value::String(right)) => {
                            Ok(Value::String(format!("{}{}", left, right)))
                        }
                        _ => Err(RuntimeError::new(
                            "Operands must be two numbers or two strings.",
                            operator,
                        )),
                    },
                    Token {
                        kind: TokenKind::MINUS,
                        ..
                    } => {
                        let (left, right) = Self::check_number_operands(operator, left, right)?;

                        Ok(Value::Number(left - right))
                    }
                    Token {
                        kind: TokenKind::STAR,
                        ..
                    } => {
                        let (left, right) = Self::check_number_operands(operator, left, right)?;

                        Ok(Value::Number(left * right))
                    }
                    Token {
                        kind: TokenKind::SLASH,
                        ..
                    } => {
                        let (left, right) = Self::check_number_operands(operator, left, right)?;

                        Ok(Value::Number(left / right))
                    }
                    _ => panic!("not a binary operator"),
                }
            }
            Expr::Grouping { expr } => Self::visit(expr),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(s), Value::Number(o)) => Some(s.total_cmp(o)),
            _ => None,
        }
    }
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

    // fn check_number_operand(&self) -> Result<f64,RuntimeError> {
    //     let Value::Number(num) = self else {
    //         return Err(RuntimeError { msg: "Operand must be a number.", token: self. })
    //     };
    //
    //     Ok(*num)
    // }
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

#[derive(Debug, Error)]
struct RuntimeError {
    msg: String,
    token: Token,
}

impl RuntimeError {
    fn new(msg: &str, token: &Token) -> Self {
        Self {
            msg: msg.to_string(),
            token: token.clone(),
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n[line {}]", self.msg, self.token.line())
    }
}
