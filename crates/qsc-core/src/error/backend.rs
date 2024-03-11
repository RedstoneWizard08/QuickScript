use miette::{Diagnostic, Report};
use thiserror::Error;

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
