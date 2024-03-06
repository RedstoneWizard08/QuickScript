use crate::ast::node::Node;
use pest::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct CallNode<'i> {
    pub span: Span<'i>,
    pub func: &'i str,
    pub args: Vec<CallArgument<'i>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArgument<'i> {
    pub span: Span<'i>,
    pub value: Node<'i>,
}
