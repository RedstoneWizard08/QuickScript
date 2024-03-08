use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolNode {
    pub span: StaticSpan,
    pub value: bool,
}
