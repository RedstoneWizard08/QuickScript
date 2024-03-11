use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Compiler Error")]
#[diagnostic(code(qsc_compiler::error), url(docsrs))]
pub struct CompilerError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,

    #[help]
    pub error: Report,
}
