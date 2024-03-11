use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: String,
}
