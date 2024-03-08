use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharNode {
    pub span: StaticSpan,
    pub value: char,
}
