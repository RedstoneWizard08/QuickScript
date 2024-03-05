use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
#[error("Lexer Error")]
#[diagnostic(
    code(qsc_lexer::error),
    url(docsrs),
    help("This is likely not a bug, but an issue with your code. Try changing things and see if it happens again.")
)]
pub struct LexerError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,
}
