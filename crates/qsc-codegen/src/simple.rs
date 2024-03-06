use std::marker::PhantomData;

use anyhow::Result;
use cranelift_object::ObjectProduct;
use qsc_ast::ast::AbstractTree;
use target_lexicon::Triple;

use super::unify::CodegenBackend;

pub struct SimpleCompiler<'a, T: CodegenBackend<'a>> {
    pub backend: T,

    _pdata0: PhantomData<&'a ()>,
}

impl<'a, T: CodegenBackend<'a>> SimpleCompiler<'a, T> {
    pub fn new(triple: Triple) -> Result<Self> {
        Ok(Self {
            backend: T::new(triple)?,
            _pdata0: PhantomData,
        })
    }

    pub fn compile(&'a mut self, tree: AbstractTree<'a>) -> Result<()> {
        let mut funcs = Vec::new();

        for node in &tree.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(func) = decl.as_function() {
                    funcs.push(func);
                }
            }
        }

        self.backend.compile(tree)?;

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
