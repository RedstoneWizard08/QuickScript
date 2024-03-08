use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq)]
pub struct FloatNode {
    pub span: StaticSpan,
    pub value: f64,
}
