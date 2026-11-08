use crate::{grammar::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
    Expression {
        expr: Expr,
    },
    Print {
        expr: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch_opt: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
