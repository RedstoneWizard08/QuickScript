use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use qsc_ast::error::LexicalError;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Processing Error")]
#[diagnostic(code(qsc_processor::error::internal), url(docsrs))]
pub struct ProcessingError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,

    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Processor Error")]
#[diagnostic(code(qsc_processor::error), url(docsrs))]
pub enum ProcessorError {
    Processor(#[from] ProcessingError),
    Ast(#[from] LexicalError),
    Unknown,
}
