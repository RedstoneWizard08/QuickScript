use crate::{block::Block, expr::Expr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conditional {
    pub cond: Box<Expr>,
    pub body: Block,
    pub elses: Vec<Else>,
    pub catch_all: Option<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Else {
    pub cond: Expr,
    pub body: Block,
}
