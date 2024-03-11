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
    pub error: Report,
}
