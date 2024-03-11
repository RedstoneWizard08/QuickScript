use crate::{
    ast::node::{block::Block, Node},
    span::StaticSpan,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub condition: Node,
    pub block: Block,
    pub else_block: Option<Block>,
}
