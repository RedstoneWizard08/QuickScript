use crate::{ast::node::Node, span::StaticSpan};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub negative: bool,
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: Node,
}
