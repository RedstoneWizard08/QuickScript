use pest::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharNode<'i> {
    pub span: Span<'i>,
    pub value: char,
}
