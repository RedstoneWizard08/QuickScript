use crate::expr::Expr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub mutable: bool,
    pub typ: Option<String>,
    pub value: Option<Box<Expr>>,
}
