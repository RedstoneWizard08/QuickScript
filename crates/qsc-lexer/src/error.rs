use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Lexer Error")]
#[diagnostic(code(qsc_lexer::error), url(docsrs))]
pub struct LexerError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,

    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}
