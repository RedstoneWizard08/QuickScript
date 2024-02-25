use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub type_: String,
    pub value: Option<Box<Expr>>,
    pub is_mutable: bool,
}
