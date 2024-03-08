use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntNode {
    pub span: StaticSpan,
    pub value: i64,
}
