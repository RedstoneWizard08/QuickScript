use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CharNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: char,
}
