use qsc_ast::expr::{Expr, ExprKind};
use anyhow::Result;
use cranelift_object::ObjectProduct;
use target_lexicon::Triple;

use super::unify::CodegenBackend;

pub struct SimpleCompiler<T: CodegenBackend> {
    pub backend: T,
}

impl<T: CodegenBackend> SimpleCompiler<T> {
    pub fn new(triple: Triple) -> Result<Self> {
        Ok(Self {
            backend: T::new(triple)?,
        })
    }

    pub fn compile(&mut self, exprs: Vec<Expr>) -> Result<()> {
        let mut funcs = Vec::new();

        for expr in exprs {
            if let ExprKind::Function(func) = expr.content {
                funcs.push(func);
            }
        }

        self.backend.compile(funcs)?;

        Ok(())
    }

    pub fn is_jit(&self) -> bool {
        self.backend.is_jit()
    }

    pub fn run(&self) -> Result<i32> {
        self.backend.run()
    }

    pub fn finalize(self) -> ObjectProduct {
        self.backend.finalize()
    }

    pub fn clif(&self) -> Result<String> {
        self.backend.clif()
    }

    pub fn vcode(&self) -> String {
        self.backend.vcode()
    }

    pub fn asm(self) -> Result<String> {
        self.backend.asm()
    }
}
