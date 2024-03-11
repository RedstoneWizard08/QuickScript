use crate::{ast::node::Node, span::StaticSpan};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub func: String,
    pub args: Vec<CallArgument>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallArgument {
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: Node,
}
