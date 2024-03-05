use pest::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringNode<'i> {
    pub span: Span<'i>,
    pub value: String,
}
