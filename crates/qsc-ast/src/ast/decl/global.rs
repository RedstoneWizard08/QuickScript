use crate::{
    ast::node::{ty::TypeNode, vis::Visibility, Node},
    span::StaticSpan,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariable {
    pub span: StaticSpan,
    pub name: String,
    pub type_: TypeNode,
    pub value: Node,
    pub vis: Visibility,
}
