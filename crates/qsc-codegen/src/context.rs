use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::Arc,
};

use cranelift_codegen::{
    ir::{Function, Value},
    CompiledCode, Context,
};
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_module::{DataDescription, DataId, Module};

use miette::NamedSource;
use parking_lot::RwLock;
use qsc_ast::ast::{decl::func::FunctionNode, node::ty::TypeNode, AbstractTree};

pub struct CodegenContext<'a, 'b> {
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, (Variable, Option<TypeNode>)>,
    pub values: HashMap<String, (Value, TypeNode)>,
    pub builder: &'b RwLock<FunctionBuilder<'a>>,
    pub ret: Option<TypeNode>,
    pub func: FunctionNode,
}

#[derive(Debug)]
pub struct DebugCodegenContext {
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, (Variable, Option<TypeNode>)>,
    pub values: HashMap<String, (Value, TypeNode)>,
    pub ret: Option<TypeNode>,
    pub func: FunctionNode,
}

impl<'a, 'b> Into<DebugCodegenContext> for &CodegenContext<'a, 'b> {
    fn into(self) -> DebugCodegenContext {
        DebugCodegenContext {
            locals: self.locals.clone(),
            vars: self.vars.clone(),
            values: self.values.clone(),
            ret: self.ret.clone(),
            func: self.func.clone(),
        }
    }
}

impl<'a, 'b> Debug for CodegenContext<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let me: DebugCodegenContext = self.into();

        me.fmt(f)
    }
}

pub struct CompilerContext<M: Module> {
    pub ctx: Context,
    pub data_desc: DataDescription,
    pub module: M,
    pub functions: HashMap<String, FunctionNode>,
    pub globals: HashMap<String, DataId>,
    pub fns: Vec<Function>,
    pub vcode: Vec<CompiledCode>,
    pub code: Arc<RwLock<HashMap<String, (String, *const u8, usize)>>>,
    pub source: NamedSource<String>,
    pub tree: AbstractTree,
}
