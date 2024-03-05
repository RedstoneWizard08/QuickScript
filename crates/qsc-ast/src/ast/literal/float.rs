use pest::Span;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatNode<'i> {
    pub span: Span<'i>,
    pub value: f64,
}
