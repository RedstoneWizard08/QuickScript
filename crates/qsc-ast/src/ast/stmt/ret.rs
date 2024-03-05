use pest::Span;
use crate::ast::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnNode<'i> {
    pub span: Span<'i>,
    pub value: Option<Node<'i>>,
}
