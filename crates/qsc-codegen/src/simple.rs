use miette::Result;
use qsc_ast::ast::AbstractTree;
use qsc_core::error::backend::BackendError;
use qsc_object::ObjectProduct;
use target_lexicon::Triple;

use super::unify::CodegenBackend;

pub struct SimpleCompiler<T: CodegenBackend> {
    pub backend: T,
}

impl<T: CodegenBackend> SimpleCompiler<T> {
    pub fn new(
        triple: Triple,
        name: String,
        source: &String,
        tree: AbstractTree,
    ) -> Result<Self, BackendError> {
        Ok(Self {
            backend: T::new(triple, name, source.clone(), tree)?,
        })
    }

    pub fn compile(&mut self) -> Result<(), BackendError> {
        let tree = self.backend.tree();
        let mut funcs = Vec::new();

        for node in &tree.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(func) = decl.as_function() {
                    funcs.push(func);
                }
            }
        }

        self.backend.compile()?;

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
