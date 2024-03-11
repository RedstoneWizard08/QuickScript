use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Processor Error")]
#[diagnostic(code(qsc_processor::error), url(docsrs))]
pub struct ProcessorError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,

    #[help]
    pub error: Report,
}
