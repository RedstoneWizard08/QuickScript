use crate::{ast::node::Node, span::StaticSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub negative: bool,
    pub span: StaticSpan,
    pub value: Node,
}
