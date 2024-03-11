use crate::{
    ast::node::{ty::TypeNode, Node},
    span::StaticSpan,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub name: String,
    pub type_: Option<TypeNode>,
    pub value: Option<Node>,
    pub mutable: bool,
}
