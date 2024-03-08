use crate::{ast::node::Node, span::StaticSpan};

#[derive(Debug, Clone, PartialEq)]
pub struct CallNode {
    pub span: StaticSpan,
    pub func: String,
    pub args: Vec<CallArgument>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArgument {
    pub span: StaticSpan,
    pub value: Node,
}
