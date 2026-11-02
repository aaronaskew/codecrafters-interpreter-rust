use std::{fmt::Display, process::exit};

use thiserror::Error;

use crate::{
    RUNTIME_ERROR_EXIT_CODE,
    environment::Environment,
    grammar::{Expr, ExprLiteral, Stmt},
    token::{LiteralKind, Token, TokenKind},
};

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in &statements {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        if let Err(e) = self.visit_stmt(stmt) {
            eprintln!("{}", e);
            exit(RUNTIME_ERROR_EXIT_CODE);
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Value {
        match self.visit_expr(expr) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                exit(RUNTIME_ERROR_EXIT_CODE);
            }
        }
    }

    fn is_truthy(&mut self, expr: &Expr) -> bool {
        match self.visit_expr(expr) {
            Ok(v) => v.is_truthy(),
            Err(_) => false,
        }
    }

    fn check_number_operand(&mut self, operator: &Token, expr: &Expr) -> Result<f64, RuntimeError> {
        if let Value::Number(num) = self.visit_expr(expr)? {
            Ok(num)
        } else {
            Err(RuntimeError::new("Operand must be number.", operator))
        }
    }

    fn check_number_operands(
        &mut self,
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<(f64, f64), RuntimeError> {
        if let Ok(Value::Number(left)) = self.visit_expr(left)
            && let Ok(Value::Number(right)) = self.visit_expr(right)
        {
            Ok((left, right))
        } else {
            Err(RuntimeError::new("Operands must be numbers.", operator))
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match &stmt {
            Stmt::Expression { expr } => {
                self.evaluate(expr);
            }
            Stmt::Print { expr } => {
                println!("{}", self.evaluate(expr));
            }
            Stmt::Var { .. } => {
                self.visit_var_stmt(stmt)?;
            }
        }

        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        let Stmt::Var { name, initializer } = stmt else {
            panic!("should be a variable statement")
        };

        let value = initializer.as_ref().map(|expr| self.evaluate(expr));

        self.environment.define(name.lexeme.clone(), value);

        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(expr_literal) => match expr_literal {
                ExprLiteral::Number(literal_kind) => {
                    let LiteralKind::Number(n) = literal_kind else {
                        panic!("must be number")
                    };

                    Ok(Value::Number(*n))
                }
                ExprLiteral::String(literal_kind) => {
                    let LiteralKind::String(s) = literal_kind else {
                        panic!("must be string");
                    };

                    Ok(Value::String(s.into()))
                }
                ExprLiteral::True => Ok(Value::Boolean(true)),
                ExprLiteral::False => Ok(Value::Boolean(false)),
                ExprLiteral::Nil => Ok(Value::Nil),
            },

            Expr::Unary { operator, right } => match operator {
                Token {
                    kind: TokenKind::MINUS,
                    ..
                } => Ok(Value::Number(-self.check_number_operand(operator, right)?)),
                Token {
                    kind: TokenKind::BANG,
                    ..
                } => Ok(Value::Boolean(!self.is_truthy(right))),
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
                    let (left_val, right_val) = (self.visit_expr(left)?, self.visit_expr(right)?);
                    Ok(Value::Boolean(left_val == right_val))
                }
                Token {
                    kind: TokenKind::BANG_EQUAL,
                    ..
                } => {
                    let (left_val, right_val) = (self.visit_expr(left)?, self.visit_expr(right)?);
                    Ok(Value::Boolean(left_val != right_val))
                }
                Token {
                    kind: TokenKind::LESS,
                    ..
                } => {
                    let (left_val, right_val) =
                        self.check_number_operands(operator, left, right)?;
                    Ok(Value::Boolean(left_val < right_val))
                }
                Token {
                    kind: TokenKind::LESS_EQUAL,
                    ..
                } => {
                    let (left_val, right_val) =
                        self.check_number_operands(operator, left, right)?;
                    Ok(Value::Boolean(left_val <= right_val))
                }
                Token {
                    kind: TokenKind::GREATER,
                    ..
                } => {
                    let (left_val, right_val) =
                        self.check_number_operands(operator, left, right)?;
                    Ok(Value::Boolean(left_val > right_val))
                }
                Token {
                    kind: TokenKind::GREATER_EQUAL,
                    ..
                } => {
                    let (left_val, right_val) =
                        self.check_number_operands(operator, left, right)?;
                    Ok(Value::Boolean(left_val >= right_val))
                }
                Token {
                    kind: TokenKind::PLUS,
                    ..
                } => match (self.visit_expr(left)?, self.visit_expr(right)?) {
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
                    let (left, right) = self.check_number_operands(operator, left, right)?;

                    Ok(Value::Number(left - right))
                }
                Token {
                    kind: TokenKind::STAR,
                    ..
                } => {
                    let (left, right) = self.check_number_operands(operator, left, right)?;

                    Ok(Value::Number(left * right))
                }
                Token {
                    kind: TokenKind::SLASH,
                    ..
                } => {
                    let (left, right) = self.check_number_operands(operator, left, right)?;

                    Ok(Value::Number(left / right))
                }
                _ => Err(RuntimeError::new("Not a binary operator", operator)),
            },
            Expr::Grouping { expr } => self.visit_expr(expr),
            Expr::Variable { name } => self
                .environment
                .get(name)?
                .clone()
                .map_or(Ok(Value::Nil), Ok),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Clone)]
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
pub(crate) struct RuntimeError {
    msg: String,
    token: Token,
}

impl RuntimeError {
    pub(crate) fn new(msg: &str, token: &Token) -> Self {
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
