use crate::expr::Expr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub func: String,
    pub args: Vec<Expr>,
}
