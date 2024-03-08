use crate::{ast::node::Node, span::StaticSpan};

use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub span: StaticSpan,
    pub operator: Operator,
    pub rhs: Node,
    pub lhs: Node,
}
