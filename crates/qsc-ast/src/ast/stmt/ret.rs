use crate::ast::node::Node;
use pest::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnNode<'i> {
    pub span: Span<'i>,
    pub value: Option<Node<'i>>,
}
