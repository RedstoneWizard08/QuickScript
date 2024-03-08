use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Lexical Error")]
#[diagnostic(code(qsc_ast::error), url(docsrs))]
pub struct LexicalError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,

    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}
