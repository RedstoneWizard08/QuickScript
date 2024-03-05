use pest::Span;

use crate::ast::node::Node;

use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr<'i> {
    pub span: Span<'i>,
    pub operator: Operator,
    pub rhs: Node<'i>,
    pub lhs: Node<'i>,
}
