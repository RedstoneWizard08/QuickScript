use pest::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolNode<'i> {
    pub span: Span<'i>,
    pub value: &'i str,
}
