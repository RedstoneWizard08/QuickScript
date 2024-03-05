use pest::Span;

use crate::ast::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr<'i> {
    pub negative: bool,
    pub span: Span<'i>,
    pub value: Node<'i>,
}
