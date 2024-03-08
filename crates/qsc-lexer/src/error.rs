use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use pest::error::Error;
use thiserror::Error;

use crate::parser::Rule;

#[derive(Debug, Error, Diagnostic)]
#[error("Parsing Error")]
#[diagnostic(code(qsc_lexer::error::parsing), url(docsrs))]
pub struct TransformerError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,

    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Lexer Error")]
#[diagnostic(code(qsc_lexer::error), url(docsrs))]
pub enum LexerError {
    Parser(#[from] Error<Rule>),
    Transformer(#[from] TransformerError),
}
