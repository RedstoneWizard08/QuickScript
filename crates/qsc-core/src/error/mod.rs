pub mod backend;
pub mod codegen;
pub mod compiler;
pub mod lexer;
pub mod lexical;
pub mod processor;

use miette::{Diagnostic, Report};
use thiserror::Error;

use self::{
    backend::BackendError, codegen::CodegenError, compiler::CompilerError, lexer::LexerError,
    lexical::LexicalError, processor::ProcessorError,
};

pub type Result<T, E = CompileError> = std::result::Result<T, E>;

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown Error")]
#[diagnostic(code(qsc_core::error::unknown), url(docsrs))]
pub struct UnknownError {
    #[help]
    #[diagnostic(transparent)]
    pub error: Report,
}

#[derive(Debug, Error, Diagnostic)]
pub enum CompileError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Ast(#[from] LexicalError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Lexer(#[from] LexerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Processor(#[from] ProcessorError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Codegen(#[from] CodegenError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Compiler(#[from] CompilerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Backend(#[from] BackendError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Unknown(#[help] UnknownError),
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
