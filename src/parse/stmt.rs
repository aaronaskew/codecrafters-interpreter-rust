use crate::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression { expr: Expr },
    Print { expr: Expr },
}
