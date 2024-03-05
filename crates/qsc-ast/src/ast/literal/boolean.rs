use pest::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolNode<'i> {
    pub span: Span<'i>,
    pub value: bool,
}
