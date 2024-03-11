use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub value: f64,
}
