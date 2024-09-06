use crate::{block::Block, vis::Visibility};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<Argument>,
    pub ret: Option<String>,
    pub block: Block,
    pub vis: Visibility,
    pub external: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub typ: String,
    pub mutable: bool,
}
