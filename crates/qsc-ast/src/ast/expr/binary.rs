use crate::{ast::node::Node, span::StaticSpan};

use super::operator::Operator;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
    #[serde(skip)]
    pub span: StaticSpan,
    pub operator: Operator,
    pub rhs: Node,
    pub lhs: Node,
}
