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

use parking_lot::RwLock;
use qsc_ast::ast::{decl::func::FunctionNode, node::ty::TypeNode};

pub struct CodegenContext<'a, 'b> {
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, (Variable, Option<TypeNode<'a>>)>,
    pub values: HashMap<String, (Value, TypeNode<'a>)>,
    pub builder: &'b RwLock<FunctionBuilder<'a>>,
    pub ret: Option<TypeNode<'a>>,
    pub func: FunctionNode<'a>,
}

#[derive(Debug)]
pub struct DebugCodegenContext<'a> {
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, (Variable, Option<TypeNode<'a>>)>,
    pub values: HashMap<String, (Value, TypeNode<'a>)>,
    pub ret: Option<TypeNode<'a>>,
    pub func: FunctionNode<'a>,
}

impl<'a, 'b> Into<DebugCodegenContext<'a>> for &CodegenContext<'a, 'b> {
    fn into(self) -> DebugCodegenContext<'a> {
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
        let me: DebugCodegenContext<'a> = self.into();

        me.fmt(f)
    }
}

pub struct CompilerContext<'a, M: Module> {
    pub ctx: Context,
    pub data_desc: DataDescription,
    pub module: M,
    pub functions: HashMap<String, FunctionNode<'a>>,
    pub globals: HashMap<String, DataId>,
    pub fns: Vec<Function>,
    pub vcode: Vec<CompiledCode>,
    pub code: Arc<RwLock<HashMap<String, (String, *const u8, usize)>>>,
}
