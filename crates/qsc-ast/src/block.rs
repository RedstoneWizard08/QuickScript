use crate::expr::Expr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub body: Vec<Expr>,
}

impl Block {
    pub fn empty() -> Block {
        Block { body: Vec::new() }
    }
}
