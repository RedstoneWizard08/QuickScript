use crate::{
    ast::node::{ty::TypeNode, vis::Visibility, Node},
    span::StaticSpan,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalVariable {
    #[serde(skip)]
    pub span: StaticSpan,
    pub name: String,
    pub type_: TypeNode,
    pub value: Node,
    pub vis: Visibility,
}
