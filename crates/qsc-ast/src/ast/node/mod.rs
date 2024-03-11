pub mod block;
pub mod data;
pub mod sym;
pub mod ty;
pub mod vis;

use crate::span::StaticSpan;

use self::data::NodeData;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    #[serde(skip)]
    pub span: StaticSpan,
    pub data: Box<NodeData>,
}
