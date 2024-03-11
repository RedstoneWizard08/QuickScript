use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: i64,
}
