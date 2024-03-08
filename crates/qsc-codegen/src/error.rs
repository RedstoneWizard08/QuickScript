use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Codegen Error")]
#[diagnostic(code(qsc_codegen::error), url(docsrs))]
pub struct CodegenError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,

    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Backend Error")]
#[diagnostic(code(qsc_codegen::backend::error), url(docsrs))]
pub struct BackendError {
    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}

impl From<Report> for BackendError {
    fn from(value: Report) -> Self {
        Self { error: value }
    }
}
