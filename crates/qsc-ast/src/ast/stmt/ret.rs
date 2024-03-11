use crate::{ast::node::Node, span::StaticSpan};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: Option<Node>,
}
