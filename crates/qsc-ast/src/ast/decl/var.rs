use pest::Span;

use crate::ast::node::{ty::TypeNode, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableNode<'i> {
    pub span: Span<'i>,
    pub name: String,
    pub type_: Option<TypeNode<'i>>,
    pub value: Option<Node<'i>>,
    pub mutable: bool,
}
