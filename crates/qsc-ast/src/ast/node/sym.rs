use crate::span::StaticSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolNode {
    pub span: StaticSpan,
    pub value: String,
}
