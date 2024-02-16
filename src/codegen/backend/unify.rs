use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use anyhow::Result;
use cranelift_codegen::Context;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{DataDescription, DataId, FuncId, Linkage, Module};

use crate::{
    ast::var::FunctionData,
    codegen::{aot::AotGenerator, jit::JitGenerator},
};

pub trait BackendInternal<'a> {
    fn builder_ctx(&mut self) -> &mut FunctionBuilderContext;
    fn ctx(&mut self) -> &mut Context;
    fn data_desc(&mut self) -> &mut DataDescription;
    fn module(&mut self) -> &mut dyn Module;
    fn functions(&mut self) -> &mut HashMap<String, FunctionData>;
    fn builder(&mut self) -> Rc<RefCell<FunctionBuilder<'a>>>;
    fn new_builder(&mut self) -> Rc<RefCell<FunctionBuilder<'a>>>;
    fn globals(&mut self) -> &mut HashMap<String, DataId>;
    fn locals(&mut self) -> &mut HashMap<String, DataId>;
    fn reset_locals(&mut self);
    fn vars(&mut self) -> &mut HashMap<String, Variable>;
    fn finalize_builder(&mut self) -> Result<()>;
    fn complete_define_func(&mut self, id: FuncId) -> Result<()>;
    fn declare_func(&mut self, name: &str, linkage: Linkage) -> Result<FuncId>;

    fn post_define(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a> BackendInternal<'a> for AotGenerator<'a> {
    fn builder_ctx(&mut self) -> &mut FunctionBuilderContext {
        &mut self.builder_ctx
    }

    fn ctx(&mut self) -> &mut Context {
        &mut self.ctx
    }

    fn data_desc(&mut self) -> &mut DataDescription {
        &mut self.data_desc
    }

    fn module(&mut self) -> &mut dyn Module {
        &mut self.module
    }

    fn functions(&mut self) -> &mut HashMap<String, FunctionData> {
        &mut self.functions
    }

    fn builder(&mut self) -> Rc<RefCell<FunctionBuilder<'a>>> {
        self.builder.as_ref().unwrap().clone()
    }

    fn new_builder(&mut self) -> Rc<RefCell<FunctionBuilder<'a>>> {
        self.internal_new_builder();
        self.builder.as_ref().unwrap().clone()
    }

    fn globals(&mut self) -> &mut HashMap<String, DataId> {
        &mut self.globals
    }

    fn locals(&mut self) -> &mut HashMap<String, DataId> {
        &mut self.locals
    }

    fn reset_locals(&mut self) {
        self.locals.clear();
    }

    fn vars(&mut self) -> &mut HashMap<String, Variable> {
        &mut self.vars
    }

    fn finalize_builder(&mut self) -> Result<()> {
        self.internal_finalize_builder()
    }

    fn complete_define_func(&mut self, id: FuncId) -> Result<()> {
        self.internal_complete_define_func(id)
    }

    fn declare_func(&mut self, name: &str, linkage: Linkage) -> Result<FuncId> {
        self.internal_declare_func(name, linkage)
    }
}

impl<'a> BackendInternal<'a> for JitGenerator<'a> {
    fn builder_ctx(&mut self) -> &mut FunctionBuilderContext {
        &mut self.builder_ctx
    }

    fn ctx(&mut self) -> &mut Context {
        &mut self.ctx
    }

    fn data_desc(&mut self) -> &mut DataDescription {
        &mut self.data_desc
    }

    fn module(&mut self) -> &mut dyn Module {
        &mut self.module
    }

    fn functions(&mut self) -> &mut HashMap<String, FunctionData> {
        &mut self.functions
    }

    fn builder(&mut self) -> Rc<RefCell<FunctionBuilder<'a>>> {
        self.builder.as_ref().unwrap().clone()
    }

    fn new_builder(&mut self) -> Rc<RefCell<FunctionBuilder<'a>>> {
        self.internal_new_builder();
        self.builder.as_ref().unwrap().clone()
    }

    fn globals(&mut self) -> &mut HashMap<String, DataId> {
        &mut self.globals
    }

    fn locals(&mut self) -> &mut HashMap<String, DataId> {
        &mut self.locals
    }

    fn reset_locals(&mut self) {
        self.locals.clear();
    }

    fn vars(&mut self) -> &mut HashMap<String, Variable> {
        &mut self.vars
    }

    fn finalize_builder(&mut self) -> Result<()> {
        self.internal_finalize_builder()
    }

    fn complete_define_func(&mut self, id: FuncId) -> Result<()> {
        self.internal_complete_define_func(id)
    }

    fn declare_func(&mut self, name: &str, linkage: Linkage) -> Result<FuncId> {
        self.internal_declare_func(name, linkage)
    }

    fn post_define(&mut self) -> Result<()> {
        self.module.finalize_definitions()?;

        Ok(())
    }
}
