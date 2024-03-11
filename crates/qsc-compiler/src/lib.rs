#[macro_use]
extern crate log;

use miette::IntoDiagnostic;
use parking_lot::RwLock;
use qsc_ast::ast::AbstractTree;
use qsc_codegen::{simple::SimpleCompiler, unify::CodegenBackend};
use qsc_core::error::Result;
use qsc_lexer::lexer::Lexer;
use qsc_object::ObjectProduct;
use qsc_processor::Processor;
use ron::ser::PrettyConfig;
use target_lexicon::Triple;

pub struct Compiler<B: CodegenBackend> {
    pub ast: RwLock<AbstractTree>,
    pub backend: RwLock<SimpleCompiler<B>>,

    pub name: String,
    pub source: String,
}

impl<B: CodegenBackend> Compiler<B> {
    pub fn compile(name: impl AsRef<str>, source: impl AsRef<str>, triple: Triple) -> Result<Self> {
        debug!("[Stage 1/3] Running lexer...");

        let mut lexer = Lexer::new(&name, &source);
        let ast = lexer.lex()?;

        debug!("[Stage 2/3] Running processor...");

        let mut proc = Processor::new(ast);
        let ast = proc.process()?;

        debug!("[Stage 3/3] Compiling...");

        let mut backend = SimpleCompiler::<B>::new(
            triple,
            name.as_ref().to_string(),
            &source.as_ref().to_string(),
            ast.clone(),
        )?;

        backend.compile()?;

        Ok(Self {
            ast: RwLock::new(ast),
            backend: RwLock::new(backend),

            name: name.as_ref().to_string(),
            source: source.as_ref().to_string(),
        })
    }

    pub fn dump_pst(name: impl AsRef<str>, source: impl AsRef<str>) -> Result<String> {
        debug!("[Stage 1/2] Running lexer...");

        let mut lexer = Lexer::new(name, source);
        let ast = lexer.lex()?;

        debug!("[Stage 2/2] Running processor...");

        let mut proc = Processor::new(ast);
        let ast = proc.process()?;

        ron::ser::to_string_pretty(&ast, PrettyConfig::default())
            .into_diagnostic()
            .map_err(|v| v.into())
    }

    pub fn dump_ast(name: impl AsRef<str>, source: impl AsRef<str>) -> Result<String> {
        debug!("[1/1] Running lexer...");

        let mut lexer = Lexer::new(name, source);
        let ast = lexer.lex()?;

        ron::ser::to_string_pretty(&ast, PrettyConfig::default())
            .into_diagnostic()
            .map_err(|v| v.into())
    }

    pub fn ast(&self) -> AbstractTree {
        self.ast.read().clone() as AbstractTree
    }

    pub fn vcode(&self) -> String {
        let code = self.backend.read().vcode();

        code
    }

    pub fn clif(&self) -> Result<String> {
        let code = self.backend.read().clif();

        code.map_err(|v| v.into())
    }

    pub fn asm(self) -> Result<String> {
        let code = RwLock::into_inner(self.backend).asm();

        code.map_err(|v| v.into())
    }

    pub fn run(self) -> Result<i32> {
        let code = RwLock::into_inner(self.backend).run();

        code.map_err(|v| v.into())
    }

    pub fn finalize(self) -> ObjectProduct {
        let code = RwLock::into_inner(self.backend).finalize();

        code
    }
}
