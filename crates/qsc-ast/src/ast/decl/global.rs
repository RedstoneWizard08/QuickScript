use crate::ast::node::{ty::TypeNode, vis::Visibility, Node};
use pest::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariable<'i> {
    pub span: Span<'i>,
    pub name: String,
    pub type_: TypeNode<'i>,
    pub value: Node<'i>,
    pub vis: Visibility,
}
