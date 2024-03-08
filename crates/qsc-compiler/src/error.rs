use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use qsc_ast::error::LexicalError;
use qsc_codegen::error::{BackendError, CodegenError};
use qsc_lexer::error::LexerError;
use qsc_processor::error::ProcessorError;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Compiler Error")]
#[diagnostic(code(qsc_compiler::error::internal), url(docsrs))]
pub struct CompilerError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub location: SourceSpan,

    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown Error")]
#[diagnostic(code(qsc_compiler::error::unknown), url(docsrs))]
pub struct UnknownError {
    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Compiler Error")]
#[diagnostic(code(qsc_compiler::error), url(docsrs))]
pub enum CompileError {
    Ast(#[from] LexicalError),
    Lexer(#[from] LexerError),
    Processor(#[from] ProcessorError),
    Codegen(#[from] CodegenError),
    Compiler(#[from] CompilerError),
    Backend(#[from] BackendError),
    Unknown(#[from] UnknownError),
}

impl From<Report> for UnknownError {
    fn from(value: Report) -> Self {
        Self { error: value }
    }
}

impl From<Report> for CompileError {
    fn from(value: Report) -> Self {
        Self::Unknown(value.into())
    }
}
