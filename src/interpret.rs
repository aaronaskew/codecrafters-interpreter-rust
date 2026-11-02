use std::{fmt::Display, process::exit};

use thiserror::Error;

use crate::{
    Expr, RUNTIME_ERROR_EXIT_CODE, Stmt,
    token::{LiteralKind, Token, TokenKind},
};

#[derive(Debug)]
pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(statements: Vec<Stmt>) {
        for stmt in &statements {
            Self::execute(stmt);
        }
    }

    fn execute(stmt: &Stmt) {
        if let Err(e) = Self::visit_stmt(stmt) {
            eprintln!("{}", e);
            exit(RUNTIME_ERROR_EXIT_CODE);
        }
    }

    pub fn evaluate(expr: &Expr) -> Value {
        match Self::visit_expr(expr) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                exit(RUNTIME_ERROR_EXIT_CODE);
            }
        }
    }

    fn is_truthy(expr: &Expr) -> bool {
        match Self::visit_expr(expr) {
            Ok(v) => v.is_truthy(),
            Err(_) => false,
        }
    }

    fn check_number_operand(operator: &Token, expr: &Expr) -> Result<f64, RuntimeError> {
        if let Value::Number(num) = Self::visit_expr(expr)? {
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
        if let Ok(Value::Number(left)) = Self::visit_expr(left)
            && let Ok(Value::Number(right)) = Self::visit_expr(right)
        {
            Ok((left, right))
        } else {
            Err(RuntimeError::new("Operands must be numbers.", operator))
        }
    }

    fn visit_stmt(stmt: &Stmt) -> Result<(), RuntimeError> {
        match &stmt {
            Stmt::Expression { expr } => {
                Self::evaluate(expr);
            }
            Stmt::Print { expr } => {
                println!("{}", Self::evaluate(expr));
            }
        }

        Ok(())
    }

    fn visit_expr(expr: &Expr) -> Result<Value, RuntimeError> {
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
            } => match operator {
                Token {
                    kind: TokenKind::EQUAL_EQUAL,
                    ..
                } => {
                    let (left_val, right_val) = (Self::visit_expr(left)?, Self::visit_expr(right)?);
                    Ok(Value::Boolean(left_val == right_val))
                }
                Token {
                    kind: TokenKind::BANG_EQUAL,
                    ..
                } => {
                    let (left_val, right_val) = (Self::visit_expr(left)?, Self::visit_expr(right)?);
                    Ok(Value::Boolean(left_val != right_val))
                }
                Token {
                    kind: TokenKind::LESS,
                    ..
                } => {
                    let (left_val, right_val) = Self::check_number_operands(operator, left, right)?;
                    Ok(Value::Boolean(left_val < right_val))
                }
                Token {
                    kind: TokenKind::LESS_EQUAL,
                    ..
                } => {
                    let (left_val, right_val) = Self::check_number_operands(operator, left, right)?;
                    Ok(Value::Boolean(left_val <= right_val))
                }
                Token {
                    kind: TokenKind::GREATER,
                    ..
                } => {
                    let (left_val, right_val) = Self::check_number_operands(operator, left, right)?;
                    Ok(Value::Boolean(left_val > right_val))
                }
                Token {
                    kind: TokenKind::GREATER_EQUAL,
                    ..
                } => {
                    let (left_val, right_val) = Self::check_number_operands(operator, left, right)?;
                    Ok(Value::Boolean(left_val >= right_val))
                }
                Token {
                    kind: TokenKind::PLUS,
                    ..
                } => match (Self::visit_expr(left)?, Self::visit_expr(right)?) {
                    (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
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
                _ => Err(RuntimeError::new("Not a binary operator", operator)),
            },
            Expr::Grouping { expr } => Self::visit_expr(expr),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
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
        write!(f, "{}\n[line {}]", self.msg, self.token.line())
    }
}
