use pest::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntNode<'i> {
    pub span: Span<'i>,
    pub value: i64,
}
