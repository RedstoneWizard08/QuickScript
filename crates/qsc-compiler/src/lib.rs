#![feature(new_uninit)]

#[macro_use]
extern crate log;

pub mod error;

use std::sync::Arc;

use error::CompileError;
use qsc_ast::ast::AbstractTree;
use qsc_codegen::{simple::SimpleCompiler, unify::CodegenBackend};
use qsc_lexer::lexer::Lexer;
use qsc_object::ObjectProduct;
use qsc_processor::Processor;
use target_lexicon::Triple;

pub type Result<T> = std::result::Result<T, CompileError>;

pub struct Compiler<B: CodegenBackend> {
    pub ast: Arc<AbstractTree>,
    pub backend: Arc<SimpleCompiler<B>>,

    pub name: String,
    pub source: String,
}

impl<B: CodegenBackend> Compiler<B> {
    pub fn new(name: impl AsRef<str>, source: impl AsRef<str>) -> Self {
        unsafe {
            Self {
                ast: Arc::new_zeroed().assume_init(),
                backend: Arc::new_zeroed().assume_init(),

                name: name.as_ref().to_string(),
                source: source.as_ref().to_string(),
            }
        }
    }

    pub fn compile(&mut self, triple: Triple) -> Result<()> {
        debug!("[Stage 1/3] Running lexer...");

        let mut lexer = Lexer::new(&self.name, &self.source);
        let ast = lexer.lex()?;

        debug!("[Stage 2/3] Running processor...");

        let mut proc = Processor::new(ast);
        let ast = proc.process()?;

        debug!("[Stage 3/3] Compiling...");

        let mut backend =
            SimpleCompiler::<B>::new(triple, self.name.clone(), &self.source, ast.clone())?;

        backend.compile()?;

        self.ast = Arc::new(ast);
        self.backend = Arc::new(backend);

        Ok(())
    }

    pub fn ast(&self) -> Arc<AbstractTree> {
        self.ast.clone()
    }

    pub fn vcode(&self) -> String {
        let code = self.backend.vcode();

        code
    }

    pub fn clif(&self) -> Result<String> {
        let code = self.backend.clif();

        code.map_err(|v| v.into())
    }

    pub fn asm(self) -> Result<String> {
        let backend = unsafe { Arc::try_unwrap(self.backend).unwrap_unchecked() };
        let code = backend.asm();

        code.map_err(|v| v.into())
    }

    pub fn run(self) -> Result<i32> {
        let backend = unsafe { Arc::try_unwrap(self.backend).unwrap_unchecked() };
        let code = backend.run();

        code.map_err(|v| v.into())
    }

    pub fn finalize(self) -> ObjectProduct {
        let backend = unsafe { Arc::try_unwrap(self.backend).unwrap_unchecked() };
        let code = backend.finalize();

        code
    }
}
