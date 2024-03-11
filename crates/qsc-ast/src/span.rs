use derivative::Derivative;
use miette::SourceSpan;
use pest::Span;
use qsc_core::conv::IntoSourceSpan;

#[derive(
    Derivative, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[derivative(Debug)]
pub struct StaticSpan {
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub input: String,
    pub start: usize,
    pub end: usize,
}

impl StaticSpan {
    pub fn new(input: String, start: usize, end: usize) -> Self {
        Self { input, start, end }
    }

    pub fn as_str(&self) -> String {
        self.input[self.start..self.end].to_string()
    }
}

impl<'i> From<Span<'i>> for StaticSpan {
    fn from(value: Span<'i>) -> Self {
        Self {
            input: value.get_input().to_string(),
            start: value.start(),
            end: value.end(),
        }
    }
}

impl IntoSourceSpan for StaticSpan {
    fn into_source_span(&self) -> SourceSpan {
        (self.start, self.end - self.start).into()
    }
}
