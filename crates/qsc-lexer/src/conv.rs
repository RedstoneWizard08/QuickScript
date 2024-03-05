use miette::SourceSpan;
use pest::Span;

pub trait IntoSourceSpan {
    fn into_source_span(&self) -> SourceSpan;
}

impl<'i> IntoSourceSpan for Span<'i> {
    fn into_source_span(&self) -> SourceSpan {
        (self.start(), self.end() - self.start()).into()
    }
}
