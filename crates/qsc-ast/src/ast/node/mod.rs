pub mod block;
pub mod data;
pub mod sym;
pub mod ty;
pub mod vis;

use pest::Span;

use self::data::NodeData;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'i> {
    pub span: Span<'i>,
    pub data: Box<NodeData<'i>>,
}
