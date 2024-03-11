use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: String,
}
