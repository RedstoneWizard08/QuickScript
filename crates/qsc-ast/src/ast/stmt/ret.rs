use crate::{ast::node::Node, span::StaticSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnNode {
    pub span: StaticSpan,
    pub value: Option<Node>,
}
