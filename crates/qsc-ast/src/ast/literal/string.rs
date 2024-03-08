use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringNode {
    pub span: StaticSpan,
    pub value: String,
}
