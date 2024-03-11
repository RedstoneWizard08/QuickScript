use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BoolNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: bool,
}
